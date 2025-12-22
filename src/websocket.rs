use tokio::sync::broadcast;

use crate::types::BinanceMessage;

struct UpstreamWebsocket {
    pub liquidations: broadcast::Sender<BinanceMessage>,
}

impl UpstreamWebsocket {
    pub fn new(&mut self, liquidations: broadcast::Sender<BinanceMessage>) -> Self {
        Self { liquidations }
    }

    pub fn connect(url: &str) {}
}
