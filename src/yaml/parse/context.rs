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

pub struct Context {
    status: Status,
}
impl Context {
    pub fn new() -> Self {
        Context {
            status: Status::InSpace,
        }
    }

    pub fn get_kvs(&self) -> Kvs {
        Kvs::new()
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }
}