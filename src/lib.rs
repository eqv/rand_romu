#![feature(test)]
extern crate test;

extern crate rand;
extern crate rand_core;
use rand::SeedableRng;
use rand_core::{impls, Error, RngCore};

pub struct RomuPrng {
    xstate: u64,
    ystate: u64,
}

impl RomuPrng {
    pub fn new(xstate: u64, ystate: u64) -> Self {
        return Self { xstate, ystate };
    }
}

impl SeedableRng for RomuPrng {
    type Seed = [u8; 16];

    fn from_seed(seed: [u8; 16]) -> RomuPrng {
        if seed == [0; 16] {
            return RomuPrng::new(0x0DDB1A5E5BAD5EEDu64, 0x519fb20ce6a199bbu64);
        }
        let x = u64::from_le_bytes([
            seed[0], seed[1], seed[2], seed[3], seed[4], seed[5], seed[6], seed[7],
        ]);
        let y = u64::from_le_bytes([
            seed[8], seed[9], seed[10], seed[11], seed[12], seed[13], seed[14], seed[15],
        ]);
        return RomuPrng::new(x, y);
    }
}

impl RngCore for RomuPrng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let xp = self.xstate;
        self.xstate = 15241094284759029579u64.wrapping_mul(self.ystate);
        self.ystate = self.ystate.wrapping_sub(xp);
        self.ystate = self.ystate.rotate_left(27);
        return xp;
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vector() {
        //test vectors obtained from test.c
        let x = 0x3c91b13ee3913664u64;
        let y = 0x863f0e37c2637d1fu64;
        let mut prng = RomuPrng::new(x, y);
        assert_eq!(0x3c91b13ee3913664, prng.next_u64());
        assert_eq!(0xdc1980b78df3115, prng.next_u64());
        assert_eq!(0x1c163b704996d2ad, prng.next_u64());
        assert_eq!(0xa000c594bb28313b, prng.next_u64());
        assert_eq!(0xfb6c42e69a523526, prng.next_u64());
        assert_eq!(0x1fcebd6988ab21d8, prng.next_u64());
        assert_eq!(0x5e0a8abf025f8f02, prng.next_u64());
        assert_eq!(0x29554b00ffab0263, prng.next_u64());
        assert_eq!(0xff5b6bb1551cf66, prng.next_u64());
    }

    use super::RomuPrng;
    use crate::rand::rngs::StdRng;
    use crate::rand::RngCore;
    use rand_pcg::Pcg64Mcg;
    use test::Bencher;

    #[bench]
    fn bench_romu(b: &mut Bencher) {
        b.iter(|| {
            //let mut prng = RomuPrng::new(12345, 45678);
            let mut prng = RomuPrng::seed_from_u64(12345);
            let n = test::black_box(10_000);
            test::black_box((0..n).fold(0, |old, _i| old ^ prng.next_u64()));
        });
    }
    #[bench]
    fn bench_std(b: &mut Bencher) {
        b.iter(|| {
            let mut prng = StdRng::seed_from_u64(12345);
            let n = test::black_box(10_000);
            test::black_box((0..n).fold(0, |old, _i| old ^ prng.next_u64()));
        });
    }
    #[bench]
    fn bench_pcg(b: &mut Bencher) {
        b.iter(|| {
            let mut prng = Pcg64Mcg::seed_from_u64(12345);
            let n = test::black_box(10_000);
            test::black_box((0..n).fold(0, |old, _i| old ^ prng.next_u64()));
        });
    }
}
