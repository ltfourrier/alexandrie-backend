use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// Generate a cryptographically secure salt, composed of 16 random bytes.
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    ChaCha20Rng::from_entropy().fill_bytes(&mut salt);
    salt
}
