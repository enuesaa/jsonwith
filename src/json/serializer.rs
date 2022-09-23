use crate::json::jsonpath::JsonPath;
use crate::json::parts::{Parts, ScalarJudger};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonParts {
    StartDict,
    StartList,
    String(String),
    Boolean(String),
    Null,
    Number(String),
    NotDefined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonPathValue {
    pub path: JsonPath,
    pub value: JsonParts,
}

#[derive(Clone)]
pub struct Serializer {
    pub buff: Vec<Parts>,
    pub pathvalues: Vec<JsonPathValue>,
}
impl Serializer {
    pub fn new(json_string: &str) -> Self {
        let mut serializer = Serializer {
            buff: Vec::new(),
            pathvalues: Vec::new(),
        };
        serializer.parse(json_string);
        serializer
    }

    fn parse(&mut self, json_string: &str) {
        let mut path = JsonPath::new();
        let mut scalar_judger = ScalarJudger::new();
        for i in json_string.chars() {
            if scalar_judger.initial {
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
                    _ => scalar_judger.resolve_next(&i),
                };
            } else {
                if !scalar_judger.resolved {
                    scalar_judger.resolve_next(&i);
                }
                if scalar_judger.resolved {
                    match i {
                        ':' => {
                            path.add_dict_key(scalar_judger.get_value());
                            scalar_judger = ScalarJudger::new();
                        }
                        '}' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue {
                                path: path.clone(),
                                value: JsonParts::String(scalar_judger.get_value()),
                            });
                            scalar_judger = ScalarJudger::new();
                            path.end_dict();
                        }
                        ']' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue {
                                path: path.clone(),
                                value: JsonParts::String(scalar_judger.get_value()),
                            });
                            scalar_judger = ScalarJudger::new();
                            path.end_list();
                        }
                        ',' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue {
                                path: path.clone(),
                                value: JsonParts::String(scalar_judger.get_value()),
                            });
                            scalar_judger = ScalarJudger::new();
                        }
                        _ => {}
                    };
                }
            }
        }
    }
}
