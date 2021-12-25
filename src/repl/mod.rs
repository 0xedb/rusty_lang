use std::io::{self, BufRead, BufReader};

use rusty_lang::{lexer::Lexer, token::Unit};

const PROMPT: &str = "$ ";

pub fn start() {
    let mut scanner = BufReader::new(io::stdin());

    // loop {
        // print!("\n\n{}", PROMPT);

        let mut input = String::new();
        scanner.read_line(&mut input).expect("failed to read line");
        // println!("input -- {}", input);
        let mut l = Lexer::new(input);
        // println!("lexer -- {:?}", l);

        let mut tok = l.next_token();

        while tok.unit != Unit::EOF {
            println!("{:?}", tok);
            tok = l.next_token();
        }
    // }
}
