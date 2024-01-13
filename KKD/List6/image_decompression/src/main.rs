use std::fs::File;
use std::env;
use std::io::Write;
use std::process;
use bit_queue::{BitQueue, Bit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Wrong number of arguments");
        process::exit(1);
    }

    let image = image::open(args[1].as_str()).expect("Failed to open image");

    let k = args[3]
        .parse::<usize>()
        .expect("Failed to parse number of colors");

    assert!(k < 8);

    let pixels = image.as_rgb8().unwrap();
}
