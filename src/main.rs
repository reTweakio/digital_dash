use iced;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use telemetry::config::Game;
use telemetry::games::forza::{ForzaParser, ForzaTelemetry};
use telemetry::parser::TelemetryParser;

mod telemetry;
mod ui;

fn main() -> iced::Result {
    let game: Game = Game::detect_game();
    let telemetry = match game {
        Game::Forza => Arc::new((Mutex::new(ForzaTelemetry::default()), Condvar::new())),
    };

    let telemetry_clone = telemetry.clone();
    thread::spawn(move || match game {
        Game::Forza => ForzaParser::parse_packets(telemetry_clone),
    });

    iced::run("Digital Dash", ForzaTelemetry::update, ForzaTelemetry::view)
}
