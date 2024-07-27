
// run rng
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

    pub fn uniform(&mut self, a: f32, b: f32) -> f32
    {
        // random float32 in [a,b)
        a + (b - a) * self.random()
    }
}


// now how to test the above
// cargo test -- --nocapture

#[cfg(test)]
mod tests {
    use super::*; // Import the RNG struct and its methods

    #[test]
    fn test_random_u32() {
        let mut rng = RNG::new(42); // Seed with a specific value
        let value = rng.random_u32();
        assert!(value <= u32::MAX, "Value should be less than or equal to u32::MAX");
        assert!(value >= 0, "Value should be greater than or equal to 0");
    }

    #[test]
    fn test_random() {
        let mut rng = RNG::new(42); // Seed with a specific value
        let value = rng.random();
        assert!(value < 1.0, "Value should be less than 1.0");
        assert!(value >= 0.0, "Value should be greater than or equal to 0.0");
    }

    #[test]
    fn test_uniform() {
        let mut rng = RNG::new(42); // Seed with a specific value
        let value = rng.uniform(1.0, 5.0);
        assert!(value < 5.0, "Value should be less than 5.0");
        assert!(value >= 1.0, "Value should be greater than or equal to 1.0");
    }

    #[test]
    fn test_multiple_random_values() {
        let mut rng = RNG::new(42); // Seed
        let values: Vec<u32> = (0..10).map(|_| rng.random_u32()).collect();
        for &value in &values {
            assert!(value <= u32::MAX, "Value should be less than or equal to u32::MAX");
            assert!(value >= 0, "Value should be greater than or equal to 0");
        }
    }
}