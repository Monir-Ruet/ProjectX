use crate::state::AppState;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.tx.subscribe();

    loop {
        tokio::select! {
            Ok(msg) = rx.recv() => {
                let _ = socket.send(Message::Text(Utf8Bytes::from(msg))).await;
            }
        }
    }
}