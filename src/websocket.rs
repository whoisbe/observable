use crate::types::Brainwave;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

pub async fn run(tx: broadcast::Sender<Brainwave>) {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind WebSocket server");
    println!("WebSocket server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let rx = tx.subscribe();
        tokio::spawn(accept_connection(stream, rx));
    }
}

async fn accept_connection(stream: TcpStream, mut rx: broadcast::Receiver<Brainwave>) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Error during handshake: {}", e);
            return;
        }
    };
    
    let (mut write, _read) = ws_stream.split();

    loop {
        match rx.recv().await {
            Ok(bw) => {
                let msg = match serde_json::to_string(&bw) {
                    Ok(s) => s,
                    Err(_) => continue,
                };

                if let Err(_) = write.send(Message::Text(msg.into())).await {
                    // Client disconnected
                    break;
                }
            }
            Err(RecvError::Lagged(n)) => {
                // Receiver fell behind; skip missed messages and continue
                eprintln!("WebSocket client lagged behind, skipped {} messages", n);
                continue;
            }
            Err(RecvError::Closed) => {
                break;
            }
        }
    }
}
