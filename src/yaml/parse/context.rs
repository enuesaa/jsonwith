use crate::data::{kv::Kv, kvs::Kvs, path::Path, tokens::Tokens};

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InDictOrArray,
    InKey,
    InValue,
    WaitingValue,
    WaitingNDictOrArray,
    WaitingN,
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
            status: Status::InDictOrArray,
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

    pub fn declare_in_dict_or_array(&mut self) {
        self.status = Status::InDictOrArray;
    }

    pub fn declare_in_key(&mut self) {
        self.status = Status::InKey;
    }

    pub fn declare_in_value(&mut self) {
        self.status = Status::InValue;
    }

    pub fn declare_waiting_value(&mut self) {
        self.status = Status::WaitingValue;
    }

    pub fn declare_waiting_n_dict_or_array(&mut self) {
        self.status = Status::WaitingNDictOrArray;
    }

    pub fn declare_waiting_n(&mut self) {
        self.status = Status::WaitingN;
    }

    pub fn push_buf(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    pub fn resolve_as_path(&mut self) {
        if let Some(_) = self.kvs.list().last() {
            self.path.pop();
        };

        self.path.push_key(&self.buf);
        self.buf = "".to_string();
    }

    pub fn resolve_value(&mut self) {
        let path = self.get_path();

        let buf = self.buf.clone();
        let value = match buf.as_str() {
            "null" => Tokens::Null,
            "false" => Tokens::Bool(false),
            "true" => Tokens::Bool(true),
            "" => Tokens::String(buf),
            _ => {
                if buf.chars().all(|c| c.is_numeric()) {
                    Tokens::Number(buf.parse::<usize>().unwrap())
                } else {
                    Tokens::String(buf)
                }
            },
        };
        self.kvs.push(Kv::with(path, value));
        self.buf = "".to_string();
    }

    pub fn start_array(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::MkArray));
    }

    pub fn start_dict(&mut self) {
        let path = self.get_path();
        self.kvs.push(Kv::with(path, Tokens::MkDict));
    }

    pub fn end_root_dict(&mut self) {
        self.kvs.push(Kv::with(Path::from("."), Tokens::EndDict));
    }
}
