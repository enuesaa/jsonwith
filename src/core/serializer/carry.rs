use crate::core::data::path::Path;

#[derive(PartialEq)]
enum ParseStatus {
    ParsingSpace,
    ParsingKey,
    ParsingValue,
}

// 何を serializer に任せるか境界線が曖昧
pub struct Carry {
    status: ParseStatus,
    path: Path,
    next_is_key: bool,
    buf: String,
}
impl Carry {
    pub fn new() -> Self {
        Carry {
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

    pub fn start_parsing_key(&mut self) {
        self.status = ParseStatus::ParsingKey;
    }

    pub fn should_start_parsing_key(&self) -> bool {
        self.next_is_key
    }

    pub fn start_parsing_value(&mut self) {
        self.status = ParseStatus::ParsingValue;
    }

    pub fn push(&mut self, c: char) {
        self.buf = self.buf.clone() + &c.to_string();
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

    pub fn start_dict(&self) {}

    pub fn end_dict(&mut self) {
        self.path.pop();
    }

    pub fn start_array(&self) {}

    pub fn end_array(&mut self) {
        self.path.pop();
    }

    pub fn get_path(&self) -> Path {
        self.path.clone()
    }

    pub fn get_buf(&self) -> String {
        self.buf.clone()
    }
}
