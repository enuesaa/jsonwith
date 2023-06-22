use crate::core::serializer::line::Line;
use crate::core::data::tokens::Tokens;

pub trait Processor {
    fn process(&self, lines: &Vec<Line>) -> Vec<Line>;
}

pub struct IndentProcessor {
    pub indent: usize,
}
impl Processor for IndentProcessor {
    fn process(&self, lines: &Vec<Line>) -> Vec<Line> {
        let mut converted_lines: Vec<Line> = Vec::new();

        let mut spaces = 0;
        for line in lines {

            let mut converted = line.clone();
            match line.get_kv_value() {
                Tokens::MkArray => {
                    converted.set_indent(spaces);
                    spaces += self.indent.clone();
                },
                Tokens::EndArray => {
                    spaces -= self.indent.clone();
                    if let Some(last) = converted_lines.last() {
                        if !last.array_start_bracket {
                            converted.set_indent(spaces);
                        };
                    };
                },
                Tokens::MkDict => {
                    converted.set_indent(spaces);
                    spaces += self.indent.clone();
                },
                Tokens::EndDict => {
                    spaces -= self.indent.clone();
                    if let Some(last) = converted_lines.last() {
                        if !last.dict_start_bracket {
                            converted.set_indent(spaces);
                        };
                    };
                },
                _ => {
                    converted.set_indent(spaces);
                },
            };
            converted_lines.push(converted);
        };
        if let Some(last) = converted_lines.last_mut() {
            last.unneed_comma();
        };

        converted_lines
    }
}
