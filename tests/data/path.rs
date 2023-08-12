use jsonwith::data::path::Path;

#[test]
fn test_new() {
    let path = Path::new();
    assert_eq!(path.to_string(), "$");
}

#[test]
fn test_root() {
    let path = Path::from("$");
    assert_eq!(path.to_string(), "$");
}

#[test]
fn test_root_array() {
    let path = Path::from("$[1]");
    assert_eq!(path.to_string(), "$[1]");
}

#[test]
fn test_from_dotted_string() {
    let path = Path::from("$.a.b.c");
    assert_eq!(path.to_string(), "$.a.b.c");
}

#[test]
fn test_from_dotted_string_but_invalid_path() {
    let path = Path::from("");
    assert_eq!(path.to_string(), "$");
}

#[test]
fn test_push_route() {
    let mut path = Path::from("$.a");
    path.push_key("bb");
    assert_eq!(path.to_string(), "$.a.bb");
}

#[test]
fn test_pop_route() {
    let mut path = Path::from("$.a.bb");
    path.pop();
    assert_eq!(path.to_string(), "$.a");
}

#[test]
fn test_create_array() {
    let mut path = Path::from("$.a.bb");
    path.push_index(0);
    assert_eq!(path.to_string(), "$.a.bb[0]");
}

#[test]
fn test_create_nested_array() {
    let mut path = Path::from("$.a.bb");
    path.push_index(0);
    path.modify_index(path.get_last_index() + 1);
    path.modify_index(path.get_last_index() + 1);
    path.push_key("cc");
    path.push_index(0);
    path.modify_index(path.get_last_index() + 1);
    assert_eq!(path.to_string(), "$.a.bb[2].cc[1]");
}

#[test]
fn test_from_array() {
    let path = Path::from("$.a[1].b");
    assert_eq!(path.to_string(), "$.a[1].b");
}
