use std::sync::{Arc, Condvar, Mutex};

pub trait TelemetryParser {
    type GameTelemetry;

    fn parse_packets(telemetry: Arc<(Mutex<Self::GameTelemetry>, Condvar)>);
}
