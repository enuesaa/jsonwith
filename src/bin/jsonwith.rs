use clap::Parser;

use jsonwith::util::read;
use jsonwith::{json2json, json2yaml};

#[derive(Debug, Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    #[clap(short = 'f', long = "format")]
    format: String,

    #[clap(short = 'i', long = "input")]
    input: String,

    #[clap(long = "indent", default_value = "2")]
    indent: usize,
}

fn main() {
    let args = Args::parse();
    if args.format == "yaml" {
        let json_string = read(&args.input);
        let yaml_string = json2yaml(&json_string, args.indent);
        print!("{}", yaml_string);
    }
    if args.format == "json" {
        let json_string = read(&args.input);
        let formatted = json2json(&json_string, args.indent);
        print!("{}", formatted);
    }
}
