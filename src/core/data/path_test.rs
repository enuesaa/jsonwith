

#[cfg(test)]
mod tests {
    use crate::core::data::path::Path;

    #[test]
    fn root_path() {
        let path = Path::new();
        assert_eq!(path.to_string(), "$");
    }

    #[test]
    fn from_dotted_string() {
        let path = Path::from("$.a.b.c");
        assert_eq!(path.to_string(), "$.a.b.c");
    }

    #[test]
    fn from_dotted_string_but_invalid_path() {
        let path = Path::from("");
        assert_eq!(path.to_string(), "$");
    }

    #[test]
    fn push_route() {
        let mut path = Path::from("$.a");
        path.push("bb");
        assert_eq!(path.to_string(), "$.a.bb");
    }

    #[test]
    fn pop_route() {
        let mut path = Path::from("$.a.bb");
        path.pop();
        assert_eq!(path.to_string(), "$.a");
    }
}

