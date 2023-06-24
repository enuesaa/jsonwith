use std::fs;
use clap::Parser;
use anyhow::anyhow;
use anyhow::Result as AnyhowResult;

use jsonwith::json2yamlv2;
use jsonwith::json2jsonv2;

#[derive(Debug, Parser)]
struct Args {
    #[clap(long = "format")]
    format: String,

    #[clap(long = "file")]
    file: String,
}

fn main() -> AnyhowResult<()> {
    let args = Args::parse();
    let format = args.format;
    let filename = args.file;

    if format == "yaml" {
        if let Ok(json_str) = fs::read_to_string(&filename) {
            let yaml_str = json2yamlv2(&json_str);
            println!("{}", yaml_str);
            return Ok(());
        } else {
            return Err(anyhow!("Failed to open file."));
        };
    };
    if format == "json" {
        if let Ok(json_str) = fs::read_to_string(&filename) {
            let formatted = json2jsonv2(&json_str);
            println!("{}", formatted);
            return Ok(());
        } else {
            return Err(anyhow!("Failed to open file."));
        };
    };

    Err(anyhow!("Argument invalid."))
}
