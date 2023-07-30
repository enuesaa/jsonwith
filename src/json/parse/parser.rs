use crate::data::kvs::Kvs;
use crate::data::tokens::Tokens;
use crate::json::parse::context::Context;
use crate::json::parse::context::Status;

pub struct Parser {
    indent: usize,
}
impl Parser {
    pub fn new() -> Self {
        Parser { indent: 2 }
    }

    // configure options like this.
    pub fn set_indent(&mut self, indent: usize) {
        self.indent = indent;
    }

    pub fn deserialize(&mut self, text: &str) -> Kvs {
        let mut context = Context::new();
        let text = format!("{} ", text); // to trigger last process in parse_number_value.
        for c in text.chars() {
            match context.status {
                Status::InSpace => self.parse_space(&mut context, c),
                Status::InNullValue => self.parse_null_value(&mut context, c),
                Status::InBoolValue => self.parse_bool_value(&mut context, c),
                Status::InNumberValue => self.parse_number_value(&mut context, c),
                Status::InStringValue => self.parse_string_value(&mut context, c),
                Status::InKey => self.parse_key(&mut context, c),
            };
        }

        context.kvs.clone()
    }

    fn parse_space(&mut self, context: &mut Context, c: char) {
        match c {
            '{' => context.start_dict(),
            '}' => context.end_dict(),
            '[' => context.start_array(),
            ']' => context.end_array(),
            '"' => {
                if context.is_waiting_value() {
                    context.declare_in_string_value();
                } else {
                    context.declare_in_key();
                };
            }
            'n' => {
                context.declare_in_null_value();
                context.push_buf(c);
            }
            't' | 'f' => {
                context.declare_in_bool_value();
                context.push_buf(c);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                context.declare_in_number_value();
                context.push_buf(c);
            }
            _ => {}
        };
    }

    fn parse_null_value(&mut self, context: &mut Context, c: char) {
        context.push_buf(c);
        if &context.get_buf() == "null" {
            context.resolve_value(Tokens::Null);
            context.declare_in_space();
        }
    }

    fn parse_bool_value(&mut self, context: &mut Context, c: char) {
        context.push_buf(c);
        let buf = &context.get_buf();
        if buf == "true" || buf == "false" {
            context.resolve_value(Tokens::Bool(buf == "true"));
            context.declare_in_space();
        };
    }

    fn parse_string_value(&self, context: &mut Context, c: char) {
        if c == '"' && !context.get_buf().ends_with('\\') {
            let value = context.get_buf();
            context.resolve_value(Tokens::String(value));
            context.declare_in_space();
        } else {
            context.push_buf(c);
        }
    }

    fn parse_number_value(&mut self, context: &mut Context, c: char) {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                context.push_buf(c);
            }
            _ => {
                let value: usize = context.get_buf().parse().unwrap();
                context.resolve_value(Tokens::Number(value));
                context.declare_in_space();

                self.parse_space(context, c);
            }
        };
    }

    fn parse_key(&mut self, context: &mut Context, c: char) {
        if c == '"' && !context.get_buf().ends_with('\\') {
            context.resolve_as_path();
        } else {
            context.push_buf(c);
        }
    }
}