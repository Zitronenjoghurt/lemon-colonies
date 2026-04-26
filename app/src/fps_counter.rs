use egui_macroquad::macroquad::prelude::get_frame_time;

#[derive(Debug, Copy, Clone)]
pub struct FpsCounter {
    avg: f64,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self { avg: 60.0 }
    }
}

impl FpsCounter {
    pub fn update(&mut self) {
        let dt = get_frame_time() as f64;
        if dt > 0.0 {
            self.avg += (1.0 / dt - self.avg) * 0.05;
        }
    }

    pub fn fps(&self) -> f64 {
        self.avg
    }
}
