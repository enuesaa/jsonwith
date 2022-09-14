use clap::Parser;

use json2any::reader::read;
use json2any::json::serializer::serialize;

#[derive(Parser)]
#[clap(disable_help_flag = true)]
struct Args {
  path: String,
}

fn main() {
  let args = Args::parse();
  let path = args.path;
  let val = read(&path);
  let _model = serialize(&val);
  // model to yaml
}
