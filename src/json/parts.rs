
/** https://qiita.com/togatoga/items/9d600e20325775f09547 */
#[derive(Debug, Copy, Clone)]
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
    Colon,     // :
    Scalar(ScalarTypes),    // "aa", 99, null, true, false
    Others,    // temporary. for development.
}


#[derive(Debug, Clone)]
pub struct ScalarJudger {
    chars: Vec<char>,
    pub resolved: bool,
    pub scalar_type: ScalarTypes,
}
impl ScalarJudger {
    pub fn new() -> Self {
        ScalarJudger {chars: Vec::new(), resolved: true, scalar_type: ScalarTypes::NotDefined}
    }

    pub fn append(&mut self, char: &char) {
        self.resolved = false;
        self.chars.push(*char);
        self.judge();
    }

    fn judge(&mut self) -> bool {
        let val: String = self.chars.clone().into_iter().collect();
        if val.chars().count() > 2 && val.starts_with("\"") && val.ends_with("\"") {
            return self.resolve_with_type(ScalarTypes::String);
        }
        if val == "true".to_string() || val == "false".to_string() {
            return self.resolve_with_type(ScalarTypes::Boolean);
        }
        if val == "null".to_string() {
            return self.resolve_with_type(ScalarTypes::Null);
        }
        // https://programming-idioms.org/idiom/137/check-if-string-contains-only-digits/2189/rust
        // @memo check last char
        if val.chars().all(char::is_numeric) {
            return self.resolve_with_type(ScalarTypes::Number);
        }
        return false;
    }

    fn resolve_with_type(&mut self, t: ScalarTypes) -> bool {
        self.resolved = true;
        self.scalar_type = t;
        let val :String = self.chars.clone().into_iter().collect();
        println!("{}", val);
        return true;
    }
}
