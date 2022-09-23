use crate::json::part::Part;

#[derive(Debug, Clone)]
pub struct Scalar {
    chars: Vec<char>,
    pub part: Part,
}
impl Scalar {
    pub fn new() -> Self {
        Scalar {chars: Vec::new(), part: Part::NotDefined}
    }

    pub fn with_next(&mut self, char: &char) {
        self.part = self.judge_next(char);
        if self.part == Part::NotDefined {
            self.chars.push(char.clone());
        }
    }

    fn judge_next(&mut self, char: &char) -> Part {
        let val: String = self.chars.clone().into_iter().collect();
        let mut chars_next = self.chars.clone();
        chars_next.push(*char);
        let val_next: String = chars_next.into_iter().collect();

        if val.chars().count() > 2 && val.starts_with('\"') && val.ends_with('\"') {
            return Part::String(self.remove_quotaion(val));
        }
        if val == *"true" || val == *"false" {
            return Part::Boolean(self.remove_quotaion(val).parse().unwrap());
        }
        if val == *"null" {
            return Part::Null;
        }
        if val.chars().count() > 1
            && val.chars().all(char::is_numeric)
            && !val_next.chars().all(char::is_numeric)
        {
            return Part::Number(val.parse().unwrap());
        }
        Part::NotDefined
    }

    pub fn get_value(&mut self) -> String {
        self.remove_quotaion(self.chars.clone().into_iter().collect())
    }

    pub fn remove_quotaion(&mut self, mut val: String) -> String {
        if val.chars().count() > 2 && val.starts_with('\"') && val.ends_with('\"') {
            val = String::from(&val[1..val.len()-1]);
        }
        val
    }

    pub fn is_initialized(&mut self) -> bool {
        self.chars.iter().count() > 0
    }

    pub fn is_resolved(&mut self) -> bool {
        self.part != Part::NotDefined
    }
}
