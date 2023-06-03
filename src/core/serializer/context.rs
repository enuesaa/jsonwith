use crate::core::data::kv::Kv;
use crate::core::data::path::Path;
use crate::core::data::kvs::Kvs;
use crate::core::data::tokens::Tokens;

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InSpace,
    InKey,
    InNullValue,
    InNumberValue,
    InStringValue,
    InBoolValue,
}

pub struct Context {
    pub kvs: Kvs,
    pub status: Status,
    pub path: Path,
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
    
    pub fn declare_in_null_value(&mut self) {
        self.status = Status::InNullValue;
    }
    
    pub fn declare_in_number_value(&mut self) {
        self.status = Status::InNumberValue;
    }
    
    pub fn declare_in_string_value(&mut self) {
        self.status = Status::InStringValue;
    }
    
    pub fn declare_in_bool_value(&mut self) {
        self.status = Status::InBoolValue;
    }

    pub fn start_dict(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv { path, value: Tokens::MkDict });
    }

    pub fn end_dict(&mut self) {
        self.pop_path_if_in_dict();
    }

    pub fn start_array(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv { path, value: Tokens::MkArray });
        self.path.nest_array();
    }

    pub fn end_array(&self) {}

    pub fn is_waiting_value(&self) -> bool {
        if let Some(last) = self.kvs.items.last() {
            return last.path.to_string() != self.path.to_string()
        };
        false
    }

    pub fn push_buf(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    pub fn resolve_as_path(&mut self) {
        self.path.push(&self.buf);
        self.buf = "".to_string();
        self.status = Status::InSpace;
    }

    pub fn resolve_value(&mut self, value: Tokens) {
        let path = self.get_path();
        self.kvs.push(Kv { path, value });
        self.buf = String::from("");
        if self.path.is_array() {
            self.path.increment();
        };
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_buf(&self) -> String {
        self.buf.clone()
    }

    pub fn push_path(&mut self, nest: &str) {
        self.path.push(nest);
    }

    pub fn pop_path_if_in_dict(&mut self) {
        if !self.path.is_array() {
            self.path.pop();
        };
    }
}
