use std::fmt;

use crate::data::kv::Kv;
use crate::data::path::Path;
use crate::data::tokens::Tokens;

#[derive(Debug, Clone)]
pub struct Line {
    kv_path: Path,
    kv_value: Tokens,
    indent: usize,
    key: String,
    colon: bool,
    value: String,
    comma: bool,
    dict_start_bracket: bool,
    dict_end_bracket: bool,
    array_start_bracket: bool,
    array_end_bracket: bool,
    ln: bool,
}
impl From<Kv> for Line {
    fn from(kv: Kv) -> Self {
        Self {
            kv_path: kv.get_path(),
            kv_value: kv.get_value(),
            indent: 0,
            key: String::from(""),
            colon: false,
            value: String::from(""),
            comma: false,
            dict_start_bracket: false,
            dict_end_bracket: false,
            array_start_bracket: false,
            array_end_bracket: false,
            ln: false,
        }
    }
}

impl Line {
    pub fn get_kv_path(&self) -> Path {
        self.kv_path.clone()
    }

    pub fn get_kv_value(&self) -> Tokens {
        self.kv_value.clone()
    }

    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn set_key(&mut self, path: &Path) {
        if path.is_last_key() {
            self.key = format!("\"{}\"", path.get_last_key());
            self.enable_colon();
        };
    }

    pub fn enable_colon(&mut self) {
        self.colon = true;
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn set_string_value(&mut self, value: &str) {
        self.value = format!("\"{}\"", value);
    }

    pub fn enable_comma(&mut self) {
        self.comma = true;
    }

    pub fn disable_comma(&mut self) {
        self.comma = false;
    }

    pub fn enable_dict_start_bracket(&mut self) {
        self.dict_start_bracket = true;
    }

    pub fn enable_dict_end_bracket(&mut self) {
        self.dict_end_bracket = true;
    }

    pub fn enable_array_start_bracket(&mut self) {
        self.array_start_bracket = true;
    }

    pub fn enable_array_end_bracket(&mut self) {
        self.array_end_bracket = true;
    }

    pub fn enable_ln(&mut self) {
        self.ln = true;
    }

    pub fn disable_ln(&mut self) {
        self.ln = false;
    }

    pub fn is_dict_start_bracket(&self) -> bool {
        self.dict_start_bracket.clone()
    }

    pub fn is_array_start_bracket(&self) -> bool {
        self.array_start_bracket.clone()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}",
            " ".repeat(self.indent),
            self.key,
            if self.colon { ": " } else { "" },
            self.value,
            if self.dict_start_bracket { "{" } else { "" },
            if self.dict_end_bracket { "}" } else { "" },
            if self.array_start_bracket { "[" } else { "" },
            if self.array_end_bracket { "]" } else { "" },
            if self.comma { "," } else { "" },
            if self.ln { "\n" } else { "" },
        )
    }
}
