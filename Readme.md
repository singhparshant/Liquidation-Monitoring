A real-time crypto liquidations monitor that streams Binance Futures forced-order events to a live web dashboard. A Rust (Tokio + Axum) backend maintains the upstream WebSocket connection, fan-outs messages via a broadcast channel, and exposes a /ws WebSocket endpoint. A Next.js + React frontend subscribes to the stream, renders a styled table with status indicators and pagination, and updates instantly as new liquidations arrive.

![image.png](attachment:c3414a87-0aa9-4bbb-b86d-2cc14c5fe76c:image.png)
