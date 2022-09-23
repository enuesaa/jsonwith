use crate::json::jsonparts::JsonParts;

#[derive(Debug, Clone)]
pub struct Scalar {
    chars: Vec<char>,
    part: JsonParts,
}
impl Scalar {
    pub fn new() -> Self {
        Scalar {chars: Vec::new(), part: JsonParts::NotDefined}
    }

    pub fn with_next(&mut self, char: &char) {
        self.part = self.judge_next(char);
        if self.part == JsonParts::NotDefined {
            self.chars.push(char.clone());
        }
    }

    fn judge_next(&mut self, char: &char) -> JsonParts {
        let val: String = self.chars.clone().into_iter().collect();
        let mut chars_next = self.chars.clone();
        chars_next.push(*char);
        let val_next: String = chars_next.into_iter().collect();

        if val.chars().count() > 2 && val.starts_with('\"') && val.ends_with('\"') {
            return JsonParts::String(val);
        }
        if val == *"true" || val == *"false" {
            return JsonParts::Boolean(val);
        }
        if val == *"null" {
            return JsonParts::Null;
        }
        if val.chars().count() > 1
            && val.chars().all(char::is_numeric)
            && !val_next.chars().all(char::is_numeric)
        {
            return JsonParts::Number(val);
        }
        JsonParts::NotDefined
    }

    pub fn get_value(&mut self) -> String {
        self.chars.clone().into_iter().collect()
    }

    pub fn is_initialized(&mut self) -> bool {
        self.chars.iter().count() > 0
    }

    pub fn is_resolved(&mut self) -> bool {
        self.part != JsonParts::NotDefined
    }
}
