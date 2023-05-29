use crate::core::data::path::Path;

#[derive(PartialEq)]
enum Status {
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
}
impl Context {
    pub fn new() -> Self {
        Context {
            status: Status::InSpace,
            path: Path::new(),
            buf: String::from(""),
        }
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

    pub fn parent_is_dict(&self) -> bool {
        // check by path
        return false;
    }

    // pub fn should_escape(&self) -> bool {
    //     if let Some(last) = self.buf.chars().last() {
    //         // todo last から2番目が\でないかチェックする
    //         return last == '\\'
    //     }
    //     false
    // }

    pub fn push(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
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
