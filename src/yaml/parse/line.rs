#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InIndent,
    InKey,
    InValue,
    WouldBreakLine,
    BreakLine,
}

pub struct Line {
    status: Status,
    indent: usize,
    has_hyphen: bool,
    key: String,
    value: String,
}
impl Line {
    pub fn new() -> Self {
        Self {
            status: Status::InIndent,
            indent: 0,
            has_hyphen: false,
            key: String::from(""),
            value: String::from(""),
        }
    }

    pub fn push(&mut self, c: char) {
        if c == '\\' {
            self.status = Status::WouldBreakLine;
            return;
        }
        if self.status == Status::WouldBreakLine {
            match c {
                'n' => {
                    self.status = Status::BreakLine;
                    return;
                },
                _ => {
                    // TODO: fix
                    self.status = Status::InValue;
                },
            }
        };

        if self.status == Status::InIndent {
            match c {
                ' ' => {
                    self.indent += 1;
                },
                '-' => {
                    self.has_hyphen = true;
                    self.status = Status::InKey;
                },
                _ => {
                    self.key = String::from(c);
                    self.status = Status::InKey;
                },
            };
            return;
        };

        if self.status == Status::InKey {
            match c {
                ' ' => {},
                ':' => {
                    self.status = Status::InValue;
                },
                _ => {
                    self.key += &String::from(c);
                },
            }
            return;
        }

        if self.status == Status::InValue {
            self.value += &String::from(c);
            return;
        }
    }

    pub fn get_indent(&self) -> usize {
        self.indent.clone()
    }

    pub fn get_has_hyphen(&self) -> bool {
        self.has_hyphen.clone()
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    pub fn is_ended(&self) -> bool {
        return self.status == Status::BreakLine;
    }
}
