use futures::{stream::select_all, Stream, StreamExt};
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;

use crate::domain::prelude::*;

/// Initialize the transactions channel.
/// This channel is used to broadcast transactions from the combined backend
/// streams that need to be sent to the websocket clients.
///
/// The broadcaster is initialized with a buffer size of 100 by default, but
/// this can be overridden by the BROADCAST_BUFFER_SIZE environment variable.
///
/// This initializer is meant to be used to create a broadcaster at App State level,
/// in order to make it available to the websocket handler.
///
/// The cancellation_token parameter allows for graceful shutdown of the background task.
///
pub async fn channel(
    cancellation_token: CancellationToken,
) -> (
    broadcast::Sender<Transaction>,
    broadcast::Receiver<Transaction>,
) {
    let buffer_size = 100;
    let buffer_size = std::env::var("BROADCAST_BUFFER_SIZE")
        .map(|s| s.parse::<usize>().unwrap_or(buffer_size))
        .unwrap_or(buffer_size);

    let (tx, rx) = broadcast::channel(buffer_size);

    // combine all streams into a single consolidated stream
    let mut stream = select_all(vec![
        stream_from_mocks(),
        // add more streams here (ex. kafka, mongodb, etc.)
    ]);

    // spawn the message stream processor
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                // Check for cancellation signal
                _ = cancellation_token.cancelled() => {
                    tracing::info!("Transaction stream shutting down gracefully");
                    break;
                }
                // Process next transaction
                transaction = stream.next() => {
                    if let Some(transaction) = transaction {
                        // ignore send errors (occurs when no receivers)
                        let _ = tx_clone.send(transaction);
                    }
                }
            }
        }
    });
    (tx, rx)
}

/// A stream that generates mock transactions
///
/// This stream is used to generate mock transactions for testing purposes.
/// It is used to simulate a stream of transactions that are being processed
/// by the backend.
///
fn stream_from_mocks() -> impl Stream<Item = Transaction> + Send {
    let stream = futures::stream::unfold((), |()| async {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let transaction = Transaction::simple_mock();
        Some((transaction, ()))
    });

    Box::pin(stream)
}
