use std::sync::mpsc::Receiver;
use crate::networking::PacketInfo;

slint::include_modules!();
pub fn run_ui(receiver: Receiver<PacketInfo>) -> Result<(), slint::PlatformError> {
    let dashboard: Dashboard = Dashboard::new()?;
    let packet_info: PacketInfo = receiver.recv().unwrap();

    dashboard.on_update_values(move || {
        let current_rpm = packet_info.get_current_rpm();
        let speed = packet_info.get_speed();
        let best_lap = packet_info.get_best_lap();
        let current_lap = packet_info.get_current_lap();
        let current_race_time = packet_info.get_current_race_time();
        let gear = packet_info.get_gear();

        dashboard.set_rpm(current_rpm);
        dashboard.set_speed(speed);
        dashboard.set_best_lap(best_lap);
        dashboard.set_current_lap(current_lap);
        dashboard.set_race_time(current_race_time);
        dashboard.set_gear(gear);
    });

    dashboard.run()
}