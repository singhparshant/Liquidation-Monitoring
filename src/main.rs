use std::{env, ops::Deref, u8};
mod types;
mod websocket;

use anyhow::{Error, Result};
// use futures_util::{StreamExt, future, pin_mut};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, protocol::Message},
};

use crate::types::BinanceMessage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let binance_url = "wss://fstream.binance.com/ws/!forceOrder@arr";

    let (mut ws_stream, _) = connect_async(binance_url).await.expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    while let Some(event) = read.next().await {
        let message = match event {
            Ok(msg) => msg,
            Err(err) => anyhow::bail!("Unknown event : {:?}", err),
        };
        match message {
            Message::Text(text_data) => {
                let message: BinanceMessage = serde_json::from_str(&text_data)
                    .map_err(|e| anyhow::anyhow!("Failed to parse message: {}", e))?;
                println!("{:#?}", message);
            }
            Message::Ping(ping) => {
                println!("Ping received");
                write.send(Message::Pong(Bytes::from("Pong"))).await?;
            }
            _ => anyhow::bail!("Unknown type"),
        }
    }

    Ok(())
}
