use anyhow::{Context, Result};
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, info, warn};

/// Per-connection handler: read bytes and echo them back.
pub async fn handle(mut socket: TcpStream, addr: SocketAddr) -> Result<()> {
    let mut total_bytes: usize = 0;
    let mut buf = [0u8; 1024];

    loop {
        // Try reading data from the client
        let n = match socket.read(&mut buf).await {
            Ok(0) => {
                // connection closed
                info!(%addr, "client closed connection");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                return Err(e).context("failed to read from socket");
            }
        };

        total_bytes += n;
        debug!(%addr, bytes = n, total = total_bytes, "read bytes");

        // Echo back the same data
        if let Err(e) = socket.write_all(&buf[..n]).await {
            return Err(e).context("failed to write to socket");
        }
        debug!(%addr, bytes = n, "echoed bytes");
    }

    warn!(%addr, total_bytes, "connection closed gracefully");
    Ok(())
}
