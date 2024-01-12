use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::env;
use std::process;

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

    let k = args[3]
        .parse::<usize>()
        .expect("Failed to parse number of colors");

    let mut output = input.clone();

    let pixels = output.as_mut_rgb8().unwrap();
    let n = pixels.width() as usize 

    for x in 0..pixels.width() {
        //TODO:
    }

    output.save(args[2].as_str()).expect("Failed to save image");
}
