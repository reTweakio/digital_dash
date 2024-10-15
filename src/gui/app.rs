use std::sync::{Arc, Condvar, Mutex};

use iced::widget::{button, Container};

use crate::gui::dashboards::forza_ui;
use crate::gui::utils::{DashboardVarient, Message};

#[derive(Default)]
pub struct Dashboard {
    telemetry: Arc<(Mutex<DashboardVarient>, Condvar)>,
    current_dashboard: DashboardVarient,
}

impl Dashboard {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update => {}
            Message::SwitchDashboard => match self.current_dashboard {
                DashboardVarient::Forza => {}
            },
        }
    }

    pub fn view(&self) -> Container<Message> {
        match self.current_dashboard {
            DashboardVarient::Forza => forza_ui::forza_dashboard(&self),
        }
    }
}
