pub struct RNG {
    state: u64,
}

impl RNG {
    pub fn new(seed: u64) -> Self {
        RNG { state: seed }
    }

    pub fn random_u32(&mut self) -> u32 {
        self.state  ^= self.state >> 12;
        self.state  ^= self.state << 25;
        self.state  ^= self.state >> 27;
        // help me write a return value
        ((self.state * 0x2545F4914F6CDD1D) >> 32) as u32
    }

    pub fn random(&mut self) -> f32 {
        // random float32 in [0,1)
        (self.random_u32() >> 8) as f32 / 16777216.0
    }

    pub fn uniform(&mut self, a: f32, b: f32)
    {
        // random float32 in [a,b)
        a + (b - a) * self.random()
    }
}