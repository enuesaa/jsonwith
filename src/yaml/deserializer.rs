use crate::json::path::{Path, JsonPathIndicator};
use crate::json::value::Value;
use crate::json::part::Part;

pub struct Deserializer {
    pub yaml_string: String,
    spaces: usize,
}
impl Deserializer {
    pub fn new() -> Self {
        Deserializer{yaml_string: String::from(""), spaces: 0}
    }

    pub fn deserialize(&mut self, values: Vec<Value>) -> String {
        for value in values {
            self.deserialize_part(value.path.clone(), value.part.clone());
        }
        self.yaml_string.clone()
    }

    fn deserialize_part(&mut self, path: Path, part: Part) {
        let is_root = path.indicators.len() == 1;
        match part {
            Part::StartDict => {self.resolve_start_dict(path, is_root)},
            Part::EndDict => {self.resolve_end_dict(path, is_root)},
            Part::StartList => {self.resolve_start_list(path, is_root)},
            Part::EndList => {self.resolve_end_list(path)},
            _ => {
                if let Some(indicator) = path.indicators.last() {
                    let key = path.value[path.value.len() - 1].clone();
                    self.resolve_key(indicator, key);
                    if indicator.indicate == *"dict" {
                        self.yaml_string += &format!("{}\n", part);
                    } else if indicator.indicate == *"list" {
                        self.yaml_string += &format!("{}\n", part);
                    } 
                }
            }
        }
    }

    fn resolve_start_dict(&mut self, path: Path, is_root: bool) {
        if !is_root {
            let indicator = &path.indicators[path.indicators.len() - 2].clone();
            let key = path.value[path.value.len() - 2].clone();
            self.resolve_key(indicator, key);
            if indicator.indicate == *"dict" {
                self.yaml_string += "\n";
            }
            self.spaces += 2;
        }
    }

    fn resolve_end_dict(&mut self, path: Path, is_root: bool) { 
        let indicator = &path.indicators[path.indicators.len() - 1].clone();
        if indicator.count == 0 {
            self.yaml_string += "{}\n";
        }
        if !is_root {
            self.spaces -= 2;
        }
    }

    fn resolve_start_list(&mut self, path: Path, is_root: bool) {
        if !is_root {
            let indicator = &path.indicators[path.indicators.len() - 2].clone();
            let key = path.value[path.value.len() - 2].clone();
            self.resolve_key(indicator, key);
            if indicator.indicate == *"dict" {
                self.yaml_string += "\n";
            }
        }
    }

    fn resolve_end_list(&mut self, path: Path) {
        let indicator = &path.indicators[path.indicators.len() - 1].clone();
        if indicator.count == 0 {
            self.yaml_string += "{}\n";
        }
    }

    fn resolve_key(&mut self, indicator: &JsonPathIndicator, key: String) {
        if indicator.indicate == *"dict" {
            let need_spaces = self.spaces - self.yaml_string.split("\n").last().unwrap_or("").as_bytes().len();
            self.yaml_string += &format!("{}{}: ", " ".repeat(need_spaces), &key[1..]);
        } else if indicator.indicate == *"list" {
            self.yaml_string += &format!("{}- ", " ".repeat(self.spaces));
        }
    }

    pub fn print_debug(&mut self, values: Vec<Value>) {
        for mut value in values {
            println!("{}\t {:?}", value.path.to_string(), value.part);
        }
    }
}
