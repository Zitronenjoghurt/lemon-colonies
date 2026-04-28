use crate::game::data::ClientData;
use crate::http::Http;
use crate::i18n::Translatable;
use crate::ui::icon;
use egui_macroquad::egui::{Response, Ui, Widget};
use lemon_colonies_core::lingo::Lingo::Logout;

pub struct ProfileMenu<'a> {
    data: &'a ClientData,
    http: &'a mut Http,
}

impl<'a> ProfileMenu<'a> {
    pub fn new(data: &'a ClientData, http: &'a mut Http) -> Self {
        Self { data, http }
    }
}

impl Widget for ProfileMenu<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| match &self.data.user_info.value() {
            Some(info) => {
                let label = format!("{} {}", icon::USER, info.public.username);
                ui.menu_button(label, |ui| {
                    if ui
                        .button(format!("{} {}", icon::SIGN_OUT, Logout.t()))
                        .clicked()
                    {
                        self.http.do_logout();
                        ui.close_menu();
                    }
                });
            }
            None => {
                ui.spinner();
                ui.label("Fetching user info...");
            }
        })
        .response
    }
}
