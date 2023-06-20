use crate::core::serializer::line::Line;
use crate::core::data::tokens::Tokens;

pub trait Processor {
    fn process(&self, lines: &Vec<Line>) -> Vec<Line>;
}

pub struct MappingProcessor {}
impl Processor for MappingProcessor {
    fn process(&self, lines: &Vec<Line>) -> Vec<Line> {
        let mut lines = lines.clone();
        for mut line in lines {
            let kv = line.get_kv();
            let path = kv.get_path();
            let value = kv.get_value();

            match value {
                Tokens::MkArray => {
                    line.set_key(&path);
                    line.need_array_start_bracket();
                },
                Tokens::EndArray => {
                    if let Some(last) = lines.last_mut() {
                        last.unneed_comma();
                        if last.array_start_bracket {
                            last.need_array_end_bracket();
                            line.unneed_ln();
                        } else {
                            line.need_array_end_bracket();
                            line.need_comma();
                        };
                    };
                },
                Tokens::MkDict => {
                    line.set_key(&path);
                    line.need_dict_start_bracket();
                },
                Tokens::EndDict => {
                    if let Some(last) = lines.last_mut() {
                        last.unneed_comma();
                        if last.dict_start_bracket {
                            last.need_dict_end_bracket();
                            line.unneed_ln();
                        } else {
                            line.need_dict_end_bracket();
                            line.need_comma();
                        };
                    };
                },
                Tokens::String(value) => {
                    line.set_key(&path);
                    line.set_string_value(&value);
                    line.need_comma();
                },
                Tokens::Number(value) => {
                    line.set_key(&path);
                    line.set_value(&value.to_string());
                    line.need_comma();
                },
                Tokens::Bool(value) => {
                    line.set_key(&path);
                    line.set_value(&value.to_string());
                    line.need_comma();
                },
                Tokens::Null => {
                    line.set_key(&path);
                    line.set_value("null");
                    line.need_comma();
                },
            };
        };
        if let Some(last) = lines.last_mut() {
            last.unneed_comma();
        };
        lines.clone()
    }
}

pub struct IndentProcessor {
    indent: usize,
}
impl Processor for IndentProcessor {
    fn process(&self, lines: &Vec<Line>) -> Vec<Line> {
        let mut lines = lines.clone();
        let mut spaces = 0;
        for mut line in lines {
            let kv = line.get_kv();
            let value = kv.get_value();

            match value {
                Tokens::MkArray => {
                    line.set_indent(spaces);
                    spaces += 2;
                },
                Tokens::EndArray => {
                    spaces -= 2;
                    if let Some(last) = lines.last() {
                        // todo refactor
                        if !last.array_start_bracket {
                            line.set_indent(spaces);
                        };
                    };
                },
                Tokens::MkDict => {
                    line.set_indent(spaces);
                    spaces += 2;
                },
                Tokens::EndDict => {
                    spaces -= 2;
                    if let Some(last) = lines.last() {
                        if !last.dict_start_bracket {
                            line.set_indent(spaces);
                        };
                    };
                },
                _ => {
                    line.set_indent(spaces);
                },
            };
        };
        if let Some(last) = lines.last_mut() {
            last.unneed_comma();
        };
        lines.clone()
    }
}
