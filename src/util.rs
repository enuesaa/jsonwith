use std::fs;

pub fn read(filename: &str) -> String {
  match fs::read_to_string(filename) {
      Err(reason) => panic!("failed to open file {}: {}", filename, reason),
      Ok(file) => file,
  }
}
