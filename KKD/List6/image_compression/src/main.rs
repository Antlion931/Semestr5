use std::fs::File;
use std::env;
use std::io::Write;
use std::process;
use std::ptr::write_bytes;
use image_compression::*;
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

    let z_filter = Filter::new(|a: f64, b: f64| (a - b) / 2.0);
    let mut rgb_z_n = [z_filter.clone(), z_filter.clone(), z_filter.clone()];

    let y_filter = Filter::new(|a: f64, b: f64| (a + b) / 2.0);
    let mut rgb_y_n = [y_filter.clone(), y_filter.clone(), y_filter.clone()];

    for y in 0..pixels.height() {
        let x_iter: Box<dyn Iterator<Item = u32>>  = if y % 2 == 0 {
            Box::new(0..pixels.width())
        } else {
            Box::new((0..pixels.width()).rev())
        };

        for x in x_iter {
            for color in 0..3 {
                let c = pixels.get_pixel(x, y)[color];

                rgb_z_n[color].update(c as f64);
                rgb_y_n[color].update(c as f64);
            }
        }
    }

    // take only every other element
    let rgb_z_2n = rgb_z_n.into_iter().map(|x| x.get_elements().into_iter().enumerate().filter(|(n, _)| n % 2 != 0).map(|(_, v)| v).collect::<Vec<_>>()).collect::<Vec<_>>();
    let rgb_y_2n = rgb_y_n.into_iter().map(|x| x.get_diffrences_of_elements().into_iter().enumerate().filter(|(n, _)| n % 2 != 0).map(|(_, v)| v).collect::<Vec<_>>()).collect::<Vec<_>>();

    let rgb_z_quantized = rgb_z_2n.iter().map(|x| quantize(x, k)).collect::<Vec<_>>();
    let rgb_y_quantized = rgb_y_2n.iter().map(|x| quantize(x, k)).collect::<Vec<_>>();

    let mut output = BitQueue::new();

    let mut bit_coount = 0;

    //store k
    for j in 0..3 {
        if k & (1 << j) == 0 {
            output.push(Bit::Zero);
        } else {
            output.push(Bit::One);
        }

        bit_coount += 1;
    }

    //store quantized values
    for color in 0..3 {
        for i in rgb_y_quantized[color].iter().chain(rgb_z_quantized[color].iter()) {
            for byte in i.to_be_bytes() {
                for j in 0..8 {
                    if byte & (1 << j) == 0 {
                        output.push(Bit::Zero);
                    } else {
                        output.push(Bit::One);
                    }
                    bit_coount += 1;
                }
            }
        }
    }

    println!("{} bits", bit_coount);

    //store colors and filters as number to quantize
    for color in 0..3 {
        let mut decoded_number = 0;

        for i in &rgb_y_2n[color] {
            let diff = i - decoded_number as f64;

            let (min_index, min) = rgb_y_quantized[color].iter().enumerate().min_by(|(_, a), (_, b)| (**a - diff).abs().partial_cmp(&(**b - diff).abs()).unwrap()).unwrap();


            decoded_number += *min as i64;

            for j in 0..k {
                if min_index & (1 << j) == 0 {
                    output.push(Bit::Zero);
                } else {
                    output.push(Bit::One);
                }
                bit_coount += 1;
            }
        }

        decoded_number = 0;

        for i in &rgb_z_2n[color] {
            let diff = i - decoded_number as f64;

            let (min_index, min) = rgb_z_quantized[color].iter().enumerate().min_by(|(_, a), (_, b)| (**a - diff).abs().partial_cmp(&(**b - diff).abs()).unwrap()).unwrap();

            decoded_number += *min as i64;

            for j in 0..k {
                if min_index & (1 << j) == 0 {
                    output.push(Bit::Zero);
                } else {
                    output.push(Bit::One);
                }
                bit_coount += 1;
            }
        }
    }

    let mut output_file = File::create(args[2].as_str()).expect("Failed to create output file");

    println!("{} bits", bit_coount);

    output_file.write_all(&output.get_queue()).unwrap();
}
