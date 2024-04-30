use crate::reporter::{Data, Snapshot};
use once_cell::sync::Lazy;
use std::any::type_name;
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    RwLock,
};

static COLLECTOR: Lazy<Collector> = Lazy::new(Collector::default);

pub fn count_in<T>() {
    let name = type_name::<T>();
    let reader = COLLECTOR.inner.read().unwrap();
    if let Some(data) = reader.stats.get(&name) {
        data.counter.fetch_add(1, Ordering::SeqCst);
    } else {
        drop(reader);
        let mut writer = COLLECTOR.inner.write().unwrap();
        let size = size_of::<T>();
        let data = CollectorData {
            size,
            counter: AtomicUsize::from(0),
        };
        writer.stats.insert(name, data);
        drop(writer);
        count_in::<T>();
    }
}

pub fn count_out<T>() {
    let name = type_name::<T>();
    let reader = COLLECTOR.inner.read().unwrap();
    if let Some(data) = reader.stats.get(&name) {
        data.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

pub fn get_snapshot() -> Snapshot {
    let blocking_reader = COLLECTOR.inner.write().unwrap();
    let mut mem_stat = Snapshot::default();
    for (name, data) in &blocking_reader.stats {
        mem_stat.stats.insert(name, data.extract());
    }
    mem_stat
}

/*
pub fn print_report() {
    println!("MEMSTAT REPORT ----------");
    let reader = COLLECTOR.inner.read().unwrap();
    for (name, data) in &reader.stats {
        let counter = data.counter.load(Ordering::SeqCst);
        let total = Size::from_bytes(counter * data.size);
        println!("{name} - {total}");
    }
    println!("-------------------------");
}
*/

#[derive(Default)]
struct Collector {
    inner: RwLock<CollectorInner>,
}

#[derive(Default)]
pub struct CollectorInner {
    stats: HashMap<&'static str, CollectorData>,
}

pub struct CollectorData {
    size: usize,
    counter: AtomicUsize,
}

impl CollectorData {
    fn extract(&self) -> Data {
        Data {
            size: self.size,
            counter: self.counter.load(Ordering::Relaxed),
        }
    }
}
