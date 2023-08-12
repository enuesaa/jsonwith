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
        // because parse_number_value() cant judge last item when value is number, push white space to notify number.
        let text = format!("{} ", text);
        for c in text.chars() {
            match context.get_status() {
                Status::InSpace => self.parse_space(&mut context, c),
                Status::InNull => self.parse_null(&mut context, c),
                Status::InBool => self.parse_bool(&mut context, c),
                Status::InNumber => self.parse_number(&mut context, c),
                Status::InString => self.parse_string(&mut context, c),
                Status::InKey => self.parse_key(&mut context, c),
            };
        }

        context.get_kvs()
    }

    fn parse_space(&mut self, context: &mut Context, c: char) {
        match c {
            '{' => context.start_dict(),
            '}' => context.end_dict(),
            '[' => context.start_array(),
            ']' => context.end_array(),
            '"' => {
                if context.is_waiting_value() {
                    context.declare_in_string();
                } else {
                    context.declare_in_key();
                };
            }
            'n' => {
                context.declare_in_null();
                context.push_buf(c);
            }
            't' | 'f' => {
                context.declare_in_bool();
                context.push_buf(c);
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                context.declare_in_number();
                context.push_buf(c);
            }
            _ => {}
        };
    }

    fn parse_null(&mut self, context: &mut Context, c: char) {
        context.push_buf(c);
        if &context.get_buf() == "null" {
            context.resolve_value(Tokens::Null);
            context.declare_in_space();
        }
    }

    fn parse_bool(&mut self, context: &mut Context, c: char) {
        context.push_buf(c);
        let buf = &context.get_buf();
        if buf == "true" || buf == "false" {
            context.resolve_value(Tokens::Bool(buf == "true"));
            context.declare_in_space();
        };
    }

    fn parse_string(&self, context: &mut Context, c: char) {
        if c == '"' && !context.get_buf().ends_with('\\') {
            let value = context.get_buf();
            context.resolve_value(Tokens::String(value));
            context.declare_in_space();
        } else {
            context.push_buf(c);
        }
    }

    fn parse_number(&mut self, context: &mut Context, c: char) {
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