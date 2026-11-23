//! The small sum a launcher works out before sending, so that sending
//! thousands of messages costs a machine real time. There is no secret in the
//! launcher (it is open source): this is what stands in for one.
//!
//! The launcher asks for a challenge, then finds a number (`nonce`) such that
//! SHA-256 of `"{challenge}:{nonce}"` starts with `bits` zero bits. The same
//! function is in the launcher (hub-core, feedback.rs); both test the same
//! example below.

use sha2::{Digest, Sha256};

pub fn zero_bits(challenge: &str, nonce: u64) -> u32 {
    let hash = Sha256::digest(format!("{challenge}:{nonce}").as_bytes());
    let mut bits = 0;
    for byte in hash {
        if byte == 0 {
            bits += 8;
        } else {
            bits += byte.leading_zeros();
            break;
        }
    }
    bits
}

pub fn valid(challenge: &str, nonce: u64, bits: u32) -> bool {
    zero_bits(challenge, nonce) >= bits
}

#[cfg(test)]
mod tests {
    use super::*;

    // The launcher's test checks the same numbers
    #[test]
    fn the_shared_example() {
        let nonce = (0..).find(|n| valid("nico", *n, 12)).unwrap();
        assert_eq!(nonce, EXAMPLE_NONCE);
        assert!(!valid("nico", nonce, 30));
    }
    const EXAMPLE_NONCE: u64 = 5238;
}
