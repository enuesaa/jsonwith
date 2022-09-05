use clap::Parser;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.path);
}
