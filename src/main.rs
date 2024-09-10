use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use telemetry::Telemetry;

mod telemetry;
mod ui;

fn main() {
    let telem = Arc::new((Mutex::new(Telemetry::default()), Condvar::new()));

    let telemetry_clone = telem.clone();
    thread::spawn(move || {
        Telemetry::parse_packets(telemetry_clone);
    });

    let ui_clone = telem.clone();
    ui::run_ui(ui_clone);
}
