use dashmap::DashMap;
use lemon_colonies_core::math::rect::Rect;
use tower_sessions_sqlx_store::sqlx::types::Uuid;

#[derive(Default)]
pub struct ChunkSubscriptions {
    subscriptions: DashMap<Uuid, Rect<i32>>,
}

impl ChunkSubscriptions {
    /// Returns the previous rect if there is one.
    pub fn subscribe(&self, id: Uuid, rect: Rect<i32>) -> Option<Rect<i32>> {
        let prev = self.subscriptions.get(&id).map(|r| *r);
        self.subscriptions.insert(id, rect);
        prev
    }

    pub fn unsubscribe(&self, id: Uuid) {
        self.subscriptions.remove(&id);
    }
}
