use axum::{
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::{Html, Response},
};

use crate::state::AppState;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, State(state)))
}

async fn handle_socket(mut socket: WebSocket, state: State<AppState>) {
    let mut rx = state.sender.subscribe();
    while let Ok(value) = rx.recv().await {
        let text = match serde_json::to_string(&value) {
            Ok(text) => text,
            Err(_) => {
                eprintln!("Could not deserialise binance message: {:?}", value);
                continue;
            }
        };
        let bytes: Utf8Bytes = text.into();
        // let (sx, rs) = socket.split();
        if socket.send(Message::Text(bytes)).await.is_err() {
            break;
        }
    }
}
