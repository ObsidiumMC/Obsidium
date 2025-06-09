mod error;
mod logger;

use error::ServerError;

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Initialize logger
    logger::init_logger();

    tracing::info!("Server starting...");
    tracing::debug!("Debug message example");
    tracing::warn!("Warning message example");
    tracing::trace!("Trace message example");
    tracing::error!("Error message example");

    // Your server logic will go here
    // For now, let's just simulate some work or an error
    if let Err(e) = run_server().await {
        tracing::error!("Server encountered an error: {}", e);
        return Err(e);
    }

    tracing::info!("Server shutting down.");
    Ok(())
}

async fn run_server() -> Result<(), ServerError> {
    // Example of how you might use the error types:
    // tokio::fs::read_to_string("non_existent_file.txt").await?;
    // Err(ServerError::Config("Missing crucial setting".to_string()))
    tracing::info!("Server running successfully (simulated).");
    Ok(())
}
