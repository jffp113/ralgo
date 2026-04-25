# ralgo

A Rust implementation of the [Algorand Go SDK](https://github.com/algorand/go-algorand-sdk), focused on cryptographic account management and node/indexer client access.

## Feature parity checklist

### Types (`types/`)

- [ ] `Address` — 32-byte public key with base32+checksum display and parsing
- [ ] `Signature` — ed25519 signature (`[u8; 64]`)
- [ ] `MasterDerivationKey` — `[u8; 32]`
- [ ] `MicroAlgos` — `u64` newtype with `to_algos()` / `from_algos()` conversion
- [ ] `Digest` — SHA-512/256 hash type
- [ ] `VotePK`, `VRFPK` — participation key types
- [ ] `MultisigSig` / `MultisigSubsig` — multisig signature structures
- [ ] `LogicSig` — smart contract signature (program + args + sig/multisig)
- [ ] `StateSchema` — TEAL key/value store schema (uint, byte counts)
- [ ] `AssetParams`, `AssetHolding`
- [ ] `AppParams`, `AppLocalState`
- [ ] `Block`, `BlockHeader`
- [ ] `Genesis`
- [ ] `StateProof`, `LightBlockHeader`

### Crypto & accounts (`crypto/`)

- [x] Key generation (`generate_key`)
- [x] `Account::new()` / `Account::from_seed()`
- [ ] `Account::from_private_key(sk)` — derive account from an ed25519 private key
- [ ] `Account::public_key()`
- [ ] `Account::private_key()`
- [ ] `Account::address()` — base32-encoded Algorand address (public key + 4-byte checksum)
- [ ] `MultisigAccount` — (version, threshold, public keys) with address derivation
- [ ] `LogicSigAccount` — delegated and escrow modes
- [ ] `sign_transaction(sk, tx)` — single account signing
- [ ] `sign_multisig_transaction(sk, multisig_account, tx)`
- [ ] `sign_logic_sig_transaction(logic_sig_account, tx)`
- [ ] `sign_bid(sk, bid)`
- [ ] `verify_bytes(pk, message, signature)` — ed25519 verification
- [ ] `verify_multisig(address, message, multisig_sig)`
- [ ] `verify_logic_sig(logic_sig_account, single_signer)`
- [ ] `get_txid(tx)` — compute transaction ID
- [ ] `compute_group_id(txns)` — compute atomic group ID
- [ ] `address_from_program(program)` — escrow address from TEAL bytecode
- [ ] `get_application_address(app_id)` — application escrow address
- [ ] `random_bytes(size)`

### Mnemonic (`mnemonic/`)

- [x] `from_key(key)` — 32-byte seed → 25-word mnemonic with checksum
- [x] `to_key(mnemonic)` — 25-word mnemonic → 32-byte seed
- [ ] `from_private_key(sk)` — ed25519 private key → mnemonic
- [ ] `to_private_key(mnemonic)` — mnemonic → ed25519 private key
- [ ] `from_master_derivation_key(mdk)` — MDK → mnemonic
- [ ] `to_master_derivation_key(mnemonic)` — mnemonic → MDK

### Core traits (`core/`)

- [ ] `Addressable::address() -> Address`
- [ ] `Signer::sign(&self, content: &[u8]) -> Signature`
- [ ] `Verifier::verify(...) -> bool`

### Transaction building (`transaction/`)

**Payment:**
- [ ] `make_payment_txn(from, to, amount, note, close_remainder_to, params)`

**Asset (ASA):**
- [ ] `make_asset_create_txn()`
- [ ] `make_asset_transfer_txn()`
- [ ] `make_asset_acceptance_txn()`
- [ ] `make_asset_freeze_txn()`
- [ ] `make_asset_revocation_txn()`
- [ ] `make_asset_config_txn()`
- [ ] `make_asset_destroy_txn()`

**Key registration:**
- [ ] `make_key_reg_txn(account, note, params, vote_key, selection_key, ...)`

**Application calls:**
- [ ] `make_application_create_tx()` / `make_application_create_tx_with_boxes()`
- [ ] `make_application_call_tx()` / `make_application_call_tx_with_boxes()`
- [ ] `make_application_opt_in_tx()`
- [ ] `make_application_no_op_tx()`
- [ ] `make_application_update_tx()`
- [ ] `make_application_delete_tx()`
- [ ] `make_application_close_out_tx()`
- [ ] `make_application_clear_state_tx()`

**Helpers:**
- [ ] `assign_group_id(txns)` — assign atomic group IDs
- [ ] `estimate_size(tx)` — estimate encoded transaction size
- [ ] `create_dryrun(client, txns, request)` — build dryrun request

### Transaction encoding (`encoding/`)

- [ ] msgpack encode/decode for `Transaction`, `SignedTxn`
- [ ] JSON encode/decode (strict and lenient variants)
- [ ] `SignedTxn::from_base64(b64)` / `Block::from_base64(b64)`

### Atomic Transaction Composer (`atc/`)

- [ ] `AtomicTransactionComposer` struct with state machine (`Building → Built → Signed → Submitted → Committed`)
- [ ] `add_transaction(txn_and_signer)`
- [ ] `add_method_call(params)` — ABI method call
- [ ] `build_group()`
- [ ] `submit(client)` — returns TXIDs
- [ ] `execute(client, wait_rounds)` — submit and wait for confirmation
- [ ] `simulate(client, request)`
- [ ] `MAX_ATOMIC_GROUP_SIZE = 16`

### ABI (`abi/`)

- [ ] `Type` — parse and represent ABI types (uint, byte, bool, array, tuple, etc.)
- [ ] `type_of(str)` — parse type string
- [ ] `make_tuple_type(types)`
- [ ] `is_transaction_type(str)`, `is_reference_type(str)` — type classification
- [ ] `Method` — name, args, returns, `get_signature()`, `get_selector()`, `get_tx_count()`
- [ ] `method_from_signature(str)` — parse method signature
- [ ] `get_method_by_name(methods, name)`
- [ ] `Arg` — name, type, description; `is_transaction_arg()`, `is_reference_arg()`
- [ ] `Contract` — name, methods, network info (`app_id` per network)
- [ ] `Interface` — ABI interface definition
- [ ] Transaction type constants: `AnyTransactionType`, `PaymentTransactionType`, `KeyRegistrationTransactionType`, `AssetConfigTransactionType`, `AssetTransferTransactionType`, `AssetFreezeTransactionType`, `ApplicationCallTransactionType`
- [ ] Reference type constants: `AccountReferenceType`, `AssetReferenceType`, `ApplicationReferenceType`

### Algod client (`client/algod`)

**Factory:**
- [ ] `make_client(address, token)`
- [ ] `make_client_with_headers(address, token, headers)`
- [ ] `make_client_with_transport(address, token, headers, transport)`

**Account:**
- [ ] `account_information(address)`
- [ ] `account_application_information(address, app_id)`
- [ ] `account_asset_information(address, asset_id)`

**Blocks:**
- [ ] `block(round)`
- [ ] `block_raw(round)`
- [ ] `get_block_hash(round)`
- [ ] `get_block_txids(round)`
- [ ] `get_block_logs(round)`
- [ ] `wait_for_block(round)`
- [ ] `get_block_timestamp_offset()` / `set_block_timestamp_offset(offset)` — dev mode

**Applications & assets:**
- [ ] `get_application_by_id(app_id)`
- [ ] `get_application_box_by_name(app_id, name)`
- [ ] `get_application_boxes(app_id)`
- [ ] `get_asset_by_id(asset_id)`

**Transactions:**
- [ ] `send_raw_transaction(raw_txn)`
- [ ] `pending_transactions()`
- [ ] `pending_transactions_by_address(address)`
- [ ] `pending_transaction_information(txid)`
- [ ] `get_transaction_proof(round, txid)`

**Node status:**
- [ ] `status()`
- [ ] `status_after_block(round)`
- [ ] `health_check()`
- [ ] `get_ready()`
- [ ] `get_version()`

**Ledger state:**
- [ ] `get_ledger_state_delta(round)`
- [ ] `get_ledger_state_delta_for_transaction_group(id)`
- [ ] `get_transaction_group_ledger_state_deltas_for_round(round)`
- [ ] `get_state_proof(round)`
- [ ] `get_sync_round()` / `set_sync_round()` / `unset_sync_round()`
- [ ] `supply()`
- [ ] `get_genesis()`
- [ ] `get_light_block_header_proof(round)`

**TEAL:**
- [ ] `teal_compile(source)`
- [ ] `teal_disassemble(bytecode)`
- [ ] `teal_dryrun(request)`

**Simulation:**
- [ ] `simulate_transaction(request)`
- [ ] `suggested_params()`

### Indexer client (`client/indexer`)

- [x] Code generation from OpenAPI spec (`opa/indexer.oas.3.json`)
- [ ] `IndexerClientBuilder` — finish `base_url`, `api_token`, custom headers
- [ ] `IndexerClient::new(...)` — finish constructor

**Account queries:**
- [ ] `lookup_account_by_id(address)`
- [ ] `lookup_account_assets(address)`
- [ ] `lookup_account_app_local_states(address)`
- [ ] `lookup_account_created_applications(address)`
- [ ] `lookup_account_created_assets(address)`
- [ ] `lookup_account_transactions(address)`

**Asset queries:**
- [ ] `lookup_asset_by_id(asset_id)`
- [ ] `lookup_asset_balances(asset_id)`
- [ ] `lookup_asset_transactions(asset_id)`

**Application queries:**
- [ ] `lookup_application_by_id(app_id)`
- [ ] `lookup_application_logs_by_id(app_id)`
- [ ] `lookup_application_box_by_id_and_name(app_id, name)`

**Block & transaction queries:**
- [ ] `lookup_block(round)`
- [ ] `lookup_transaction(txid)`

**Search:**
- [ ] `search_accounts(params)`
- [ ] `search_for_assets(params)`
- [ ] `search_for_applications(params)`
- [ ] `search_for_application_boxes(app_id)`
- [ ] `search_for_transactions(params)`
- [ ] `search_for_block_headers(params)`

**Health:**
- [ ] `health_check()`

### KMD client (`client/kmd`)

**Factory:**
- [ ] `make_client(address, token)`

**Wallet management:**
- [ ] `list_wallets()`
- [ ] `create_wallet(name, password, driver, mdk)`
- [ ] `rename_wallet(id, password, new_name)`
- [ ] `init_wallet_handle(id, password)`
- [ ] `release_wallet_handle(handle)`
- [ ] `renew_wallet_handle(handle)`
- [ ] `get_wallet(handle)`

**Key management:**
- [ ] `generate_key(handle)`
- [ ] `import_key(handle, secret_key)`
- [ ] `list_keys(handle)`
- [ ] `export_key(handle, password, address)`
- [ ] `delete_key(handle, password, address)`
- [ ] `export_master_derivation_key(handle, password)`

**Multisig:**
- [ ] `import_multisig(handle, version, threshold, public_keys)`
- [ ] `list_multisig(handle)`
- [ ] `export_multisig(handle, password, address)`
- [ ] `delete_multisig(handle, password, address)`

**Transaction signing:**
- [ ] `sign_transaction(handle, password, tx)`
- [ ] `sign_transaction_with_specific_public_key(handle, password, tx, public_key)`
- [ ] `multisig_sign_transaction(handle, password, tx, public_key, partial)`
- [ ] `version()`

### Logic / TEAL source maps (`logic/`)

- [ ] `SourceMap` — decode and query TEAL source maps
- [ ] `get_line_for_pc(pc)` — source line from program counter
- [ ] `get_pcs_for_line(line)` — all PCs from source line

### Auction (`auction/`)

- [ ] `make_bid(bidder_address, bid_amount, max_price, bid_id, auction_address, auction_id)`
- [ ] `Bid` type

### Utilities

- [ ] `wait_for_confirmation(client, txid, wait_rounds)` — poll until confirmed or rounds elapsed
- [ ] `OAdd`, `OSub`, `OMul`, `OAdd16` — overflow-safe arithmetic
- [ ] Constants: `MIN_TXN_FEE = 1000`, `MAX_ATOMIC_GROUP_SIZE = 16`, `NUM_ADDITIONAL_BYTES_AFTER_SIGNING = 75`
