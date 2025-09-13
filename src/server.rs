use anyhow::{Context, Result};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn};

use crate::handler;



/// Starts the listener and accepts connections.
///
/// This function keeps running until the process exits.
pub async fn run(bind: &str) -> Result<()> {
    // Bind the TCP listener
    let listener = TcpListener::bind(bind)
        .await
        .with_context(|| format!("failed to bind to {}", bind))?;

    info!(addr = bind, "listener bound");

    loop {
        // Accept an incoming connection
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!(%addr, "accepted connection");
                // Spawn a task to handle the client so we can accept new clients
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(socket, addr).await {
                        // Log the error; the server keeps running
                        warn!(%addr, error = %e, "connection handler ended with error");
                    } else {
                        info!(%addr, "connection handler finished");
                    }
                });
            }
            Err(e) => {
                // Accept failure is often transient — log and continue accepting
                warn!(error = %e, "failed to accept socket");
            }
        }
    }
}

async fn handle_connection(socket: TcpStream, addr: SocketAddr) -> Result<()> {
    // Enter a tracing span scoped to this connection
    let span = tracing::info_span!("connection", %addr);
    let _enter = span.enter();

    // Delegate to handler module
    handler::handle(socket, addr).await
}
