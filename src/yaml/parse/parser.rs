use crate::data::kv::Kv;
use crate::data::kvs::Kvs;
use crate::data::path::Path;
use crate::data::tokens::Tokens;
use crate::yaml::parse::line::Line;

pub struct Parser {
    kvs: Kvs,
    last_indent: usize,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            kvs: Kvs::new(),
            last_indent: 0,
        }
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        self.push_root_mkdict();

        let mut line = Line::new();
        for c in text.chars() {
            if line.is_ended() {
                self.push_context(line);
                line = Line::new();
            }
            line.push(c);
        }
        self.push_context(line);
        self.push_root_enddict();

        self.kvs.clone()
    }

    fn push_context(&mut self, line: Line) {
        let mut path = self.get_last_path();

        let last_indent = self.get_last_indent();
        if last_indent >= line.get_indent() {
            path.pop();
        }
        if last_indent > line.get_indent() {
            self.push(path.clone(), Tokens::EndDict);
        }
        if last_indent <= line.get_indent() {
            path.push(&line.get_key());
        }

        self.set_last_indent(line.get_indent());

        if !line.has_value() {
            self.push(path, Tokens::MkDict);
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
        self.push(path, value);
    }

    fn push_root_mkdict(&mut self) {
        self.push(Path::new(), Tokens::MkDict);
    }

    fn push_root_enddict(&mut self) {
        if self.get_last_indent() > 0 {
            let mut path = self.get_last_path();
            path.pop();
            self.push(path, Tokens::EndDict);
        }
        self.push(Path::new(), Tokens::EndDict);
    }

    fn push(&mut self, path: Path, value: Tokens) {
        self.kvs.push(Kv::with(path, value));
    }

    fn get_last_path(&self) -> Path {
        if let Some(last) = self.kvs.list().last() {
            return last.get_path();
        };
        Path::new()
    }

    fn get_last_indent(&self) -> usize {
        self.last_indent.clone()
    }

    fn set_last_indent(&mut self, indent: usize) {
        self.last_indent = indent;
    }
}
