use clap::Parser;

mod file;
use self::file::read;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
  path: String,
}

fn main() {
  let args = Args::parse();
  let content = read(&args.path);
  println!("{}", content);
}
