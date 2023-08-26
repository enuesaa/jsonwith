use std::fmt;

use crate::data::kv::Kv;
use crate::data::path::Path;
use crate::data::tokens::Tokens;

#[derive(Debug, Clone)]
pub struct Line {
    kv_path: Path,
    kv_value: Tokens,
    indent: usize,
    hyphen: bool,
    key: String,
    colon: bool,
    value: String,
    empty_dict_blancket: bool,
    empty_array_blancket: bool,
    ln: bool,
}
impl From<Kv> for Line {
    fn from(kv: Kv) -> Self {
        Self {
            kv_path: kv.get_path(),
            kv_value: kv.get_value(),
            indent: 0,
            hyphen: false,
            key: String::from(""),
            colon: false,
            value: String::from(""),
            empty_dict_blancket: false,
            empty_array_blancket: false,
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

    pub fn enable_hyphen(&mut self) {
        self.hyphen = true;
    }

    pub fn disable_hyphen(&mut self) {
        self.hyphen = false;
    }

    pub fn set_key(&mut self, path: &Path) {
        if path.is_last_key() {
            self.key = path.get_last_key().to_string();
            self.enable_colon();
        };
    }

    pub fn enable_colon(&mut self) {
        self.colon = true;
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }

    pub fn enable_empty_dict_blancket(&mut self) {
        self.empty_dict_blancket = true;
    }

    pub fn enable_empty_array_blancket(&mut self) {
        self.empty_array_blancket = true;
    }

    pub fn enable_ln(&mut self) {
        self.ln = true;
    }

    pub fn disable_ln(&mut self) {
        self.ln = false;
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{}",
            " ".repeat(self.indent),
            if self.hyphen { "- " } else { "" },
            self.key,
            if self.colon { ": " } else { "" },
            self.value,
            if self.empty_dict_blancket { "{}" } else { "" },
            if self.empty_array_blancket { "[]" } else { "" },
            if self.ln { "\n" } else { "" },
        )
    }
}