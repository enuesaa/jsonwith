use clap::Parser;

mod file;
use self::file::read;
mod strparser;
use self::strparser::parse;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
  path: String,
}

fn main() {
  let args = Args::parse();
  let val = read(&args.path);
  parse(&val.as_str());
}
