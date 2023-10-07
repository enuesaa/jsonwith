use crate::data::{kv::Kv, kvs::Kvs, path::Path, tokens::Tokens};

#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InSpace,
    InKey,
    InString,
    WaitingValue,
    WaitingNewline,
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

    pub fn declare_in_string(&mut self) {
        self.status = Status::InString;
    }

    pub fn declare_waiting_value(&mut self) {
        self.status = Status::WaitingValue;
    }

    pub fn declare_waiting_newline(&mut self) {
        self.status = Status::WaitingNewline;
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

        let buf = self.buf.clone();
        let value = match buf.as_str() {
            "null" => Tokens::Null,
            "false" => Tokens::Bool(false),
            "true" => Tokens::Bool(true),
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

    pub fn start_root_dict(&mut self) {
        self.kvs.push(Kv::with(Path::from("."), Tokens::MkDict));
    }

    pub fn end_root_dict(&mut self) {
        self.kvs.push(Kv::with(Path::from("."), Tokens::EndDict));
    }
}
