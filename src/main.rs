use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use telemetry::Telemetry;

mod telemetry;
mod ui;

fn main() {
    let telem = Arc::new((Mutex::new(Telemetry::default()), Condvar::new()));

    let telemetry_clone = Arc::clone(&telem);
    thread::spawn(move || {
        Telemetry::parse_packets(telemetry_clone);
    });

    let ui_clone = Arc::clone(&telem);
    ui::run_ui(ui_clone);
}
