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
        drop(reader);
        let mut writer = COLLECTOR.inner.write().unwrap();
        let size = size_of::<T>();
        let data = Data {
            size,
            counter: AtomicUsize::from(0),
        };
        writer.stats.insert(name, data);
        drop(writer);
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

#[cfg(test)]
mod test {
    use super::*;

    #[allow(dead_code)]
    struct Value(u64);

    impl Value {
        pub fn new(value: u64) -> Self {
            count_in::<Self>();
            Self(value)
        }
    }

    #[test]
    fn test_collector() {
        for x in 0..1_000_000 {
            let _value = Value::new(x);
        }
        print_report();
    }
}
