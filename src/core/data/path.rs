use std::fmt;

// json path like 
// $.a
// $.a.b
// $.a.0.b // differs from json path

#[derive(Clone, Debug)]
pub struct Path {
    route: Vec<String>,
}
impl Path {
    pub fn new() -> Self {
        Path {
            route: vec![],
        }
    }

    pub fn push(&mut self, nest: &str) {
        self.route.push(nest.to_string());
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
        let route: Vec<String> = trimed.split(".").map(|s| s.to_string()).collect();
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
        write!(f, "$.{}", self.route.join("."))
    }
}
