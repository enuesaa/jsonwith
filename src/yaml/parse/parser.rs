use crate::data::kv::Kv;
use crate::data::kvs::Kvs;
use crate::data::path::Path;
use crate::data::tokens::Tokens;
use crate::yaml::parse::line::Line;

pub struct Parser {
    kvs: Kvs,
    path: Path,
    last_indent: usize,
}
impl Parser {
    pub fn new() -> Self {
        Parser {
            kvs: Kvs::new(),
            path: Path::new(),
            last_indent: 0,
        }
    }

    pub fn parse(&mut self, text: &str) -> Kvs {
        self.push(Tokens::MkDict);

        let mut line = Line::new();
        for c in text.chars() {
            line.push(c);
            if line.is_ended() {
                self.push_line(&mut line);
                line = Line::new();
            }
        }
        self.push_line(&mut line);

        while self.path.len() > 0 {
            self.append_close_tag();
        }

        self.kvs.clone()
    }

    fn push_line(&mut self, line: &mut Line) {
        line.flush();
        if self.last_indent > line.get_indent() {
            // todo check logic
            let retain_index = (self.last_indent - line.get_indent()) / 2;
            while self.path.len() > retain_index + 1 {
                self.append_close_tag();
            };
            if line.has_hyphen() {
                if !self.path.is_last_index() {
                    self.push(Tokens::MkArray);
                };
                self.plus_index();
            };
            if line.has_key() {
                self.push(Tokens::MkDict);
            };
        }
        if self.last_indent < line.get_indent() {
            if line.has_hyphen() {
                if !self.path.is_last_index() {
                    self.push(Tokens::MkArray);
                };
                self.plus_index();
            };
            if line.has_key() {
                self.push(Tokens::MkDict);
            };
        }
        if self.last_indent == line.get_indent() {
            if line.has_hyphen() {
                if !self.path.is_last_index() {
                    // close prev dict
                    self.append_close_tag();
                };
                self.plus_index();
                if line.has_key() {
                    self.push(Tokens::MkDict);
                };
            } else {
                self.path.pop();
            };
        };

        self.last_indent = line.get_indent().clone();

        if line.has_key() {
            self.path.push(&line.get_key());
        };
        if line.has_value() {
            self.push(line.get_value_as_token());
        };
    }

    fn push(&mut self, value: Tokens) {
        self.kvs.push(Kv::with(self.path.clone(), value));
    }

    fn plus_index(&mut self) {
        if self.path.is_last_index() {
            let index = self.path.get_last_index();
            self.path.modify_index(index + 1);
        } else {
            self.path.push_index(0);
        }
    }

    fn append_close_tag(&mut self) {
        if self.path.is_last_index() {
            self.path.pop();
            self.push(Tokens::EndArray);
        } else {
            self.path.pop();
            self.push(Tokens::EndDict);
        }
    }
}
