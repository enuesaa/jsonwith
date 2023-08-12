use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum PathItem {
    Key(String),
    Index(usize),
}

// like json path
// - $.a
// - $.a.b
// - $.a[0].b
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    route: Vec<PathItem>,
}
impl Path {
    pub fn new() -> Self {
        Path { route: vec![] }
    }

    pub fn push(&mut self, s: &str) {
        if s.starts_with("[") && s.ends_with("]") {
            let i = s
                .trim_start_matches("[")
                .trim_end_matches("]")
                .parse::<usize>()
                .unwrap();
            self.route.push(PathItem::Index(i));
        } else {
            self.route.push(PathItem::Key(s.to_string()));
        }
    }

    pub fn push_key(&mut self, key: &str) {
        self.route.push(PathItem::Key(key.to_string()));
    }

    pub fn push_index(&mut self, i: usize) {
        self.route.push(PathItem::Index(i));
    }

    pub fn modify(&mut self, s: &str) {
        self.pop();
        self.push(s);
    }

    pub fn modify_key(&mut self, key: &str) {
        if self.is_last_key() {
            self.pop();
            self.push_key(key);
        };
    }

    pub fn modify_index(&mut self, i: usize) {
        if self.is_last_index() {
            self.pop();
            self.push_index(i);
        };
    }

    pub fn pop(&mut self) {
        self.route.pop();
    }

    pub fn get_last_key(&self) -> String {
        if let Some(PathItem::Key(s)) = self.route.last() {
            return s.to_string();
        };
        "".to_string()
    }

    pub fn get_last_index(&self) -> usize {
        if let Some(PathItem::Index(i)) = self.route.last() {
            return i.clone();
        };
        0
    }

    pub fn is_last_key(&self) -> bool {
        !self.is_last_index()
    }

    pub fn is_last_index(&self) -> bool {
        if let Some(PathItem::Index(_)) = self.route.last() {
            return true;
        };
        false
    }
}

impl From<&str> for Path {
    // from dotted like `$.a[0].b`
    fn from(dotted: &str) -> Self {
        if !dotted.starts_with("$") {
            return Path::new();
        };
        if dotted == "$" {
            return Path::new();
        };

        let mut path = Path::new();
        dotted // $.a[0].b
            .trim_start_matches("$") // .a[0].b
            .replace("[", ".[") // `.a.[0].b`
            .split(".")
            .skip(1)
            .for_each(|s| {
                path.push(s);
            });     
        path
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.route.len() == 0 {
            return write!(f, "$");
        };
        let values: Vec<String> = self
            .route
            .iter()
            .map(|i| match i {
                PathItem::Index(i) => format!("[{}]", i),
                PathItem::Key(k) => format!(".{}", k.clone()),
            })
            .collect();

        write!(f, "${}", values.join(""))
    }
}
