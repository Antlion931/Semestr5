#![no_main]

use std::io::{Cursor, Read};
use libfuzzer_sys::fuzz_target;
extern crate elias_coding;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    let mut d = Vec::new();
    let mut buffer = [0; 2];

    while let Ok(_) = cursor.read_exact(&mut buffer) {
        d.push(u16::from_be_bytes(buffer));
    }

    let x = elias_coding::gamma_encode(&d);
    assert_eq!(elias_coding::gamma_decode(x), d);
});

