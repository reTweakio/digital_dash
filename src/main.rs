use std::thread;
use std::sync::mpsc;

mod networking;
mod ui;

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(|| {
        networking::parse_packets(sender);
    });

    ui::run_ui(receiver);
}
