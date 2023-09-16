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
                Status::InSpace => self.parse_space(&mut context, c),
                Status::InNull => self.parse_null(&mut context, c),
                Status::InBool => self.parse_bool(&mut context, c),
                Status::InNumber => self.parse_number(&mut context, c),
                Status::InString => self.parse_string(&mut context, c),
                Status::InKey => self.parse_key(&mut context, c),
            };
        };

        context.get_kvs()
    }

    fn parse_space(&mut self, context: &mut Context, c: char) {}
    fn parse_bool(&mut self, context: &mut Context, c: char) {}
    fn parse_number(&mut self, context: &mut Context, c: char) {}
    fn parse_string(&mut self, context: &mut Context, c: char) {}
    fn parse_key(&mut self, context: &mut Context, c: char) {}
    fn parse_null(&mut self, context: &mut Context, c: char) {}
}
