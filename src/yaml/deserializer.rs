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
            let path = value.path.clone();
            let part = value.part.clone();
            match part {
                Part::StartDict => {self.resolve_start_dict(path)},
                Part::EndDict => {self.resolve_end_dict(path)},
                Part::StartList => {self.resolve_start_list(path)},
                Part::EndList => {self.resolve_end_list(path)},
                _ => {self.resolve_others(path, part)}
            }
        }
        self.yaml_string.clone()
    }

    fn resolve_others(&mut self, mut path: Path, part: Part) {
        let indicator = &path.get_last_indicator();
        let key = path.value[path.value.len() - 1].clone();
        self.resolve_key(indicator, key);
        self.yaml_string += &format!("{}\n", part);
    }

    fn resolve_start_dict(&mut self, mut path: Path) {
        if !path.is_root() {
            let indicator = &path.get_previous_indicator();
            let key = path.value[path.value.len() - 2].clone();
            self.resolve_key(indicator, key);
            if indicator.indicate == *"dict" {
                self.yaml_string += "\n";
            }
            self.spaces += 2;
        }
    }

    fn resolve_end_dict(&mut self, mut path: Path) { 
        let indicator = &path.get_last_indicator();
        if indicator.count == 0 {
            self.yaml_string.pop();
            self.yaml_string += "{}\n";
        }
        if !path.is_root() {
            self.spaces -= 2;
        }
    }

    fn resolve_start_list(&mut self, mut path: Path) {
        if !path.is_root() {
            let indicator = &path.get_previous_indicator();
            let key = path.value[path.value.len() - 2].clone();
            self.resolve_key(indicator, key);
            if indicator.indicate == *"dict" {
                self.yaml_string += "\n";
            }
        }
    }

    fn resolve_end_list(&mut self, mut path: Path) {
        let indicator = &path.get_last_indicator();
        if indicator.count == 0 {
            self.yaml_string.pop();
            self.yaml_string += "[]\n";
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
