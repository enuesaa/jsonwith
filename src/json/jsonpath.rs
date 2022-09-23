#[derive(Debug, Clone, PartialEq)]
struct JsonPathIndicator {
    indicate: String,
    count: usize,
}
impl JsonPathIndicator {
    fn add_count(&mut self) {
        self.count += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonPath {
    pub value: Vec<String>, // [".", ".aaa", "[]", "[0]."]
    indicators: Vec<JsonPathIndicator>,
}
impl JsonPath {
    pub fn new() -> Self {
        JsonPath {value: Vec::new(), indicators: Vec::new()}
    }

    pub fn start_dict(&mut self) {
        self.add_something_item();
        self.indicators.push(JsonPathIndicator{indicate: String::from("dict"), count: 0});
        self.value.push(".".to_string());
    }

    pub fn add_dict_key(&mut self, value: String) {
        if let Some(last) = self.indicators.last_mut() {
            last.add_count();
        }
        if let Some(last_value) = self.value.last_mut() {
            *last_value = String::from(".") + &value;
        }
    }

    pub fn end_dict(&mut self) {
        self.value.pop();
        self.indicators.pop();
    }

    pub fn start_list(&mut self) {
        self.indicators.push(JsonPathIndicator{indicate: String::from("list"), count: 0});
        self.value.push(String::from("[]"));
    }

    pub fn add_something_item(&mut self) {
        let last = self.indicators.last();
        if last != None && last.unwrap().indicate == String::from("list") {
            self.add_list_item();
        }
        // dict の場合は add_dict_key() で既にキーが追加されている
    }

    pub fn add_list_item(&mut self) {
        if let Some(last) = self.indicators.last_mut() {
            if let Some(last_value) = self.value.last_mut() {
                *last_value = String::from("[") + &last.count.to_string() + "]";
            }
            last.add_count();
        }
    }

    pub fn end_list(&mut self) {
        self.value.pop();
        self.indicators.pop();
    }

    pub fn to_string(&mut self) -> String {
        self.value.clone().join("")
    }
}
