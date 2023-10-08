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
        if last_indent > line.get_indent() {
            path.pop();
            context.push(path.clone(), Tokens::EndDict);
        } else if last_indent < line.get_indent() {
            path.push(&line.get_key());
        } else {
            path.pop();
            path.push(&line.get_key());
        }
        

        if !line.has_value() {
            context.push(path, Tokens::MkDict);
            return;
        }

        let buf = line.get_value();
        let value = match buf.as_str() {
            "null" => Tokens::Null,
            "false" => Tokens::Bool(false),
            "true" => Tokens::Bool(true),
            "" => Tokens::String(buf),
            _ => {
                if buf.chars().all(|c| c.is_numeric()) {
                    Tokens::Number(buf.parse::<usize>().unwrap())
                } else {
                    Tokens::String(buf)
                }
            },
        };
        context.push(path, value);
    }

    fn push_root_mkdict(&self, context: &mut Context) {
        context.push(Path::new(), Tokens::MkDict);
    }

    fn push_root_enddict(&self, context: &mut Context) {
        context.push(Path::new(), Tokens::EndDict);
    }
}
