use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tracing::info;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsBroadcastMessage {
    pub event: String,
    pub project_id: Option<i64>,
    pub user_id: Option<i64>,
    pub data: serde_json::Value,
}

#[derive(Clone)]
pub struct WsHub {
    pub tx: broadcast::Sender<WsBroadcastMessage>,
}

impl WsHub {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1000);
        WsHub { tx }
    }

    pub fn broadcast(&self, msg: WsBroadcastMessage) {
        let _ = self.tx.send(msg);
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.ws_hub.tx.subscribe();

    // Spawn a task to forward broadcast messages to this client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(text.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn task to receive ping/client messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(t) => {
                    // Client can send ping or subscribe requests
                    info!("[WS] Received client message: {}", t);
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    // If either task finishes, abort the other
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
