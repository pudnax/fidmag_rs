pub struct BinarySeq {
    num: usize,
    counter: usize,
}

impl BinarySeq {
    pub fn new(num: usize) -> BinarySeq {
        BinarySeq { num, counter: 0 }
    }
}

impl Iterator for BinarySeq {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.counter < (1 << (self.num + 1)) {
            Some(
                (0..self.num)
                    .enumerate()
                    .map(|(i, _)| self.counter / (1 << self.num - i) % 2)
                    .collect(),
            )
        } else {
            self.counter = 0;
            None
        };
        self.counter += 1;
        res
    }
}
