use crate::json::part::Part;
use crate::json::path::{JsonPathIndicator, Path};
use crate::json::value::Value;
use crate::yaml::line::Line;

pub struct Deserializer {
    pub lines: Vec<Line>,
    spaces: usize,
}
impl Deserializer {
    pub fn new() -> Self {
        Deserializer {
            lines: Vec::new(),
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
        line.set_value(part.to_string());
        self.lines.push(line);
    }

    fn resolve_start_dict_(&mut self, mut path: Path) {
        if !path.is_root() {
            let indicator = &path.get_previous_indicator();
            let key = path.value[path.value.len() - 2].clone();
            let line = self.resolve_key_(indicator, key);
            self.lines.push(line);
            self.spaces += 2;
        }
    }

    fn resolve_end_dict_(&mut self, mut path: Path) {
        let indicator = &path.get_last_indicator();
        if indicator.count == 0 {
            if let Some(line) = self.lines.last_mut() {
                line.enable_empty_dict_blancket();
            }
        }
        if !path.is_root() {
            self.spaces -= 2;
        }
    }

    fn resolve_start_list_(&mut self, mut path: Path) {
        if !path.is_root() {
            let indicator = &path.get_previous_indicator();
            let key = path.value[path.value.len() - 2].clone();
            let line = self.resolve_key_(indicator, key);
            self.lines.push(line);
        }
    }

    fn resolve_end_list_(&mut self, mut path: Path) {
        let indicator = &path.get_last_indicator();
        if indicator.count == 0 {
            if let Some(line) = self.lines.last_mut() {
                line.enable_empty_list_blancket();
            }
        }
    }

    fn resolve_key_(&mut self, indicator: &JsonPathIndicator, key: String) -> Line {
        let mut line: Line = Line::new();
        if self.lines.len() > 1 && self.lines.last_mut().unwrap().is_hyphen_only() {
            line = self.lines.pop().unwrap();
        }
        if indicator.indicate == *"dict" {
            line.set_spaces(if line.need_hyphen {
                self.spaces - 2
            } else {
                self.spaces
            });
            line.set_key(key[1..].to_string());
            line.enable_colon();
        } else if indicator.indicate == *"list" {
            line.set_spaces(self.spaces);
            line.enable_hyphen();
        }
        line
    }

    pub fn print_debug(&mut self, values: Vec<Value>) {
        for mut value in values {
            println!("{}\t {:?}", value.path.to_string(), value.part);
        }
    }
}
