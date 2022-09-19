use clap::Parser;

use jsonwith_formatter::json::serializer::{Serializer};
use std::fs;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let val = read(&path);
    let _serializer = Serializer::new(&val);
    // model to yaml
}

pub fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}
