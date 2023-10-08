use crate::data::{kvs::Kvs, kv::Kv, path::Path, tokens::Tokens};

pub struct Context {
    kvs: Kvs,
    last_indent: usize,
}
impl Context {
    pub fn new() -> Self {
        Context {
            kvs: Kvs::new(),
            last_indent: 0,
        }
    }

    pub fn get_kvs(&self) -> Kvs {
        self.kvs.clone()
    }

    pub fn push(&mut self, path: Path, value: Tokens) {
        self.kvs.push(Kv::with(path, value));
    }

    pub fn get_last_path(&self) -> Path {
        if let Some(last) = self.kvs.list().last() {
            return last.get_path();
        };
        Path::new()
    }

    pub fn get_last_indent(&self) -> usize {
        self.last_indent.clone()
    }

    pub fn set_last_indent(&mut self, indent: usize) {
        self.last_indent = indent;
    }
}
