use crate::ws::Ws;
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, Ui, Widget};

pub struct ConnectionStatusWidget<'a> {
    pub ws: &'a mut Ws,
}

impl<'a> ConnectionStatusWidget<'a> {
    pub fn new(ws: &'a mut Ws) -> Self {
        Self { ws }
    }
}

impl Widget for ConnectionStatusWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| match self.ws.state() {
            crate::ws::WsState::Idle => {
                ui.label("Disconnected");
                if ui.button("Connect").clicked() {
                    self.ws.connect();
                }
            }
            crate::ws::WsState::Connecting(_) => {
                ui.spinner();
                ui.label("Connecting...");
            }
            crate::ws::WsState::Connected(_) => {
                ui.label("Connected");
            }
            crate::ws::WsState::Error(err) => {
                ui.colored_label(egui::Color32::RED, err);
                if ui.button("Reconnect").clicked() {
                    self.ws.connect();
                }
            }
        })
        .response
    }
}
