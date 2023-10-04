use crate::data::tokens::Tokens;
use crate::json::render::line::Line;
use crate::json::render::processor::Processor;

pub struct MappingProcessor {
    lines: Vec<Line>,
}
impl MappingProcessor {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn remove_last_comma(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.disable_comma();
        };
    }

    fn remove_last_ln(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.disable_ln();
        };
    }

    fn is_last_start_array(&self) -> bool {
        if let Some(last) = self.lines.last() {
            return last.is_array_start_bracket();
        };
        false
    }

    fn is_last_start_dict(&self) -> bool {
        if let Some(last) = self.lines.last() {
            return last.is_dict_start_bracket();
        };
        false
    }
}

impl Processor for MappingProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();
        converted.enable_ln();

        match line.get_kv_value() {
            Tokens::MkArray => {
                converted.set_key(&converted.get_kv_path());
                converted.enable_array_start_bracket();
            }
            Tokens::EndArray => {
                self.remove_last_comma();
                if self.is_last_start_array() {
                    self.remove_last_ln();
                };
                converted.enable_array_end_bracket();
                converted.enable_comma();
            }
            Tokens::MkDict => {
                converted.set_key(&converted.get_kv_path());
                converted.enable_dict_start_bracket();
            }
            Tokens::EndDict => {
                self.remove_last_comma();
                if self.is_last_start_dict() {
                    self.remove_last_ln();
                };
                converted.enable_dict_end_bracket();
                converted.enable_comma();
            }
            Tokens::String(value) => {
                converted.set_key(&converted.get_kv_path());
                converted.set_string_value(&value);
                converted.enable_comma();
            }
            Tokens::Number(value) => {
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value.to_string());
                converted.enable_comma();
            }
            Tokens::Bool(value) => {
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value.to_string());
                converted.enable_comma();
            }
            Tokens::Null => {
                converted.set_key(&converted.get_kv_path());
                converted.set_value("null");
                converted.enable_comma();
            }
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        let mut lines = self.lines.clone();
        if let Some(last) = lines.last_mut() {
            last.disable_comma();
        };
        lines
    }
}
