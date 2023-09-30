use std::time::Duration;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::templates::{Timing, TimingType};

pub struct Data {
    timings: Vec<Timing>,
    current_running: Option<Timing>,
    id: usize,
}

impl Data {
    fn clear(&mut self) {
        self.timings.clear();
        self.current_running = None;
        self.id = 0;
    }
}

fn now() -> u64 {
    return std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_millis() as u64;
}

lazy_static::lazy_static! {
    static ref DATA: Mutex<Data> = Mutex::new(Data {
        timings: Vec::new(),
        current_running: None,
        id: 0,
    });
}

pub async fn push_timing(timing_type: TimingType) -> Result<()> {
    let mut data = DATA.lock().await;
    if let Some(mut timing) = data.current_running.take() {
        if timing.timing_type == timing_type {
            timing.stop = now();
            return Ok(());
        }
        data.current_running = Some(timing);
        return Err(anyhow::anyhow!("Mismatched timing type"));
    }

    let timing = Timing {
        timing_type,
        start: now(),
        stop: 0,
        id: data.id,
    };

    data.id += 1;
    data.timings.push(timing);

    return Ok(());
}

pub async fn get_timings() -> Vec<Timing> {
    let data = DATA.lock().await;
    return data.timings.clone();
}

pub async fn clear_timings() {
    let mut data = DATA.lock().await;
    data.clear();
}

