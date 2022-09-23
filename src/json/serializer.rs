use crate::json::jsonpart::JsonPart;
use crate::json::jsonpath::JsonPath;
use crate::json::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonValue {
    pub path: JsonPath,
    pub value: JsonPart,
}
impl JsonValue {
    pub fn new(path: &JsonPath, value: JsonPart) -> Self {
        JsonValue { path: path.clone(), value }
    }
}

#[derive(Clone)]
pub struct Serializer {
    pub values: Vec<JsonValue>,
}
impl Serializer {
    pub fn new(json_string: &str) -> Self {
        let mut serializer = Serializer {values: Vec::new()};
        serializer.parse(json_string);
        serializer
    }

    fn parse(&mut self, json_string: &str) {
        let mut path = JsonPath::new();
        let mut scalar = Scalar::new();
        for i in json_string.chars() {
            if !scalar.is_initialized() {
                match i {
                    '{' => {
                        path.start_dict();
                        self.values.push(JsonValue::new(&path, JsonPart::StartDict));
                    }
                    '}' => path.end_dict(),
                    '[' => {
                        path.start_list();
                        self.values.push(JsonValue::new(&path, JsonPart::StartList));
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
                            path.add_something_item();
                            self.values.push(JsonValue::new(&path, JsonPart::String(scalar.get_value())));
                            scalar = Scalar::new();
                            path.end_dict();
                        }
                        ']' => {
                            path.add_something_item();
                            self.values.push(JsonValue::new(&path, JsonPart::String(scalar.get_value())));
                            scalar = Scalar::new();
                            path.end_list();
                        }
                        ',' => {
                            path.add_something_item();
                            self.values.push(JsonValue::new(&path, JsonPart::String(scalar.get_value())));
                            scalar = Scalar::new();
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}
