use crate::websocket::connection::ConnectionId;
use dashmap::DashMap;
use lemon_colonies_core::math::coords::ChunkCoords;
use lemon_colonies_core::math::rect::Rect;

#[derive(Default)]
pub struct ChunkSubscriptions {
    subscriptions: DashMap<ConnectionId, Rect<i32>>,
}

impl ChunkSubscriptions {
    /// Returns the previous rect if there is one.
    pub fn subscribe(&self, id: ConnectionId, rect: Rect<i32>) -> Option<Rect<i32>> {
        metrics::gauge!("ws.subscribed_chunk_area").increment(rect.area() as f64);

        let prev = self.subscriptions.get(&id).map(|r| *r);
        if let Some(prev) = prev {
            metrics::gauge!("ws.subscribed_chunk_area").decrement(prev.area() as f64);
        }

        self.subscriptions.insert(id, rect);
        prev
    }

    pub fn unsubscribe(&self, id: ConnectionId) {
        if let Some((_, rect)) = self.subscriptions.remove(&id) {
            metrics::gauge!("ws.subscribed_chunk_area").decrement(rect.area() as f64);
        }
    }

    pub fn connections_for_chunk(&self, coords: ChunkCoords) -> Vec<ConnectionId> {
        self.subscriptions
            .iter()
            .filter(|entry| entry.value().contains_point(&coords.point()))
            .map(|entry| *entry.key())
            .collect()
    }
}
