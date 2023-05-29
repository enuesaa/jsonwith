use std::fmt;

// json path like 
// $.a
// $.a.b
// $.a[0].b

#[derive(Clone, Debug)]
struct PathItem {
    value: String,
    index: bool, // this represents array index.
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
        self.route.push(PathItem { value: nest.to_string(), index: false });
    }

    pub fn push_index(&mut self, nest: &str) {
        self.route.push(PathItem { value: nest.to_string(), index: true });
    }

    pub fn is_index(&self) -> bool {
        if let Some(last) = self.route.last() {
            return last.index
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
            PathItem { value: v, index: false }
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
        let values: Vec<String> = self.route.iter().map(|i| i.value.clone()).collect();
        write!(f, "$.{}", values.join("."))
    }
}
