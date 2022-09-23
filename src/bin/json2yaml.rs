use clap::Parser;

use jsonwith_formatter::json::serializer::Serializer;
use jsonwith_formatter::yaml::deserializer::Deserializer;
use std::fs;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let json_string = read(&args.path);
    let mut serializer = Serializer::new();
    serializer.serialize(&json_string);

    let values = serializer.values.clone();
    let mut deserializer = Deserializer::new(values);
    let yaml_string = deserializer.deserialize();
    print!("{}", yaml_string);
}

pub fn read(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Err(reason) => panic!("failed to open file {}: {}", filename, reason),
        Ok(file) => file,
    }
}
