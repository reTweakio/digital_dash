use iced::widget::Container;
use iced::Task;

use crate::gui::dashboards::forza_ui;
use crate::gui::utils::{DashboardVarient, Message, Telemetry};

#[derive(Default)]
pub struct Dashboard {
    telemetry: Telemetry,
    current_dashboard: DashboardVarient,
}

impl Dashboard {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoOp => Task::none(),

            Message::SwitchDashboard => match self.current_dashboard {
                DashboardVarient::None => Task::none(),
                DashboardVarient::Forza => Task::none(),
            },

            Message::UpdateForzaUI {
                current_rpm,
                max_rpm,
                speed,
                best_lap,
                current_lap,
                delta,
                gear,
                accel,
                brake,
                position,
                temp_left_f,
                temp_right_f,
                temp_left_r,
                temp_right_r,
                lap_number,
            } => Task::none(),

            Message::UpdateTelemetry => {
                let telem_clone = self.telemetry.clone();
                Task::perform(
                    async {
                        match telem_clone {
                            Telemetry::None => Message::NoOp,
                            Telemetry::Forza(telemetry) => {
                                let (lock, cvar) = &*telemetry;
                                let data = cvar.wait(lock.lock().unwrap()).unwrap();

                                Message::UpdateForzaUI {
                                    current_rpm: data.get_current_rpm(),
                                    max_rpm: data.get_max_rpm(),
                                    speed: data.get_speed(),
                                    best_lap: data.get_best_lap(),
                                    current_lap: data.get_current_lap(),
                                    delta: data.get_delta(),
                                    gear: data.get_gear(),
                                    accel: data.get_accel(),
                                    brake: data.get_brake(),
                                    position: data.get_position(),
                                    temp_left_f: data.get_temp_left_f(),
                                    temp_right_f: data.get_temp_right_f(),
                                    temp_left_r: data.get_temp_left_r(),
                                    temp_right_r: data.get_temp_right_r(),
                                    lap_number: data.get_lap_number(),
                                }
                            }
                        }
                    },
                    |message| message,
                )
            }
        }
    }

    pub fn view(&self) -> Container<Message> {
        match self.current_dashboard {
            DashboardVarient::None => forza_ui::forza_dashboard(&self),
            DashboardVarient::Forza => forza_ui::forza_dashboard(&self),
        }
    }
}
