use std::fmt;

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

    pub fn from(dotted: &str) -> Self {
        // TODO
        // dotted like `aaa[0].bbb`
        Path { route: vec![] }
    }

    pub fn push(&mut self, nest: &str) {
        self.route.push(nest.to_string());
    }

    pub fn pop(&mut self) {
        self.route.pop();
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.route.join("."))
    }
}
