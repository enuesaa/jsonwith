use clap::Parser;

use std::fs;
use jsonwith_formatter::json2yaml;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let json_string = read(&args.path);
    let yaml_string = json2yaml(&json_string);
    print!("{}", yaml_string);
}

pub fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}
