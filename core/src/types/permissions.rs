use bitflags::bitflags;

bitflags! {
    pub struct Permissions: i64 {
        const METRICS = 1 << 0;
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self::empty()
    }
}
