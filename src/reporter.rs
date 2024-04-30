use super::collector;
use anyhow::Error;
use size::Size;
use std::cmp::Ordering as Order;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

pub struct Data {
    pub size: usize,
    pub counter: usize,
}

impl Data {
    pub fn total(&self) -> usize {
        self.size * self.counter
    }

    pub fn by_total(left: &Self, right: &Self) -> Order {
        right.total().cmp(&left.total())
    }
}

#[derive(Default)]
pub struct Snapshot {
    pub stats: HashMap<&'static str, Data>,
}

pub struct Reporter {
    started: Instant,
    active: Arc<AtomicBool>,
    interval: Duration,
    file_name: String,
}

impl Reporter {
    pub fn spawn(rate_ms: u64, file_name: &str) -> ReporterHandle {
        let active = Arc::new(AtomicBool::new(true));
        let mut this = Self {
            started: Instant::now(),
            active: active.clone(),
            interval: Duration::from_millis(rate_ms),
            file_name: file_name.to_string(),
        };
        let handle = spawn(move || this.routine());
        ReporterHandle { handle, active }
    }

    fn routine(&mut self) -> Result<(), Error> {
        let mut file = File::create(&self.file_name)?;
        while self.active.load(Ordering::Relaxed) {
            let elapsed = self.started.elapsed();
            let snapshot = collector::get_snapshot();
            write!(file, "Elapsed: {}\n", elapsed.as_millis())?;
            let mut stats: Vec<_> = snapshot.stats.into_iter().collect();
            stats.sort_by(|(_, l), (_, r)| Data::by_total(l, r));
            for (name, data) in stats {
                write!(file, "{name}: {}\n", Size::from_bytes(data.total()))?;
            }
            write!(file, "\n")?;
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
