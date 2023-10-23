use std::env::args;
use std::fs;
use std::process;
use KMP::pattern_finder;

fn main() {
    let args: Vec<_> = args().skip(1).collect();

    if args.len() != 2 {
        println!("Wrong usage, KMP <pattern> <file>");
        process::exit(1);
    }

    let pattern = &args[0];
    let file = &args[1];

    if let Ok(content) = fs::read_to_string(file) {
        println!("{:?}", pattern_finder(pattern, content.as_str()));
    } else {
        println!("Couldn't open a file");
        process::exit(1);
    }
}
