use crate::types::Brainwave;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Deserialize)]
struct FirebaseAuthResponse {
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: Option<String>,
    #[serde(rename = "expiresIn")]
    expires_in: Option<String>,
}

const FIREBASE_API_KEY: &str = "AIzaSyB0TkZ83Fj0CIzn8AAmE-Osc92s3ER8hy8";
const FIREBASE_DATABASE_URL: &str = "https://neurosity-device.firebaseio.com";

pub async fn run(
    broadcast_tx: broadcast::Sender<Brainwave>,
    mpsc_tx: mpsc::Sender<Brainwave>,
    cancel: CancellationToken,
) -> Result<(), String> {
    let (device_id, email, password) = load_env()?;
    let api_key = std::env::var("FIREBASE_API_KEY").unwrap_or_else(|_| FIREBASE_API_KEY.to_string());

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client build failed: {}", e))?;

    println!("Collector: Authenticating...");
    let id_token = firebase_login(&client, &email, &password, &api_key).await?;
    println!("Collector: Logged in. Connecting to stream...");

    stream_loop(&client, &device_id, &id_token, broadcast_tx, mpsc_tx, cancel).await
}

fn load_env() -> Result<(String, String, String), String> {
    dotenvy::dotenv().ok();
    let device_id = std::env::var("DEVICE_ID").map_err(|_| "DEVICE_ID not set")?;
    let email = std::env::var("EMAIL").map_err(|_| "EMAIL not set")?;
    let password = std::env::var("PASSWORD").map_err(|_| "PASSWORD not set")?;
    Ok((device_id, email, password))
}

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
        return Err(format!("Login failed: {}", res.status()));
    }
    let auth: FirebaseAuthResponse = res.json().await.map_err(|e| format!("{}", e))?;
    Ok(auth.id_token)
}

async fn stream_loop(
    client: &Client,
    device_id: &str,
    id_token: &str,
    broadcast_tx: broadcast::Sender<Brainwave>,
    mpsc_tx: mpsc::Sender<Brainwave>,
    cancel: CancellationToken,
) -> Result<(), String> {
    let base = FIREBASE_DATABASE_URL.trim_end_matches('/');
    let device_path = format!("{}/devices/{}", base, device_id);

    // 1) Push to subscriptions
    let push_url = format!("{}/subscriptions.json?auth={}", device_path, id_token);
    let push_res = client
        .post(&push_url)
        .body("null")
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Push subscription failed: {}", e))?;
    let push_body: serde_json::Value = push_res.json().await.map_err(|e| format!("{}", e))?;
    let client_id = push_body.get("name").and_then(|n| n.as_str()).ok_or("No client_id")?;

    // 2) Set timestamp
    let client_url = format!("{}/clients/{}.json?auth={}", device_path, client_id, id_token);
    client.put(&client_url).json(&serde_json::json!({ ".sv": "timestamp" })).send().await.ok();

    // 3) Subscribe
    let sub_url = format!("{}/subscriptions.json?auth={}", device_path, id_token);
    let sub_body = serde_json::json!({
        "clientId": client_id,
        "metric": "brainwaves",
        "labels": ["raw"],
        "atomic": false
    });
    client.post(&sub_url).json(&sub_body).send().await.map_err(|e| format!("Sub failed: {}", e))?;

    // 4) Stream
    // Increase timeout for long-lived stream
    let stream_client = Client::builder()
        .timeout(Duration::from_secs(3600)) // 1 hour timeout (reconnect logic needed eventually)
        .no_gzip().no_deflate().no_brotli()
        .build().map_err(|e| format!("{}", e))?;

    let stream_url = format!("{}/metrics/brainwaves/raw.json?auth={}", device_path, id_token);
    let res = stream_client.get(&stream_url).header("Accept", "text/event-stream").send().await.map_err(|e| format!("{}", e))?;
    
    let mut stream = res.bytes_stream();
    let mut buf = Vec::<u8>::new();
    let mut data_buf = String::new();

    println!("Collector: Streaming brainwaves...");

    loop {
        let chunk_result = tokio::select! {
            r = stream.next() => match r {
                Some(res) => res,
                None => break,
            },
            _ = cancel.cancelled() => {
                println!("Collector: Shutdown requested, stopping stream");
                break;
            }
        };

        let chunk = match chunk_result {
            Ok(c) => c,
            Err(e) => return Err(format!("Stream error: {}", e)),
        };
        buf.extend_from_slice(&chunk);
        
        while let Some(i) = buf.iter().position(|&b| b == b'\n') {
            let line = String::from_utf8_lossy(&buf[..i]).trim().to_string();
            buf.drain(..=i);
            
            if line.starts_with("data: ") {
                data_buf = line.trim_start_matches("data: ").to_string();
            } else if line.is_empty() && !data_buf.is_empty() {
                if data_buf != "null" {
                    if let Ok(bw_raw) = serde_json::from_str::<serde_json::Value>(&data_buf) {
                        // Firebase RTDB REST SSE wraps events in {"path": "/", "data": <payload>}.
                        // The brainwave itself is {"data": [...], "timestamp": ..., "label": ...}.
                        // Unwrap the envelope so we parse the inner brainwave, not the outer object.
                        let payload = if bw_raw.get("path").is_some() {
                            bw_raw.get("data")
                        } else {
                            Some(&bw_raw)
                        };
                        if let Some(payload) = payload {
                            if let (Some(data), Some(ts), Some(label)) = (
                                payload.get("data").and_then(|v| v.as_array()),
                                payload.get("timestamp").and_then(|v| v.as_i64()),
                                payload.get("label").and_then(|v| v.as_str())
                            ) {
                            let data_vec: Vec<f64> = data.iter().filter_map(|v| v.as_f64()).collect();
                            if data_vec.len() == 8 {
                                let bw = Brainwave {
                                    timestamp: ts,
                                    data: data_vec,
                                    label: label.to_string(),
                                };
                                
                                // Send to channels
                                let _ = broadcast_tx.send(bw.clone());
                                let _ = mpsc_tx.send(bw).await;
                            }
                        }
                        }
                    }
                }
                data_buf.clear();
            }
        }
    }
    Ok(())
}
