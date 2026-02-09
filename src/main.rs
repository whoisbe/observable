//! Stream EEG brainwaves from a Neurosity Crown.
//!
//! Authenticates with Firebase (email/password), connects to the device via
//! Firebase Realtime Database, subscribes to the "brainwaves" metric (raw),
//! and prints incoming JSON to the console.

use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;
use tokio_stream::StreamExt;

/// Firebase Identity Toolkit sign-in response.
#[derive(Debug, Deserialize)]
struct FirebaseAuthResponse {
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: Option<String>,
    #[serde(rename = "expiresIn")]
    expires_in: Option<String>,
}

/// Load env vars and return (device_id, email, password).
fn load_env() -> Result<(String, String, String), String> {
    dotenvy::dotenv().ok();
    let device_id = std::env::var("DEVICE_ID").map_err(|_| "DEVICE_ID not set in .env")?;
    let email = std::env::var("EMAIL").map_err(|_| "EMAIL not set in .env")?;
    let password = std::env::var("PASSWORD").map_err(|_| "PASSWORD not set in .env")?;
    if device_id.is_empty() || email.is_empty() || password.is_empty() {
        return Err("DEVICE_ID, EMAIL and PASSWORD must be non-empty in .env".into());
    }
    Ok((device_id, email, password))
}

/// Neurosity uses Firebase Auth. Default project config from Neurosity SDK.
const FIREBASE_API_KEY: &str = "AIzaSyB0TkZ83Fj0CIzn8AAmE-Osc92s3ER8hy8";
const FIREBASE_DATABASE_URL: &str = "https://neurosity-device.firebaseio.com";

/// Sign in with email/password and return the ID token.
async fn firebase_login(
    client: &Client,
    email: &str,
    password: &str,
    api_key: &str,
) -> Result<String, String> {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        api_key
    );
    let body = serde_json::json!({
        "email": email,
        "password": password,
        "returnSecureToken": true
    });
    let res = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Login request failed: {}", e))?;
    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Login failed ({}): {}", status, text));
    }
    let auth: FirebaseAuthResponse = res
        .json()
        .await
        .map_err(|e| format!("Invalid login response: {}", e))?;
    Ok(auth.id_token)
}

/// Create a subscription so the device streams brainwaves; then stream raw JSON from RTDB.
async fn stream_brainwaves(
    client: &Client,
    device_id: &str,
    id_token: &str,
) -> Result<(), String> {
    let base = FIREBASE_DATABASE_URL.trim_end_matches('/');
    let device_path = format!("{}/devices/{}", base, device_id);

    // 1) Push to subscriptions to get a key we use as clientId
    let push_url = format!("{}/subscriptions.json?auth={}", device_path, id_token);
    let push_res = client
        .post(&push_url)
        .body("null")
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Push subscription key failed: {}", e))?;
    if !push_res.status().is_success() {
        let text = push_res.text().await.unwrap_or_default();
        return Err(format!("Push subscription key failed: {}", text));
    }
    let push_body: serde_json::Value = push_res
        .json()
        .await
        .map_err(|e| format!("Invalid push response: {}", e))?;
    let client_id = push_body
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or("No subscription key in response")?;

    // 2) Set client timestamp so the backend knows we're connected
    let client_url = format!("{}/clients/{}.json?auth={}", device_path, client_id, id_token);
    let timestamp_body = serde_json::json!({ ".sv": "timestamp" });
    client
        .put(&client_url)
        .json(&timestamp_body)
        .send()
        .await
        .map_err(|e| format!("Set client timestamp failed: {}", e))?;

    // 3) Create brainwaves subscription (metric: brainwaves, label: raw)
    let sub_url = format!("{}/subscriptions.json?auth={}", device_path, id_token);
    let sub_body = serde_json::json!({
        "clientId": client_id,
        "metric": "brainwaves",
        "labels": ["raw"],
        "atomic": false
    });
    let sub_res = client
        .post(&sub_url)
        .json(&sub_body)
        .send()
        .await
        .map_err(|e| format!("Create subscription failed: {}", e))?;
    if !sub_res.status().is_success() {
        let text = sub_res.text().await.unwrap_or_default();
        return Err(format!("Create subscription failed: {}", text));
    }

    // 4) Stream from metrics/brainwaves/raw via SSE
    // Use a client with gzip disabled so chunk boundaries don't break decompression ("error decoding response body").
    let stream_client = Client::builder()
        .timeout(Duration::from_secs(30))
        .no_gzip()
        .no_deflate()
        .no_brotli()
        .build()
        .map_err(|e| format!("Stream HTTP client build failed: {}", e))?;
    let stream_url = format!(
        "{}/metrics/brainwaves/raw.json?auth={}",
        device_path, id_token
    );
    println!("Streaming brainwaves from device {} (Ctrl+C to stop)…\n", device_id);

    let res = stream_client
        .get(&stream_url)
        .header("Accept", "text/event-stream")
        .send()
        .await
        .map_err(|e| format!("Stream request failed: {}", e))?;
    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Stream failed ({}): {}", status, text));
    }

    let mut stream = res.bytes_stream();
    let mut buf = Vec::<u8>::new();
    let mut data_buf = String::new();
    let mut chunk_count: u64 = 0;
    while let Some(chunk_result) = stream.next().await {
        chunk_count += 1;
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => {
                let mut msg = format!("Stream chunk #{} failed: {}", chunk_count, e);
                if let Some(source) = e.source() {
                    msg.push_str(&format!("\n  Caused by: {}", source));
                    let mut s: &dyn std::error::Error = source;
                    while let Some(next) = s.source() {
                        msg.push_str(&format!("\n    {}", next));
                        s = next;
                    }
                }
                msg.push_str(&format!(
                    "\n  Buffer length before error: {} bytes",
                    buf.len()
                ));
                if !buf.is_empty() {
                    let tail = buf.len().min(500);
                    let tail_slice = &buf[buf.len() - tail..];
                    msg.push_str(&format!(
                        "\n  Last {} bytes (utf8-lossy): {:?}",
                        tail,
                        String::from_utf8_lossy(tail_slice)
                    ));
                }
                eprintln!("{}", msg);
                return Err(msg);
            }
        };
        buf.extend_from_slice(&chunk);
        while let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(&buf[..i]).trim_end_matches('\r').to_string();
            buf.drain(..=i);
            if line.is_empty() {
                if !data_buf.is_empty() && data_buf != "null" {
                    println!("{}", data_buf);
                }
                data_buf.clear();
            } else if line.starts_with("data: ") {
                data_buf = line.trim_start_matches("data: ").to_string();
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let (device_id, email, password) = load_env()?;
    let api_key = std::env::var("FIREBASE_API_KEY").unwrap_or_else(|_| FIREBASE_API_KEY.to_string());

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client build failed: {}", e))?;

    println!("Authenticating…");
    let id_token = firebase_login(&client, &email, &password, &api_key).await?;
    println!("Logged in.\n");

    stream_brainwaves(&client, &device_id, &id_token).await
}
