use crate::crypto::{Key, generate_key};
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use rand::RngExt;
use rand::rand_core::UnwrapErr;
use rand::rngs::SysRng;

struct Account {
    seed: Key,
    key: super::PrivateKey,
}

impl Account {
    fn new() -> Self {
        Self::generate()
    }

    fn generate() -> Self {
        Self::from_seed(generate_key())
    }

    fn from_seed(seed: Key) -> Self {
        let k = SigningKey::from_bytes(&seed);
        Self {
            seed,
            key: super::PrivateKey {
                key: SigningKey::from_bytes(&seed),
            },
        }
    }

    fn from_private_key() -> Self {
        todo!("implement me");
    }

    fn public_key(&self) -> super::PublicKey {
        todo!("implement me");
    }

    fn private_key(&self) -> super::PrivateKey {
        todo!("implement me");
    }

    fn address(&self) {
        todo!("implement me");
    }

    fn seed(&self) -> &Key {
        &self.seed
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::generate()
    }
}
