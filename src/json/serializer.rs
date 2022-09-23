use crate::json::parts::{Parts, ScalarJudger};

pub struct JsonPath {
    pub value: Vec<String>, // [".", ".aaa", "[]", "[0]."]
    list_i_vec: Vec<usize>,
    list_i_type: Vec<String>,
}
impl JsonPath {
    pub fn new() -> Self {
        JsonPath {
            value: Vec::new(),
            list_i_vec: Vec::new(),
            list_i_type: Vec::new(),
        }
    }

    pub fn start_dict(&mut self) {
        self.list_i_vec.push(0);
        self.list_i_type.push("dict".to_string());
        self.value.push(".".to_string());
    }

    pub fn add_dict_key(&mut self, value: String) {
        let last = self.list_i_vec.last().unwrap().clone();
        self.list_i_vec.pop();
        self.list_i_vec.push(last + 1);

        let converted = Vec::from([".".to_string(), value]);
        self.value.pop();
        self.value.push(converted.join(""));
    }

    pub fn end_dict(&mut self) {
        self.value.pop();
        self.list_i_vec.pop();
        self.list_i_type.pop();
    }

    pub fn start_list(&mut self) {
        self.list_i_vec.push(0);
        self.list_i_type.push("list".to_string());

        self.value.push("[]".to_string());
    }

    pub fn add_something_item(&mut self) {
        let itemtype = self.list_i_type.last().unwrap().clone();
        if itemtype == "list".to_string() {
            self.add_list_item();
        }
        // dict の場合は add_dict_key() で既にキーが追加されている
    }

    pub fn add_list_item(&mut self) {
        let last = self.list_i_vec.last().unwrap().clone();
        self.list_i_vec.pop();
        self.list_i_vec.push(last + 1);

        let converted = Vec::from(["[".to_string(), last.to_string(), "]".to_string()]);
        self.value.pop();
        self.value.push(converted.join(""));
    }

    pub fn end_list(&mut self) {
        // let last = self.list_i_vec.last().unwrap().clone();
        // if last > 0 {
            self.value.pop();
        // }
        self.list_i_vec.pop();
        self.list_i_type.pop();
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
                        path.start_dict();
                        self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::StartDict});
                    },
                    '}' => {
                        path.end_dict();
                    },
                    '[' => {
                        path.start_list();
                        self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::StartList});
                    },
                    ']' => {
                        path.end_list();
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
                            path.add_dict_key(scalar_judger.get_value());
                            scalar_judger = ScalarJudger::new();
                        },
                        '}' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::String(scalar_judger.get_value())});
                            scalar_judger = ScalarJudger::new();
                            path.end_dict();
                        },
                        ']' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::String(scalar_judger.get_value())});
                            scalar_judger = ScalarJudger::new();
                            path.end_list();
                        },
                        ',' => {
                            path.add_something_item();
                            self.pathvalues.push(JsonPathValue{path: path.to_string(), value: JsonParts::String(scalar_judger.get_value())});
                            scalar_judger = ScalarJudger::new();
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
