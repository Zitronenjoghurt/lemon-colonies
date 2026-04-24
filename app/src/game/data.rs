use crate::ws::fetchable::Fetchable;

#[derive(Default)]
pub struct ClientData {
    pub colony_positions: Fetchable<Vec<(i32, i32)>>,
}

impl ClientData {
    pub fn update(&mut self, ws: &mut crate::ws::Ws) {
        if self.colony_positions.needs_fetch() {
            self.colony_positions.set_pending();
            ws.request_colony_positions();
        }
    }
}
