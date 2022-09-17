
/** https://qiita.com/togatoga/items/9d600e20325775f09547 */
#[derive(Debug, Clone)]
pub enum ScalarTypes {
    NotDefined,
    String,
    Boolean,
    Null,
    Number,
}

#[derive(Debug)]
pub enum Parts {
    StartDict, // {
    EndDict,   // }
    StartList, // [
    EndList,   // ]
    Comma,     // ,
    Scalar(ScalarTypes),    // "aa", 99, null, true, false
    Others,    // temporary. for development.
}


#[derive(Debug, Clone)]
pub struct ScalarJudger {
    chars: Vec<char>,
}
impl ScalarJudger {
    pub fn new() -> Self {
        ScalarJudger {
            chars: Vec::new(),
        }
    }

    pub fn is_empty(&mut self) -> bool {
        return self.chars.is_empty();
    }

    pub fn append(&mut self, char: &char) -> ScalarTypes {
        self.chars.push(*char);
        return self.judge();
    }

    /**
     * 現状 副作用あり
     */
    fn judge(&mut self) -> ScalarTypes {
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
        if val.chars().all(char::is_numeric) {
            /* @memo check last char */
            self.initialize();
            return ScalarTypes::Number;
        }
        return ScalarTypes::NotDefined;
    }

    fn initialize(&mut self) {
        self.chars = Vec::new();
    }
}
