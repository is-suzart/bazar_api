use tracing_subscriber;
use tracing::Level;

pub fn tracer() {
    tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .with_target(true)
    .init();
}