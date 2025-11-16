// use crate::{api::ws, domain::prelude::*};
use crate::domain::prelude::*;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct AppState {
    /// The sender for the heartbeat channel.
    /// Used to broadcast heartbeats to the websocket clients.
    pub heartbeat_tx: broadcast::Sender<Heartbeat>,

    /// The sender for the transactions channel.
    /// Used to broadcast transactions to the websocket clients.
    pub transactions_tx: broadcast::Sender<Transaction>,

    /// The cancellation token for graceful shutdown.
    /// Used to signal background tasks to stop.
    pub cancellation_token: CancellationToken,
}

impl AppState {
    pub fn new(
        heartbeat_tx: broadcast::Sender<Heartbeat>,
        transactions_tx: broadcast::Sender<Transaction>,
        cancellation_token: CancellationToken,
    ) -> Self {
        Self {
            transactions_tx,
            heartbeat_tx,
            cancellation_token,
        }
    }
}
