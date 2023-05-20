
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
