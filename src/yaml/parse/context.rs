use crate::data::kvs::Kvs;

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InSpace,
    InKey,
    InNull,
    InNumber,
    InString,
    InBool,
}

pub struct Context {}
impl Context {
    pub fn new() -> Self {
        Context {}
    }

    pub fn get_kvs(&self) -> Kvs {
        Kvs::new()
    }

    pub fn get_status(&self) -> Status {
        Status::InBool
    }
}