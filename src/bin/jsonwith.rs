use clap::{Parser, Subcommand};

use jsonwith::cli::commands::{FormatArgs, Json2yamlArgs};
use jsonwith::cli::handlers::{handle_format, handle_json2yaml};

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

fn main() {
    let args = Cli::parse();
    let action = args.action;

    match action {
        Actions::Format(args) => handle_format(args),
        Actions::Json2yaml(args) => handle_json2yaml(args),
    };
}
