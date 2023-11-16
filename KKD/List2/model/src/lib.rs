pub const CODE_BITS: u32 = 65;
pub const COUNT_BITS: u32 = 63;
pub const MAX_COUNT: u128 = (1 << COUNT_BITS) - 1;
pub const MAX_CODE: u128 = (1 << CODE_BITS) - 1;
pub const ONE_FOURTH: u128 = 1 << (CODE_BITS - 2);
pub const ONE_HALF: u128 = 1 << (CODE_BITS - 1);
pub const THREE_FOURTHS: u128 = ONE_HALF + ONE_FOURTH;
pub const BLOCK_SIZE: usize = 256;

pub struct Model {
    cumulative_counts: Vec<u128>,
    frozen: bool,
}

pub enum Code {
    Byte(u8),
    END
}

pub struct Probability {
    pub low: u128,
    pub high: u128,
    pub count: u128,
}

impl Model {
    pub fn new() -> Self {
        Self { cumulative_counts: (0..258).collect(), frozen: false }
    }

    pub fn get_char(&self, value: u128) -> (Code, Probability) {
        for i in 0..256 {
          if value < self.cumulative_counts[i+1] {
            return (Code::Byte(i as u8), self.get_probability(i as u8));
          }
        }
        (Code::END, self.get_probability_of_end())
    }

    pub fn get_probability(&self, byte: u8) -> Probability {
        Probability {low: self.cumulative_counts[byte as usize], high: self.cumulative_counts[byte as usize + 1], count: self.cumulative_counts[257]}
    }

    pub fn get_probability_of_end(&self) -> Probability {
        Probability { low: self.cumulative_counts[256], high: self.cumulative_counts[257], count: self.cumulative_counts[257]}
    }

    pub fn update(&mut self, byte: u8) {
        if self.frozen {
            return;
        }

        for i in byte as usize + 1..258 {
            self.cumulative_counts[i] += 1;
        }

        if self.cumulative_counts[257] >= MAX_COUNT {
            println!("Frozen");
            self.frozen = true;
        }
    }

    pub fn get_count(&self) -> u128 {
        self.cumulative_counts[257]
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_consts() {
        assert!(MAX_CODE > ONE_HALF);
        assert!(ONE_HALF > ONE_FOURTH);
        assert!(THREE_FOURTHS > ONE_HALF);
    }
}
