#![forbid(unsafe_code)]

mod token;

use core::panic;
use std::io::Write;

use crate::token::lex;

const DEFAULT_PROMPT: &str = "> ";

fn main() {
    // Check for prompt environment variable; use default if unavailable
    let prompt = std::env::var("ASSH_PROMPT").unwrap_or(DEFAULT_PROMPT.to_string());

    loop {
        print!("{prompt}");
        if let Err(err) = std::io::stdout().flush() {
            panic!("Failed to flush stdout: {err}");
        }

        let mut input = String::new();
        if let Err(err) = std::io::stdin().read_line(&mut input) {
            eprintln!("{err}");
        }
        let input = input;
        println!("{:#?}", lex(input.as_str()));
    }
}
