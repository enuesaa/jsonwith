use crate::data::kv::Kv;
use crate::data::kvs::Kvs;
use crate::data::path::Path;
use crate::data::tokens::Tokens;
use crate::yaml::parse::line::Line;

pub struct Parser {
    kvs: Kvs,
    last_indent: usize,
    last_has_hyphen: bool,
    next_mk_path: Option<Path>,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            kvs: Kvs::new(),
            last_indent: 0,
            last_has_hyphen: false,
            next_mk_path: None,
        }
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        self.push_mkdict(Path::new());

        let mut line = Line::new();
        for c in text.chars() {
            line.push(c);
            if line.is_ended() {
                self.push_line(line);
                line = Line::new();
            }
        }
        self.push_line(line);
        self.append_close_tags();
        self.push_enddict(Path::new());

        self.kvs.clone()
    }

    fn push_line(&mut self, line: Line) {
        let mut path = self.get_last_path();

        if self.last_indent > line.get_indent() {
            path.pop();
            self.push(path.clone(), Tokens::EndDict);
            path.pop();
            path.push(&line.get_key());
        }
        if self.last_indent < line.get_indent() {
            if let Some(next) = self.next_mk_path.clone() {
                if line.has_hyphen() {
                    self.push(next.clone(), Tokens::MkArray);
                } else {
                    self.push(next.clone(), Tokens::MkDict);
                }
                self.next_mk_path = None;
            };
            path.push(&line.get_key());
        }
        if self.last_indent == line.get_indent() {
            path.pop();
            path.push(&line.get_key());
        }
        // if !self.last_has_hyphen && line.has_hyphen() {
        //     self.push(next.clone(), Tokens::MkArray);
        // }

        self.last_indent = line.get_indent().clone();

        if !line.has_value() {
            self.next_mk_path = Some(path.clone());
            return;
        }
        self.push(path, self.judge_token(line.get_value()));
    }

    fn judge_token(&self, text: String) -> Tokens {
        match text.as_str() {
            "null" => Tokens::Null,
            "false" => Tokens::Bool(false),
            "true" => Tokens::Bool(true),
            "" => Tokens::String(text),
            _ => {
                if text.chars().all(|c| c.is_numeric()) {
                    Tokens::Number(text.parse::<usize>().unwrap())
                } else {
                    Tokens::String(text)
                }
            },
        }
    }

    fn push(&mut self, path: Path, value: Tokens) {
        self.kvs.push(Kv::with(path, value));
    }

    fn push_mkdict(&mut self, path: Path) {
        self.push(path, Tokens::MkDict);
    }

    fn push_enddict(&mut self, path: Path) {
        self.push(path, Tokens::EndDict);
    }

    fn append_close_tags(&mut self) {
        if self.last_indent > 0 {
            let mut path = self.get_last_path();
            path.pop();
            self.push_enddict(path);
        }
    }

    fn get_last_path(&self) -> Path {
        if let Some(last) = self.kvs.list().last() {
            return last.get_path();
        };
        Path::new()
    }
}
