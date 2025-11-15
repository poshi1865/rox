use rox::lexer;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rox [filepath]");
        std::process::exit(-1);
    }
    let source_file_path: String = args[1].to_string();

    lexer::print_tokens(source_file_path);
}
