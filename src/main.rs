use anyhow::Result;
use clap::Parser;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod server;
mod handler;  // declare handler.rs as a module


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Bind address, e.g. 127.0.0.1:7878
    #[arg(long, default_value = "127.0.0.1:7878")]
    bind: String,

    /// Logging filter (RUST_LOG style). Example: "info", "debug"
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI args
    let args = Args::parse();

    // Initialize tracing subscriber with env-filter semantics using the provided log_level
    // This prints structured logs to stdout and respects the chosen level.
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&args.log_level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    info!("Starting rust-echo-server");
    info!(bind = %args.bind, "binding and listening");

    // Run server (this will run until CTRL-C / process exit)
    server::run(&args.bind).await?;

    Ok(())
}
