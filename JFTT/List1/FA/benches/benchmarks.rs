#![feature(test)]
extern crate test;

use test::{black_box, Bencher};
use FA::longest_prefix_of_a_that_is_suffix_of_b;

#[bench]
fn bench_10_incorrect_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 10].iter().collect();
    let second: String = vec!['b'; 10].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_100_incorrect_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 100].iter().collect();
    let second: String = vec!['b'; 100].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_1000_incorect_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 1000].iter().collect();
    let second: String = vec!['b'; 1000].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_10_000_incorect_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 10_000].iter().collect();
    let second: String = vec!['b'; 10_000].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_10_correct_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 10].iter().collect();
    let second: String = vec!['a'; 10].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_100_correct_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 100].iter().collect();
    let second: String = vec!['a'; 100].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_1000_corect_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 1000].iter().collect();
    let second: String = vec!['a'; 1000].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}

#[bench]
fn bench_10_000_corect_length_longest_prefix_of_a_that_is_suffix_of_b(b: &mut Bencher) {
    let first: String = vec!['a'; 10_000].iter().collect();
    let second: String = vec!['a'; 10_000].iter().collect();

    b.iter(|| {
        black_box(longest_prefix_of_a_that_is_suffix_of_b(
            black_box(&first),
            black_box(&second),
        ))
    });
}
