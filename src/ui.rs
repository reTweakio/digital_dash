use crate::networking::PacketInfo;
use slint::{ModelRc, SharedString, VecModel};
use std::sync::mpsc::Receiver;
use std::thread;

slint::include_modules!();

fn update_rpm_lights(rpm: f32, max_rpm: f32, rpm_lights: &mut Vec<bool>) {
    let starting_rpm: f32 = max_rpm * 0.5;
    let step_size: f32 = ((max_rpm - 1000.0) - starting_rpm) / (rpm_lights.len() as f32);

    for (i, on_status) in rpm_lights.iter_mut().enumerate() {
        *on_status = rpm >= starting_rpm + (step_size * i as f32);
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
            let best_lap: String = packet_info.get_best_lap();
            let current_lap: String = packet_info.get_current_lap();
            let gear: i32 = packet_info.get_gear();
            let accel: f32 = packet_info.get_accel();
            let brake: f32 = packet_info.get_brake();
            let position: i32 = packet_info.get_position();
            let temp_left_f: f32 = packet_info.get_temp_left_f();
            let temp_right_f: f32 = packet_info.get_temp_right_f();
            let temp_left_r: f32 = packet_info.get_temp_left_r();
            let temp_right_r: f32 = packet_info.get_temp_right_r();
            let lap_number: i32 = packet_info.get_lap_number();

            let mut delta: String = String::from("");
            let mut new_best: bool = true;

            if packet_info.get_last_lap().is_some() {
                delta = packet_info.get_delta();
                new_best = delta.starts_with('-');
            }

            let mut rpm_lights: Vec<bool> = vec![false; 15];
            update_rpm_lights(current_rpm, max_rpm, &mut rpm_lights);

            weak_dashboard.upgrade_in_event_loop(move |dash: Dashboard| {
                dash.set_rpm(current_rpm);
                dash.set_speed(speed);
                dash.set_best_lap(SharedString::from(best_lap));
                dash.set_current_lap(SharedString::from(current_lap));
                dash.set_gear(gear);
                dash.set_accel(accel);
                dash.set_brake(brake);
                dash.set_position(position);
                dash.set_temp_left_f(temp_left_f);
                dash.set_temp_right_f(temp_right_f);
                dash.set_temp_left_r(temp_left_r);
                dash.set_temp_right_r(temp_right_r);
                dash.set_lap_number(lap_number);
                dash.set_rpm_lights(ModelRc::new(VecModel::from(rpm_lights)));
                dash.set_delta(SharedString::from(delta));
                dash.set_new_best(new_best);
                }).unwrap();
        }
    });

    dashboard.run().unwrap();
}
