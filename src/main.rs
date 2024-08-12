use std::sync::mpsc;
use std::thread;

mod telemetry;
mod ui;

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || loop {
        telemetry::Telemetry::parse_packets(&sender);
    });

    ui::run_ui(receiver);
}
