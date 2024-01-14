use elias_coding::decode;
use std::fs;
use std::env;
use std::io::Write;
use std::process;
use bit_queue::{BitQueue, Bit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Wrong number of arguments");
        process::exit(1);
    }

    let encoded_file = fs::read(&args[1]).expect("Unable to read file");

    if let Some((size, mut rest_of_file)) = decode(&encoded_file, &[], 2) {
        let width = size[0] as usize;
        let height = size[1] as usize;

        let mut k = 0;

        for _ in 0..3 {
            k <<= 1;
            match rest_of_file.pop() {
                Some(Bit::One) => k += 1,
                Some(Bit::Zero) => (),
                None => {
                    eprintln!("Error: File ended before k could be read");
                    process::exit(1);
                }
            }
        }

        let mut rgb_y_2n_quantized = [Vec::new(), Vec::new(), Vec::new()];
        let mut rgb_z_2n_quantized = [Vec::new(), Vec::new(), Vec::new()];

        for color in 0..3 {
            for _ in 0..(1 << k) {
                let mut number = 0u64;

                for _ in 0..64 {
                    number <<= 1;
                    match rest_of_file.pop() {
                        Some(Bit::One) => number += 1,
                        Some(Bit::Zero) => (),
                        None => {
                            eprintln!("Error: File ended before k could be read");
                            process::exit(1);
                        }
                    }
                }
                let number = f64::from_be_bytes(number.to_be_bytes());
                rgb_y_2n_quantized[color].push(number);
            }

            for _ in 0..(1 << k) {
                let mut number = 0u64;

                for _ in 0..64 {
                    number <<= 1;
                    match rest_of_file.pop() {
                        Some(Bit::One) => number += 1,
                        Some(Bit::Zero) => (),
                        None => {
                            eprintln!("Error: File ended before k could be read");
                            process::exit(1);
                        }
                    }
                }
                let number = f64::from_be_bytes(number.to_be_bytes());
                rgb_z_2n_quantized[color].push(number);
            }
        }

        let mut rgb_y_2n = [Vec::new(), Vec::new(), Vec::new()];
        let mut rgb_z_2n = [Vec::new(), Vec::new(), Vec::new()];

        for color in 0..3 {
            for _ in 0..(height*width / 2 + 1) {
                let mut number = 0;
                for _ in 0..k {
                    number <<= 1;
                    match rest_of_file.pop() {
                        Some(Bit::One) => number += 1,
                        Some(Bit::Zero) => (),
                        None => {
                            eprintln!("Error: File ended before k could be read");
                            process::exit(1);
                        }
                    }
                }
                rgb_y_2n[color].push(rgb_y_2n_quantized[color][number]);
            }

            for _ in 0..(height*width / 2 + 1) {
                let mut number = 0;
                for _ in 0..k {
                    number <<= 1;
                    match rest_of_file.pop() {
                        Some(Bit::One) => number += 1,
                        Some(Bit::Zero) => (),
                        None => {
                            eprintln!("Error: File ended before k could be read");
                            process::exit(1);
                        }
                    }
                }
                rgb_z_2n[color].push(rgb_z_2n_quantized[color][number]);
            }
        }

        let mut rgb_y_2n_after_diffrences = [vec![rgb_y_2n[0][0]], vec![rgb_y_2n[1][0]], vec![rgb_y_2n[2][0]]];

        for color in 0..3 {
            for i in 1..rgb_y_2n[color].len() {
                rgb_y_2n_after_diffrences[color].push(rgb_y_2n_after_diffrences[color][i - 1] + rgb_y_2n[color][i]);
            }
        }

        let rgb_y_2n = rgb_y_2n_after_diffrences;
        let rgb_z_2n = rgb_z_2n;

        let mut colors = [Vec::new(), Vec::new(), Vec::new()];

        for color in 0..3 {
            colors[color].push(rgb_y_2n[color][0] + rgb_z_2n[color][0]);
            for i in 1..rgb_y_2n[color].len() {
                colors[color].push(rgb_y_2n[color][i] - rgb_z_2n[color][i]);
                colors[color].push(rgb_y_2n[color][i] + rgb_z_2n[color][i]);
            }
        }
        
        let mut image = image::RgbImage::new(width as u32, height as u32);

        let mut index = 0;

        for y in 0..height as u32 {
            let x_iter: Box<dyn Iterator<Item = u32>>  = if y % 2 == 0 {
                Box::new(0..width as u32)
            } else {
                Box::new((0..width as u32).rev())
            };

            for x in x_iter {
                *image.get_pixel_mut(x, y) = image::Rgb([
                    colors[0][index] as u8,
                    colors[1][index] as u8,
                    colors[2][index] as u8,
                ]);
                index += 1;
            }
        }

        image.save(&args[2]).unwrap();
    }
}
