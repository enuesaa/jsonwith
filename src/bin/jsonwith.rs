use clap::Parser;

use jsonwith_formatter::util::read;
use jsonwith_formatter::{json2json, json2yaml};

#[derive(Debug, Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    #[clap(short = 'f', long = "format")]
    format: String,

    #[clap(short = 'i', long = "input")]
    input: String,
}

fn main() {
    let args = Args::parse();
    if args.format == "yaml" {
        let json_string = read(&args.input);
        let yaml_string = json2yaml(&json_string);
        print!("{}", yaml_string);
    }
    if args.format == "json" {
        let json_string = read(&args.input);
        let formatted = json2json(&json_string);
        print!("{}", formatted);
    }
}
