use std::env;
use std::fs;
use std::io::Write;
use std::process;

struct Encoded {
    message: Vec<u8>,
    entropy: f64,
    avg_lenght: f64,
}

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

fn encode(message: impl AsRef<[u8]>) -> Result<Encoded, String> {
    let mut bytes_counts = vec![0u64; u8::MAX as usize + 1];
    let message = message.as_ref();
    let mut result = Vec::with_capacity(message.len());
    
    for (n, byte) in message.iter().enumerate() {
        bytes_counts[*byte as usize] += 1;

    }

    Ok(Encoded { message: result, entropy: entropy_in(&bytes_counts, message.len() as u64)?, avg_lenght: 0.0 })
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Wrong amount of operands, usage: coder <file_to_read> <file_to_write>");
        process::exit(1);
    }

    let file_to_read = args.get(1).expect("We know there are args is len 3");
    let file_to_write = args.get(2).expect("We know there are args is len 3");

    if let (Ok(input), Ok(mut output)) = (fs::read(file_to_read), fs::File::create(file_to_write)) {
        let encoded = encode(&input).unwrap();
        output.write_all(&encoded.message).unwrap();
        println!("Entropy = {}", encoded.entropy);
        println!("avg lenght = {}", encoded.avg_lenght);
        println!("Compression = {}", input.len() as f64 / encoded.message.len() as f64);
    } else {
        println!("Problem with files");
        process::exit(1);
    }

}
