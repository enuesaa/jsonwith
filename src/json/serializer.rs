use crate::json::parts::{Parts, ScalarJudger, Scalar};

pub struct JsonPath {
    pub value: Vec<String>, // [".", "aaa", ".", "0"]
}
impl JsonPath {
    pub fn new() -> Self {
        JsonPath { value: Vec::new() }
    }

    pub fn push(&mut self, value: &mut str) {
        self.value.push(value.to_string());
    }
    
    pub fn removelast(&mut self) {
        self.value.pop();
    }

    pub fn to_string(self) -> String {
        self.value.join("")
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
        let mut scalar_judger = ScalarJudger::new();
        for i in json_string.chars() {
            // @todo fix. this code checks `scalar judger is initialized`.
            if !scalar_judger.resolved {
                scalar_judger.resolve_next(&i);
                if scalar_judger.resolved {
                    self.buff.push(Parts::Scalar(Scalar{scalar_type: scalar_judger.clone().scalar_type, value: scalar_judger.clone().get_value()}));
                    scalar_judger = ScalarJudger::new();
                    match i {
                        '{' => self.buff.push(Parts::StartDict),
                        '}' => self.buff.push(Parts::EndDict),
                        '[' => self.buff.push(Parts::StartList),
                        ']' => self.buff.push(Parts::EndList),
                        ',' => self.buff.push(Parts::Comma),
                        ':' => self.buff.push(Parts::Colon),
                        '\n' => {}
                        ' ' => {}
                        _ => {}
                    };
                }
            } else {
                match i {
                    '{' => {
                        self.pathvalues.push(JsonPathValue{path: "".to_string(), value: JsonParts::StartDict});
                    },
                    '}' => {},
                    '[' => {
                        self.pathvalues.push(JsonPathValue{path: "".to_string(), value: JsonParts::StartList});
                    },
                    ']' => {},
                    ',' => {},
                    ':' => {},
                    '\n' => {}
                    ' ' => {}
                    _ => {
                        scalar_judger.resolve_next(&i);
                        if scalar_judger.resolved {
                            self.buff.push(Parts::Scalar(Scalar{scalar_type: scalar_judger.clone().scalar_type, value: scalar_judger.clone().get_value()}));
                            scalar_judger = ScalarJudger::new();
                        }
                    }
                };
            }
        }
    }
}
