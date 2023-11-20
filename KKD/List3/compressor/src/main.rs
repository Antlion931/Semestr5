extern crate args;


use args::{Args,ArgsError};
use getopts::Occur;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::process;
use elias_coding;
use lzw;
use bit_queue::Bit;
use fibonaci_coding;

const DELTA_FLAG: [Bit; 2] = [Bit::Zero, Bit::Zero];
const OMEGA_FLAG: [Bit; 2] = [Bit::Zero, Bit::One];
const GAMMA_FLAG: [Bit; 2] = [Bit::One, Bit::Zero];
const FIBONACI_FLAG: [Bit; 2] = [Bit::One, Bit::One];

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

enum Task {
    EliasDelta(String),
    EliasGamma(String),
    EliasOmega(String),
    Fibonaci(String),
    Decompress(String),
}

fn main() {
    if let Ok((compress_suffix, decompress_suffix, tasks)) = parse(&env::args().collect()) {
        let compress_suffix = compress_suffix.as_str();
        let decompress_suffix = decompress_suffix.as_str();
        for t in tasks {
            let (file_to_read, file_to_write) = match &t {
                Task::Decompress(x) => (x, x.to_owned() + "." + decompress_suffix),
                Task::EliasDelta(x) => (x, x.to_owned() + ".d." + compress_suffix),
                Task::EliasGamma(x) => (x, x.to_owned() + ".g." + compress_suffix),
                Task::EliasOmega(x) => (x, x.to_owned() + ".o." + compress_suffix),
                Task::Fibonaci(x) => (x, x.to_owned() + ".f." + compress_suffix),
            };

            if let (Ok(input), Ok(mut output)) = (fs::read(file_to_read), fs::File::create(&file_to_write)) {
                let mut message = Vec::new();
                let input_len = input.len();

                match t {
                    Task::EliasDelta(_) => {
                        message = elias_coding::delta_encode(lzw::encode(&input), &DELTA_FLAG);
                    }
                    Task::EliasGamma(_) => {
                        message = elias_coding::gamma_encode(lzw::encode(&input), &GAMMA_FLAG);
                    }
                    Task::EliasOmega(_) => {
                        message = elias_coding::omega_encode(lzw::encode(&input), &OMEGA_FLAG);
                    }
                    Task::Fibonaci(_) => {
                        message = fibonaci_coding::encode(lzw::encode(&input), &FIBONACI_FLAG);
                    }
                    Task::Decompress(_) => {
                        let mut encoded_lzw = Vec::new();

                        if let Some(r) = elias_coding::delta_decode(&input, &DELTA_FLAG) {
                            encoded_lzw = r;
                        }
                        
                        if let Some(r) = elias_coding::omega_decode(&input, &OMEGA_FLAG) {
                            encoded_lzw = r;
                        }

                        if let Some(r) = elias_coding::gamma_decode(&input, &GAMMA_FLAG) {
                            encoded_lzw = r;
                        }

                        if let Some(r) = fibonaci_coding::decode(&input, &FIBONACI_FLAG) {
                            encoded_lzw = r;
                        }

                        message = lzw::decode(encoded_lzw);
                    }
                }

                output.write_all(&message).unwrap();
                println!("For {}", file_to_read);
                println!("Entropy before = {}", entropy_for(&input).unwrap());
                println!("Entropy after = {}", entropy_for(&message).unwrap());
                println!("avg lenght = {}", (message.len() * 8) as f64 / input_len as f64);
                println!("Compression = {}", input_len as f64 / message.len() as f64);
                println!("Difference = {}", message.len().abs_diff(input_len));

            } else {
                eprintln!("Problem with files {} or {}", file_to_read, file_to_write);
                continue;
            }
        }
    } else {
        eprintln!("Problem with arguments");
    }
}

fn entropy_for(message: &[u8]) -> Result<f64, String> {
    let mut counts = vec![0; u8::MAX as usize + 1];
    let mut count = 0;

    for byte in message {
        counts[*byte as usize] += 1;
        count += 1;
    }

    entropy_in(counts.as_slice(), count)
}

// (compress_suffix, decompress_suffix, tasks)
fn parse(input: &Vec<String>) -> Result<(String, String, Vec<Task>), ArgsError> {
    let mut args = Args::new("compressor", "for simple compresion and decompresion");
    args.flag("h", "help", "Print the usage menu");
    args.option("",
        "ds",
        "This command will change suffix for decompressed files",
        "NAME",
        Occur::Optional,
        Some(String::from("decompressed")));
    args.option("",
        "cs",
        "This command will change suffix for compressed files",
        "NAME",
        Occur::Optional,
        Some(String::from("compressed")));
    args.option("d",
        "elias_delta",
        "This command will compress file with elias delta and lzw codings, and save it with it name '<file_name>.d.<compress_suffix>'",
        "FILE",
        Occur::Multi,
        None);
    args.option("g",
        "elias_gamma",
        "This command will compress file with elias gamma and lzw codings, and save it with it name '<file_name>.g.<compress_suffix>'",
        "FILE",
        Occur::Multi,
        None);
    args.option("o",
        "elias_omega",
        "This command will compress file with elias omega and lzw codings, and save it with it name '<file_name>.o.<compress_suffix>'",
        "FILE",
        Occur::Multi,
        None);
    args.option("f",
        "fibonaci",
        "This command will compress file with fibonaci and lzw codings, and save it with it name '<file_name>.f.<compress_suffix>'",
        "FILE",
        Occur::Multi,
        None);
    args.option("",
        "decmprs",
        "This command will try to decompress file and save it with it name '<file_name>.<decompress_suffix>', will panic if could not",
        "FILE",
        Occur::Multi,
        None);

    args.parse(input)?;

    let help = args.value_of("help")?;
    if help {
        println!("{}", args.full_usage());
    }

    let compress_suffix = args.value_of::<String>("cs")?;
    let decompress_suffix = args.value_of::<String>("ds")?;

    let tasks: Vec<_> = args.values_of::<String>("elias_delta").unwrap_or(Vec::new()).into_iter().map(|x| Task::EliasDelta(x)).chain(
        args.values_of::<String>("elias_gamma").unwrap_or(Vec::new()).into_iter().map(|x| Task::EliasGamma(x))).chain(
        args.values_of::<String>("elias_omega").unwrap_or(Vec::new()).into_iter().map(|x| Task::EliasOmega(x))).chain(
        args.values_of::<String>("fibonaci").unwrap_or(Vec::new()).into_iter().map(|x| Task::Fibonaci(x))).chain(
        args.values_of::<String>("decmprs").unwrap_or(Vec::new()).into_iter().map(|x| Task::Decompress(x))).collect();

    Ok((compress_suffix, decompress_suffix, tasks))
}
