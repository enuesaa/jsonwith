use crate::data::kv::Kv;
use crate::data::kvs::Kvs;
use crate::data::path::Path;
use crate::data::tokens::Tokens;

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

    pub fn mk_array(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::MkArray));
        self.path.push_index(0);
    }

    pub fn end_array(&mut self) {
        self.path.pop();
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::EndArray));
    }

    pub fn mk_dict(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::MkDict));
    }

    pub fn end_dict(&mut self) {
        self.path.pop();
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::EndDict));
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_buf(&self) -> String {
        self.buf.clone()
    }

    pub fn push_buf(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    pub fn is_waiting_value(&self) -> bool {
        if let Some(last) = self.kvs.list().last() {
            return last.get_path().to_string() != self.path.to_string();
        };
        true
    }

    pub fn resolve_as_path(&mut self) {
        if let Some(last) = self.kvs.list().last() {
            if last.get_value() != Tokens::MkArray && last.get_value() != Tokens::MkDict {
                self.path.pop();
            };
        };

        self.path.push_key(&self.buf);
        self.buf = "".to_string();
        self.status = Status::InSpace; // TODO: refactor
    }

    // TODO: should refcator argument value
    pub fn resolve_value(&mut self, value: Tokens) {
        let path = self.get_path();
        self.kvs.push(Kv::with(path, value));
        self.buf = String::from("");
        if self.path.is_last_index() {
            self.path.modify_index(self.path.get_last_index() + 1);
        };
    }

    pub fn get_kvs(&self) -> Kvs {
        self.kvs.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }
}
