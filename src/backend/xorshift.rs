//! XorShift random number generator implementation.
//!
//! XorShift generators are a class of pseudorandom number generators that use
//! XOR and shift operations to produce sequences of numbers with good statistical properties.
//!
//! # Characteristics
//!
//! - State size: 8 bytes
//! - Period: 2<sup>64</sup>−1
//! - Speed: Very Fast
//! - Quality: Good
//!
//! # Example
//!
//! ```rust
//! use alea::{Rng, backend::XorShift};
//!
//! let backend = XorShift::new(987654321);
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [George Marsaglia (2003), "Xorshift RNGs"](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)
//! - [Wikipedia: Xorshift](https://en.wikipedia.org/wiki/Xorshift)

use super::RandomBackend;

/// XorShift random number generator struct.
pub struct XorShift {
    state: u64,
}

impl XorShift {
    /// Creates a new `XorShift` instance with the given seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    ///
    /// # Panics
    ///
    /// Panics if the seed is zero, as zero is an invalid state for the XorShift algorithm.
    pub fn new(seed: u64) -> Self {
        assert!(seed != 0, "Seed value cannot be zero.");
        Self { state: seed }
    }
}

impl RandomBackend for XorShift {
    /// Generates the next random `u64` using the XorShift algorithm.
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
