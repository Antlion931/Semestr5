#![no_main]

use std::io::{Cursor, Read};
use libfuzzer_sys::fuzz_target;
extern crate fibonaci_coding;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    let mut d = Vec::new();
    let mut buffer = [0; 2];

    while let Ok(_) = cursor.read_exact(&mut buffer) {
        d.push(u16::from_be_bytes(buffer));
    }

    let x = fibonaci_coding::encode(&d);
    assert_eq!(fibonaci_coding::decode(x), d);
});

