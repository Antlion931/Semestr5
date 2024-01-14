use elias_coding::encode;
use std::fs::File;
use std::env;
use std::io::Write;
use std::process;
use image_compression::*;
use bit_queue::Bit;

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

    for color in 0..3 {
        rgb_z_n[color].update_with_last();
        rgb_z_n[color].update_with_last();
        rgb_y_n[color].update_with_last();
        rgb_y_n[color].update_with_last();
    }

    // take only every other element
    let rgb_z_2n = rgb_z_n.into_iter().map(|x| x.get_elements().into_iter().enumerate().filter(|(n, _)| n % 2 != 0).map(|(_, v)| v).collect::<Vec<_>>()).collect::<Vec<_>>();
    let rgb_y_2n = rgb_y_n.into_iter().map(|x| x.get_elements().into_iter().enumerate().filter(|(n, _)| n % 2 != 0).map(|(_, v)| v).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut rgb_y_2n_after_diffrences = [Vec::new(), Vec::new(), Vec::new()];
    for color in 0..3 {
        rgb_y_2n_after_diffrences[color].push(rgb_y_2n[color][0]);
        for i in 1..rgb_y_2n[color].len() {
            rgb_y_2n_after_diffrences[color].push(rgb_y_2n[color][i] - rgb_y_2n[color][i - 1]);
        }
    }

    let rgb_y_2n = rgb_y_2n_after_diffrences;

   /* let mut universal_quantizer = vec![0.0f64];

    for _ in 0..2usize.pow(k as u32) - 1 {
        let mut new_universal_quantizer = Vec::new();

        new_universal_quantizer.push((-255.0 + universal_quantizer[0]) / 2.0);

        for i in 1..universal_quantizer.len() {
            new_universal_quantizer.push((universal_quantizer[i - 1] + universal_quantizer[i]) / 2.0);
        }

        new_universal_quantizer.push((255.0 + universal_quantizer[universal_quantizer.len() - 1]) / 2.0);

        universal_quantizer = new_universal_quantizer;
    }

    let rgb_z_quantized = vec![universal_quantizer.clone(), universal_quantizer.clone(), universal_quantizer.clone()];
    let rgb_y_quantized = vec![universal_quantizer.clone(), universal_quantizer.clone(), universal_quantizer.clone()];
*/
    let rgb_z_quantized = rgb_z_2n.iter().map(|x| quantize(x, k)).collect::<Vec<_>>();
    let rgb_y_quantized = rgb_y_2n.iter().map(|x| quantize(x, k)).collect::<Vec<_>>();

    let mut output = encode(vec![pixels.width(), pixels.height()], &[]);

    //store k
    for j in (0..3).rev() {
        if k & (1 << j) == 0 {
            output.push(Bit::Zero);
        } else {
            output.push(Bit::One);
        }
    }

    //store quantized values
    for color in 0..3 {
        for i in rgb_y_quantized[color].iter().chain(rgb_z_quantized[color].iter()) {
            for byte in i.to_be_bytes() {
                for j in (0..8).rev() {
                    if byte & (1 << j) == 0 {
                        output.push(Bit::Zero);
                    } else {
                        output.push(Bit::One);
                    }
                }
            }
        }
    }

    //store colors and filters as number to quantize
    for color in 0..3 {
        let mut diff = 0.0;

        for i in &rgb_y_2n[color] {
            let ii = i + diff;

            let (min_index, min) = rgb_y_quantized[color].iter().enumerate().min_by(|(_, a), (_, b)| (*a - ii).abs().partial_cmp(&(*b - ii).abs()).unwrap()).unwrap();

            diff += i - min;

            for j in (0..k).rev() {
                if min_index & (1 << j) == 0 {
                    output.push(Bit::Zero);
                } else {
                    output.push(Bit::One);
                }
            }
        }

        diff = 0.0;

        for i in &rgb_z_2n[color] {
            let ii = i + diff;

            let (min_index, min) = rgb_z_quantized[color].iter().enumerate().min_by(|(_, a), (_, b)| (*a - ii).abs().partial_cmp(&(*b - ii).abs()).unwrap()).unwrap();

            diff += i - min;

            for j in (0..k).rev() {
                if min_index & (1 << j) == 0 {
                    output.push(Bit::Zero);
                } else {
                    output.push(Bit::One);
                }
            }
        }
    }

    let mut output_file = File::create(args[2].as_str()).expect("Failed to create output file");

    output_file.write_all(&output.get_queue()).unwrap();
}
