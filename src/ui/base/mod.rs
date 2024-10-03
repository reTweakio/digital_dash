fn update_rpm_lights(rpm: f32, max_rpm: f32, rpm_lights: &mut Vec<bool>) {
    let starting_rpm: f32 = max_rpm * 0.5;
    let step_size: f32 = ((max_rpm - 1000.0) - starting_rpm) / (rpm_lights.len() as f32);

    for (i, on_status) in rpm_lights.iter_mut().enumerate() {
        *on_status = rpm >= starting_rpm + (step_size * i as f32);
    }
}
