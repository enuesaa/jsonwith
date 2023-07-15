use std::io::IsTerminal;
use clap::{Parser, Subcommand, Args};

use jsonwith::{jsonformat, json2yaml};

#[derive(Parser)]
#[command(
    name = "jsonwith",
    about = "JSON Parser",
    disable_help_subcommand = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Subcommand)]
pub enum Actions {
    Format(FormatArgs),
    Json2yaml(Json2yamlArgs),
}

#[derive(Args)]
pub struct FormatArgs {
    pub json: Option<String>,
}

#[derive(Args)]
pub struct Json2yamlArgs {
    pub json: String,
}

fn main() {
    let args = Cli::parse();
    let action = args.action;

    match action {
        Actions::Format(args) => {
            let json = args.json.unwrap_or_else(|| {
                if std::io::stdin().is_terminal() {
                    return String::from("");
                };
                let mut input = String::from("");
                let _ = std::io::stdin().read_line(&mut input);
                input
            });
            if json.len() == 0 {
                println!("Error: missing required argument `json`");
                std::process::exit(0);
            };
            let result = jsonformat(&json);
            println!("{}", result);
        },
        Actions::Json2yaml(args) => {
            let result = json2yaml(&args.json);
            println!("{}", result);
        },
    };
}
