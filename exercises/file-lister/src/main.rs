mod arg_parser;
mod file_handler;

use arg_parser::Args;
use file_handler::list_dir;

fn main() {
    let args = Args::parse_args();
    list_dir(&args.path, args.json, args.recursive, args.all);
}
