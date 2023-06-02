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
    status: Status,
    pub path: Path,
    buf: String,
    waiting_value: bool,
    parent_is_array: bool,
    index: usize,
}
impl Context {
    pub fn new() -> Self {
        Context {
            kvs: Kvs::new(),
            status: Status::InSpace,
            path: Path::new(),
            buf: String::from(""),
            waiting_value: false,
            parent_is_array: false,
            index: 0,
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn declare_in_space(&mut self) {
        self.status = Status::InSpace;
        self.waiting_value = false;
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
        self.pop_path();
    }

    pub fn start_array(&mut self) {
        self.parent_is_array = true;
        self.waiting_value = true;
        let path = self.get_path();
        self.kvs.push(Kv { path, value: Tokens::MkArray });
    }

    pub fn end_array(&mut self) {
        self.parent_is_array = false;
        self.waiting_value = false;
        self.index = 0;
    }

    pub fn is_waiting_value(&self) -> bool {
        self.waiting_value
    }

    pub fn push_buf(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    pub fn resolve_as_path(&mut self) {
        self.path.push(&self.buf);
        self.buf = "".to_string();
        self.waiting_value = true;
        self.status = Status::InSpace;
    }

    pub fn resolve_value(&mut self, value: Tokens) {
        // フラグをとりたいなあ
        if self.parent_is_array {
            if self.index > 0 {
                self.pop_path();
            }
            self.push_path(&self.index.to_string());
        };
        let path = self.get_path();
        self.kvs.push(Kv { path, value });
        self.buf = String::from("");
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

    pub fn pop_path(&mut self) {
        self.path.pop();
    }
}
