mod wordlist;

use crate::crypto::{KEY_SIZE, Key};
use crate::mnemonic::wordlist::{WORD_LIST, WordList};
use sha2::{Digest, Sha512_256};
use crate::mnemonic::MnemonicError::*;

#[derive(thiserror::Error, Debug)]
pub enum MnemonicError {
    #[error("mnemonic contains invalid word list")]
    InvalidWords,

    #[error("wrong word count: expected={expected} actual:{0}", expected=MNEMONIC_LEN_WORDS)]
    WrongWordCount(usize),

    #[error("wrong checksum")]
    WrongChecksum,
}

type Result<T> = std::result::Result<T, MnemonicError>;

const MNEMONIC_LEN_WORDS: usize = 25;

// from_key converts a 32-byte key into a 25 word mnemonic. The generated
// mnemonic includes a checksum. Each word in the mnemonic represents 11 bits
// of data, and the last 11 bits are reserved for the checksum.
pub fn from_key(key: &Key) -> String {
    let checksum_word = checksum(key);
    let uint11_array = to_uint11_array(key);
    let words = apply_words(&uint11_array, WORD_LIST);
    format!("{} {}", words.join(" "), checksum_word)
}

// ToKey converts a mnemonic generated using this library into the source
// key used to create it. It returns an error if the passed mnemonic has an
// incorrect checksum, if the number of words is unexpected, or if one
// of the passed words is not found in the words list.
pub fn to_key(mnemonic: &str) -> Result<Key> {
    let words: Vec<&str> = mnemonic.split_whitespace().collect();

    if words.len() != MNEMONIC_LEN_WORDS {
        return Err(WrongWordCount(words.len()))
    }

    let mut u11_vec = words
        .iter()
        .copied()
        .map(|word| index_of(word, WORD_LIST))
        .collect::<Option<Vec<u32>>>()
        .ok_or(InvalidWords)?;

    // remove checksum word
    u11_vec.pop();

    let mut byte_vec = to_byte_array(&u11_vec);

    assert_eq!(byte_vec.len(), KEY_SIZE + 1, "bit-packing invariant violated");

    // Check that the last one is actually 0 and chop it
    let last_byte = byte_vec
        .pop()
        .expect("safe: size was checked");

    assert_eq!(last_byte, 0, "padding bits invariant violated");

    // Pull out the checksum
    let checksum = checksum(&byte_vec);

    let last_word = words.last().copied().expect("safe: size was checked");

    if last_word != checksum {
        return Err(WrongChecksum)
    }

    let key = Key::try_from(byte_vec).
        expect("safe: length asserted above");

    Ok(key)
}

fn index_of(needle: &str, word_list: WordList) -> Option<u32> {
    word_list.
        iter().
        copied().
        position(|word| word ==needle).
        // safe: word_list as 2^11 words
        map(|position| position as u32)
}

fn apply_words(array: &[u32], words: WordList) -> Vec<&'static str> {
    array
        .iter()
        .map(|&index| words[index as usize])
        .collect()
}

fn checksum(key: &[u8]) -> &'static str {
    let hash = Sha512_256::digest(key);
    let (hash_part, _) = hash.split_at(2);
    let hash_part = to_uint11_array(hash_part);
    apply_words(&hash_part, WORD_LIST)
        .first()
        .expect("2 bytes will yield 2 words")
}

const U11_MASK: u32 = 0x7ff;
const U11_BIT_SIZE: u8 = 11;

// This function may result in an extra empty byte
// https://stackoverflow.com/a/51452614
fn to_uint11_array(bytes: &[u8]) -> Vec<u32> {
    let mut buffer: u32 = 0;
    let mut num_bits: u8 = 0;

    // we are allocating more than we need (8*len) / 11
    let mut output = Vec::with_capacity(bytes.len());

    for &byte in bytes {
        buffer |= u32::from(byte) << num_bits;
        num_bits += 8;

        if num_bits >= U11_BIT_SIZE {
            output.push(buffer & U11_MASK);
            buffer >>= U11_BIT_SIZE;
            num_bits -= U11_BIT_SIZE;
        }
    }

    if num_bits != 0 {
        output.push(buffer & U11_MASK)
    }

    output
}

// This function may result in an extra empty byte
// https://stackoverflow.com/a/51452614
fn to_byte_array(u11_vec: &[u32]) -> Vec<u8> {
    let mut buffer: u32 = 0;
    let mut bit_num: u8 = 0;

    // over allocating
    let mut result: Vec<u8> = Vec::with_capacity(u11_vec.len());

    // 11 - 8 + 11
    for &u11 in u11_vec {
        buffer |= u11 << bit_num;
        bit_num += 11;
        while bit_num >= 8 {
            result.push(buffer as u8);
            buffer >>= 8;
            bit_num -= 8;
        }
    }

    if buffer != 0 {
        result.push(buffer as u8)
    }

    result
}

/*
TODO to migrate to rust

// FromPrivateKey is a helper that converts an ed25519 private key to a
// human-readable mnemonic
func FromPrivateKey(sk ed25519.PrivateKey) (string, error) {
	seed := sk.Seed()
	return FromKey(seed)
}

// ToPrivateKey is a helper that converts a mnemonic directly to an ed25519
// private key
func ToPrivateKey(mnemonic string) (sk ed25519.PrivateKey, err error) {
	seedBytes, err := ToKey(mnemonic)
	if err != nil {
		return
	}
	return ed25519.NewKeyFromSeed(seedBytes), nil
}

// FromMasterDerivationKey is a helper that converts an MDK to a human-readable
// mnemonic
func FromMasterDerivationKey(mdk types.MasterDerivationKey) (string, error) {
	return FromKey(mdk[:])
}

// ToMasterDerivationKey is a helper that converts a mnemonic directly to a
// master derivation key
func ToMasterDerivationKey(mnemonic string) (mdk types.MasterDerivationKey, err error) {
	mdkBytes, err := ToKey(mnemonic)
	if err != nil {
		return
	}
	if len(mdkBytes) != len(mdk) {
		panic("recovered mdk is wrong length")
	}
	copy(mdk[:], mdkBytes)
	return
}
 */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto;

    #[test]
    fn from_to_key() {
        let key = crypto::generate_key();
        let mnemonic = from_key(&key);
        let recovered = to_key(&mnemonic).unwrap();
        assert_eq!(key, recovered)
    }

    #[test]
    fn convert_u11_u8() {
        let key = crypto::generate_key();
        let u11_array = to_uint11_array(&key);
        let mut u8_array = to_byte_array(&u11_array);
        u8_array.pop();

        assert_eq!(key.to_vec(), u8_array);
    }
}
