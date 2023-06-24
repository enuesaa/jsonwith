use crate::core::data::tokens::Tokens;
use crate::core::yaml_serializer::line::Line;
use crate::core::yaml_serializer::processor::Processor;

pub struct MappingProcessor {
    indent: usize,
    spaces: usize,
    lines: Vec<Line>,
}
impl MappingProcessor {
    pub fn new() -> Self {
        Self { indent: 2, spaces: 0, lines: Vec::new() }
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
}

impl Processor for MappingProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();

        match line.get_kv_value() {
            Tokens::MkArray => {
                converted.set_indent(self.spaces);
                converted.set_key(&converted.get_kv_path());
                converted.enable_ln();
                self.increment_space();
            },
            Tokens::EndArray => {
                self.decrement_space();
            },
            Tokens::MkDict => {
                converted.set_indent(self.spaces);
                converted.set_key(&converted.get_kv_path());
                converted.enable_ln();
                if !self.is_root_dict() {
                    self.increment_space();
                };
            },
            Tokens::EndDict => {
                self.decrement_space();
            },
            Tokens::String(value) => {
                converted.set_indent(self.spaces);
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value);
                converted.enable_ln();
            },
            Tokens::Number(value) => {
                converted.set_indent(self.spaces);
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value.to_string());
                converted.enable_ln();
            },
            Tokens::Bool(value) => {
                converted.set_indent(self.spaces);
                converted.set_key(&converted.get_kv_path());
                converted.set_value(&value.to_string());
                converted.enable_ln();
            },
            Tokens::Null => {
                converted.set_indent(self.spaces);
                converted.set_key(&converted.get_kv_path());
                converted.set_value("null");
                converted.enable_ln();
            },
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        self.lines.clone()
    }
}
