#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    explicit_outlives_requirements,
    keyword_idents,
    impl_trait_redundant_captures,
    keyword_idents_2024,
    let_underscore_drop,
    macro_use_extern_crate,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    non_ascii_idents,
    // non_exhaustive_omitted_patterns, unstable
    noop_method_call,
    redundant_imports,
    redundant_lifetimes,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_numeric_casts,
    trivial_casts,
    unit_bindings,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_lifetimes,
    unused_qualifications,

    clippy::style,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,

    // clippy::restriction,
    clippy::create_dir,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::little_endian_bytes,
    clippy::missing_asserts_for_indexing,
    clippy::mixed_read_write_in_expression,
    clippy::panic,
    clippy::partial_pub_fields,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::tests_outside_test_module,
)]

#![allow(clippy::missing_errors_doc)] // REMOVE

use std::sync::Arc;

mod ecdsa;
pub use ecdsa::*;
mod ecdh;
pub use ecdh::*;
mod hkdf;
pub use hkdf::*;
pub mod memory_tracker;
pub mod format;
#[cfg(feature = "track")]
pub mod use_memory_tracker;

use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey};
use aws_lc_rs::rand;
use std::backtrace::Backtrace;

/// Digest Algorithm. Default is Sha512
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum DigestAlg {
    Sha256,
    Sha384,
    #[default]
    Sha512,
}

/// Error type for this crate
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Error {
    /// Text associated with the error
    pub message: String,
    /// Backtrace at time of creation
    pub backtrace: Arc<Backtrace>,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}
impl Eq for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Crypto Error {}", self.message)
    }
}

impl std::error::Error for Error {}

#[track_caller]
fn serr(s: String) -> Error {
    Error {
        message: s,
        backtrace: Arc::new(Backtrace::capture()),
    }
}

#[track_caller]
fn err<T>(s: String) -> Result<T, Error> {
    Err(serr(s))
}

/// Fill `bytes` with random data.
pub fn generate_random_bytes(bytes: &mut [u8]) -> Result<(), Error> {
    rand::fill(bytes)
        .map_err(|_| serr(format!("Failed to generate {} random bytes.", bytes.len())))?;
    Ok(())
}

/// AES GCM Algorithm. Default is `Aes256Gcm`
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum AesGcm {
    Aes128Gcm,
    Aes192Gcm,
    #[default]
    Aes256Gcm,
}

impl AesGcm {
    #[must_use]
    /// Return key length associated with the algorithm
    pub const fn key_len(&self) -> u8 {
        match self {
            Self::Aes128Gcm => 16,
            Self::Aes192Gcm => 24,
            Self::Aes256Gcm => 32,
        }
    }
}

const fn get_aes_alg(alg: AesGcm) -> &'static aws_lc_rs::aead::Algorithm {
    match alg {
        AesGcm::Aes128Gcm => &aws_lc_rs::aead::AES_128_GCM,
        AesGcm::Aes192Gcm => &aws_lc_rs::aead::AES_192_GCM,
        AesGcm::Aes256Gcm => &aws_lc_rs::aead::AES_256_GCM,
    }
}

/// Encrypt message with AES GCM
pub fn aes_encrypt(
    alg: AesGcm,
    iv: &[u8],
    key: &[u8],
    message: &[u8],
    aad: &[u8],
    cipher_text: &mut Vec<u8>,
) -> Result<(), Error> {
    let alg = get_aes_alg(alg);
    let old_size = cipher_text.len();
    cipher_text.extend_from_slice(message);
    let cipher_slice = &mut cipher_text[old_size..];
    let key = UnboundKey::new(alg, key).map_err(|e| serr(format!("new {e:?}")))?;
    let nonce = Nonce::try_assume_unique_for_key(iv).map_err(|e| serr(format!("new {e:?}")))?;
    let key = LessSafeKey::new(key);
    let aad = Aad::from(aad);
    let tag = key
        .seal_in_place_separate_tag(nonce, aad, cipher_slice)
        .map_err(|e| serr(format!("Seal {e:?}")))?;
    cipher_text.extend_from_slice(tag.as_ref());
    Ok(())
}

/// Decrypt message with AES GCM
pub fn aes_decrypt(
    alg: AesGcm,
    key: &[u8],
    cipher_text: &[u8],
    auth_tag: &[u8],
    iv: &[u8],
    aad: &[u8],
    plain_text: &mut [u8],
) -> Result<(), Error> {
    if plain_text.len() < cipher_text.len() {
        return err(format!(
            "aes_decrypt : Cipher text length of {} is less than plain text length of {}.",
            plain_text.len(),
            cipher_text.len()
        ));
    }
    let alg = get_aes_alg(alg);
    let key = UnboundKey::new(alg, key).map_err(|e| serr(format!("new {e:?}")))?;
    let nonce = Nonce::try_assume_unique_for_key(iv).map_err(|e| serr(format!("new {e:?}")))?;
    let key = LessSafeKey::new(key);
    let aad = Aad::from(aad);
    key.open_separate_gather(nonce, aad, cipher_text, auth_tag, plain_text)
        .map_err(|e| serr(format!("gather {e:?}")))?;
    Ok(())
}
