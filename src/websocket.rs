use crate::types::Brainwave;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
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

    while let Ok(bw) = rx.recv().await {
        let msg = match serde_json::to_string(&bw) {
            Ok(s) => s,
            Err(_) => continue,
        };
        
        if let Err(e) = write.send(Message::Text(msg.into())).await {
            // Client disconnected
            break;
        }
    }
}
