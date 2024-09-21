use crate::telemetry::Telemetry;

use slint::{ModelRc, SharedString, VecModel};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

slint::include_modules!();

fn update_rpm_lights(rpm: f32, max_rpm: f32, rpm_lights: &mut Vec<bool>) {
    let starting_rpm: f32 = max_rpm * 0.5;
    let step_size: f32 = ((max_rpm - 1000.0) - starting_rpm) / (rpm_lights.len() as f32);

    for (i, on_status) in rpm_lights.iter_mut().enumerate() {
        *on_status = rpm >= starting_rpm + (step_size * i as f32);
    }
}

pub fn run_ui(telem: Arc<(Mutex<Telemetry>, Condvar)>) {
    let dashboard: Dashboard = Dashboard::new().unwrap();
    let weak_dashboard: slint::Weak<Dashboard> = dashboard.as_weak();

    thread::spawn(move || loop {
        let (lock, cvar) = &*telem;
        let telem = lock.lock().unwrap();
        let telem = cvar.wait(telem).unwrap();

        let current_rpm: f32 = telem.get_current_rpm();
        let max_rpm: f32 = telem.get_max_rpm();
        let speed: f32 = telem.get_speed();
        let best_lap: String = telem.get_best_lap();
        let current_lap: String = telem.get_current_lap();
        let gear: i32 = telem.get_gear();
        let accel: f32 = telem.get_accel();
        let brake: f32 = telem.get_brake();
        let position: i32 = telem.get_position();
        let temp_left_f: f32 = telem.get_temp_left_f();
        let temp_right_f: f32 = telem.get_temp_right_f();
        let temp_left_r: f32 = telem.get_temp_left_r();
        let temp_right_r: f32 = telem.get_temp_right_r();
        let lap_number: i32 = telem.get_lap_number();

        let mut rpm_lights: Vec<bool> = vec![false; 15];
        update_rpm_lights(current_rpm, max_rpm, &mut rpm_lights);

        let telem_copy = telem.clone();
        weak_dashboard
            .upgrade_in_event_loop(move |dash: Dashboard| {
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

                if lap_number > 2 {
                    let delta: String = telem_copy.get_delta();
                    dash.set_delta(SharedString::from(&delta));
                    dash.set_new_best(delta.starts_with('-'));
                }
            })
            .unwrap();

        cvar.notify_one();
    });

    dashboard.run().unwrap();
}
