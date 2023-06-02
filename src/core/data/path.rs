use std::fmt;

// json path like 
// $.a
// $.a.b
// $.a[0].b

#[derive(Clone, Debug)]
struct PathItem {
    value: String,
     // If value is usize, it represents array index. If value is none, it represents value is not array.
    index: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Path {
    route: Vec<PathItem>,
}
impl Path {
    pub fn new() -> Self {
        Path {
            route: vec![],
        }
    }

    pub fn push(&mut self, nest: &str) {
        self.route.push(PathItem { value: nest.to_string(), index: None });
    }

    pub fn increment(&mut self) {
        if let Some(last) = self.route.last_mut() {
            if let Some(index) = last.index {
                last.index = Some(index + 1)
            } else {
                last.index = Some(0);
            };
        }
    }

    pub fn is_array(&self) -> bool {
        if let Some(last) = self.route.last() {
            return last.index.is_some()
        }
        false
    }

    pub fn pop(&mut self) {
        self.route.pop();
    }
}

impl From<&str> for Path {
    // from dotted like `$.a.0.b`
    fn from(dotted: &str) -> Self {
        if !dotted.starts_with("$") {
            return Path { route: vec![] }
        }

        let trimed = dotted.trim_start_matches("$.");
        let route: Vec<PathItem> = trimed.split(".").map(|s| s.to_string()).into_iter().map(|v| {
            PathItem { value: v, index: None }
        }).collect();
        Path { route }
    }
}

// see https://doc.rust-lang.org/std/string/trait.ToString.html
// impl ToString for Path {
//     fn to_string(&self) -> String {
//         "$".to_string() + &self.route.join(".")
//     }
// }

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.route.len() == 0 {
            return write!(f, "$");
        };
        let values: Vec<String> = self.route.iter().map(|i| {
            if let Some(index) = i.index {
                return i.value.clone() + &format!("[{}]", index);
            };
            return i.value.clone();
        }).collect();

        write!(f, "$.{}", values.join("."))
    }
}
