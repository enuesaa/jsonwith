use crate::data::kvs::Kvs;
use crate::yaml::parse::context::Context;
use crate::yaml::parse::context::Status;

pub struct Parser {}
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        let mut context = Context::new();
        context.start_root_dict();

        for c in text.chars() {
            match context.get_status() {
                Status::InSpace => self.parse_space(&mut context, c),
                Status::InString => self.parse_string(&mut context, c),
                Status::InKey => self.parse_key(&mut context, c),
                Status::WaitingValue => self.parse_waiting_value(&mut context, c),
                Status::WaitingNewline => self.parse_waiting_newline(&mut context, c),
            };
        }
        context.resolve_value();
        context.end_root_dict();

        context.get_kvs()
    }

    fn parse_space(&mut self, context: &mut Context, c: char) {
        println!("here is space: {:?}", c);
        match c {
            '-' => {}
            _ => {
                context.declare_in_key();
                context.push_buf(c);
            } // key
        };
    }

    fn parse_string(&mut self, context: &mut Context, c: char) {
        println!("here is string: {:?}", c);
        match c {
            '\\' => {
                context.resolve_value();
                context.declare_waiting_newline();
            }
            _ => {
                context.push_buf(c);
            }
        }
    }

    fn parse_key(&mut self, context: &mut Context, c: char) {
        println!("here is key: {:?}", c);
        match c {
            ':' => {
                context.resolve_as_path();
                context.declare_waiting_value();
            }
            _ => {
                context.push_buf(c);
            }
        };
    }

    fn parse_waiting_value(&mut self, context: &mut Context, c: char) {
        println!("here is waiting value: {:?}", c);
        match c {
            ' ' => {}
            _ => {
                // we can not distinguish string or null. so, treat as string.
                context.declare_in_string();
                context.push_buf(c);
            }
        }
    }

    fn parse_waiting_newline(&mut self, context: &mut Context, c: char) {
        println!("here is waiting newline: {:?}", c);
        match c {
            // oh..
            'n' => {
                context.declare_in_space();
            }
            _ => {}
        }
    }
}
