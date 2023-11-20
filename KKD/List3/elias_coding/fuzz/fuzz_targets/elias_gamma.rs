#![no_main]

use std::io::{Cursor, Read};
use libfuzzer_sys::fuzz_target;
extern crate elias_coding;
use bit_queue::Bit;

fuzz_target!(|data: &[u8]| {
    if data.len() == 0 {
        return;
    }

    let flag_size = data[0] as usize;

    if data.len()  <= flag_size + 1 {
        return;
    }

    let mut flag = Vec::new();

    for i in 1..=flag_size {
        if data[i] & 1 == 1 {
            flag.push(Bit::One)
        } else {
            flag.push(Bit::Zero)
        }
    }

    let mut cursor = Cursor::new(data);
    let mut d = Vec::new();
    let mut buffer = [0; 2];

    while let Ok(_) = cursor.read_exact(&mut buffer) {
        d.push(u16::from_be_bytes(buffer));
    }

    let x = elias_coding::gamma_encode(&d, &flag);
    assert_eq!(elias_coding::gamma_decode(&x, &flag), Some(d));

});

