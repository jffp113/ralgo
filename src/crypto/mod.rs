use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use rand::RngExt;
use rand::rand_core::UnwrapErr;
use rand::rngs::SysRng;
use thiserror::Error;

pub mod account;


#[derive(Error, Debug)]
pub enum CryptoError {}

pub const KEY_SIZE: usize = 32;
pub type Key = [u8; KEY_SIZE];

pub fn generate_key() -> Key {
    let mut key: Key = Key::default();
    let mut rnd = UnwrapErr(SysRng::default());
    rnd.fill(&mut key);
    key
}

struct PrivateKey {
    key: SigningKey,
}

struct PublicKey {
    key: VerifyingKey,
}
