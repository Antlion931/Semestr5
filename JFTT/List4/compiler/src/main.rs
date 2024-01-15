mod ast;

use lalrpop_util::lalrpop_mod;
use std::io::{self, BufRead};
use std::fs;

lalrpop_mod!(pub parser);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let code = fs::read_to_string(&args[1]).expect("Error reading file");

    let ast = parser::program_all(&code).unwrap();

    println!("{:#?}", ast);
}

