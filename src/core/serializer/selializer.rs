use crate::core::data::kvs::Kvs;
use crate::core::data::kv::Kv;
use crate::core::data::tokens::Tokens;
use crate::core::serializer::context::Context;

pub struct Serializer {
    indent: usize,
    kvs: Kvs,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer {
            indent: 2,
            kvs: Kvs::new(),
        }
    }

    // append serialize options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn serialize(&mut self, text: &str) -> Kvs {

        let mut context = Context::new();
        for i in text.chars() {
            if context.in_space() {
                self.serialize_space(&mut context, i);
            }
            context.push(i);

            if context.in_null_value() {
                if context.get_buf() == "null".to_string() {
                    self.kvs.push(Kv { path: context.get_path(), value: Tokens::Null });
                    context.declare_in_space();
                }
            }
            if context.in_bool_value() {
                if context.get_buf() == "true".to_string() {
                    self.kvs.push(Kv { path: context.get_path(), value: Tokens::Bool(true) });
                    context.declare_in_space();
                }
                if context.get_buf() == "false".to_string() {
                    self.kvs.push(Kv { path: context.get_path(), value: Tokens::Bool(false) });
                    context.declare_in_space();
                }
            }
            // number
            if context.in_string_value() {
                if context.get_buf().ends_with('"') {
                    self.kvs.push(Kv { path: context.get_path(), value: Tokens::String(context.get_buf()) });
                    context.declare_in_space();
                }
            }
            if context.in_key() {
                if context.get_buf().ends_with('"') {
                    context.path.push(&context.get_buf());
                    context.declare_in_space();
                }
            }
        };

        self.kvs.clone()
    }

    fn serialize_space(&mut self, context: &mut Context, c: char) {
        match c {
            '{' => {
                context.start_dict(); // key をどうやって渡すか
                self.push_kv(context, Tokens::MkDict);
            },
            '}' => {
                context.end_dict();
            },
            '[' => {
                self.push_kv(context, Tokens::MkArray);
            },
            ']' => {},
            '"' => {
                if context.parent_is_dict() {
                    context.declare_in_key();
                } else {
                    context.declare_in_string_value();
                }
                context.push(c);
            },
            'n' => {
                context.declare_in_null_value();
                context.push(c);
            },
            't'|'f' => {
                context.declare_in_bool_value();
                context.push(c);
            },
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                context.declare_in_number_value();
                context.push(c);
            },
            _ => {},
        };
    }

    fn push_kv(&mut self, context: &Context, value: Tokens) {
        self.kvs.push(Kv { path: context.get_path(), value });
    }

    // pub fn found_quotation(&mut self) {
    //     if self.in_space() {
    //         if self.should_declare_key() {
    //             self.declare_key();
    //         } else {
    //             self.declare_value();
    //             self.push('"');
    //         };
    //     };
    // }

    // pub fn found_value(&mut self, c: char) {
    //     self.declare_value();
    //     self.buf = self.buf.clone() + &c.to_string();
    // }
}
