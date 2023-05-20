use std::fmt;
use crate::core::data::kv::Kv;

pub struct Kvs {
    items: Vec<Kv>,
}

impl Kvs {
    pub fn new() -> Self {
        Kvs { items: vec![] }
    }

    pub fn push(&mut self, kv: Kv) {
        self.items.push(kv)
    }
}

impl fmt::Display for Kvs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
