use crate::core::data::tokens::Tokens;
use crate::core::serializer::line::Line;
use crate::core::serializer::processor::Processor;

pub struct IndentProcessor {
    indent: usize,
    spaces: usize,
    lines: Vec<Line>,
}
impl IndentProcessor {
    pub fn new(indent: usize) -> Self {
        Self {
            indent,
            spaces: 0,
            lines: Vec::new(),
        }
    }

    fn increment_space(&mut self) {
        self.spaces += self.indent.clone();
    }

    fn decrement_space(&mut self) {
        self.spaces -= self.indent.clone();
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
}

impl Processor for IndentProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();
        converted.need_ln();

        match line.get_kv_value() {
            Tokens::MkArray => {
                converted.set_indent(self.spaces);
                self.increment_space();
            }
            Tokens::EndArray => {
                self.decrement_space();
                if self.is_last_start_array() {
                    converted.unneed_ln();
                } else {
                    converted.set_indent(self.spaces);
                }
            }
            Tokens::MkDict => {
                converted.set_indent(self.spaces);
                self.increment_space();
            }
            Tokens::EndDict => {
                self.decrement_space();
                if self.is_last_start_dict() {
                    converted.unneed_ln();
                } else {
                    converted.set_indent(self.spaces);
                };
            }
            _ => {
                converted.set_indent(self.spaces);
            }
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        self.lines.clone()
    }
}
