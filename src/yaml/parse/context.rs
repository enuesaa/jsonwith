use crate::data::{kvs::Kvs, path::Path, kv::Kv, tokens::Tokens};

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InSpace,
    InKey,
    InNull,
    InNumber,
    InString,
    InBool,
    InWaitingValue,
}

pub struct Context {
    kvs: Kvs,
    status: Status,
    path: Path,
    buf: String,
}
impl Context {
    pub fn new() -> Self {
        Context {
            kvs: Kvs::new(),
            status: Status::InSpace,
            path: Path::new(),
            buf: String::from(""),
        }
    }

    pub fn get_kvs(&self) -> Kvs {
        self.kvs.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn declare_in_space(&mut self) {
        self.status = Status::InSpace;
    }

    pub fn declare_in_key(&mut self) {
        self.status = Status::InKey;
    }

    pub fn declare_in_null(&mut self) {
        self.status = Status::InNull;
    }

    pub fn declare_in_number(&mut self) {
        self.status = Status::InNumber;
    }

    pub fn declare_in_string(&mut self) {
        self.status = Status::InString;
    }

    pub fn declare_in_bool(&mut self) {
        self.status = Status::InBool;
    }

    pub fn declare_in_waiting_value(&mut self) {
        self.status = Status::InWaitingValue;
    }

    pub fn push_buf(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    pub fn resolve_as_path(&mut self) {
        self.path.push_key(&self.buf);
        self.buf = "".to_string();
    }

    pub fn resolve_value(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::String(self.buf.clone())));
        self.buf = "".to_string();
    }

    pub fn start_root_dict(&mut self) {
        self.kvs.push(Kv::with(Path::from("."), Tokens::MkDict));
    }

    pub fn end_root_dict(&mut self) {
        self.kvs.push(Kv::with(Path::from("."), Tokens::EndDict));
    }
}
