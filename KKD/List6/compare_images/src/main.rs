use image::GenericImageView;
use image::{ImageBuffer, Rgb};
use rand::Rng;
use std::env;
use std::process;

fn MSE<'a>(a: impl Iterator<Item = &'a Rgb<u8>>, b: impl Iterator<Item = &'a Rgb<u8>>) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;

    for (pixel_a, pixel_b) in a.zip(b) {
        let r = pixel_a[0] as f64 - pixel_b[0] as f64;
        let g = pixel_a[1] as f64 - pixel_b[1] as f64;
        let b = pixel_a[2] as f64 - pixel_b[2] as f64;

        sum += r * r + g * g + b * b;
        count += 1;
    }

    sum / count as f64
}

fn color_MSE<'a>(a: impl Iterator<Item = u8>, b: impl Iterator<Item = u8>) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;

    for (color_a, color_b) in a.zip(b) {
        let color = color_a as f64 - color_b as f64;

        sum += color * color;
        count += 1;
    }

    sum / count as f64
}

fn SNR<'a>(a: impl Iterator<Item = &'a Rgb<u8>>, b: impl Iterator<Item = &'a Rgb<u8>>) -> f64 {
    let mut mse_sum = 0.0;
    let mut sum = 0.0;

    for (pixel_a, pixel_b) in a.zip(b) {
        let r = pixel_a[0] as f64 - pixel_b[0] as f64;
        let g = pixel_a[1] as f64 - pixel_b[1] as f64;
        let b = pixel_a[2] as f64 - pixel_b[2] as f64;

        mse_sum += r * r + g * g + b * b;

        let r = pixel_a[0] as f64;
        let g = pixel_a[1] as f64;
        let b = pixel_a[2] as f64;

        sum += r * r + g * g + b * b;
    }

    (sum as f64 / mse_sum).log10() * 10.0
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Wrong number of arguments");
        process::exit(1);
    }

    let original = image::open(args[1].as_str()).expect("Failed to open image");
    let decoded = image::open(args[2].as_str()).expect("Failed to open image");


    println!("MSE: {}", MSE(original.as_rgb8().unwrap().pixels(), decoded.as_rgb8().unwrap().pixels()));
    println!("MSE red: {}", color_MSE(original.as_rgb8().unwrap().pixels().map(|p| p[0]), decoded.as_rgb8().unwrap().pixels().map(|p| p[0])));
    println!("MSE green: {}", color_MSE(original.as_rgb8().unwrap().pixels().map(|p| p[1]), decoded.as_rgb8().unwrap().pixels().map(|p| p[1])));
    println!("MSE blue: {}", color_MSE(original.as_rgb8().unwrap().pixels().map(|p| p[2]), decoded.as_rgb8().unwrap().pixels().map(|p| p[2])));
    println!("SNR: {} dB", SNR(original.as_rgb8().unwrap().pixels(), decoded.as_rgb8().unwrap().pixels()));
}
