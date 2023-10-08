use crate::data::kvs::Kvs;
use crate::yaml::parse::context::Context;
use crate::yaml::parse::line::Line;

pub struct Parser {}
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        let mut context = Context::new();

        let mut line = Line::new();
        for c in text.chars() {
            if line.is_ended() {

                line = Line::new();
                continue;
            }
            line.push(c);
        }

        context.get_kvs()
    }
}
