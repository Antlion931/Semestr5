use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::process;
use model::{Model, Probability};

struct Encoded {
    message: Vec<u8>,
    lenght: u64,
}

impl Encoded {
    fn new() -> Self {
        Self { message: Vec::new(), lenght: 0}
    }

    fn push(&mut self, zero: bool) {
        let mut coded_byte = 0;
        if self.lenght % 8 != 0 {
            coded_byte = self.message.pop().expect("There is always one becouse first time we push one, and whenever we pop, we push");
        }

        if zero {
            self.lenght += 1;
            self.message.push(coded_byte);
        } else {
            coded_byte |= 1 << (7 - (self.lenght % 8));
            self.lenght += 1;
            self.message.push(coded_byte);
        }
    }

    fn get_message(&self) -> &[u8] {
        &self.message
    }
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

fn encode(message: impl AsRef<[u8]>) -> Result<(Encoded, f64), String> {
    let mut bytes_counts = vec![0u64; u8::MAX as usize + 1];
    let message = message.as_ref();
    let mut result = Encoded::new();
    result.push(false); // arithmetic encoding

    let mut model = Model::new();

    let mut pending = 0;
    let mut low = 0;
    let mut high = model::MAX_CODE;

    for byte in message {
       bytes_counts[*byte as usize] += 1;
    }

    let entropy = entropy_in(&bytes_counts, message.len() as u64)?;

    let push_with_pending = |zero: bool, pending: &mut u64, result: &mut Encoded| {
      result.push(zero);
      for _ in 0..*pending {
          result.push(!zero);
      }
      *pending = 0;
    };

    let work_with_probabilities = |p: Probability, low: &mut u128, high: &mut u128, pending: &mut u64, result: &mut Encoded| {
      let range = *high - *low + 1;
      *high = *low + (range * p.high / p.count) - 1;
      *low = *low + (range * p.low / p.count);

      loop {
          if *high < model::ONE_HALF {
              push_with_pending(true, pending, result);
          } else if *low >= model::ONE_HALF {
              push_with_pending(false, pending, result);
          } else if *low >= model::ONE_FOURTH && *high < model::THREE_FOURTHS {
              *pending += 1;
              *low -= model::ONE_FOURTH;
              *high -= model::ONE_FOURTH;
          } else {
              break;
          }
          *high <<= 1;
          *high += 1;
          *low <<= 1;
          *high &= model::MAX_CODE;
          *low &= model::MAX_CODE;
      }
    };
    
    for byte in message {
        work_with_probabilities(model.get_probability(*byte), &mut low, &mut high, &mut pending, &mut result);
        model.update(*byte);
    }

    work_with_probabilities(model.get_probability_of_end(), &mut low, &mut high, &mut pending, &mut result);

    pending += 1;
    if  low < model::ONE_FOURTH {
        push_with_pending(true, &mut pending, &mut result);
    } else {
        push_with_pending(false, &mut pending, &mut result);
    }

    if result.get_message().len() >=  message.len() + 8 {
        result = Encoded::new();
        result.push(true); // normal encoding
        
        for byte in message {
            for i in (0..8).rev() {
                result.push(byte & (1 << i) == 0);
            }
        }
    }

    Ok((result, entropy))
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Wrong amount of operands, usage: encoder <file_to_read> <file_to_write>");
        process::exit(1);
    }

    let file_to_read = args.get(1).expect("We know there are args is len 3");
    let file_to_write = args.get(2).expect("We know there are args is len 3");

    if let (Ok(input), Ok(mut output)) = (fs::read(file_to_read), fs::File::create(file_to_write)) {
        let (encoded, entropy) = encode(&input).unwrap();
        output.write_all(encoded.get_message()).unwrap();
        println!("Entropy = {}", entropy);
        println!("avg lenght = {}", (encoded.get_message().len() * 8) as f64 / input.len() as f64);
        println!("Compression = {}", input.len() as f64 / encoded.get_message().len() as f64);
        println!("Difference = {}", encoded.get_message().len().abs_diff(input.len()));
    } else {
        println!("Problem with files");
        process::exit(1);
    }

}
