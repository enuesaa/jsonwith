use std::fs;
use clap::Parser;
use anyhow::anyhow;
use anyhow::Result;

use jsonwith::json2yamlv2;
use jsonwith::{json2json, json2yaml, json2jsonv2};

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

fn main() -> Result<()> {
    let args = Args::parse();
    let format = args.format;
    let filename = args.input;

    if format == "yaml" {
        if let Ok(json_str) = fs::read_to_string(&filename) {
            let yaml_str = json2yaml(&json_str, args.indent);
            println!("{}", yaml_str);
            return Ok(());
        } else {
            return Err(anyhow!("Failed to open file."));
        };
    };
    if format == "json" {
        if let Ok(json_str) = fs::read_to_string(&filename) {
            let formatted = json2json(&json_str, args.indent);
            println!("{}", formatted);
            return Ok(());
        } else {
            return Err(anyhow!("Failed to open file."));
        };
    };

    if format == "json2jsonv2" {
        if let Ok(json_str) = fs::read_to_string("./tests/assets/minimum.json") {
            json2jsonv2(&json_str);
            return Ok(());
        };
    };

    if format == "json2yamlv2" {
        if let Ok(json_str) = fs::read_to_string("./tests/assets/minimum.json") {
            json2yamlv2(&json_str);
            return Ok(());
        };
    };

    Err(anyhow!("Argument invalid."))
}
