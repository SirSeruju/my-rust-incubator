use tracing;
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber;
use tracing_subscriber::prelude::*;

fn set_global_subscriber() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let writer = std::io::stdout
        .with_max_level(Level::WARN)
        .or_else(std::io::stderr);
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(false)
        .flatten_event(true)
        .with_writer(writer);
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

fn set_local_subscriber() -> Result<tracing::subscriber::DefaultGuard, std::io::Error> {
    let writer = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("access.log")?;
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .with_target(false)
        .flatten_event(true)
        .with_writer(writer);
    let subscriber = tracing_subscriber::Registry::default().with(layer);
    Ok(tracing::subscriber::set_default(subscriber))
}

fn main() {
    let _ = set_global_subscriber();
    debug!(test = 1, "DEBUG");
    info!("INFO");
    warn!("WARN");
    error!("ERROR");
    trace!("TRACE");

    let _local = set_local_subscriber();
    debug!(test = 1, "DEBUG");
    info!("INFO");
    warn!("WARN");
    error!("ERROR");
    trace!("TRACE");
}
