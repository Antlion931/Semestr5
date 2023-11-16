use model::{Model, Probability};
use std::env;
use std::fs;
use std::io::Write;
use std::process;

struct Encoded<'a> {
    message: &'a [u8],
    lenght: u64,
}

impl<'a> Encoded<'a> {
    fn new(message: &'a [u8]) -> Self {
        Self { message, lenght: 0 }
    }

    fn poped_zero(&mut self) -> Option<bool> {
        if self.lenght as usize >= self.message.len() * 8 {
            return None;
        }

        let result = self.message[self.lenght as usize >> 3] & (1 << (7 - (self.lenght % 8))) == 0;
        self.lenght += 1;
        Some(result)
    }
}

#[cfg(test)]
mod test {
    use super::Encoded;

    #[test]
    fn works() {
        let buffor = [0u8, u8::MAX];
        let mut encoded = Encoded::new(&buffor);

        for _ in 0..8 {
            assert!(encoded.poped_zero());
        }

        for _ in 0..8 {
            assert!(!encoded.poped_zero());
        }

    }

}

fn decode(message: impl AsRef<[u8]>) -> Result<Vec<u8>, String> {
    let mut encoded = Encoded::new(message.as_ref());
    let mut result = Vec::new();

    if encoded.poped_zero().unwrap() {
        for _ in 1..message.as_ref().len() {
            let mut value = 0;
            for _ in 0..8 {
                value <<= 1;
                if !encoded.poped_zero().unwrap() {
                    value += 1;
                }
            }
            result.push(value);
        }

        return Ok(result);
    }
    let mut model = Model::new();

    let mut low = 0;
    let mut high = model::MAX_CODE;
    let mut value = 0;

    for _ in 0..model::CODE_BITS {
        value <<= 1;
        if !encoded.poped_zero().unwrap() {
            value += 1;
        }
    }

    loop {
        let range = high - low + 1;
        let scaled_value = ((value - low + 1) * model.get_count() - 1) / range;
        let (char, p) = model.get_char(scaled_value);

        match char {
            model::Code::END => {
                break;
            }
            model::Code::Byte(b) => {
                result.push(b);
                model.update(b);
                high = low + (range * p.high) / p.count - 1;
                low = low + (range * p.low) / p.count;
                loop {
                    if high < model::ONE_HALF {
                    } else if low >= model::ONE_HALF {
                        value -= model::ONE_HALF;
                        low -= model::ONE_HALF;
                        high -= model::ONE_HALF;
                    } else if low >= model::ONE_FOURTH && high < model::THREE_FOURTHS {
                        value -= model::ONE_FOURTH;
                        low -= model::ONE_FOURTH;
                        high -= model::ONE_FOURTH;
                    } else {
                        break;
                    }
                    low <<= 1;
                    high <<= 1;
                    high += 1;
                    value <<= 1;
                    if encoded.poped_zero().filter(|x| !x).is_some() {
                        value += 1;
                    }
                }
            }
        }
    }

    Ok(result)
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        println!("Wrong amount of operands, usage: decoder <file_to_read> <file_to_write>");
        process::exit(1);
    }

    let file_to_read = args.get(1).expect("We know there are args is len 3");
    let file_to_write = args.get(2).expect("We know there are args is len 3");

    if let (Ok(input), Ok(mut output)) = (fs::read(file_to_read), fs::File::create(file_to_write)) {
        let result = decode(input).unwrap();
        output.write_all(&result).unwrap();
    } else {
        println!("Problem with files");
        process::exit(1);
    }
}
