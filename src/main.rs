mod server;
mod state;
mod types;
mod websocket;

use anyhow::{Error, Result};
use axum::{Router, routing::any};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, protocol::Message},
};

use crate::{server::handler, state::AppState, types::BinanceMessage};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let binance_url = "wss://fstream.binance.com/ws/!forceOrder@arr";

    let (tx, _) = broadcast::channel::<BinanceMessage>(1024);
    let tx_for_ws = tx.clone();

    tokio::spawn(async move {
        let broadcast_stream = tx_for_ws;
        let (mut _ws_stream, _) = connect_async(binance_url).await.expect("Failed to connect");

        let (mut write, mut read) = _ws_stream.split();

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
                    // broadcast::Sender::send() returns Err when there are zero active receivers.
                    // That's normal (e.g. before any /ws client connects), so don't treat it as an error.
                    if broadcast_stream.receiver_count() > 0 {
                        let _ = broadcast_stream.send(message);
                    }
                }
                Message::Ping(_ping) => {
                    println!("Ping received");
                    write.send(Message::Pong(Bytes::from("Pong"))).await?;
                }
                _ => anyhow::bail!("Unknown type"),
            }
        }

        Ok(())
    });

    let app = Router::new()
        // .route("/", get(index))
        .route("/ws", any(handler))
        .with_state(AppState { sender: tx.clone() });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
