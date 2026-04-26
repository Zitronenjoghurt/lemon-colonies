use crate::ws::Ws;
use egui_macroquad::macroquad::prelude::get_time;
use std::collections::VecDeque;
use std::fmt::Display;

const SAMPLE_COUNT: usize = 18;
const HEARTBEAT_INTERVAL: f64 = 5.0;
const HEARTBEAT_TIMEOUT: f64 = 15.0;

#[derive(Debug, Copy, Clone)]
struct SyncSample {
    offset: f64,
    rtt: f64,
}

pub struct ServerTime {
    samples: VecDeque<SyncSample>,
    offset: f64,
    rtt: f64,
    pending_since: Option<f64>,
    last_pong: f64,
    last_ping: f64,
}

impl Default for ServerTime {
    fn default() -> Self {
        Self {
            samples: VecDeque::with_capacity(SAMPLE_COUNT),
            offset: 0.0,
            rtt: 0.0,
            pending_since: None,
            last_pong: get_time(),
            last_ping: f64::NEG_INFINITY,
        }
    }
}

impl ServerTime {
    pub fn update(&mut self, ws: &mut Ws) {
        if !ws.is_connected() {
            return;
        }

        let now = get_time();
        if now - self.last_ping >= HEARTBEAT_INTERVAL {
            self.last_ping = now;
            self.pending_since = Some(now);
            ws.ping(now);
        }
    }

    pub fn handle_pong(&mut self, client_time: f64, server_time: f64) {
        let now = get_time();
        self.last_pong = now;
        self.pending_since = None;

        let rtt = now - client_time;
        let offset = server_time - client_time - rtt / 2.0;

        let sample = SyncSample { offset, rtt };

        if self.samples.len() >= SAMPLE_COUNT {
            self.samples.pop_front();
        }
        self.samples.push_back(sample);

        self.recalculate();
    }

    fn recalculate(&mut self) {
        if self.samples.is_empty() {
            return;
        }

        let mut sorted = self.samples.iter().copied().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.rtt.partial_cmp(&b.rtt).unwrap());
        sorted.truncate(SAMPLE_COUNT / 3);

        let new_offset = sorted.iter().map(|s| s.offset).sum::<f64>() / sorted.len() as f64;
        let new_rtt = sorted.iter().map(|s| s.rtt).sum::<f64>() / sorted.len() as f64;

        if self.samples.len() <= 3 {
            self.offset = new_offset;
            self.rtt = new_rtt;
        } else {
            self.offset += (new_offset - self.offset) * 0.3;
            self.rtt += (new_rtt - self.rtt) * 0.3;
        }
    }

    pub fn ready(&self) -> bool {
        !self.samples.is_empty()
    }

    pub fn is_timed_out(&self) -> bool {
        self.ready() && get_time() - self.last_pong >= HEARTBEAT_TIMEOUT
    }

    pub fn to_local(&self, server_time: f64) -> f64 {
        server_time - self.offset
    }

    pub fn now(&self) -> f64 {
        get_time() + self.offset
    }

    pub fn elapsed_since(&self, server_time: f64) -> f64 {
        self.now() - server_time
    }

    pub fn latency(&self) -> f64 {
        self.rtt
    }
}

pub struct DateTime {
    pub year: i64,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
    pub fn from_unix(ts: f64) -> Self {
        let secs = ts as i64;
        let days = secs.div_euclid(86400);
        let time_of_day = secs.rem_euclid(86400);

        let hour = (time_of_day / 3600) as u8;
        let minute = ((time_of_day % 3600) / 60) as u8;
        let second = (time_of_day % 60) as u8;

        // Days since 1970-01-01, using the civil calendar algorithm
        // https://howardhinnant.github.io/date_algorithms.html
        let z = days + 719468;
        let era = z.div_euclid(146097);
        let doe = z.rem_euclid(146097);
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        let year = if month <= 2 { y + 1 } else { y };

        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}
