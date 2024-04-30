use std::collections::HashMap;

pub struct Data {
    pub size: usize,
    pub counter: usize,
}

#[derive(Default)]
pub struct MemStat {
    pub stats: HashMap<&'static str, Data>,
}

pub struct Reporter {}

impl Reporter {}
