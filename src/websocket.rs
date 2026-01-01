use tokio::sync::broadcast;

use crate::types::LiquidationEvent;

#[allow(dead_code)]
pub struct UpstreamWebsocket {
    pub liquidations: broadcast::Sender<LiquidationEvent>,
}

#[allow(dead_code)]
impl UpstreamWebsocket {
    pub fn new(liquidations: broadcast::Sender<LiquidationEvent>) -> Self {
        Self { liquidations }
    }

    pub fn connect_and_send(&self, _url: &str) {
        // TODO: Implement generic websocket connection logic
    }
}
