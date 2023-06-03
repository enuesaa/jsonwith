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
        path.push_key("bb");
        assert_eq!(path.to_string(), "$.a.bb");
    }

    #[test]
    fn pop_route() {
        let mut path = Path::from("$.a.bb");
        path.pop();
        assert_eq!(path.to_string(), "$.a");
    }

    #[test]
    fn create_array() {
        let mut path = Path::from("$.a.bb");
        path.push_index();
        assert_eq!(path.to_string(), "$.a.bb[0]");
    }

    #[test]
    fn create_nested_array() {
        let mut path = Path::from("$.a.bb");
        path.push_index();
        path.increment();
        path.increment();
        path.push_key("cc");
        path.push_index();
        path.increment();
        assert_eq!(path.to_string(), "$.a.bb[2].cc[1]");
    }

    #[test]
    fn from_array() {
        let path = Path::from("$.a[1].b");
        assert_eq!(path.to_string(), "$.a[1].b");
    }
}
