use crate::data::tokens::Tokens;
use crate::yaml::render::line::Line;
use crate::yaml::render::processor::Processor;

pub struct MappingProcessor {
    indent: usize,
    spaces: usize,
    lines: Vec<Line>,
    should_index_if_next_is_array: bool,
}
impl MappingProcessor {
    pub fn new() -> Self {
        Self {
            indent: 2,
            spaces: 0,
            lines: Vec::new(),
            should_index_if_next_is_array: false,
        }
    }

    fn increment_space(&mut self) {
        self.spaces += self.indent.clone();
    }

    fn decrement_space(&mut self) {
        if self.spaces > 0 {
            self.spaces -= self.indent.clone();
        };
    }

    fn is_root(&self) -> bool {
        self.lines.iter().count() == 0
    }

    fn is_last_start_array(&mut self) -> bool {
        if let Some(last) = self.lines.last() {
            return last.get_kv_value() == Tokens::MkArray;
        };
        false
    }

    fn is_last_start_dict(&mut self) -> bool {
        if let Some(last) = self.lines.last() {
            return last.get_kv_value() == Tokens::MkDict;
        };
        false
    }

    fn modify_last_need_empty_arary_brancket(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.enable_empty_array_blancket();
        };
    }

    fn modify_last_need_empty_dict_brancket(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.enable_empty_dict_blancket();
        };
    }
}

impl Processor for MappingProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();

        match line.get_kv_value() {
            Tokens::MkArray => {
                converted.set_indent(self.spaces);
                if !self.is_root() {
                    converted.enable_ln();
                };
                if self.should_index_if_next_is_array {
                    self.increment_space();
                };
                self.should_index_if_next_is_array = true;
            }
            Tokens::EndArray => {
                if self.is_last_start_array() {
                    self.modify_last_need_empty_arary_brancket();
                };
            }
            Tokens::MkDict => {
                converted.set_indent(self.spaces);
                if !self.is_root() {
                    converted.enable_ln();
                    self.increment_space();
                };
            }
            Tokens::EndDict => {
                if self.is_last_start_dict() {
                    self.modify_last_need_empty_dict_brancket();
                };
                self.decrement_space();
            }
            Tokens::String(value) => {
                converted.set_indent(self.spaces);
                converted.set_value(&value);
                converted.enable_ln();
            }
            Tokens::Number(value) => {
                converted.set_indent(self.spaces);
                converted.set_value(&value.to_string());
                converted.enable_ln();
            }
            Tokens::Bool(value) => {
                converted.set_indent(self.spaces);
                converted.set_value(&value.to_string());
                converted.enable_ln();
            }
            Tokens::Null => {
                converted.set_indent(self.spaces);
                converted.set_value("null");
                converted.enable_ln();
            }
        };

        if line.get_kv_value() != Tokens::MkArray {
            self.should_index_if_next_is_array = false;
        };
        if line.get_kv_value() != Tokens::EndArray && line.get_kv_value() != Tokens::EndDict {
            if converted.get_kv_path().is_last_index() {
                converted.enable_hyphen();
            } else {
                converted.set_key(&converted.get_kv_path());
            };
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        self.lines.clone()
    }
}
