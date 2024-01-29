pub struct Counter {
    index: u64,
}

impl Counter {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self) -> u64 {
        self.index += 1;
        self.index - 1
    }
}
