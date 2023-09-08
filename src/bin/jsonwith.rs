use std::io::IsTerminal;
use clap::{Parser, Subcommand, Args};

use jsonwith::{jsonformat, json2yaml};

#[derive(Parser)]
#[command(
    name = "jsonwith",
    about = "JSON Parser",
    disable_help_subcommand = true,
    version,
)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    /// format json
    Format(FormatArgs),
    /// convert json to yaml
    Json2yaml(Json2yamlArgs),
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

fn main() {
    let args = Cli::parse();
    let action = args.action;

    match action {
        Actions::Format(args) => {
            let json = args.json.unwrap_or_else(|| read_stdin());
            if json.len() == 0 {
                println!("Error: missing required argument `json`");
                std::process::exit(0);
            };
            let result = jsonformat(&json, args.indent);
            println!("{}", result);
        },
        Actions::Json2yaml(args) => {
            let json = args.json.unwrap_or_else(|| read_stdin());
            if json.len() == 0 {
                println!("Error: missing required argument `json`");
                std::process::exit(0);
            };
            let result = json2yaml(&json, args.indent);
            println!("{}", result);
        },
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
