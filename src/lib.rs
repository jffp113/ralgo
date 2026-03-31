mod core;
mod crypto;
pub mod mnemonic;
mod client;

use thiserror::Error;
use crate::crypto::CryptoError;
use crate::mnemonic::MnemonicError;
use crate::client::indexer::IndexerError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mnemonic error: {0}")]
    Mnemonic(#[from] MnemonicError),

    #[error("cryto error: {0}")]
    Crypto(#[from] CryptoError),

    #[error("indexer error: {0}")]
    Indexer(#[from] IndexerError)
}