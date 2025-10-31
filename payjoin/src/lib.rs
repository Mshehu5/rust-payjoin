#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Payjoin implementation in Rust
//!
//! Supercharge payment batching to save you fees and preserve your privacy.
//!
//! This library implements both [BIP 78 Payjoin V1](https://github.com/bitcoin/bips/blob/master/bip-0078.mediawiki) and [BIP 77 Payjoin V2](https://github.com/bitcoin/bips/blob/master/bip-0077.md).
//!
//! Only the latest BIP 77 Payjoin V2 is enabled by default. To use BIP 78 Payjoin V1, enable the `v1` feature.
//!
//! The library is perfectly IO-agnostic — in fact, it does no IO by default without the `io` feature.
//!
//! Types relevant to a Payjoin Directory as defined in BIP 77 are available in the [`directory`] module enabled by
//!  the `directory` feature.
//!
//! ## Examples and integrations
//!
//! - [`payjoin-cli`](https://github.com/payjoin/rust-payjoin/tree/master/payjoin-cli): reference
//!   implementation using Bitcoin Core. Supports both V1 and V2.
//! - [`nolooking`](https://github.com/chaincase-app/nolooking): LND integration. Note: currently a
//!   V1-focused example; it does not include V2 integration.
//! - [`bitmask-core`](https://github.com/diba-io/bitmask-core): BDK integration example.
//! - [`payjoin-ffi`](https://github.com/payjoin/rust-payjoin/tree/master/payjoin-ffi): language
//!   bindings for mobile/desktop (e.g., Python and Dart) that can be used to integrate Payjoin in
//!   non-Rust apps.
//!
//! ## Disclaimer ⚠️ WIP
//!
//! **Use at your own risk. This crate has not yet been reviewed by independent Rust and Bitcoin security professionals.**

#[cfg(not(any(feature = "directory", feature = "v1", feature = "v2")))]
compile_error!("At least one of the features ['directory', 'v1', 'v2'] must be enabled");

#[cfg(any(feature = "v2", feature = "directory"))]
pub(crate) mod bech32;
#[cfg(feature = "directory")]
#[cfg_attr(docsrs, doc(cfg(feature = "directory")))]
pub mod directory;

#[cfg(feature = "_core")]
pub(crate) mod core;
#[cfg(feature = "_core")]
pub use crate::core::*;
