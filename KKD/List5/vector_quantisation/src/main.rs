use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::env;
use std::process;

fn MSE(a: &[&Rgb<u8>], b: &[&Rgb<u8>]) -> f64 {
    let mut sum = 0.0;

    for (pixel_a, pixel_b) in a.iter().zip(b.iter()) {
        let r = pixel_a[0] as f64 - pixel_b[0] as f64;
        let g = pixel_a[1] as f64 - pixel_b[1] as f64;
        let b = pixel_a[2] as f64 - pixel_b[2] as f64;

        sum += r * r + g * g + b * b;
    }

    sum / a.len() as f64
}

fn SNR(a: &[&Rgb<u8>], b: &[&Rgb<u8>]) -> f64 {
    let mse = MSE(a, b);

    let mut sum = 0.0;

    for pixel_a in a.iter() {
        let r = pixel_a[0] as f64;
        let g = pixel_a[1] as f64;
        let b = pixel_a[2] as f64;

        sum += r * r + g * g + b * b;
    }

    let a= sum / a.len() as f64;

    (a / mse).log10() * 10.0
}

fn average_colors(pixels: &[&Rgb<u8>], color_to_block: &[usize], blocks: usize) -> Vec<Rgb<u8>> {
    let mut red_sums_in_blocks = vec![0; blocks];
    let mut green_sums_in_blocks = vec![0; blocks];
    let mut blue_sums_in_blocks = vec![0; blocks];
    let mut counts_in_blocks = vec![0; blocks];

    for (pixel, block) in pixels.iter().zip(color_to_block.iter()) {
        red_sums_in_blocks[*block] += pixel[0] as usize;
        green_sums_in_blocks[*block] += pixel[1] as usize;
        blue_sums_in_blocks[*block] += pixel[2] as usize;
        counts_in_blocks[*block] += 1;
    }

    red_sums_in_blocks
        .into_iter()
        .zip(green_sums_in_blocks.into_iter())
        .zip(blue_sums_in_blocks.into_iter())
        .zip(counts_in_blocks.into_iter())
        .map(|(((r, g), b), count)| {
            if count != 0 {
                Rgb([(r / count) as u8, (g / count) as u8, (b / count) as u8])
            } else {
                Rgb([0, 0, 0])
            }
        })
        .collect()
}

fn color_distance(a: &Rgb<u8>, b: &Rgb<u8>) -> u32 {
    let r = (a[0] as i16 - b[0] as i16).abs();
    let g = (a[1] as i16 - b[1] as i16).abs();
    let b = (a[2] as i16 - b[2] as i16).abs();

    r as u32 + g as u32 + b as u32
}

fn blocks_from_colors(pixels: &[&Rgb<u8>], colors: &[Rgb<u8>], old_blocks: &[usize]) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let new_colors: Vec<_> = colors
        .iter()
        .map(|c| {
            let r;
            let g;
            let b;

            if rng.gen_bool(0.5) {
                r = c[0].saturating_add(1);
            } else {
                r = c[0].saturating_sub(1);
            }

            if rng.gen_bool(0.5) {
                g = c[1].saturating_add(1);
            } else {
                g = c[1].saturating_sub(1);
            }

            if rng.gen_bool(0.5) {
                b = c[2].saturating_add(1);
            } else {
                b = c[2].saturating_sub(1);
            }

            Rgb([r, g, b])
        })
        .collect();

    pixels
        .iter()
        .enumerate()
        .map(|(n, p)| {
            let current_distance = color_distance(p, &colors[old_blocks[n]]);
            let new_distance = color_distance(p, &new_colors[old_blocks[n]]);
            if current_distance <= new_distance {
                old_blocks[n]
            } else {
                old_blocks[n] + colors.len()
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Wrong number of arguments");
        process::exit(1);
    }

    let mut input = image::open(args[1].as_str()).expect("Failed to open image");
    let pixels = input
        .as_mut_rgb8()
        .expect("Failed to convert image to RGB8")
        .pixels()
        .collect::<Vec<_>>();

    let n = args[3]
        .parse::<usize>()
        .expect("Failed to parse number of colors");
    let mut blocks = vec![0; pixels.len()];
    let mut colors = average_colors(&pixels, &blocks, 1);

    for _ in 0..n {
        blocks = blocks_from_colors(&pixels, &colors, &blocks);
        colors = average_colors(&pixels, &blocks, colors.len() * 2);
    }

    let mut output = input.clone();
        
    output.as_mut_rgb8()
        .expect("Failed to convert image to RGB8")
        .pixels_mut()
        .zip(blocks.into_iter())
        .for_each(|(pixel, block)| {
            *pixel = colors[block];
        });
    output.save(args[2].as_str()).expect("Failed to save image");

    let input_pixels = input
        .as_rgb8()
        .expect("Failed to convert image to RGB8")
        .pixels()
        .collect::<Vec<_>>();

    let output_pixels = output
        .as_rgb8()
        .expect("Failed to convert image to RGB8")
        .pixels()
        .collect::<Vec<_>>();
 
    println!("MSE: {}", MSE(&input_pixels, &output_pixels));
    println!("SNR: {} dB", SNR(&input_pixels, &output_pixels));
}
