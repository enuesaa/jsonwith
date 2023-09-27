use crate::data::tokens::Tokens;
use crate::yaml::render::line::Line;
use crate::yaml::render::processor::Processor;

pub struct DictInArrayProcessor {
    lines: Vec<Line>,
    rm_next_indent: bool,
}
impl DictInArrayProcessor {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            rm_next_indent: false,
        }
    }
}

impl Processor for DictInArrayProcessor {
    fn push(&mut self, line: &Line) {
        let mut converted = line.clone();

        match line.get_kv_value() {
            Tokens::MkDict => {
                if converted.get_kv_path().is_last_index() {
                    converted.disable_ln();
                    self.rm_next_indent = true;
                };
            }
            _ => {
                if self.rm_next_indent {
                    converted.set_indent(0);
                    self.rm_next_indent = false;
                };
            }
        };

        self.lines.push(converted);
    }

    fn process(&self) -> Vec<Line> {
        self.lines.clone()
    }
}
