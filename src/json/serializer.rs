use crate::json::jsonparts::JsonParts;
use crate::json::jsonpath::JsonPath;
use crate::json::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonPathValue {
    pub path: JsonPath,
    pub value: JsonParts,
}

#[derive(Clone)]
pub struct Serializer {
    pub pathvalues: Vec<JsonPathValue>,
}
impl Serializer {
    pub fn new(json_string: &str) -> Self {
        let mut serializer = Serializer {pathvalues: Vec::new()};
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
                        self.pathvalues.push(JsonPathValue {
                            path: path.clone(),
                            value: JsonParts::StartDict,
                        });
                    }
                    '}' => path.end_dict(),
                    '[' => {
                        path.start_list();
                        self.pathvalues.push(JsonPathValue {
                            path: path.clone(),
                            value: JsonParts::StartList,
                        });
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
                            self.pathvalues.push(JsonPathValue {
                                path: path.clone(),
                                value: JsonParts::String(scalar.get_value()),
                            });
                            scalar = Scalar::new();
                            path.end_dict();
                        }
                        ']' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue {
                                path: path.clone(),
                                value: JsonParts::String(scalar.get_value()),
                            });
                            scalar = Scalar::new();
                            path.end_list();
                        }
                        ',' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue {
                                path: path.clone(),
                                value: JsonParts::String(scalar.get_value()),
                            });
                            scalar = Scalar::new();
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}
