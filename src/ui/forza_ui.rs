use crate::telemetry::games::forza::ForzaTelemetry;
use iced::widget::{column, text, Column};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Update,
}

impl ForzaTelemetry {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Update => {}
        }
    }

    pub fn view(&self) -> Column<Message> {
        let gear = text("4");
        let interface = column![gear];
        interface
    }
}
