mod collector;
mod reporter;

pub use reporter::Reporter;
use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;

pub struct MemTrack<T> {
    _type: PhantomData<T>,
}

impl<T> Clone for MemTrack<T> {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<T> PartialOrd for MemTrack<T> {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}

impl<T> Ord for MemTrack<T> {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl<T> PartialEq for MemTrack<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for MemTrack<T> {}

impl<T> fmt::Debug for MemTrack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MemTrack")
    }
}

impl<T> Default for MemTrack<T> {
    fn default() -> Self {
        collector::count_in::<T>();
        Self { _type: PhantomData }
    }
}

impl<T> Drop for MemTrack<T> {
    fn drop(&mut self) {
        collector::count_out::<T>();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct Struct1 {
        _mem: MemTrack<Self>,
        value: u64,
    }

    impl Struct1 {
        pub fn new(value: u64) -> Self {
            Self {
                _mem: MemTrack::default(),
                value,
            }
        }
    }

    #[allow(dead_code)]
    struct Struct2 {
        _mem: MemTrack<Self>,
        value: u32,
    }

    impl Struct2 {
        pub fn new(value: u32) -> Self {
            Self {
                _mem: MemTrack::default(),
                value,
            }
        }
    }

    #[test]
    fn test_collector() {
        let mut data_1 = Vec::new();
        let mut data_2 = Vec::new();
        let reporter = Reporter::spawn(100, "report.txt");
        for x in 0..1_000_000 {
            let value_1 = Struct1::new(x);
            data_1.push(value_1.clone());
            data_1.push(value_1);
            let value_2 = Struct2::new(x as u32);
            data_2.push(value_2);
        }
        reporter.join().unwrap();
    }
}
