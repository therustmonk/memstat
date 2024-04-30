mod collector;

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
