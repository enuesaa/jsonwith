use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    spaces: usize,
    pub need_hyphen: bool,
    key: String,
    need_colon: bool,
    value: String,
    need_empty_dict_blancket: bool,
    need_empty_list_blancket: bool,
}
impl Line {
    pub fn new() -> Self {
        Self {
            spaces: 0,
            need_hyphen: false,
            key: String::from(""),
            need_colon: false,
            value: String::from(""),
            need_empty_dict_blancket: false,
            need_empty_list_blancket: false,
        }
    }

    pub fn set_spaces(&mut self, spaces: usize) {
        self.spaces = spaces;
    }

    pub fn enable_hyphen(&mut self) {
        self.need_hyphen = true;
    }

    pub fn disable_hyphen(&mut self) {
        self.need_hyphen = false;
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn enable_colon(&mut self) {
        self.need_colon = true;
    }

    pub fn disable_colon(&mut self) {
        self.need_colon = false;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn enable_empty_dict_blancket(&mut self) {
        self.need_empty_dict_blancket = true;
    }

    pub fn disable_empty_dict_blancket(&mut self) {
        self.need_empty_dict_blancket = false;
    }

    pub fn enable_empty_list_blancket(&mut self) {
        self.need_empty_list_blancket = true;
    }

    pub fn disable_empty_list_blancket(&mut self) {
        self.need_empty_list_blancket = false;
    }

    pub fn is_hyphen_only(&mut self) -> bool {
        self.need_hyphen
            && self.key == *""
            && !self.need_colon
            && self.value == *""
            && !self.need_empty_dict_blancket
            && !self.need_empty_list_blancket
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}{}{}{}{}{}{}",
            " ".repeat(self.spaces),
            if self.need_hyphen { "- " } else { "" },
            self.key,
            if self.need_colon { ": " } else { "" },
            self.value,
            if self.need_empty_dict_blancket {
                "{}"
            } else {
                ""
            },
            if self.need_empty_list_blancket {
                "[]"
            } else {
                ""
            },
        )
    }
}
