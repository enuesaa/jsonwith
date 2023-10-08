#[derive(PartialEq, Clone, Debug)]
pub enum Status {
    InIndent,
    InKey,
    InValue,
    WouldBreakLine,
    BreakLine,
}

#[derive(Clone)]
pub struct Line {
    status: Status,
    indent: usize,
    hyphen: bool,
    key: Option<String>,
    value: Option<String>,
}
impl Line {
    pub fn new() -> Self {
        Self {
            status: Status::InIndent,
            indent: 0,
            hyphen: false,
            key: None,
            value: None,
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
                    self.hyphen = true;
                    self.indent += 2;
                    self.status = Status::InKey;
                },
                _ => {
                    self.key = Some(String::from(c));
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
                    if let Some(k) = self.key.clone() {
                        self.key = Some(k + &String::from(c));
                    };
                },
            }
            return;
        }

        if self.status == Status::InValue {
            if c == ' ' {
                return;
            }
            if let Some(v) = self.value.clone() {
                self.value = Some(v + &String::from(c));
            } else {
                self.value = Some(String::from(c));
            };
            return;
        }
    }

    pub fn get_indent(&self) -> usize {
        self.indent.clone()
    }

    pub fn has_hyphen(&self) -> bool {
        self.hyphen.clone()
    }

    pub fn get_key(&self) -> String {
        if let Some(key) = self.key.clone() {
            return key.to_string();
        }
        String::from("")
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    pub fn get_value(&self) -> String {
        if let Some(value) = self.value.clone() {
            return value.to_string();
        }
        String::from("")
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_ended(&self) -> bool {
        return self.status == Status::BreakLine;
    }
}
