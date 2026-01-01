use axum::{
    extract::{
        State,
        ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    response::Response,
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
            Err(e) => {
                eprintln!(
                    "Could not serialize liquidation event: {:?}, error: {}",
                    value, e
                );
                continue;
            }
        };
        let bytes: Utf8Bytes = text.into();
        if socket.send(Message::Text(bytes)).await.is_err() {
            break;
        }
    }
}
