use crate::json::line::Line;
use crate::json::part::Part;
use crate::json::path::{JsonPathIndicator, Path};
use crate::json::value::Value;

pub struct Deserializer {
    pub lines: Vec<Line>,
    pub indent: usize,
    spaces: usize,
}
impl Deserializer {
    pub fn new() -> Self {
        Deserializer {
            lines: Vec::new(),
            indent: 2,
            spaces: 0,
        }
    }

    pub fn deserialize(&mut self, values: Vec<Value>) -> String {
        for value in values {
            let path = value.path.clone();
            let part = value.part.clone();
            match part {
                Part::StartDict => self.resolve_start_dict_(path),
                Part::EndDict => self.resolve_end_dict_(path),
                Part::StartList => self.resolve_start_list_(path),
                Part::EndList => self.resolve_end_list_(path),
                _ => self.resolve_others_(path, part),
            }
        }
        self.lines.iter().map(|v| v.to_string()).collect()
    }

    fn resolve_others_(&mut self, mut path: Path, part: Part) {
        let indicator = &path.get_last_indicator();
        let key = path.value[path.value.len() - 1].clone();
        let mut line = self.resolve_key_(indicator, key);
        match part {
            Part::String(a) => line.set_value(format!("\"{}\"", a)),
            _ => line.set_value(part.to_string()),
        };
        line.enable_comma();
        self.lines.push(line);
    }

    fn resolve_start_dict_(&mut self, mut path: Path) {
        if !path.is_root() {
            let indicator = &path.get_previous_indicator();
            let key = path.value[path.value.len() - 2].clone();
            let mut line = self.resolve_key_(indicator, key);
            line.enable_start_dict_blancket();
            self.lines.push(line);
        } else {
            let mut line = Line::new();
            line.enable_start_dict_blancket();
            self.lines.push(line);
        }
        self.spaces += self.indent;
    }

    fn resolve_end_dict_(&mut self, mut path: Path) {
        self.spaces -= self.indent;
        let indicator = &path.get_last_indicator();
        if indicator.count == 0 {
            if let Some(line) = self.lines.last_mut() {
                line.enable_end_dict_blancket();
                if path.is_root() {
                    line.disable_comma();
                } else {
                    line.enable_comma();
                };
            }
        } else {
            if let Some(line) = self.lines.last_mut() {
                line.disable_comma();
            }
            let mut line = Line::new();
            line.set_spaces(self.spaces);
            line.enable_end_dict_blancket();
            if path.is_root() {
                line.disable_comma();
            } else {
                line.enable_comma();
            };
            self.lines.push(line);
        }
    }

    fn resolve_start_list_(&mut self, mut path: Path) {
        if !path.is_root() {
            let indicator = &path.get_previous_indicator();
            let key = path.value[path.value.len() - 2].clone();
            let mut line = self.resolve_key_(indicator, key);
            line.enable_start_list_blancket();
            self.lines.push(line);
        } else {
            let mut line = Line::new();
            line.enable_start_list_blancket();
            self.lines.push(line);
        }
        self.spaces += self.indent;
    }

    fn resolve_end_list_(&mut self, mut path: Path) {
        self.spaces -= self.indent;
        let indicator = &path.get_last_indicator();
        if indicator.count == 0 {
            if let Some(line) = self.lines.last_mut() {
                line.enable_end_list_blancket();
                if path.is_root() {
                    line.disable_comma();
                } else {
                    line.enable_comma();
                };
            }
        } else {
            if let Some(line) = self.lines.last_mut() {
                line.disable_comma();
            }
            let mut line = Line::new();
            line.set_spaces(self.spaces);
            line.enable_end_list_blancket();
            if path.is_root() {
                line.disable_comma();
            } else {
                line.enable_comma();
            };
            self.lines.push(line);
        }
    }

    fn resolve_key_(&mut self, indicator: &JsonPathIndicator, key: String) -> Line {
        let mut line: Line = Line::new();
        if indicator.indicate == *"dict" {
            line.set_spaces(self.spaces);
            line.set_key(format!("\"{}\"", key[1..].to_string()));
            line.enable_colon();
        } else if indicator.indicate == *"list" {
            line.set_spaces(self.spaces);
        }
        line
    }

    pub fn print_debug(&mut self, values: Vec<Value>) {
        for mut value in values {
            println!("{}\t {:?}", value.path.to_string(), value.part);
        }
    }
}
