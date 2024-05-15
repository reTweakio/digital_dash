use std::thread;
use std::sync::mpsc::Receiver;
use crate::networking::PacketInfo;

slint::include_modules!();

pub fn run_ui(receiver: Receiver<PacketInfo>) {
    let dashboard: Dashboard = Dashboard::new().unwrap();
    let weak_dashboard = dashboard.as_weak();

    thread::spawn(move || {
        let dashboard = weak_dashboard.clone();
        loop {
            let packet_info: PacketInfo = receiver.recv().unwrap();

            let current_rpm: f32 = packet_info.get_current_rpm();
            let speed: f32 = packet_info.get_speed();
            let best_lap: f32 = packet_info.get_best_lap();
            let current_lap: f32 = packet_info.get_current_lap();
            let gear: i32 = packet_info.get_gear();

            dashboard.upgrade_in_event_loop(move |dashboard| dashboard.set_rpm(current_rpm)).unwrap();
            dashboard.upgrade_in_event_loop(move |dashboard| dashboard.set_speed(speed)).unwrap();
            dashboard.upgrade_in_event_loop(move |dashboard| dashboard.set_best_lap(best_lap)).unwrap();
            dashboard.upgrade_in_event_loop(move |dashboard| dashboard.set_current_lap(current_lap)).unwrap();
            dashboard.upgrade_in_event_loop(move |dashboard| dashboard.set_gear(gear)).unwrap();
            
        }
    });

    dashboard.run().unwrap();
}