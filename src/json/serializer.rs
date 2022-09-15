use crate::json::parts::{Parts, ScalarTypes};

#[derive(Debug, Clone)]
struct ScalarJudger {
    chars: Vec<char>,
}
impl ScalarJudger {
    pub fn new() -> Self {
        ScalarJudger {
            chars: Vec::new(),
        }
    }

    fn append(&mut self, char: &char) {
        self.chars.push(*char);
    }

    /**
     * 現状 副作用あり
     */
    fn judge(&mut self) -> ScalarTypes {
        /* @memo cloneをすべきなのか、良く分かっていない */
        let val: String = self.chars.clone().into_iter().collect();
        if val == " ".to_string() {
            self.initialize();
            return ScalarTypes::NotDefined;
        }
        if val.starts_with("\"") && val.ends_with("\"") {
            self.initialize();
            return ScalarTypes::String;
        }
        if val == "true".to_string() || val == "false".to_string() {
            self.initialize();
            return ScalarTypes::Boolean;    
        }
        if val == "null".to_string() {
            self.initialize();
            return ScalarTypes::Null;    
        }
        /* https://programming-idioms.org/idiom/137/check-if-string-contains-only-digits/2189/rust */
        /* @memo check last char */
        if val.chars().all(char::is_numeric) {
            self.initialize();
            return ScalarTypes::Number;
        }
        return ScalarTypes::NotDefined;
    }

    fn initialize(&mut self) {
        self.chars = Vec::new();
    }
}

pub fn serialize(val: &str) {
    let mut buff: Vec<Parts> = Vec::new();
    let mut scalar_judger = ScalarJudger::new();

    for i in val.chars() {
        match i {
            '{' => buff.push(Parts::StartDict),
            '}' => buff.push(Parts::EndDict),
            '[' => buff.push(Parts::StartList),
            ']' => buff.push(Parts::EndList),
            ',' => buff.push(Parts::Comma),
            '\n' => {}
            _ => {
                scalar_judger.append(&i);
                let scalar_type = scalar_judger.judge();
                /* @todo refactor */
                match scalar_type {
                    ScalarTypes::NotDefined => {},
                    _ => { buff.push(Parts::Scalar(scalar_type)) },
                }
            }
        };
    }
    println!("{:?}", buff);
}
