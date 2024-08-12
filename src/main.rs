use std::sync::mpsc;
use std::thread;

mod networking;
mod telemetry;
mod ui;

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(|| {
        networking::parse_packets(sender);
    });

    ui::run_ui(receiver);
}
