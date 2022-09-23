/** https://qiita.com/togatoga/items/9d600e20325775f09547 */

/** @todo いろいろネストしているが一次元にする. たとえばPartsのenum定義の一つとしてStringをもってくる */
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarTypes {
    NotDefined,
    String,
    Boolean,
    Null,
    Number,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub scalar_type: ScalarTypes,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parts {
    StartDict,           // {
    EndDict,             // }
    StartList,           // [
    EndList,             // ]
    Comma,               // ,
    Colon,               // :
    Scalar(Scalar), // "aa", 99, null, true, false
    Others,              // temporary. for development.
}

#[derive(Debug, Clone)]
pub struct ScalarJudger {
    chars: Vec<char>,
    pub resolved: bool,
    pub initial: bool,
    pub scalar_type: ScalarTypes,
}
impl ScalarJudger {
    pub fn new() -> Self {
        // @todo fix resolved
        ScalarJudger {
            chars: Vec::new(),
            resolved: true,
            initial: true,
            scalar_type: ScalarTypes::NotDefined,
        }
    }

    pub fn resolve_next(&mut self, char: &char) {
        self.initial = false;
        self.scalar_type = self.judge_next(char);
        if self.scalar_type == ScalarTypes::NotDefined {
            self.resolved = false;
            self.chars.push(*char);
        } else {
            let a: String = self.chars.clone().into_iter().collect();
            println!("{}", a);
            self.resolved = true;
        }
    }

    fn judge_next(&mut self, char: &char) -> ScalarTypes {
        let val: String = self.chars.clone().into_iter().collect();
        let mut chars_next = self.chars.clone();
        chars_next.push(*char);
        let val_next: String = chars_next.into_iter().collect();

        if val.chars().count() > 2 && val.starts_with('\"') && val.ends_with('\"') {
            return ScalarTypes::String;
        }
        if val == *"true" || val == *"false" {
            return ScalarTypes::Boolean;
        }
        if val == *"null" {
            return ScalarTypes::Null;
        }
        if val.chars().count() > 1
            && val.chars().all(char::is_numeric)
            && !val_next.chars().all(char::is_numeric)
        {
            return ScalarTypes::Number;
        }
        ScalarTypes::NotDefined
    }

    pub fn get_value(&mut self) -> String {
        self.chars.clone().into_iter().collect()
    }
}

impl Default for ScalarJudger {
    fn default() -> Self {
        Self::new()
    }
}
