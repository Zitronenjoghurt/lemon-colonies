use crate::websocket::connection::ConnectionId;
use dashmap::DashMap;
use lemon_colonies_core::math::point::Point;
use lemon_colonies_core::math::rect::Rect;

#[derive(Default)]
pub struct ChunkSubscriptions {
    subscriptions: DashMap<ConnectionId, Rect<i32>>,
}

impl ChunkSubscriptions {
    /// Returns the previous rect if there is one.
    pub fn subscribe(&self, id: ConnectionId, rect: Rect<i32>) -> Option<Rect<i32>> {
        let prev = self.subscriptions.get(&id).map(|r| *r);
        self.subscriptions.insert(id, rect);
        prev
    }

    pub fn unsubscribe(&self, id: ConnectionId) {
        self.subscriptions.remove(&id);
    }

    pub fn connections_for_chunk(&self, coords: &Point<i32>) -> Vec<ConnectionId> {
        self.subscriptions
            .iter()
            .filter(|entry| entry.value().contains(coords))
            .map(|entry| *entry.key())
            .collect()
    }
}
