pub struct XorShift128 {
    state: [u64; 2],
}

impl XorShift128 {
    pub fn new(seed1: u64, seed2: u64) -> Self {
        XorShift128 {
            state: [seed1, seed2],
        }
    }

    pub fn random_in_range(min: u64, max: u64) -> u64 {
        // 乱数生成
        let seed1 = 12345;
        let seed2 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut rng = XorShift128::new(seed1, seed2);

        rng.next() % (max - min + 1) + min
    }

    fn next(&mut self) -> u64 {
        let s1 = self.state[0];
        let mut s0 = self.state[1];
        self.state[0] = s0;
        s0 ^= s0 << 23;
        self.state[1] = s0 ^ s1 ^ (s0 >> 17) ^ (s1 >> 26);
        self.state[1].wrapping_add(s1)
    }
}