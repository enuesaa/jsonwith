use crate::data::kvs::Kvs;
use crate::data::path::Path;
use crate::data::tokens::Tokens;
use crate::yaml::parse::context::Context;
use crate::yaml::parse::line::Line;

pub struct Parser {}
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        let mut context = Context::new();
        self.push_root_mkdict(&mut context);

        let mut line = Line::new();
        for c in text.chars() {
            println!("{:?}", c);
            if line.is_ended() {
                println!("ended");
                self.push_context(&mut context, line);
                line = Line::new();
            }
            line.push(c);
        }
        self.push_context(&mut context, line);
        self.push_root_enddict(&mut context);

        context.get_kvs()
    }

    fn push_context(&self, context: &mut Context, line: Line) {
        let mut path = context.get_last_path();

        let last_indent = context.get_last_indent();
        if last_indent >= line.get_indent() {
            path.pop();
        }
        if last_indent <= line.get_indent() {
            path.push(&line.get_key());
        }

        let value = Tokens::String(line.get_value());
        context.push(path, value);
    }

    fn push_root_mkdict(&self, context: &mut Context) {
        context.push(Path::new(), Tokens::MkDict);
    }

    fn push_root_enddict(&self, context: &mut Context) {
        context.push(Path::new(), Tokens::EndDict);
    }
}
