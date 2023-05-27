use std::fmt;
use std::fmt::Debug;
use crate::core::data::kv::Kv;

#[derive(Clone, Debug)]
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

    pub fn render(&self) {
        for kv in self.items.iter() {
            let path = kv.path();
            let value = kv.value();
            print!("{}: {:?}\n", path, value);
        };
    }
}

impl fmt::Display for Kvs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::from("");
        for kv in self.items.iter() {
            let path = kv.path();
            let value = kv.value();
            ret += &format!("{}: {:?}\n", path, value);
        };
        write!(f, "{}", ret)
    }
}