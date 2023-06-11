use crate::core::data::kvs::Kvs;
use crate::core::data::path::PathItem;
use crate::core::data::tokens::Tokens;

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
        let mut ret = String::from("");

        let mut need_comma = false;
        for kv in kvs.list() {
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
