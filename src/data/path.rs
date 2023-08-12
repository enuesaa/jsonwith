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

    pub fn increment_index(&mut self) {
        if let Some(last) = self.route.last_mut() {
            if let PathItem::Index(i) = last {
                *last = PathItem::Index(*i + 1);
            };
        };
    }

    pub fn pop(&mut self) {
        self.route.pop();
    }

    #[deprecated]
    pub fn get_last(&self) -> Option<PathItem> {
        if let Some(last) = self.route.last() {
            return Some(last.clone());
        };
        None
    }

    pub fn is_last_index(&self) -> bool {
        if let Some(PathItem::Index(_)) = self.get_last() {
            return true;
        };
        false
    }
}

impl From<&str> for Path {
    // from dotted like `$.a[0].b`
    fn from(dotted: &str) -> Self {
        if !dotted.starts_with("$") {
            return Path { route: vec![] };
        };
        if dotted == "$" {
            return Path { route: vec![] };
        };

        // convert `$.a[0].b` to `.a.[0].b`
        let prefmt = dotted.trim_start_matches("$").replace("[", ".[");
        let route: Vec<PathItem> = prefmt
            .split(".")
            .skip(1)
            .map(|s| {
                let value = s.to_string();
                if s.starts_with("[") && s.ends_with("]") {
                    let i = s
                        .trim_start_matches("[")
                        .trim_end_matches("]")
                        .parse::<usize>()
                        .unwrap();
                    PathItem::Index(i)
                } else {
                    PathItem::Key(value)
                }
            })
            .collect();

        Path { route }
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
