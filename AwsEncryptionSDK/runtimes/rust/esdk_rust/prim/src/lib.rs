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
    // missing_docs,
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
    // clippy::panic, because mpl enums are stupid
    clippy::partial_pub_fields,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::tests_outside_test_module,
)]
#![allow(clippy::cargo_common_metadata)] // REMOVE
#![allow(clippy::multiple_crate_versions)] // nothing to be done
#![allow(clippy::option_if_let_else)] // disagree
#![allow(clippy::cast_possible_truncation)] // REMOVE
#![allow(clippy::missing_panics_doc)] // REMOVE
#![allow(clippy::cast_sign_loss)] // REMOVE
#![allow(clippy::redundant_pub_crate)] // broken, conflicts with unreachable_pub
#![allow(clippy::wildcard_imports)] // REMOVE
#![allow(clippy::missing_errors_doc)] // REMOVE
#![allow(clippy::too_many_lines)] // disagree
#![allow(unused_crate_dependencies)] // broken

mod ecdsa;
pub use ecdsa::*;
mod hkdf;
pub use hkdf::*;

use aws_lc_rs::aead::{Aad, LessSafeKey, Nonce, UnboundKey};
use aws_lc_rs::rand;
use std::backtrace::Backtrace;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum DigestAlg {
    Sha256,
    Sha384,
    #[default]
    Sha512,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Error {
    pub msg: String,
    pub backtrace: Backtrace,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Crypto Error {}", self.msg)
    }
}

impl std::error::Error for Error {}

#[track_caller]
fn serr(s: String) -> Error {
    Error {
        msg: s,
        backtrace: Backtrace::capture(),
    }
}

#[track_caller]
fn err<T>(s: String) -> Result<T, Error> {
    Err(serr(s))
}

pub fn generate_random_bytes(bytes: &mut [u8]) -> Result<(), Error> {
    rand::fill(bytes)
        .map_err(|_| serr(format!("Failed to generate {} random bytes.", bytes.len())))?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub struct DoAESEncryptOutput {
    pub cipher_text: Vec<u8>,
    pub auth_tag: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
#[non_exhaustive]
pub enum AesGcm {
    Aes128Gcm,
    Aes192Gcm,
    #[default]
    Aes256Gcm,
}

impl AesGcm {
    #[must_use]
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

pub fn aes_encrypt(
    alg: AesGcm,
    iv: &[u8],
    key: &[u8],
    msg: &[u8],
    aad: &[u8],
    cipher_text: &mut Vec<u8>,
) -> Result<(), Error> {
    let alg = get_aes_alg(alg);
    let old_size = cipher_text.len();
    cipher_text.extend_from_slice(msg);
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
