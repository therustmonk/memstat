use super::collector;
use anyhow::Error;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

pub struct Data {
    pub size: usize,
    pub counter: usize,
}

#[derive(Default)]
pub struct Snapshot {
    pub stats: HashMap<&'static str, Data>,
}

pub struct Reporter {
    active: Arc<AtomicBool>,
    interval: Duration,
}

impl Reporter {
    pub fn spawn(ms: u64) -> ReporterHandle {
        let active = Arc::new(AtomicBool::new(true));
        let mut this = Self {
            active: active.clone(),
            interval: Duration::from_millis(ms),
        };
        let handle = spawn(move || this.routine());
        ReporterHandle { handle, active }
    }

    fn routine(&mut self) -> Result<(), Error> {
        while self.active.load(Ordering::Relaxed) {
            let snapshot = collector::get_snapshot();
            sleep(self.interval);
        }
        Ok(())
    }
}

pub struct ReporterHandle {
    handle: JoinHandle<Result<(), Error>>,
    active: Arc<AtomicBool>,
}

impl ReporterHandle {
    pub fn join(self) -> Result<(), Error> {
        self.active.store(false, Ordering::Relaxed);
        self.handle
            .join()
            .map_err(|_| Error::msg("Reporter thread failed"))?
    }
}
