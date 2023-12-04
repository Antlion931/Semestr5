extern crate args;

use image::{Rgb, ImageBuffer}; 
use args::{Args,ArgsError};
use std::env;
use getopts::Occur;
use std::cmp::{max, min};

fn entropy_in(counts: &[u64], count: u64) -> Result<f64, String> {
    let mut sum = 0.0;

    if count == 0 {
        return Err("count zero")?;
    }

    for c in counts {
        if *c == 0 {
            continue;
        }

        let probability = *c as f64 / count as f64;
        let information = -probability.log2();

        sum += probability * information
    }

    if sum.is_finite() {
        Ok(sum)
    } else {
        Err("Not a number")?
    }
}

fn entropy_for<'a>(message: impl Iterator<Item = &'a u8>) -> Result<f64, String> {
    let mut counts = vec![0; u8::MAX as usize + 1];
    let mut count = 0;

    for byte in message {
        counts[*byte as usize] += 1;
        count += 1;
    }

    entropy_in(counts.as_slice(), count)
}

fn predictor_w(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let w = match x.checked_sub(1) {
                Some(new_x) => buffer.get_pixel(new_x, y),
                None => &Rgb([0, 0, 0]),
            };

            let value = pixel[i].wrapping_sub(w[i]);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_nw(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let nw = match (x.checked_sub(1), y.checked_sub(1)) {
                (Some(new_x), Some(new_y)) => buffer.get_pixel(new_x, new_y),
                _ => &Rgb([0, 0, 0]),
            };

            let value = pixel[i].wrapping_sub(nw[i]);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_n(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let n = match y.checked_sub(1) {
                Some(new_y) => buffer.get_pixel(x, new_y),
                None => &Rgb([0, 0, 0]),
            };

            let value = pixel[i].wrapping_sub(n[i]);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_n_plus_w_minus_nw(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let n = match y.checked_sub(1) {
                Some(new_y) => buffer.get_pixel(x, new_y),
                None => &Rgb([0, 0, 0]),
            };

            let w = match x.checked_sub(1) {
                Some(new_x) => buffer.get_pixel(new_x, y),
                None => &Rgb([0, 0, 0]),
            };

            let nw = match (x.checked_sub(1), y.checked_sub(1)) {
                (Some(new_x), Some(new_y)) => buffer.get_pixel(new_x, new_y),
                _ => &Rgb([0, 0, 0]),
            };

            let value = pixel[i].wrapping_add(nw[i]).wrapping_sub(n[i]).wrapping_sub(w[i]);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_nwnw2(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let n = match y.checked_sub(1) {
                Some(new_y) => buffer.get_pixel(x, new_y),
                None => &Rgb([0, 0, 0]),
            };

            let w = match x.checked_sub(1) {
                Some(new_x) => buffer.get_pixel(new_x, y),
                None => &Rgb([0, 0, 0]),
            };

            let nw = match (x.checked_sub(1), y.checked_sub(1)) {
                (Some(new_x), Some(new_y)) => buffer.get_pixel(new_x, new_y),
                _ => &Rgb([0, 0, 0]),
            };

            let value = ((w[i].wrapping_sub(nw[i])) / 2).wrapping_add(n[i]);

            let value = pixel[i].wrapping_add(value);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_wnnw2(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let n = match y.checked_sub(1) {
                Some(new_y) => buffer.get_pixel(x, new_y),
                None => &Rgb([0, 0, 0]),
            };

            let w = match x.checked_sub(1) {
                Some(new_x) => buffer.get_pixel(new_x, y),
                None => &Rgb([0, 0, 0]),
            };

            let nw = match (x.checked_sub(1), y.checked_sub(1)) {
                (Some(new_x), Some(new_y)) => buffer.get_pixel(new_x, new_y),
                _ => &Rgb([0, 0, 0]),
            };

            let value = ((n[i].wrapping_sub(nw[i])) / 2).wrapping_add(w[i]);

            let value = pixel[i].wrapping_add(value);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_nw2(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let n = match y.checked_sub(1) {
                Some(new_y) => buffer.get_pixel(x, new_y),
                None => &Rgb([0, 0, 0]),
            };

            let w = match x.checked_sub(1) {
                Some(new_x) => buffer.get_pixel(new_x, y),
                None => &Rgb([0, 0, 0]),
            };

            let value = (n[i].wrapping_add(w[i])) / 2;

            let value = pixel[i].wrapping_add(value);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn predictor_new_standard(buffer: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut r = Vec::new();
    let mut g = Vec::new();
    let mut b = Vec::new();

    for i in 0..3 {
        for (x, y, pixel) in buffer.enumerate_pixels() {
            let n = match y.checked_sub(1) {
                Some(new_y) => buffer.get_pixel(x, new_y),
                None => &Rgb([0, 0, 0]),
            };

            let w = match x.checked_sub(1) {
                Some(new_x) => buffer.get_pixel(new_x, y),
                None => &Rgb([0, 0, 0]),
            };

            let nw = match (x.checked_sub(1), y.checked_sub(1)) {
                (Some(new_x), Some(new_y)) => buffer.get_pixel(new_x, new_y),
                _ => &Rgb([0, 0, 0]),
            };

            let value = if nw[i] >= max(n[i], w[i]) {
                min(n[i], w[i])
            } else if nw[i] <= min(n[i], w[i]) {
                max(n[i], w[i])
            } else {
                w[i].wrapping_add(n[i]).wrapping_sub(nw[i])
            };

            let value = pixel[i].wrapping_add(value);

            match i {
                0 => r.push(value),
                1 => g.push(value),
                2 => b.push(value),
                _ => (),
            }
        }
    }

    (r, g, b)
}

fn main() {
    if let Ok(buffer) = parse(&env::args().collect()) {
        let r = buffer.pixels().map(|p| p[0]).collect::<Vec<u8>>();
        let g = buffer.pixels().map(|p| p[1]).collect::<Vec<u8>>();
        let b = buffer.pixels().map(|p| p[2]).collect::<Vec<u8>>();

        let entropy = entropy_for(r.iter().chain(g.iter()).chain(b.iter())).unwrap();
        let r_entropy = entropy_for(r.iter()).unwrap();
        let g_entropy = entropy_for(g.iter()).unwrap();
        let b_entropy = entropy_for(b.iter()).unwrap();

        println!("Original: {:?}", entropy);
        println!("Original red: {:?}", r_entropy);
        println!("Original green: {:?}", g_entropy);
        println!("Original blue: {:?}", b_entropy);

        println!("");

        let predictors: Vec<(Box<dyn Fn(&ImageBuffer<Rgb<u8>, Vec<u8>>) -> (Vec<u8>, Vec<u8>, Vec<u8>)>, &str)> = vec![
            (Box::new(predictor_n), "Predictor N"),
            (Box::new(predictor_w), "Predictor W"),
            (Box::new(predictor_nw), "Predictor NW"),
            (Box::new(predictor_n_plus_w_minus_nw), "Predictor N+W-NW"),
            (Box::new(predictor_nwnw2), "Predictor N+(W-NW)/2"),
            (Box::new(predictor_wnnw2), "Predictor W+(N-NW)/2"),
            (Box::new(predictor_nw2), "Predictor (N+W)/2"),
            (Box::new(predictor_new_standard), "Predictor New Standard"),
        ];

        let mut min = 0;
        let mut min_entropy = f64::MAX;

        let mut min_r = 0;
        let mut min_r_entropy = f64::MAX;

        let mut min_g = 0;
        let mut min_g_entropy = f64::MAX;

        let mut min_b = 0;
        let mut min_b_entropy = f64::MAX;

        for (i, (predictor, name)) in predictors.iter().enumerate() {
            let (r, g, b) = predictor(&buffer);
            
            let entropy = entropy_for(r.iter().chain(g.iter()).chain(b.iter())).unwrap();

            let r_entropy = entropy_for(r.iter()).unwrap();
            let g_entropy = entropy_for(g.iter()).unwrap();
            let b_entropy = entropy_for(b.iter()).unwrap();

            println!("{}: {:?}", name, entropy);
            println!("{} red: {:?}", name, r_entropy);
            println!("{} green: {:?}", name, g_entropy);
            println!("{} blue: {:?}", name, b_entropy);

            if entropy < min_entropy {
                min_entropy = entropy;
                min = i;
            }

            if r_entropy < min_r_entropy {
                min_r_entropy = r_entropy;
                min_r = i;
            }

            if g_entropy < min_g_entropy {
                min_g_entropy = g_entropy;
                min_g = i;
            }

            if b_entropy < min_b_entropy {
                min_b_entropy = b_entropy;
                min_b = i;
            }

            println!("");
        }

        println!("Min: {}", predictors[min].1);
        println!("Min red: {}", predictors[min_r].1);
        println!("Min green: {}", predictors[min_g].1);
        println!("Min blue: {}", predictors[min_b].1);
    } else {
        eprintln!("Problem with arguments");
    }
}

fn parse(input: &Vec<String>) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, ArgsError> {
    let mut args = Args::new("jpegls_tester", "for simple compresion and decompresion");
    args.flag("h", "help", "Print the usage menu");
    args.option("f",
        "file",
        "This command will test JPEG-LS compression on given file", 
        "FILE",
        Occur::Req,
        None);

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
    }

    let file = args.value_of::<String>("file")?;

    match image::open(file) {
        Ok(dynamic_image) => {
            // Convert the dynamic image to an RGBA image buffer
            let rgb_image = dynamic_image.to_rgb8();
            Ok(rgb_image)
        }
        Err(err) => {
            Err(ArgsError::new("", format!("Error opening file: {}", err).as_str()))
        }
    }
}
