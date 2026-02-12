mod collector;
mod parquet_writer;
mod types;
mod websocket;

use crate::types::Brainwave;
use tokio::sync::{broadcast, mpsc};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Observable Daemon (Phase 2)...");

    let cancel = CancellationToken::new();
    let cancel_clone = cancel.clone();

    // Create channels
    // Broadcast: 100 capacity (drop if lagging) for WebSocket (Real-time UI)
    let (broadcast_tx, _) = broadcast::channel::<Brainwave>(100);

    // MPSC: 10000 capacity (backpressure if lagging) for Parquet Writer (Storage)
    let (mpsc_tx, mpsc_rx) = mpsc::channel::<Brainwave>(10000);

    // Spawn Parquet Writer (must complete to flush buffered data on shutdown)
    println!("Spawning Parquet Writer...");
    let parquet_handle = tokio::spawn(parquet_writer::run(mpsc_rx));

    // Spawn WebSocket Server
    println!("Spawning WebSocket Server...");
    let ws_tx = broadcast_tx.clone();
    tokio::spawn(websocket::run(ws_tx));

    // Spawn Collector
    println!("Spawning Collector...");
    let collector_broadcast = broadcast_tx.clone();
    let collector_mpsc = mpsc_tx;
    // mpsc_tx moved to collector; when collector stops, channel closes and parquet writer can flush

    tokio::spawn(async move {
        if let Err(e) = collector::run(collector_broadcast, collector_mpsc, cancel_clone).await {
            eprintln!("Collector stopped with error: {}", e);
        }
    });

    // Keep running until Ctrl+C
    match tokio::signal::ctrl_c().await {
        Ok(()) => {
            println!("\nReceived Ctrl+C. Shutting down...");
        }
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Signal collector to stop (drops mpsc_tx -> closes channel)
    cancel.cancel();

    // Wait for parquet writer to drain and flush any buffered data
    if let Err(e) = parquet_handle.await {
        eprintln!("Parquet writer panicked: {:?}", e);
    }

    Ok(())
}
