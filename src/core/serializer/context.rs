use crate::core::data::path::Path;

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
    status: Status,
    pub path: Path,
    buf: String,
    waiting_value: bool,
    // index
}
impl Context {
    pub fn new() -> Self {
        Context {
            status: Status::InSpace,
            path: Path::new(),
            buf: String::from(""),
            waiting_value: false,
        }
    }

    pub fn get_status(&self) -> Status {
        self.status.clone()
    }

    pub fn in_space(&self) -> bool {
        self.status == Status::InSpace
    }

    pub fn in_key(&self) -> bool {
        self.status == Status::InKey
    }

    pub fn in_null_value(&self) -> bool {
        self.status == Status::InNullValue
    }

    pub fn in_number_value(&self) -> bool {
        self.status == Status::InNumberValue
    }

    pub fn in_string_value(&self) -> bool {
        self.status == Status::InStringValue
    }

    pub fn in_bool_value(&self) -> bool {
        self.status == Status::InBoolValue
    }

    pub fn declare_in_space(&mut self) {
        self.status = Status::InSpace;
        self.buf = String::from("");
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
        // parent_is_dict
    }

    pub fn end_dict(&mut self) {
        self.pop_path();
    }

    pub fn start_array(&mut self) {
        self.waiting_value = true;
    }

    pub fn end_array(&mut self) {
        self.waiting_value = false;
    }

    pub fn is_waiting_value(&self) -> bool {
        self.waiting_value
    }

    pub fn push(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    pub fn resolve_as_path(&mut self) {
        self.path.push(&self.buf);
        self.buf = "".to_string();
        self.waiting_value = true;
        self.status = Status::InSpace;
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
