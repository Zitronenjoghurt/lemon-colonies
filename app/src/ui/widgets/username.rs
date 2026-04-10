use crate::http::{Http, RequestState};
use egui_macroquad::egui;
use egui_macroquad::egui::{Response, Ui, Widget};

pub struct UsernameWidget<'a> {
    pub http: &'a mut Http,
}

impl<'a> UsernameWidget<'a> {
    pub fn new(http: &'a mut Http) -> Self {
        Self { http }
    }
}

impl Widget for UsernameWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| match &self.http.me {
            RequestState::Idle => {
                if ui.button("Retry").clicked() {
                    self.http.fetch_me();
                };
            }
            RequestState::Loading(_) => {
                ui.spinner();
                ui.label("Fetching user info...");
            }
            RequestState::Success(info) => {
                ui.label(info.public.username.as_str());
            }
            RequestState::Error(err) => {
                let mut retry = false;
                ui.vertical(|ui| {
                    ui.colored_label(egui::Color32::RED, format!("Failed: {}", err));
                    if ui.button("Retry").clicked() {
                        retry = true;
                    }
                });
                if retry {
                    self.http.fetch_me();
                }
            }
        })
        .response
    }
}
