use std::fmt;
use crate::core::data::kv::Kv;

pub struct Collection {
    pub items: Vec<Kv>,
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}
