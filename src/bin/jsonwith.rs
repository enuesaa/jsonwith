use clap::{Parser, Subcommand, Args};

use jsonwith::json2yaml;
use jsonwith::json2json;

#[derive(Parser, Debug)]
#[command(name = "jsonwith", about = "JSON Parser", disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    pub action: Actions,
}

#[derive(Args, Debug)]
struct FormatArgs {
    pub json: String,
}

#[derive(Args, Debug)]
struct Json2yamlArgs {
    pub json: String,
}

#[derive(Subcommand, Debug)]
enum Actions {
    Format(FormatArgs),
    Json2yaml(Json2yamlArgs),
}

fn main() {
    let args = Cli::parse();
    let action = args.action;

    match action {
        Actions::Format(args) => {
            let formatted = json2json(&args.json);
            println!("{}", formatted);
        },
        Actions::Json2yaml(args) => {
            let formatted = json2yaml(&args.json);
            println!("{}", formatted);
        },
    };
}
