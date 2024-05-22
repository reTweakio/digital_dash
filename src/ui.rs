use std::thread;
use std::sync::mpsc::Receiver;
use slint::{ModelRc, VecModel};
use crate::networking::PacketInfo;

slint::include_modules!();

fn update_rpm_lights(rpm: f32, max_rpm: f32, rpm_lights: &mut Vec<bool>) {
    let step_size = max_rpm / rpm_lights.len() as f32;
    let lights_on = (rpm / step_size).ceil() as usize;
    
    for i in 0..rpm_lights.len() {
        rpm_lights[i] = i < lights_on;
    }
}


pub fn run_ui(receiver: Receiver<PacketInfo>) {
    let dashboard: Dashboard = Dashboard::new().unwrap();
    let weak_dashboard: slint::Weak<Dashboard> = dashboard.as_weak();

    thread::spawn(move || {
        loop {
            let packet_info: PacketInfo = receiver.recv().unwrap();

            let current_rpm: f32 = packet_info.get_current_rpm();
            let max_rpm: f32 = packet_info.get_max_rpm();
            let speed: f32 = packet_info.get_speed();
            let best_lap: f32 = packet_info.get_best_lap();
            let current_lap: f32 = packet_info.get_current_lap();
            let race_time: f32 = packet_info.get_current_race_time();
            let gear: i32 = packet_info.get_gear();
            let accel: f32 = packet_info.get_accel();
            let brake: f32 = packet_info.get_brake();
            let position: i32 = packet_info.get_position();
            let temp_left_f: f32 = packet_info.get_temp_left_f();
            let temp_right_f: f32 = packet_info.get_temp_right_f();
            let temp_left_r: f32 = packet_info.get_temp_left_r();
            let temp_right_r: f32 = packet_info.get_temp_right_r();
            let lap_number: i32 = packet_info.get_lap_number();

            let mut rpm_lights: Vec<bool> = vec![false; 15];
            update_rpm_lights(current_rpm, max_rpm, &mut rpm_lights);
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_rpm_lights(ModelRc::new(VecModel::from(rpm_lights)))).unwrap();

            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_rpm(current_rpm)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_speed(speed)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_best_lap(best_lap)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_current_lap(current_lap)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_race_time(race_time)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_gear(gear)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_accel(accel)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_brake(brake)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_position(position)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_temp_left_f(temp_left_f)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_temp_right_f(temp_right_f)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_temp_left_r(temp_left_r)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_temp_right_r(temp_right_r)).unwrap();
            weak_dashboard.upgrade_in_event_loop(move |dashboard: Dashboard| dashboard.set_lap_number(lap_number)).unwrap();
        }
    });

    dashboard.run().unwrap();
}