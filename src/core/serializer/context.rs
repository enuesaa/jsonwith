use crate::core::data::kvs::Kvs;
use crate::core::data::path::Path;
use crate::core::data::kv::Kv;
use crate::core::data::tokens::Tokens;

#[derive(PartialEq)]
enum ParseStatus {
    ParsingSpace,
    ParsingKey,
    ParsingValue,
}

pub struct Context {
    kvs: Kvs,
    status: ParseStatus,
    path: Path,
    next_is_key: bool,
    buf: String,
}
impl Context {
    pub fn new() -> Self {
        Context {
            kvs: Kvs::new(),
            status: ParseStatus::ParsingSpace,
            path: Path::new(),
            next_is_key: true,
            buf: String::from(""),
        }
    }

    pub fn in_space(&self) -> bool {
        self.status == ParseStatus::ParsingSpace
    }

    pub fn in_key(&self) -> bool {
        self.status == ParseStatus::ParsingKey
    }

    pub fn in_value(&self) -> bool {
        self.status == ParseStatus::ParsingValue
    }

    pub fn declare_key(&mut self) {
        self.status = ParseStatus::ParsingKey;
    }

    pub fn declare_value(&mut self) {
        self.status = ParseStatus::ParsingValue;
    }

    pub fn declare_space(&mut self) {
        self.status = ParseStatus::ParsingSpace;
    }

    pub fn should_declare_key(&self) -> bool {
        self.next_is_key
    }

    pub fn should_escape(&self) -> bool {
        if let Some(last) = self.buf.chars().last() {
            // todo last から2番目が\でないかチェックする
            return last == '\\'
        }
        false
    }

    pub fn should_end_with_quotation(&self) -> bool {
        self.buf.starts_with("\"")
    }

    pub fn push(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
    }

    fn resolve_as_key(&mut self) {
        let buf = self.buf.clone();
        self.path.pop();
        self.path.push(&buf);
        self.buf = String::from("");
        self.next_is_key = false;

        self.status = ParseStatus::ParsingSpace;
    }

    fn resolve_as_value(&mut self) {
        // ここで値を返した方がいいか
        self.buf = String::from("");
        self.next_is_key = true;

        self.status = ParseStatus::ParsingSpace;
    }

    pub fn resolve(&mut self) {
        if self.in_key() {
            self.resolve_as_key();
        } else if self.in_value() {
            self.resolve_as_value();
        }
    }

    pub fn start_dict(&mut self) {
        self.kvs.push(Kv::new(self.get_path(), Tokens::MkDict))
    }

    pub fn end_dict(&mut self) {
        self.path.pop();
    }

    pub fn start_array(&mut self) {
        self.kvs.push(Kv::new(self.get_path(), Tokens::MkArray))
    }

    pub fn end_array(&mut self) {
        self.path.pop();
    }

    pub fn found_quotation(&mut self) {
        if self.in_space() {
            if self.should_declare_key() {
                self.declare_key();
            } else {
                self.declare_value();
                self.push('"');
            };
        };
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_buf(&self) -> String {
        self.buf.clone()
    }
}
