use axum::{routing::get, Router};
use tokio_util::sync::CancellationToken;
use txapi::{api, core::prelude::*, stream};

/// Check if health check mode is requested
fn is_health_check() -> bool {
    std::env::args().any(|arg| arg == "--health")
}

/// Initialize the application state.
///
/// This function initializes the application state by injecting all the
/// necessary dependencies into the AppState struct.
///
/// The main dependencies are the websocket channel senders, which are used to broadcast
/// messages to the websocket clients.
///
async fn init_app_state(cancellation_token: CancellationToken) -> AppState {
    let (transactions_tx, _) = stream::transactions::channel(cancellation_token.clone()).await;
    let (heartbeat_tx, _) = stream::heartbeat::channel(cancellation_token.clone()).await;

    AppState {
        heartbeat_tx,
        transactions_tx,
        cancellation_token,
    }
}
fn init_logger() {
    use tracing_subscriber;

    // Get log level from environment variable, default to INFO
    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "INFO".to_string())
        .parse::<tracing::Level>()
        .unwrap_or(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .init();
}

#[tokio::main]
async fn main() {
    // Handle health check mode
    if is_health_check() {
        match health_check().await {
            Ok(_) => {
                println!("Health check: OK");
                std::process::exit(0);
            }
            Err(e) => {
                eprintln!("Health check: FAILED - {}", e);
                std::process::exit(1);
            }
        }
    }

    init_logger();

    // Create a cancellation token for graceful shutdown
    let cancellation_token = CancellationToken::new();
    let app_state = init_app_state(cancellation_token.clone()).await;

    let app = Router::new()
        .route("/health", get(api::health::endpoint))
        .route("/ws/v1", get(api::ws::endpoint))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999")
        .await
        .unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());
    println!("Press Ctrl+C to shutdown gracefully");

    // Spawn the server with graceful shutdown
    let server = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(cancellation_token.clone()));

    // Run the server
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }

    println!("Server shutdown complete");
}

/// Wait for shutdown signal (Ctrl+C) and trigger cancellation
async fn shutdown_signal(cancellation_token: CancellationToken) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("\nReceived Ctrl+C, initiating graceful shutdown...");
        },
        _ = terminate => {
            println!("\nReceived SIGTERM, initiating graceful shutdown...");
        },
    }

    // Signal all background tasks to shut down
    cancellation_token.cancel();
}

/// Perform health check by attempting to connect to the service
async fn health_check() -> Result<(), String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let port = std::env::var("PORT").unwrap_or_else(|_| "9999".to_string());
    let url = format!("http://localhost:{}/health", port);

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to {}: {}", url, e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(format!("Health check failed with status: {}", response.status()))
    }
}
