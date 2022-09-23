use crate::json::value::Value;
use crate::json::path::Path;
use crate::json::scalar::Scalar;

#[derive(Clone)]
pub struct Serializer {
    pub values: Vec<Value>,
}
impl Serializer {
    pub fn new() -> Self {
        Serializer {values: Vec::new()}
    }

    pub fn serialize(&mut self, json_string: &str) {
        let mut path = Path::new();
        let mut scalar = Scalar::new();
        for i in json_string.chars() {
            if !scalar.is_initialized() {
                match i {
                    '{' => {
                        path.start_dict();
                        self.values.push(Value::start_dict(&path));
                    }
                    '}' => path.end_dict(),
                    '[' => {
                        path.start_list();
                        self.values.push(Value::start_list(&path));
                    }
                    ']' => path.end_list(),
                    ',' | ':' | '\n' | ' ' => {}
                    _ => scalar.with_next(&i),
                };
            } else {
                if !scalar.is_resolved() {
                    scalar.with_next(&i);
                }
                if scalar.is_resolved() {
                    match i {
                        ':' => {
                            path.add_dict_key(scalar.get_value());
                            scalar = Scalar::new();
                        }
                        '}' => {
                            self.values.push(Value::scalar(&path, scalar));
                            scalar = Scalar::new();
                            path.end_dict();
                        }
                        ']' => {
                            path.add_list_key_if_in_list_scope();
                            self.values.push(Value::scalar(&path, scalar));
                            scalar = Scalar::new();
                            path.end_list();
                        }
                        ',' => {
                            path.add_list_key_if_in_list_scope();
                            self.values.push(Value::scalar(&path, scalar));
                            scalar = Scalar::new();
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}
