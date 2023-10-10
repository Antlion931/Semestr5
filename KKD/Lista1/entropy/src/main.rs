use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<_> = env::args().collect();
    
    if args.len() != 2 {
        println!("Wrong amount of operands, usage: ./entropy <file_path>");
        process::exit(1)
    }

    let file_path = args.get(1).expect("We know there are args is len 2");

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let bytes = contents.as_bytes();

    println!("{bytes:?}");
}
