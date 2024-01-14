use bit_queue::{Bit, BitQueue};

const FIRST_24_FIB_NUMBERS_WITHOUT_FIRST: [u32; 23] = [
    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368,
];

pub fn encode(message: impl AsRef<[u32]>, flag: &[Bit]) -> BitQueue {
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

    result
}

pub fn decode<'a>(message: &'a Vec<u8>, flag: &'a [Bit], mut amount: usize) -> Option<(Vec<u32>, BitQueue<'a>)> {
    let mut message = BitQueue::new_with_bytes(message);

    for b in flag {
        if message.pop().filter(|x| x == b).is_none() {
            return None;
        }
    }

    let mut result = Vec::new();

    'outer: while message.can_pop() && amount > 0 {
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
        amount -= 1;
        result.push(number - 1); // number coded plus one to avoid zero
    }

    Some((result, message))
}
