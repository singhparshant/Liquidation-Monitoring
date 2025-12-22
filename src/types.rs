use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct BinanceMessage {
    e: String,       // Event Type
    E: u64,          // Event Time
    o: BinanceOrder, // Order
}
