fn judge_strtype (val: &char) -> &str {
  match val {
    '{' => "start dict",
    '}' => "end dict",
    '"' => "quotation",
    '[' => "start list",
    ']' => "end list",
    _ => "others"
  }
}

pub fn parse (val: &str) {
  for i in val.chars() {
    let strtype = judge_strtype(&i);
    println!("{}: {}", strtype, i);
  }
}
