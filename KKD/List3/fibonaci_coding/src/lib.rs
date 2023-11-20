use bit_queue::{Bit, BitQueue};

// as we encode u16 we don't need more
const FIRST_24_FIB_NUMBERS_WITHOUT_FIRST: [u32; 23] = [
    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368,
];

pub fn encode(message: impl AsRef<[u16]>, flag: &[Bit]) -> Vec<u8> {
    let mut result = BitQueue::new();

    for b in flag {
        result.push(*b);
    }

    // reusable vector just to not west memory on continuous allocations
    let mut workhouse_vec = Vec::new(); // stores encoded number in reverse

    for m in message.as_ref() {
        workhouse_vec.push(Bit::One); // ends with double one
        let mut k = *m as u32 + 1; // we encode number plus one to avoid zero
        let mut first_one = false;

        for fib in FIRST_24_FIB_NUMBERS_WITHOUT_FIRST.iter().rev() {
            if *fib <= k {
                first_one = true;
                k -= fib;
                workhouse_vec.push(Bit::One);
            } else if first_one {
                // do not add leading zeros
                workhouse_vec.push(Bit::Zero);
            }
        }

        // reverse add add to result
        while let Some(x) = workhouse_vec.pop() {
            result.push(x);
        }
    }

    result.get_queue()
}

pub fn decode(message: &Vec<u8>, flag: &[Bit]) -> Option<Vec<u16>> {
    let mut message = BitQueue::new_with_bytes(message);

    for b in flag {
        if message.pop().filter(|x| x == b).is_none() {
            return None;
        }
    }

    let mut result = Vec::new();

    'outer: while message.can_pop() {
        let mut number = 0u32;
        let mut last_was_one = false;
        let mut index = 0;

        loop {
            let x = message.pop();

            if last_was_one && x == Some(Bit::One) {
                // double one, ready for new number
                break;
            }

            match x {
                Some(Bit::Zero) => last_was_one = false,
                Some(Bit::One) => {
                    last_was_one = true;
                    number += FIRST_24_FIB_NUMBERS_WITHOUT_FIRST[index];
                }
                None => break 'outer, // ends with zeros
            }

            index += 1;
        }

        result.push((number - 1) as u16); // number coded plus one to avoid zero
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_simple_test() {
        assert_eq!(encode(vec![9], &[]), vec![76]);
    }

    #[test]
    fn decode_simple_test() {
        assert_eq!(decode(&vec![76], &[]), Some(vec![9]));
    }

    #[test]
    fn max_value() {
        let x = encode(vec![u16::MAX], &[]);
        assert_eq!(decode(&x, &[]), Some(vec![u16::MAX]));
    }

    #[test]
    fn flag_simple_test() {
        let x = encode(vec![142, 54], &[Bit::One, Bit::Zero]);
        assert_eq!(decode(&x, &[Bit::One, Bit::Zero]), Some(vec![142, 54]));
    }

    #[test]
    fn fuzz_failed_test() {
        let x = encode(vec![16705, 2570], &[]);
        assert_eq!(decode(&x, &[]), Some(vec![16705, 2570]));
    }
}
