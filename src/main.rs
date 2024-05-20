mod token;

use std::{env, io};

use crate::token::lex;

const DEFAULT_PROMPT: &str = "> ";

fn main() {
    // Check for prompt environment variable; use default if unavailable
    let prompt = env::var("ASSH_PROMPT").unwrap_or(DEFAULT_PROMPT.to_string());

    loop {
        print!("{prompt}");

        let mut input = String::new();
        if let Err(err) = io::stdin().read_line(&mut input) {
            eprintln!("{err}");
        }
        let input = input;
        println!("{:#?}", lex(input.as_str()));
    }
}
