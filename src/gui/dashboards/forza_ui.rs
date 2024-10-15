use iced::widget::{column, text, Container};

use crate::gui::app::Dashboard;
use crate::gui::utils::Message;

pub fn forza_dashboard(dashboard: &Dashboard) -> Container<Message> {
    let rpm_lights = text!("**********************");
    let interface = Container::new(rpm_lights);
    interface
}
