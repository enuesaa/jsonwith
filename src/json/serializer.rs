use crate::json::parts::{Parts, ScalarJudger};

pub struct JsonPath {
    pub value: Vec<String>, // [".", "aaa", ".", "0"]
}
impl JsonPath {
    pub fn new() -> Self {
        JsonPath { value: Vec::new() }
    }

    pub fn push(&mut self, value: String) {
        self.value.push(value);
    }
    
    pub fn removelast(&mut self) {
        self.value.pop();
    }

    pub fn to_string(&mut self) -> String {
        self.value.clone().join("")
    }
}

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
    path: String,
    value: JsonParts,
}

#[derive(Clone)]
pub struct Serializer {
    pub buff: Vec<Parts>,
    pub pathvalues: Vec<JsonPathValue>
}
impl Serializer {
    pub fn new(json_string: &str) -> Self {
        let mut serializer = Serializer {buff: Vec::new(), pathvalues: Vec::new()};
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
                        path.push(".".to_string());
                        self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::StartDict});
                    },
                    '}' => {
                        path.removelast();
                    },
                    '[' => {
                        path.push("[".to_string());
                        self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::StartList});
                    },
                    ']' => {
                        path.removelast();
                    },
                    ',' => {},
                    ':' => {},
                    '\n' => {}
                    ' ' => {}
                    _ => {
                        scalar_judger.resolve_next(&i);
                        // if scalar_judger.resolved {
                            // value, key
                            // self.buff.push(Parts::Scalar(Scalar{scalar_type: scalar_judger.clone().scalar_type, value: scalar_judger.clone().get_value()}));
                            // scalar_judger = ScalarJudger::new();
                        // }
                    }
                };
            } else {
                if !scalar_judger.resolved {
                    scalar_judger.resolve_next(&i);
                }
                if scalar_judger.resolved {
                    match i {
                        ':' => {
                            path.push(scalar_judger.get_value());
                            scalar_judger = ScalarJudger::new();
                        },
                        '}' => {
                            self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::String(scalar_judger.get_value())});
                            scalar_judger = ScalarJudger::new();
                            path.removelast(); // last key of dict like "aaa-key1"
                            path.removelast(); // dict dot like "."
                            path.removelast(); // dict name like "aaa"
                        },
                        ']' => {
                            self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::String(scalar_judger.get_value())});
                            scalar_judger = ScalarJudger::new();
                            path.removelast();
                        },
                        ',' => {
                            self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::String(scalar_judger.get_value())});
                            scalar_judger = ScalarJudger::new();
                            path.removelast();
                        },
                        '\n' => {}
                        ' ' => {}
                        _ => {}
                    };
                }
            }
        }
    }
}
