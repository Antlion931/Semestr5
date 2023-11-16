#[derive(Debug, PartialEq, Eq)]
pub enum Bit {
    Zero,
    One,
}

pub struct BitQueue {
    queue: Vec<u8>,
    first: u64,
    last: u64,
}

impl BitQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            first: 0,
            last: 0,
        }
    }

    pub fn push(&mut self, bit: Bit) {
        let mut coded_byte = 0;
        if self.last % 8 != 0 {
            coded_byte = self.queue.pop().expect(
                "There is always one becouse first time we push one, and whenever we pop, we push",
            );
        }

        match bit {
            Bit::Zero => {
                self.last += 1;
                self.queue.push(coded_byte);
            }
            Bit::One => {
                coded_byte |= 1 << (7 - (self.last % 8));
                self.last += 1;
                self.queue.push(coded_byte);
            }
        }
    }

    pub fn new_with_bytes(bytes: Vec<u8>) -> Self {
        let last = bytes.len() as u64 * 8;
        Self {
            queue: bytes,
            first: 0,
            last,
        }
    }

    pub fn pop(&mut self) -> Option<Bit> {
        if self.first >= self.last {
            return None;
        }

        let result = if self.queue[self.first as usize >> 3] & (1 << (7 - (self.first % 8))) == 0 {
            Bit::Zero
        } else {
            Bit::One
        };
        self.first += 1;
        Some(result)
    }

    pub fn get_queue(self) -> Vec<u8> {
        self.queue
    }

    pub fn can_pop(&self) -> bool {
        self.first < self.last
    }

    pub fn fill_rest_with_ones(&mut self) {
        while self.last % 8 != 0 {
            self.push(Bit::One);
        }
    }
}


