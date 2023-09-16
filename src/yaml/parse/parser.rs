use crate::data::kvs::Kvs;
use crate::data::tokens::Tokens;
// use crate::yaml::parse::context::Context;
// use crate::yaml::parse::context::Status;

pub struct Parser {}
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        Kvs::new()
    }
}