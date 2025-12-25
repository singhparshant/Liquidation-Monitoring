use std::thread::spawn;

use tokio::sync::broadcast;

use crate::types::BinanceMessage;

pub struct UpstreamWebsocket {
    pub liquidations: broadcast::Sender<BinanceMessage>,
}

impl UpstreamWebsocket {
    pub fn new(&mut self, liquidations: broadcast::Sender<BinanceMessage>) -> Self {
        Self { liquidations }
    }

    pub fn connect_and_send(&self, url: &str) {}
}
