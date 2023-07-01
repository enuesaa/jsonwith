use crate::{json2json, json2yaml};
use crate::cli::commands::{FormatArgs, Json2yamlArgs};

pub fn handle_format(args: FormatArgs) {
    let result = json2json(&args.json);
    println!("{}", result);
}

pub fn handle_json2yaml(args: Json2yamlArgs) {
    let result = json2yaml(&args.json);
    println!("{}", result);
}
