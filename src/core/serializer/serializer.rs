use crate::core::data::kvs::Kvs;
use crate::core::data::path::PathItem;
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

    pub fn serialize_withline(&mut self, kvs: Kvs) -> String {
        let mut ret = String::from("");

        let mut spaces = 0;
        for kv in kvs.list() {
            let path = kv.get_path();
            let value = kv.get_value();
            let mut line = Line::new();
            line.set_indent(spaces);

            if value != Tokens::EndArray && value != Tokens::EndDict {
                match path.get_last() {
                    PathItem::Key(key) => {
                        if key != "" {
                            line.set_key(format!("\"{}\"", key));
                            line.need_colon();   
                        };
                    },
                    _ => {},
                };
            };
            // 本当は次の value を見ないといけない
            if value != Tokens::MkArray && value != Tokens::MkDict {
                line.need_comma();
            };

            match value {
                Tokens::MkArray => {
                    line.need_array_start_bracket();
                    spaces += 2;
                },
                Tokens::EndArray => {
                    spaces -= 2;
                    line.set_indent(spaces);
                    line.need_array_end_bracket();
                },
                Tokens::MkDict => {
                    line.need_dict_start_bracket();
                    spaces += 2;
                },
                Tokens::EndDict => {
                    spaces -= 2;
                    line.set_indent(spaces);
                    line.need_dict_end_bracket();
                },
                Tokens::String(value) => {
                    line.set_value(format!("\"{}\"", value));
                },
                Tokens::Number(value) => {
                    line.set_value(value.to_string());
                },
                Tokens::Bool(value) => {
                    if value {
                        line.set_value(String::from("true"));
                    } else {
                        line.set_value(String::from("false"));
                    };
                },
                Tokens::Null => {
                    line.set_value(String::from("null"));
                },
            };
            ret += &line.to_string();
        };

        ret
    }

    pub fn serialize(&mut self, kvs: Kvs) -> String {
        let mut ret = String::from("");

        let mut need_comma = false;
        for kv in kvs.list() {
            // line
            let path = kv.get_path();
            let value = kv.get_value();
            if let PathItem::Key(key) = path.get_last() {
                if key != "" {
                    match value {
                        Tokens::EndArray => {},
                        Tokens::EndDict => {},
                        _ => {
                            if need_comma {
                                ret += ",";
                            };
                            ret += &format!("\"{}\":", key)
                        },
                    };
                };
            };

            need_comma = true;
            let formatted = match value {
                Tokens::MkArray => {
                    need_comma = false;
                    self.serialize_mkarray()
                },
                Tokens::EndArray => {
                    need_comma = false;
                    self.serialize_endarray()
                },
                Tokens::MkDict => {
                    need_comma = false;
                    self.serialize_mkdict()
                },
                Tokens::EndDict => {
                    need_comma = false;
                    self.serialize_enddict()
                },
                Tokens::String(value) => self.serialize_string(value),
                Tokens::Number(value) => self.serialize_number(value),
                Tokens::Bool(value) => self.serialize_bool(value),
                Tokens::Null => self.serialize_null(),
            };
            ret += &formatted;
        };

        ret
    }

    fn serialize_mkarray(&self) -> String {
        "[".to_string()
    }

    fn serialize_endarray(&self) -> String {
        "]".to_string()
    }

    fn serialize_mkdict(&self) -> String {
        "{".to_string()
    }

    fn serialize_enddict(&self) -> String {
        "}".to_string()
    }

    fn serialize_string(&self, value: String) -> String {
        format!("\"{}\"", value)
    }

    fn serialize_number(&self, value: usize) -> String {
        format!("{}", value)
    }

    fn serialize_bool(&self, value: bool) -> String {
        format!("{}", value)
    }

    fn serialize_null(&self) -> String {
        format!("null")
    }
}
