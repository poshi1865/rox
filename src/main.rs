use rox::lexer::Lexer;
use rox::lexer::RoxError;

use std::env;

fn main() -> Result<(), RoxError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rox [filepath]");
        std::process::exit(-1);
    }
    let source_file_path: String = args[1].to_string();

    let lexer = Lexer::new(source_file_path).map_err(RoxError::IOError)?;

    let mut token_len = 0;
    for token in lexer.into_iter() {
        println!("{}", token);
        token_len += 1;
    }
    println!("TOKEN LEN: {}", token_len);

    Ok(())
}
