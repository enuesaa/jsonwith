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

        for kv in kvs.list() {
            let path = kv.get_path();
            if let PathItem::Key(key) = path.get_last() {
                if key != "" {
                    ret += &format!("\"{}\":", key);
                };
            };
            // seriialize path here

            let formatted = match kv.get_value() {
                Tokens::MkArray => self.serialize_mkarray(),
                Tokens::MkDict => self.serialize_mkdict(),
                Tokens::String(value) => self.serialize_string(value),
                Tokens::Number(value) => self.serialize_number(value),
                Tokens::Bool(value) => self.serialize_bool(value),
                Tokens::Null => self.serialize_null(),
            };
            ret += &formatted;
        };
        println!("{}", ret);

        String::from("")
    }

    fn serialize_mkarray(&self) -> String {
        "[".to_string()
    }

    fn serialize_mkdict(&self) -> String {
        "{".to_string()
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
