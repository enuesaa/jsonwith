use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    spaces: usize,
    key: String,
    need_colon: bool,
    value: String,
    need_comma: bool,
    need_start_dict_blancket: bool,
    need_end_dict_blancket: bool,
    need_start_list_blancket: bool,
    need_end_list_blancket: bool,
}
impl Line {
    pub fn new() -> Self {
        Self {
            spaces: 0,
            key: String::from(""),
            need_colon: false,
            value: String::from(""),
            need_comma: false,
            need_start_dict_blancket: false,
            need_end_dict_blancket: false,
            need_start_list_blancket: false,
            need_end_list_blancket: false,
        }
    }

    pub fn set_spaces(&mut self, spaces: usize) {
        self.spaces = spaces;
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

    pub fn enable_comma(&mut self) {
        self.need_comma = true;
    }

    pub fn disable_comma(&mut self) {
        self.need_comma = false;
    }

    pub fn enable_start_dict_blancket(&mut self) {
        self.need_start_dict_blancket = true;
    }

    pub fn disable_start_dict_blancket(&mut self) {
        self.need_start_dict_blancket = false;
    }

    pub fn enable_end_dict_blancket(&mut self) {
        self.need_end_dict_blancket = true;
    }

    pub fn disable_end_dict_blancket(&mut self) {
        self.need_end_dict_blancket = false;
    }

    pub fn enable_start_list_blancket(&mut self) {
        self.need_start_list_blancket = true;
    }

    pub fn disable_start_list_blancket(&mut self) {
        self.need_start_list_blancket = false;
    }

    pub fn enable_end_list_blancket(&mut self) {
        self.need_end_list_blancket = true;
    }

    pub fn disable_end_list_blancket(&mut self) {
        self.need_end_list_blancket = false;
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            " ".repeat(self.spaces),
            self.key,
            if self.need_colon { ": " } else { "" },
            self.value,
            if self.need_start_dict_blancket {
                "{"
            } else {
                ""
            },
            if self.need_end_dict_blancket { "}" } else { "" },
            if self.need_start_list_blancket {
                "["
            } else {
                ""
            },
            if self.need_end_list_blancket { "]" } else { "" },
            if self.need_comma { "," } else { "" },
        )
    }
}
