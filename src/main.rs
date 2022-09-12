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
  println!("{}", args.path);
  let a = open(&args.path);
  println!("{:?}", a);
}
