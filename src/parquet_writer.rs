use crate::types::Brainwave;
use arrow::array::{Float64Array, Int64Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::Utc;
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use std::fs::{self, File};
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::task;

const BATCH_SIZE: usize = 60 * 256; // ~1 minute of data (256Hz)

pub async fn run(mut rx: Receiver<Brainwave>) {
    let mut buffer: Vec<Brainwave> = Vec::with_capacity(BATCH_SIZE);
    
    // Create data/raw directory
    let _ = fs::create_dir_all("data/raw");

    println!("ParquetWriter: Started. Buffering {} samples...", BATCH_SIZE);

    while let Some(bw) = rx.recv().await {
        buffer.push(bw);
        if buffer.len() >= BATCH_SIZE {
            let chunk = std::mem::replace(&mut buffer, Vec::with_capacity(BATCH_SIZE));
            
            task::spawn_blocking(move || {
                write_batch(&chunk);
            }).await.expect("Parquet flush task failed");
        }
    }

    // Flush any remaining buffered data on channel close (e.g. Ctrl+C shutdown)
    if !buffer.is_empty() {
        let n = buffer.len();
        task::spawn_blocking(move || {
            write_batch(&buffer);
        })
        .await
        .expect("Parquet flush task failed");
        println!("ParquetWriter: Flushed {} records on shutdown", n);
    }
}

fn write_batch(buffer: &[Brainwave]) {
    let now = Utc::now();
    let date_dir = format!("data/raw/{}", now.format("%Y-%m-%d"));
    fs::create_dir_all(&date_dir).unwrap_or_else(|_| panic!("Failed to create dir: {}", date_dir));

    let filename = format!("{}/{}.parquet", date_dir, now.format("%H-%M-%S"));
    
    let schema = Arc::new(Schema::new(vec![
        Field::new("timestamp", DataType::Int64, false),
        Field::new("label", DataType::Utf8, false),
        Field::new("ch1", DataType::Float64, false),
        Field::new("ch2", DataType::Float64, false),
        Field::new("ch3", DataType::Float64, false),
        Field::new("ch4", DataType::Float64, false),
        Field::new("ch5", DataType::Float64, false),
        Field::new("ch6", DataType::Float64, false),
        Field::new("ch7", DataType::Float64, false),
        Field::new("ch8", DataType::Float64, false),
    ]));

    let timestamps: Vec<i64> = buffer.iter().map(|b| b.timestamp).collect();
    let labels: Vec<&str> = buffer.iter().map(|b| b.label.as_str()).collect();
    
    // Efficiently unzip channels? Or map individually (safer)
    let ch1: Vec<f64> = buffer.iter().map(|b| b.data.get(0).cloned().unwrap_or(0.0)).collect();
    let ch2: Vec<f64> = buffer.iter().map(|b| b.data.get(1).cloned().unwrap_or(0.0)).collect();
    let ch3: Vec<f64> = buffer.iter().map(|b| b.data.get(2).cloned().unwrap_or(0.0)).collect();
    let ch4: Vec<f64> = buffer.iter().map(|b| b.data.get(3).cloned().unwrap_or(0.0)).collect();
    let ch5: Vec<f64> = buffer.iter().map(|b| b.data.get(4).cloned().unwrap_or(0.0)).collect();
    let ch6: Vec<f64> = buffer.iter().map(|b| b.data.get(5).cloned().unwrap_or(0.0)).collect();
    let ch7: Vec<f64> = buffer.iter().map(|b| b.data.get(6).cloned().unwrap_or(0.0)).collect();
    let ch8: Vec<f64> = buffer.iter().map(|b| b.data.get(7).cloned().unwrap_or(0.0)).collect();

    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Int64Array::from(timestamps)),
            Arc::new(StringArray::from(labels)),
            Arc::new(Float64Array::from(ch1)),
            Arc::new(Float64Array::from(ch2)),
            Arc::new(Float64Array::from(ch3)),
            Arc::new(Float64Array::from(ch4)),
            Arc::new(Float64Array::from(ch5)),
            Arc::new(Float64Array::from(ch6)),
            Arc::new(Float64Array::from(ch7)),
            Arc::new(Float64Array::from(ch8)),
        ],
    ).expect("Failed to create RecordBatch");

    let file = File::create(&filename).expect("Failed to create Parquet file");
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();
    let mut writer = ArrowWriter::try_new(file, schema, Some(props)).expect("Failed to create ArrowWriter");
    
    writer.write(&batch).expect("Failed to write batch");
    writer.close().expect("Failed to close writer");
    
    println!("ParquetWriter: Flushed {} records to {}", buffer.len(), filename);
}
