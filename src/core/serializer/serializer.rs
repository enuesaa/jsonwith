use crate::core::data::kvs::Kvs;
use crate::core::data::tokens::Tokens;
use crate::core::serializer::line::Line;
use crate::core::serializer::processor::Processor;

pub struct Serializer {
    lines: Vec<Line>
}
impl Serializer {
    pub fn serialize(kvs: Kvs) -> Self {
        let mut lines: Vec<Line> = Vec::new();

        for kv in kvs.list() {
            let path = kv.get_path();
            let value = kv.get_value();

            let mut line = Line::from(kv.clone());
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
            lines.push(line);
        };
        if let Some(last) = lines.last_mut() {
            last.unneed_comma();
        };

        Serializer { lines }
    }

    pub fn process<T: Processor>(&mut self, processor: T) -> &mut Self {
        self.lines = processor.process(&self.lines);
        self
    }

    pub fn get_raw(&self) -> String {
        self.lines.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("")   
    }
}
