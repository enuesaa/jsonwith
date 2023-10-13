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
        self.push_mkdict();

        let mut line = Line::new();
        for c in text.chars() {
            line.push(c);
            if line.is_ended() {
                line.flush();
                self.push_line(line);
                line = Line::new();
            }
        }
        line.flush();
        self.push_line(line);

        self.append_close_tags();

        self.kvs.clone()
    }

    fn push_line(&mut self, line: Line) {
        if self.last_indent > line.get_indent() {
            self.append_close_tag_of_prev_dict();
            self.path.pop();
        }
        if self.last_indent < line.get_indent() {
            if line.has_hyphen() {
                if !self.path.is_last_index() {
                    self.push_mkarray();
                }
                self.plus_index();
            };
            if line.has_key() {
                self.push_mkdict();
            };
        }
        if self.last_indent == line.get_indent() {
            if line.has_hyphen() {
                if !self.path.is_last_index() {
                    self.append_close_tag_of_prev_dict();
                };
                self.plus_index();
                if line.has_key() {
                    self.push_mkdict();
                };
            } else {
                if self.path.is_last_index() {
                    self.path.pop();
                    self.push_endarray();
                }
                self.path.pop();    
            };
        };

        self.last_indent = line.get_indent().clone();

        if line.has_key() {
            self.path.push(&line.get_key());
        };
        if line.has_value() {
            self.push(self.judge_token(line.get_value()));
        };
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

    fn push(&mut self, value: Tokens) {
        self.kvs.push(Kv::with(self.path.clone(), value));
    }

    fn push_mkdict(&mut self) {
        self.push(Tokens::MkDict);
    }

    fn push_enddict(&mut self) {
        self.push(Tokens::EndDict);
    }

    fn push_mkarray(&mut self) {
        self.push(Tokens::MkArray);
    }

    fn push_endarray(&mut self) {
        self.push(Tokens::EndArray);
    }

    fn plus_index(&mut self) {
        if self.path.is_last_index() {
            let index = self.path.get_last_index();
            self.path.modify_index(index + 1);
        } else {
            self.path.push_index(0);
        }
    }

    fn append_close_tag_of_prev_dict(&mut self) {
        self.path.pop();
        self.push_enddict();
    }

    fn append_close_tags(&mut self) {
        while self.path.len() > 0 {
            if self.path.is_last_index() {
                self.path.pop();
                self.push_endarray();
            } else {
                self.path.pop();
                self.push_enddict();
            }
        }
    }
}
