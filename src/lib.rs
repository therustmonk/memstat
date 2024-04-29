use once_cell::sync::Lazy;
use size::Size;
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
        let mut writer = COLLECTOR.inner.write().unwrap();
        writer.stats.entry(name).or_insert_with(|| {
            let size = size_of::<T>();
            Data {
                size,
                counter: AtomicUsize::from(0),
            }
        });
        count_in::<T>();
    }
}

pub fn print_report() {
    let reader = COLLECTOR.inner.read().unwrap();
    for (name, data) in &reader.stats {
        let counter = data.counter.load(Ordering::SeqCst);
        let total = Size::from_bytes(counter * data.size);
        println!("{name} - {total}");
    }
}

#[derive(Default)]
struct Collector {
    inner: RwLock<CollectorInner>,
}

#[derive(Default)]
struct CollectorInner {
    stats: HashMap<&'static str, Data>,
}

struct Data {
    size: usize,
    counter: AtomicUsize,
}
