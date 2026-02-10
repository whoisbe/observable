use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Brainwave {
    pub timestamp: i64,
    pub data: Vec<f64>,
    pub label: String,
}

// For internal communication
#[derive(Debug, Clone)]
pub enum Event {
    Brainwave(Brainwave),
}
