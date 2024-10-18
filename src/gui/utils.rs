use std::sync::{Arc, Condvar, Mutex};

use crate::telemetry::games::forza::ForzaTelemetry;

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    SwitchDashboard,
    UpdateTelemetry,
    UpdateForzaUI {
        current_rpm: f32,
        max_rpm: f32,
        speed: f32,
        best_lap: String,
        current_lap: String,
        delta: String,
        gear: i32,
        accel: f32,
        brake: f32,
        position: i32,
        temp_left_f: f32,
        temp_right_f: f32,
        temp_left_r: f32,
        temp_right_r: f32,
        lap_number: i32,
    },
}

#[derive(Default)]
pub enum DashboardVarient {
    #[default]
    None,
    Forza,
}

#[derive(Default, Clone)]
pub enum Telemetry {
    #[default]
    None,
    Forza(Arc<(Mutex<ForzaTelemetry>, Condvar)>),
}
