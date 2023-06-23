use crate::core::data::tokens::Tokens;
use crate::core::serializer::line::Line;
use crate::core::serializer::processor::Processor;

pub struct MappingProcessor {
    lines: Vec<Line>,
}
impl MappingProcessor {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    fn remove_last_comma(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.unneed_comma();
        };
    }

    fn is_last_start_array(&self) -> bool {
        if let Some(last) = self.lines.last() {
            return last.array_start_bracket; // todo refactor
        };
        false
    }

    fn is_last_start_dict(&self) -> bool {
        if let Some(last) = self.lines.last() {
            return last.dict_start_bracket;
        };
        false
    }

    fn modify_last_need_array_end_bracket(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.need_array_end_bracket();
        };
    }

    fn modify_last_need_dict_end_bracket(&mut self) {
        if let Some(last) = self.lines.last_mut() {
            last.need_dict_end_bracket();
        };
    }
}

impl Processor for MappingProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();

        match line.get_kv_value() {
            Tokens::MkArray => {
                converted.set_key(&converted.get_kv_path());
                converted.need_array_start_bracket();
            },
            Tokens::EndArray => {
                self.remove_last_comma();
                if self.is_last_start_array() {
                    self.modify_last_need_array_end_bracket();
                    // converted.unneed_ln();
                } else {
                    converted.need_array_end_bracket();
                    converted.need_comma();
                };
            },
            Tokens::MkDict => {
                converted.set_key(&converted.get_kv_path());
                converted.need_dict_start_bracket();
            },
            Tokens::EndDict => {
                self.remove_last_comma();
                if self.is_last_start_dict() {
                    self.modify_last_need_dict_end_bracket();
                    // converted.unneed_ln();
                } else {
                    converted.need_dict_end_bracket();
                    converted.need_comma();
                };
            },
            Tokens::String(value) => {
                converted.set_key(&converted.get_kv_path());
                converted.set_string_value(&value);
                converted.need_comma();
            },
            Tokens::Number(value) => {
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value.to_string());
                converted.need_comma();
            },
            Tokens::Bool(value) => {
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value.to_string());
                converted.need_comma();
            },
            Tokens::Null => {
                converted.set_key(&converted.get_kv_path());
                converted.set_value("null");
                converted.need_comma();
            },
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        let mut lines = self.lines.clone();
        if let Some(last) = lines.last_mut() {
            last.unneed_comma();
        };
        lines
    }
}
