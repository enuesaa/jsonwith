use crate::data::kvs::Kvs;

pub struct Context {
    kvs: Kvs,
}
impl Context {
    pub fn new() -> Self {
        Context {
            kvs: Kvs::new(),
        }
    }

    pub fn get_kvs(&self) -> Kvs {
        self.kvs.clone()
    }
}
