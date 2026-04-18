use crate::http::{Http, RequestState};
use crate::ui::icon;
use crate::ui::widgets::username::UsernameWidget;
use egui_macroquad::egui::{Response, Ui, Widget};

pub struct ProfileMenu<'a> {
    pub http: &'a mut Http,
}

impl<'a> ProfileMenu<'a> {
    pub fn new(http: &'a mut Http) -> Self {
        Self { http }
    }
}

impl Widget for ProfileMenu<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| match &self.http.me {
            RequestState::Success(info) => {
                let label = format!("{} {}", icon::USER, info.public.username);
                ui.menu_button(label, |ui| {
                    if ui.button(format!("{} Logout", icon::SIGN_OUT)).clicked() {
                        self.http.do_logout();
                        ui.close_menu();
                    }
                });
            }
            _ => {
                UsernameWidget::new(self.http).ui(ui);
            }
        })
        .response
    }
}
