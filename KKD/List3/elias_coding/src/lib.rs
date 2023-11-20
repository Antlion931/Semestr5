use bit_queue::{Bit, BitQueue};

pub fn delta_encode(message: impl AsRef<[u16]>, flag: &[Bit]) -> Vec<u8> {
    let mut result = BitQueue::new();

    for b in flag {
        result.push(*b);
    }

    for m in message.as_ref() {
        let m = *m as u32 + 1; // encoded number plus one to avoid zero
        let n = u32::BITS - m.leading_zeros(); // number of bits in m
        let k = u32::BITS - n.leading_zeros(); // number of bits in n

        // add k - 1 zeros to result
        for _ in 0..(k - 1) {
            result.push(Bit::Zero);
        }

        // take binary represantation of n and skip until first one
        let mut binary_represent = BitQueue::new_with_owned_bytes(n.to_be_bytes().to_vec());
        while let Some(Bit::Zero) = binary_represent.pop() {} // skips first one

        // add skiped one to result and rest of bits
        result.push(Bit::One);

        while let Some(x) = binary_represent.pop() {
            result.push(x);
        }

        // take binary representation of m and skipp until first one
        let mut binary_represent = BitQueue::new_with_owned_bytes(m.to_be_bytes().to_vec());
        while let Some(Bit::Zero) = binary_represent.pop() {} // skips first one

        // don't add skiped to result, but add rest of bits
        while let Some(x) = binary_represent.pop() {
            result.push(x);
        }
    }

    result.get_queue()
}

pub fn delta_decode(message: &Vec<u8>, flag: &[Bit]) -> Option<Vec<u16>> {
    let mut message = BitQueue::new_with_bytes(message);

    for b in flag {
        if message.pop().filter(|x| x == b).is_none() {
            return None;
        }
    }

    let mut result = Vec::new();

    'outer: while message.can_pop() {
        let mut k = 1; // number of bits to read

        loop {
            let x = message.pop();
            if Some(Bit::One) == x {
                // skips first one
                break;
            } else if x.is_none() {
                break 'outer;
            }
            k += 1;
        }

        // skiped first one, so add this to n, and k next bits
        let mut n: u32 = 1 << (k - 1);

        for x in (0..k - 1).rev() {
            // we don't skip any one here
            match message.pop() {
                Some(Bit::Zero) => {}
                Some(Bit::One) => n += 1 << x,
                None => panic!("Wrong code, not enough bits"),
            }
        }

        // this time there was no one skiped, but we will note code MSB, so we need to add
        let mut number: u32 = 1 << (n - 1);
        number -= 1; // we code numbers plus one to avoid zero

        for x in (0..n - 1).rev() {
            match message.pop() {
                Some(Bit::Zero) => {}
                Some(Bit::One) => number += 1 << x,
                None => panic!("Wrong code, not enough bits"),
            }
        }

        result.push(number as u16);
    }

    Some(result)
}

pub fn gamma_encode(message: impl AsRef<[u16]>, flag: &[Bit]) -> Vec<u8> {
    let mut result = BitQueue::new();

    for b in flag {
        result.push(*b);
    }

    for m in message.as_ref() {
        let m = *m as u32 + 1; // encoded number plus one to avoid zero
        let n = u32::BITS - m.leading_zeros(); // number of bits in m

        // add n-1 zeroes to result
        for _ in 0..(n - 1) {
            result.push(Bit::Zero);
        }

        // take binary represantation of m, and skip until first one
        let mut binary_represent = BitQueue::new_with_owned_bytes(m.to_be_bytes().to_vec());
        while let Some(Bit::Zero) = binary_represent.pop() {} // skips first one

        // add skipped one and rest of bits to result
        result.push(Bit::One);

        while let Some(x) = binary_represent.pop() {
            result.push(x);
        }
    }

    result.get_queue()
}

pub fn gamma_decode(message: &Vec<u8>, flag: &[Bit]) -> Option<Vec<u16>> {
    let mut message = BitQueue::new_with_bytes(message);

    for b in flag {
        if message.pop().filter(|x| x == b).is_none() {
            return None;
        }
    }

    let mut result = Vec::new();

    'outer: while message.can_pop() {
        let mut n = 1; // number of bits to read

        loop {
            let x = message.pop();
            if Some(Bit::One) == x {
                // skips first one
                break;
            } else if x.is_none() {
                break 'outer;
            }
            n += 1;
        }

        // we have skiped first one, so we need to add this
        let mut number: u32 = 1 << (n - 1);
        number -= 1; // we encoded number plus one to avoid zero

        for x in (0..n - 1).rev() {
            match message.pop() {
                Some(Bit::Zero) => {}
                Some(Bit::One) => number += 1 << x,
                None => panic!("Wrong code, not enough bits"),
            }
        }

        result.push(number as u16);
    }

    Some(result)
}

pub fn omega_encode(message: impl AsRef<[u16]>, flag: &[Bit]) -> Vec<u8> {
    let mut result = BitQueue::new();

    for b in flag {
        result.push(*b);
    }

    // reusable vector just to not west memory on continuous allocations
    let mut workhouse_vec_a = Vec::new(); // stores encoded number in reverse
    let mut workhouse_vec_b = Vec::new(); // used to reverse k

    for m in message.as_ref() {
        workhouse_vec_a.push(Bit::Zero); // last in correct encoded should be zero
        let mut k = *m as u32 + 1;
        while k > 1 {
            // take binary represantation of k, and skip until first one
            let mut binary_represent = BitQueue::new_with_owned_bytes(k.to_be_bytes().to_vec());
            while let Some(Bit::Zero) = binary_represent.pop() {} // skips first one

            // reverse number using b and add it to a
            while let Some(x) = binary_represent.pop() {
                workhouse_vec_b.push(x);
            }

            while let Some(x) = workhouse_vec_b.pop() {
                workhouse_vec_a.push(x);
            }

            // add skipped one and rest of bits to result
            workhouse_vec_a.push(Bit::One);

            k = u32::BITS - k.leading_zeros() - 1;
        }

        while let Some(x) = workhouse_vec_a.pop() {
            result.push(x);
        }
    }
    result.fill_rest_with_ones(); // special case, in omega singla zero represents number zero

    result.get_queue()
}

pub fn omega_decode(message: &Vec<u8>, flag: &[Bit]) -> Option<Vec<u16>> {
    let mut message = BitQueue::new_with_bytes(message);

    for b in flag {
        if message.pop().filter(|x| x == b).is_none() {
            return None;
        }
    }

    let mut result = Vec::new();

    'outer: while message.can_pop() {
        let mut n = 1;
        loop {
            let x = message.pop();
            if Some(Bit::One) == x {
                //skip of first one

                // we have skiped first one, so we need to add this
                let mut number: u32 = 1 << n;

                for x in (0..n).rev() {
                    // read next n bits, n + 1 in total
                    match message.pop() {
                        Some(Bit::Zero) => {}
                        Some(Bit::One) => number += 1 << x,
                        None => break 'outer, // end of encoded, ones in the end
                    }
                }
                n = number;
            } else if x.is_none() {
                break 'outer; // end of encoded, ones in the end
            } else {
                break;
            }
        }
        result.push((n - 1) as u16) // number was encoded plus 1 to avoid zero
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamma_encode_simple_test() {
        assert_eq!(gamma_encode(vec![137], &[]), vec![1, 20]);
    }

    #[test]
    fn gamma_flag_simple_test() {
        let x = gamma_encode(vec![142, 54], &[Bit::One, Bit::Zero]);
        assert_eq!(gamma_decode(&x, &[Bit::One, Bit::Zero]), Some(vec![142, 54]));
    }

    #[test]
    fn gamma_decode_simple_test() {
        assert_eq!(gamma_decode(&vec![1, 20], &[]), Some(vec![137]));
    }

    #[test]
    fn gamma_max_value() {
        let x = gamma_encode(vec![u16::MAX], &[]);
        assert_eq!(gamma_decode(&x, &[]), Some(vec![u16::MAX]));
    }

    #[test]
    fn delta_encode_simple_test() {
        assert_eq!(delta_encode(vec![137], &[]), vec![16, 40]);
    }

    #[test]
    fn delta_flag_simple_test() {
        let x = delta_encode(vec![142, 54], &[Bit::One, Bit::Zero]);
        assert_eq!(delta_decode(&x, &[Bit::One, Bit::Zero]), Some(vec![142, 54]));
    }

    #[test]
    fn delta_decode_simple_test() {
        assert_eq!(delta_decode(&vec![16, 40], &[]), Some(vec![137]));
    }

    #[test]
    fn delta_max_value() {
        let x = gamma_encode(vec![u16::MAX], &[]);
        assert_eq!(gamma_decode(&x, &[]), Some(vec![u16::MAX]));
    }

    #[test]
    fn omega_encode_simple_test() {
        assert_eq!(omega_encode(vec![137], &[]), vec![188, 83]);
    }

    #[test]
    fn omega_flag_simple_test() {
        let x = omega_encode(vec![142, 54], &[Bit::One, Bit::Zero]);
        assert_eq!(omega_decode(&x, &[Bit::One, Bit::Zero]), Some(vec![142, 54]));
    }

    #[test]
    fn omega_decode_simple_test() {
        assert_eq!(omega_decode(&vec![188, 83], &[]), Some(vec![137]));
    }

    #[test]
    fn omega_max_value() {
        let x = omega_encode(vec![u16::MAX], &[]);
        assert_eq!(omega_decode(&x, &[]), Some(vec![u16::MAX]));
    }
}
