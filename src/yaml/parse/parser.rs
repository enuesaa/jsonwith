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

        for c in text.chars() {
            match context.get_status() {
                Status::InDictOrArray => self.parse_dict_or_array(&mut context, c),
                Status::InValue => self.parse_value(&mut context, c),
                Status::InKey => self.parse_key(&mut context, c),
                Status::WaitingValue => self.parse_waiting_value(&mut context, c),
                Status::WaitingNDictOrArray => self.parse_waiting_n_dict_or_array(&mut context, c),
                Status::WaitingN => self.parse_waiting_n(&mut context, c),
            };
        }
        context.resolve_value();
        context.end_root_dict(); // todo: refactor

        context.get_kvs()
    }

    fn parse_dict_or_array(&mut self, context: &mut Context, c: char) {
        println!("here is dict or array: {:?}", c);
        match c {
            ' ' => {},
            '-' => {
                // array
                context.start_array();
                context.declare_in_dict_or_array(); // here may be string 
            }
            _ => {
                context.start_dict();
                context.declare_in_key();
                context.push_buf(c);
            }
        };
    }

    fn parse_value(&mut self, context: &mut Context, c: char) {
        println!("here is value: {:?}", c);
        match c {
            '\\' => {
                context.resolve_value(); // this is tmp
                context.declare_waiting_n();
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
            '\\' => {
                context.declare_waiting_n_dict_or_array();
            }
            ' ' => {}
            _ => {
                // we can not distinguish string or null. so, treat as string.
                context.declare_in_value();
                context.push_buf(c);
            }
        }
    }

    fn parse_waiting_n_dict_or_array(&mut self, context: &mut Context, c: char) {
        println!("here is waiting n dict or array: {:?}", c);
        match c {
            'n' => {
                context.declare_in_dict_or_array();
            },
            _ => {},
        }
    }

    fn parse_waiting_n(&mut self, context: &mut Context, c: char) {
        println!("here is waiting n: {:?}", c);
        match c {
            // oh..
            'n' => {
                context.declare_in_key();
            }
            _ => {}
        }
    }
}
