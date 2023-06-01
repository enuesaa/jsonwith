use crate::core::data::kvs::Kvs;
use crate::core::data::kv::Kv;
use crate::core::data::tokens::Tokens;
use crate::core::serializer::context::Context;

use super::context::Status;

pub struct Serializer {
    indent: usize,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer {
            indent: 2,
        }
    }

    // configure options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn serialize(&mut self, text: &str) -> Kvs {
        let mut context = Context::new();
        for c in text.chars() {
            match context.get_status() {
                Status::InSpace => self.serialize_space(&mut context, c),
                Status::InNullValue => self.serialize_null_value(&mut context, c),
                Status::InBoolValue => self.serialize_bool_value(&mut context, c),
                Status::InNumberValue => {},
                Status::InStringValue => self.serialize_string_value(&mut context, c),
                Status::InKey => self.serialize_key(&mut context, c),
            };
        };

        context.kvs.clone()
    }

    fn serialize_space(&mut self, context: &mut Context, c: char) {
        match c {
            '{' => {
                context.start_dict();
                self.push_kv(context, Tokens::MkDict);
            },
            '}' => {
                context.end_dict();
            },
            '[' => {
                context.start_array();
                self.push_kv(context, Tokens::MkArray);
            },
            ']' => {
                context.end_array();
            },
            '"' => {
                if context.is_waiting_value() {
                    context.declare_in_string_value();
                } else {
                    context.declare_in_key();
                };
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

    fn serialize_null_value(&mut self, context: &mut Context, c: char) {
        context.push(c);
        if context.get_buf() == "null".to_string() {
            self.push_kv(context, Tokens::Null);
            context.declare_in_space();
        }
    }

    fn serialize_bool_value(&mut self, context: &mut Context, c: char) {
        context.push(c);
        if context.get_buf() == "true".to_string() {
            self.push_kv(context, Tokens::Bool(true));
            context.declare_in_space();
        }
        if context.get_buf() == "false".to_string() {
            self.push_kv(context, Tokens::Bool(false));
            context.declare_in_space();
        }
    }

    fn serialize_string_value(&mut self, context: &mut Context, c: char) {
        if c == '"' && !context.get_buf().ends_with('\\') { 
            let value = context.get_buf();
            self.push_kv(context, Tokens::String(value));
            context.declare_in_space();
        } else {
            context.push(c);
        }
    }

    fn serialize_key(&mut self, context: &mut Context, c: char) {
        if c == '"' && !context.get_buf().ends_with('\\') {
            context.pop_path();
            context.resolve_as_path();
            // context.declare_in_space();
        } else {
            context.push(c);
        }
    }

    fn push_kv(&mut self, context: &mut Context, value: Tokens) {
        context.kvs.push(Kv { path: context.get_path(), value });
    }
}
