use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct BinanceOrder {
    s: String,  // Symbol
    S: String,  // Side
    o: String,  // Order Type
    f: String,  // Time in Force
    q: String,  // Original Quantity
    p: String,  // Price
    ap: String, // Average Price
    X: String,  // Order Status
    l: String,  // Order Last Filled Quantity
    z: String,  // Order Filled Accumulated Quantity
    T: u64,     // Order Trade Time
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct BinanceMessage {
    e: String,       // Event Type
    E: u64,          // Event Time
    o: BinanceOrder, // Order
}

/// Aave V3 LiquidationCall event
/// Event signature: LiquidationCall(address,address,address,uint256,uint256,address,uint16)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AaveLiquidation {
    pub collateral_asset: String, // Address of the collateral asset
    pub debt_asset: String,       // Address of the debt asset
    pub user: String,             // Address of the user being liquidated
    pub debt_to_cover: String,    // Amount of debt covered (in debt asset)
    pub liquidated_collateral_amount: String, // Amount of collateral liquidated
    pub liquidator: String,       // Address of the liquidator
    pub receive_a_token: bool,    // Whether liquidator receives aToken or underlying
    pub block_number: u64,        // Block number of the liquidation
    pub transaction_hash: String, // Transaction hash
    pub timestamp: u64,           // Block timestamp
}

/// Unified liquidation event enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LiquidationEvent {
    #[serde(rename = "binance")]
    Binance(BinanceMessage),
    #[serde(rename = "aave")]
    Aave(AaveLiquidation),
}
