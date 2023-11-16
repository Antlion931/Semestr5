#![no_main]

use libfuzzer_sys::fuzz_target;
extern crate lzw;

fuzz_target!(|data: &[u8]| {
    let x = lzw::encode(data);
    assert_eq!(lzw::decode(x), data);
});
