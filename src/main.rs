use std::sync::mpsc;
use std::thread;

use telemetry::Telemetry;

mod telemetry;
mod ui;

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        Telemetry::parse_packets(&sender);
    });

    ui::run_ui(receiver);
}
