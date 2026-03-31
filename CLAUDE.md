# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Run tests in a specific module
cargo test mnemonic::tests

# Check (faster than build, no linking)
cargo check
```

The `setup.sh` exports `TEST_PKCS11_MODULE` pointing to the SoftHSM2 library — source it if PKCS#11 tests are added.

## Architecture

This is a Rust implementation of Algorand-style cryptographic account management, modelled after [go-algorand-sdk](https://github.com/algorand/go-algorand-sdk/blob/main/crypto/account.go).

**`src/crypto/`** — primitives layer
- `mod.rs`: defines `Key = [u8; 32]`, `PrivateKey` (wraps `ed25519_dalek::SigningKey`), `PublicKey` (wraps `VerifyingKey`), and `generate_key()` using `SysRng`.
- `account.rs`: `Account` struct holding a seed (`Key`) and derived `PrivateKey`. Construction via `Account::new()` / `Account::from_seed(key)`. Several methods (`public_key`, `private_key`, `address`, `from_private_key`) are stubbed with `todo!`.

**`src/mnemonic/`** — mnemonic encoding layer
- Converts a 32-byte key ↔ 25-word mnemonic (24 data words + 1 checksum word).
- Each word encodes 11 bits; the 2-byte SHA-512/256 hash prefix provides the checksum word.
- `wordlist.rs`: 2048-word BIP-39 word list.
- `to_key()` (mnemonic → key) is not yet implemented.

**`src/core/`** — traits (`Addressable`, `Signer`, `Verifier`) — all method bodies are commented out / pending.

**Key data flow:** `generate_key()` → raw 32-byte seed → `Account::from_seed` creates `SigningKey` → `from_key()` encodes seed as mnemonic → (future) `to_key()` reverses it.
