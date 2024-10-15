#[derive(Debug, Clone)]
pub enum Message {
    Update,
    SwitchDashboard,
}

#[derive(Default)]
pub enum DashboardVarient {
    #[default]
    Forza,
}
