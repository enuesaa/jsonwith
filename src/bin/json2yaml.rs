use clap::Parser;

use jsonwith_formatter::json::serializer::serialize;
use jsonwith_formatter::reader::read;

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
