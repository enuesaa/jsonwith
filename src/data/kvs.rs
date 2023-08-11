use crate::data::kv::Kv;
use std::{fmt, vec};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn list(&self) -> Vec<Kv> {
        self.items.clone()
    }
}

impl From<Vec<Kv>> for Kvs {
    fn from(items: Vec<Kv>) -> Self {
        Kvs { items }
    }
}

impl fmt::Display for Kvs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::from("");
        for kv in self.items.iter() {
            let path = kv.get_path();
            let value = kv.get_value();
            ret += &format!("{}: {:?}\n", path, value);
        }
        write!(f, "{}", ret)
    }
}
