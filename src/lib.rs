use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static COLLECTOR: Lazy<Collector> = Lazy::new(Collector::default);

#[derive(Default)]
struct Collector {
    stats: HashMap<&'static str, Data>,
}

struct Data {}
