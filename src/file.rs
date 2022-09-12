use std::fs::File;

pub fn open (filename: &str) -> File {
  match File::open(filename) {
    Err(reason) => panic!("failed to open file {}: {}", filename, reason),
    Ok(file) => file,
  }
}