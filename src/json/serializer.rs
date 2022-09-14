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

// expecter
pub fn serialize (val: &str) {
  for i in val.chars() {
    let strtype = judge_strtype(&i);
    println!("{}: {}", strtype, i);
  }
}


struct Expecter {}

trait Expect {
  fn next(&self) -> String;
}

impl Expect for Expecter {
  fn next(&self) -> String {
    String::new()
  }
}
