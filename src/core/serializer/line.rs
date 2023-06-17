use std::fmt;

#[derive(Clone)]
pub struct Line {
    indent: usize,
    key: String,
    colon: bool,
    value: String,
    comma: bool,
    dict_start_bracket: bool,
    dict_end_bracket: bool,
    array_start_bracket: bool,
    array_end_bracket: bool,
}
impl Line {
    pub fn new() -> Self {
        Self {
            indent: 2,
            key: String::from(""),
            colon: false,
            value: String::from(""),
            comma: false,
            dict_start_bracket: false,
            dict_end_bracket: false,
            array_start_bracket: false,
            array_end_bracket: false,
        }
    }

    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn set_key(&mut self, key: String) {
        self.key = key;
    }

    pub fn need_colon(&mut self) {
        self.colon = true;
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn need_comma(&mut self) {
        self.comma = true;
    }

    pub fn need_dict_start_bracket(&mut self) {
        self.dict_start_bracket = true;
    }

    pub fn need_dict_end_bracket(&mut self) {
        self.dict_end_bracket = true;
    }

    pub fn need_array_start_bracket(&mut self) {
        self.array_start_bracket = true;
    }

    pub fn need_array_end_bracket(&mut self) {
        self.array_end_bracket = true;
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "{}{}{}{}{}{}{}{}{}",
            " ".repeat(self.indent),
            self.key,
            if self.colon { ": " } else { "" },
            self.value,
            if self.dict_start_bracket {
                "{"
            } else {
                ""
            },
            if self.dict_end_bracket { "}" } else { "" },
            if self.array_start_bracket {
                "["
            } else {
                ""
            },
            if self.array_end_bracket { "]" } else { "" },
            if self.comma { "," } else { "" },
        )
    }
}
