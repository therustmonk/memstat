mod collector;
mod reporter;

pub use collector::print_report;
use std::marker::PhantomData;

pub struct MemTrack<T> {
    _type: PhantomData<T>,
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
        for x in 0..1_000_000 {
            let value_1 = Struct1::new(x);
            data_1.push(value_1);
            let value_2 = Struct2::new(x as u32);
            data_2.push(value_2);
            if x % 1000 == 0 {
                print_report();
            }
        }
    }
}
