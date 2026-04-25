use crate::ws::fetchable::Fetchable;
use lemon_colonies_core::math::coords::ChunkCoords;
use lemon_colonies_core::types::user_info::PrivateUserInfo;
use std::collections::HashSet;

#[derive(Default)]
pub struct ClientData {
    pub colony_positions: Fetchable<HashSet<ChunkCoords>>,
    pub owned_chunks: Fetchable<HashSet<ChunkCoords>>,
    pub user_info: Fetchable<PrivateUserInfo>,
}

impl ClientData {
    pub fn update(&mut self, ws: &mut crate::ws::Ws) {
        if self.colony_positions.needs_fetch() {
            self.colony_positions.set_pending();
            ws.request_colony_positions();
        }

        if self.owned_chunks.needs_fetch() {
            self.owned_chunks.set_pending();
            ws.request_owned_chunks();
        }

        if self.user_info.needs_fetch() {
            self.user_info.set_pending();
            ws.request_user_info();
        }
    }
}
