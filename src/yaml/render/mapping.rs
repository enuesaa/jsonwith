use crate::core::data::tokens::Tokens;
use crate::yaml::render::line::Line;
use crate::yaml::render::processor::Processor;

pub struct MappingProcessor {
    indent: usize,
    spaces: usize,
    lines: Vec<Line>,
}
impl MappingProcessor {
    pub fn new() -> Self {
        Self {
            indent: 2,
            spaces: 0,
            lines: Vec::new(),
        }
    }

    fn increment_space(&mut self) {
        self.spaces += self.indent.clone();
    }

    fn decrement_space(&mut self) {
        // todo refactor
        if self.spaces > 0 {
            self.spaces -= self.indent.clone();
        };
    }

    fn is_root_dict(&self) -> bool {
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

    fn append_key_or_hyphen(&mut self, line: &mut Line) {
        if line.get_kv_path().is_last_index() {
            line.enable_hyphen();
        } else {
            line.set_key(&line.get_kv_path());
        };
    }
}

impl Processor for MappingProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();

        match line.get_kv_value() {
            Tokens::MkArray => {
                converted.set_indent(self.spaces);
                converted.enable_ln();
                self.append_key_or_hyphen(&mut converted);
            }
            Tokens::EndArray => {
                if self.is_last_start_array() {
                    self.modify_last_need_empty_arary_brancket();
                };
            }
            Tokens::MkDict => {
                converted.set_indent(self.spaces);
                if !self.is_root_dict() {
                    converted.enable_ln();
                    self.increment_space();
                };
                self.append_key_or_hyphen(&mut converted);
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
                self.append_key_or_hyphen(&mut converted);
            }
            Tokens::Number(value) => {
                converted.set_indent(self.spaces);
                converted.set_value(&value.to_string());
                converted.enable_ln();
                self.append_key_or_hyphen(&mut converted);
            }
            Tokens::Bool(value) => {
                converted.set_indent(self.spaces);
                converted.set_value(&value.to_string());
                converted.enable_ln();
                self.append_key_or_hyphen(&mut converted);
            }
            Tokens::Null => {
                converted.set_indent(self.spaces);
                converted.set_value("null");
                converted.enable_ln();
                self.append_key_or_hyphen(&mut converted);
            }
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        self.lines.clone()
    }
}
