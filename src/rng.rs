use rand::{distributions::Standard, prelude::Distribution, rngs::StdRng, Rng, SeedableRng};

pub struct Random {
    // TODO: We don't need cryptographic security
    rng: StdRng,
}
impl Random {
    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }
}
impl Default for Random {
    fn default() -> Random {
        Random {
            rng: StdRng::seed_from_u64(0x0EA4_F7EE_CAFE_F00D),
        }
    }
}
