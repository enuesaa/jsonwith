use crate::core::data::kvs::Kvs;
use crate::core::data::tokens::Tokens;
use crate::core::serializer::line::Line;

pub struct Serializer {
    indent: usize,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer { indent: 2 }
    }

    // configure options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn serialize(&mut self, kvs: Kvs) -> String {
        let mut lines: Vec<Line> = Vec::new();

        let mut spaces = 0;
        for kv in kvs.list() {
            let path = kv.get_path();
            let value = kv.get_value();
            let mut line = Line::new();

            match value {
                Tokens::MkArray => {
                    line.set_indent(spaces);
                    line.set_key(&path);
                    line.need_array_start_bracket();
                    spaces += 2;
                },
                Tokens::EndArray => {
                    if let Some(last) = lines.last_mut() {
                        last.unneed_comma();
                        spaces -= 2;
                        if last.array_start_bracket {
                            last.need_array_end_bracket();
                            line.unneed_ln();
                        } else {
                            line.set_indent(spaces);
                            line.need_array_end_bracket();
                            line.need_comma();
                        };
                    };
                },
                Tokens::MkDict => {
                    line.set_indent(spaces);
                    line.set_key(&path);
                    line.need_dict_start_bracket();
                    spaces += 2;
                },
                Tokens::EndDict => {
                    if let Some(last) = lines.last_mut() {
                        last.unneed_comma();
                        spaces -= 2;
                        if last.dict_start_bracket {
                            last.need_dict_end_bracket();
                            line.unneed_ln();
                        } else {
                            line.set_indent(spaces);
                            line.need_dict_end_bracket();
                            line.need_comma();
                        };
                    };
                },
                Tokens::String(value) => {
                    line.set_indent(spaces);
                    line.set_key(&path);
                    line.set_string_value(&value);
                    line.need_comma();
                },
                Tokens::Number(value) => {
                    line.set_indent(spaces);
                    line.set_key(&path);
                    line.set_value(&value.to_string());
                    line.need_comma();
                },
                Tokens::Bool(value) => {
                    line.set_indent(spaces);
                    line.set_key(&path);
                    line.set_value(&value.to_string());
                    line.need_comma();
                },
                Tokens::Null => {
                    line.set_indent(spaces);
                    line.set_key(&path);
                    line.set_value("null");
                    line.need_comma();
                },
            };
            lines.push(line);
        };
        if let Some(last) = lines.last_mut() {
            last.unneed_comma();
        };

        println!("{:?}", lines.iter().map(|l| l.to_string()).collect::<Vec<String>>());

        lines.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("")
    }
}
