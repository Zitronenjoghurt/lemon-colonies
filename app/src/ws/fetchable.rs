use egui_macroquad::macroquad::prelude::get_time;

#[derive(Default)]
pub struct Fetchable<T> {
    value: Option<T>,
    state: FetchState,
    refetch_interval: Option<f64>,
}

#[derive(Default)]
enum FetchState {
    #[default]
    Idle,
    Pending {
        since: f64,
    },
    Done {
        at: f64,
    },
}

impl<T> Fetchable<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            state: FetchState::Idle,
            refetch_interval: None,
        }
    }

    pub fn with_refetch(mut self, interval_secs: f64) -> Self {
        self.refetch_interval = Some(interval_secs);
        self
    }

    pub fn needs_fetch(&self) -> bool {
        match &self.state {
            FetchState::Idle => true,
            FetchState::Pending { .. } => false,
            FetchState::Done { at } => match self.refetch_interval {
                Some(interval) => get_time() - at >= interval,
                None => false,
            },
        }
    }

    pub fn set_pending(&mut self) {
        self.state = FetchState::Pending { since: get_time() };
    }

    pub fn set_value(&mut self, value: T) {
        self.value = Some(value);
        self.state = FetchState::Done { at: get_time() };
    }

    pub fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.state, FetchState::Pending { .. })
    }
}
