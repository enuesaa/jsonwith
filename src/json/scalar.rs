use crate::json::jsonpart::JsonPart;

#[derive(Debug, Clone)]
pub struct Scalar {
    chars: Vec<char>,
    part: JsonPart,
}
impl Scalar {
    pub fn new() -> Self {
        Scalar {chars: Vec::new(), part: JsonPart::NotDefined}
    }

    pub fn with_next(&mut self, char: &char) {
        self.part = self.judge_next(char);
        if self.part == JsonPart::NotDefined {
            self.chars.push(char.clone());
        }
    }

    fn judge_next(&mut self, char: &char) -> JsonPart {
        let val: String = self.chars.clone().into_iter().collect();
        let mut chars_next = self.chars.clone();
        chars_next.push(*char);
        let val_next: String = chars_next.into_iter().collect();

        if val.chars().count() > 2 && val.starts_with('\"') && val.ends_with('\"') {
            return JsonPart::String(val);
        }
        if val == *"true" || val == *"false" {
            return JsonPart::Boolean(val);
        }
        if val == *"null" {
            return JsonPart::Null;
        }
        if val.chars().count() > 1
            && val.chars().all(char::is_numeric)
            && !val_next.chars().all(char::is_numeric)
        {
            return JsonPart::Number(val);
        }
        JsonPart::NotDefined
    }

    pub fn get_value(&mut self) -> String {
        self.chars.clone().into_iter().collect()
    }

    pub fn is_initialized(&mut self) -> bool {
        self.chars.iter().count() > 0
    }

    pub fn is_resolved(&mut self) -> bool {
        self.part != JsonPart::NotDefined
    }
}
