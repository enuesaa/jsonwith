use clap::Parser;

mod file;
use self::file::open;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
  path: String,
}

fn main() {
  let args = Args::parse();
  let content = open(&args.path);
  println!("{}", content);
}
