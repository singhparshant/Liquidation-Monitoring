mod server;
mod state;
mod types;
mod websocket;

use alloy::{
    primitives::Address, providers::ProviderBuilder, rpc::types::Filter, sol, sol_types::SolEvent,
};
use alloy_provider::{Provider, WsConnect};
use anyhow::{Error, Result};
use axum::{Router, routing::any};
use futures::StreamExt;
use futures_util::SinkExt;
use tokio::sync::broadcast;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Bytes, protocol::Message},
};

use crate::{
    server::handler,
    state::AppState,
    types::{AaveLiquidation, BinanceMessage, LiquidationEvent},
};

// Define the Aave V3 LiquidationCall event using Alloy's sol! macro
sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract AaveV3Pool {
        event LiquidationCall(
            address indexed collateralAsset,
            address indexed debtAsset,
            address indexed user,
            uint256 debtToCover,
            uint256 liquidatedCollateralAmount,
            address liquidator,
            bool receiveAToken
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let binance_url = "wss://fstream.binance.com/ws/!forceOrder@arr";
    let alchemy_wss = "wss://eth-mainnet.g.alchemy.com/v2/KIVje_uKFG-DSq4bd0bkP37sOiV33T0z";

    let (tx, _) = broadcast::channel::<LiquidationEvent>(1024);
    let tx_for_ws = tx.clone();
    let tx_for_aave = tx.clone();

    let _binance_handle = tokio::spawn(async move {
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
                    // Wrap in LiquidationEvent enum
                    let liquidation_event = LiquidationEvent::Binance(message);
                    // broadcast::Sender::send() returns Err when there are zero active receivers.
                    // That's normal (e.g. before any /ws client connects), so don't treat it as an error.
                    if broadcast_stream.receiver_count() > 0 {
                        let _ = broadcast_stream.send(liquidation_event);
                    }
                }
                Message::Ping(_ping) => {
                    println!("Ping received from Binance");
                    write.send(Message::Pong(Bytes::from("Pong"))).await?;
                }
                _ => anyhow::bail!("Unknown message type"),
            }
        }

        Ok(())
    });

    let _alchemy_handle = tokio::spawn(async move {
        let provider = ProviderBuilder::new()
            .connect_ws(WsConnect::new(alchemy_wss))
            .await
            .unwrap();

        // Aave V3 Pool contract address
        let aave_pool_address = "0x87870Bca3F3fD6335C3F4ce8392D69350B4fA4E2"
            .parse::<Address>()
            .unwrap();

        // Filter for liquidation events
        let filter = Filter::new()
            .address(aave_pool_address)
            .event_signature(AaveV3Pool::LiquidationCall::SIGNATURE_HASH);

        println!(
            "🎯 Subscribing to Aave V3 liquidations at {}",
            aave_pool_address
        );

        // Subscribe to stream
        let sub = provider.subscribe_logs(&filter).await.unwrap();
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            // Decode the LiquidationCall event
            match AaveV3Pool::LiquidationCall::decode_log(&log.inner) {
                Ok(event) => {
                    // Get block information
                    let block_number = log.block_number.unwrap();
                    let tx_hash = log.transaction_hash.map(|h| format!("{:?}", h)).unwrap();

                    // Get block timestamp (requires additional RPC call)
                    let timestamp = if let Some(block_num) = log.block_number {
                        match provider.get_block_by_number(block_num.into()).await {
                            Ok(Some(block)) => block.header.timestamp,
                            _ => 0,
                        }
                    } else {
                        0
                    };

                    let liquidation = AaveLiquidation {
                        collateral_asset: format!("{:?}", event.collateralAsset),
                        debt_asset: format!("{:?}", event.debtAsset),
                        user: format!("{:?}", event.user),
                        debt_to_cover: event.debtToCover.to_string(),
                        liquidated_collateral_amount: event.liquidatedCollateralAmount.to_string(),
                        liquidator: format!("{:?}", event.liquidator),
                        receive_a_token: event.receiveAToken,
                        block_number,
                        transaction_hash: tx_hash.clone(),
                        timestamp,
                    };

                    println!("🔴 Aave Liquidation detected!");
                    println!("   User: {}", liquidation.user);
                    println!(
                        "   Debt covered: {} ({})",
                        liquidation.debt_to_cover, liquidation.debt_asset
                    );
                    println!(
                        "   Collateral: {} ({})",
                        liquidation.liquidated_collateral_amount, liquidation.collateral_asset
                    );
                    println!("   Liquidator: {}", liquidation.liquidator);
                    println!("   Tx: https://etherscan.io/tx/{}", tx_hash);

                    // Broadcast to websocket clients
                    let liquidation_event = LiquidationEvent::Aave(liquidation);
                    if tx_for_aave.receiver_count() > 0 {
                        let _ = tx_for_aave.send(liquidation_event);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to decode LiquidationCall event: {:?}", e);
                }
            }
        }

        Ok::<(), Error>(())
    });

    // for handle in [binance_handle, alchemy_handle] {
    //     match handle.await.unwrap() {
    //         Ok(res) => {}
    //         _ => anyhow::bail!("yooo"),
    //     }
    // }

    let app = Router::new()
        // .route("/", get(index))
        .route("/ws", any(handler))
        .with_state(AppState { sender: tx.clone() });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server running on http://0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
