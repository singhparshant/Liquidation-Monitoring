use tokio::sync::broadcast;

use crate::types::BinanceMessage;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sender: broadcast::Sender<BinanceMessage>,
}

impl AppState {
    pub fn new() {}
}
