
use crate::models::{DictModel, ArrayModel, ValueTrait};

pub fn serialize (val: &str) {
  let mut buff :Vec<&str> = Vec::new();

  for i in val.chars() {
    match i {
      '{' => buff.push(DictModel{}.get_name()),
      // '}' => buff.push(DictModel{}.get_name()),
      // '"' => buff.push("others"),
      '[' => buff.push(ArrayModel{}.get_name()),
      // ']' => buff.push("end list"),
      _ => buff.push("others"),
    };
    println!("{:?}", buff);
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
