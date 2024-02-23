use clap::{Args, Parser, Subcommand, crate_version, CommandFactory};
use std::io::IsTerminal;

use jsonwith::{json2yaml, jsonformat, yaml2json};

#[derive(Parser)]
#[command(
    name = "jsonwith",
    about = "Toy JSON Parser & Formatter",
    disable_help_subcommand = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Option<Actions>,

    #[arg(short = 'v', long = "version", help = "Print Version", global = true)]
    pub version: bool,
}

#[derive(Subcommand)]
pub enum Actions {
    /// format json
    Format(FormatArgs),
    /// convert json to yaml
    Json2yaml(Json2yamlArgs),
    /// convert yaml to json [under development]
    Yaml2json(Yaml2jsonArgs),
}

#[derive(Args)]
pub struct FormatArgs {
    /// json string like '{"a":"b"}'
    pub json: Option<String>,

    /// indent size
    #[arg(long, default_value_t = 2)]
    pub indent: usize,
}

#[derive(Args)]
pub struct Json2yamlArgs {
    /// json string like '{"a":"b"}'
    pub json: Option<String>,

    /// indent size
    #[arg(long, default_value_t = 2)]
    pub indent: usize,
}

#[derive(Args)]
pub struct Yaml2jsonArgs {
    /// yaml string like 'a: b\nc: d'
    pub yaml: Option<String>,
}

fn main() {
    let args = Cli::parse();
    if args.version {
        println!("{}", crate_version!());
        return;
    }

    match args.action {
       Some(Actions::Format(args)) => {
            let json = args.json.unwrap_or_else(|| read_stdin());
            if json.len() == 0 {
                println!("Error: missing required argument `json`");
                std::process::exit(0);
            };
            let result = jsonformat(&json, args.indent);
            println!("{}", result);
        }
        Some(Actions::Json2yaml(args)) => {
            let json = args.json.unwrap_or_else(|| read_stdin());
            if json.len() == 0 {
                println!("Error: missing required argument `json`");
                std::process::exit(0);
            };
            let result = json2yaml(&json, args.indent);
            println!("{}", result);
        }
        Some(Actions::Yaml2json(args)) => {
            println!("Warning: The `yaml2json` subcommand is under development.");
            let yaml = args.yaml.unwrap_or_else(|| read_stdin());
            if yaml.len() == 0 {
                println!("Error: missing required argument `yaml`");
                std::process::exit(0);
            };
            let result = yaml2json(&yaml, 2);
            println!("{}", result);
        }
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
        }
    };
}

fn read_stdin() -> String {
    if std::io::stdin().is_terminal() {
        return String::from("");
    };
    let mut input = String::from("");
    let _ = std::io::stdin().read_line(&mut input);
    input
}
