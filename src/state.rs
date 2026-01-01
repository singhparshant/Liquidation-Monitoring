use tokio::sync::broadcast;

use crate::types::LiquidationEvent;

#[derive(Debug, Clone)]
pub struct AppState {
    pub sender: broadcast::Sender<LiquidationEvent>,
}

#[allow(dead_code)]
impl AppState {
    pub fn new(sender: broadcast::Sender<LiquidationEvent>) -> Self {
        Self { sender }
    }
}
