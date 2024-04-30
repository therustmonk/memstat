use once_cell::sync::Lazy;
use size::Size;
use std::any::type_name;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem::size_of;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    RwLock,
};

pub struct MemTrack<T> {
    _type: PhantomData<T>,
}

impl<T> Default for MemTrack<T> {
    fn default() -> Self {
        count_in::<T>();
        Self { _type: PhantomData }
    }
}

impl<T> Drop for MemTrack<T> {
    fn drop(&mut self) {
        count_out::<T>();
    }
}

static COLLECTOR: Lazy<Collector> = Lazy::new(Collector::default);

fn count_in<T>() {
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

fn count_out<T>() {
    let name = type_name::<T>();
    let reader = COLLECTOR.inner.read().unwrap();
    if let Some(data) = reader.stats.get(&name) {
        data.counter.fetch_sub(1, Ordering::SeqCst);
    }
}

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
    struct Struct1(u64);

    impl Struct1 {
        pub fn new(value: u64) -> Self {
            count_in::<Self>();
            Self(value)
        }
    }

    #[allow(dead_code)]
    struct Struct2(u32);

    impl Struct2 {
        pub fn new(value: u32) -> Self {
            count_in::<Self>();
            Self(value)
        }
    }

    #[test]
    fn test_collector() {
        for x in 0..1_000_000 {
            let _1 = Struct1::new(x);
            let _2 = Struct2::new(x as u32);
        }
        print_report();
    }
}
