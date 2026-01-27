//! This is an awesome crate
//!
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
    clippy::panic,
    clippy::partial_pub_fields,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::tests_outside_test_module,

    // clippy::indexing_slicing,
    clippy::wildcard_enum_match_arm,
    // clippy::allow_attributes_without_reason,
    clippy::unneeded_field_pattern,

)]
#![allow(clippy::multiple_crate_versions)] // nothing to be done
#![allow(clippy::option_if_let_else)] // disagree
#![allow(clippy::redundant_pub_crate)] // broken, conflicts with unreachable_pub
#![allow(clippy::wildcard_imports)] // REMOVE
#![allow(clippy::missing_errors_doc)] // REMOVE
#![allow(clippy::too_many_lines)] // disagree
#![allow(unused_crate_dependencies)] // broken

#[cfg(feature = "kms")]
#[cfg_attr(docsrs, doc(cfg(feature = "kms")))]
pub mod client_supplier;
pub mod cmm;
pub use cmm::CryptographicMaterialsManagerRef;
pub mod agreement;
pub mod cmc;
pub mod commitment;
pub mod error;
pub mod keyring;
pub use keyring::KeyringRef;
#[cfg(feature = "ddb")]
#[cfg_attr(docsrs, doc(cfg(feature = "ddb")))]
pub mod keystore;
#[cfg(feature = "kms")]
#[cfg_attr(docsrs, doc(cfg(feature = "kms")))]
pub mod kms_keyring;
pub mod materials;
pub mod suites;
// pub mod types;

pub(crate) trait MplPrivate {}

use crate::suites::AlgorithmSuite;
use std::fmt::Debug;
use std::fmt::Formatter;
use zeroize::Zeroize;

#[derive(PartialEq, Eq, Clone, Default, Hash)]
/// Secrets, e.g. Plain text keys
#[allow(clippy::exhaustive_structs)]
pub struct Secret(pub Vec<u8>);

impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Secret of size {}", self.0.len())
    }
}

impl Secret {
    #[must_use]
    pub const fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

//= aws-encryption-sdk-specification/framework/structures.md#structure-1
//= type=implication
//# The encryption context is a key-value mapping of arbitrary, non-secret, UTF-8 encoded strings.
//# It is used during [encryption](../client-apis/encrypt.md) and [decryption](../client-apis/decrypt.md) to provide additional authenticated data (AAD).
pub type EncryptionContext = std::collections::HashMap<String, String>;

//= aws-encryption-sdk-specification/framework/structures.md#overview
//= type=implication
//# While these structures will usually be represented as objects, lower level languages MAY represent
//# these fields in a less strictly defined way as long as all field properties are still upheld.

// Encryption Materials

// In the future, we have several improvements we can consider here:
//   1. Model these materials structures as resources, in order to move towards "smarter"
//      materials. This would allow us to tightly define the valid interactions with
//      materials and prevent dangerous or unexpected uses of them.
//   2. Use different materials structures for keyrings and CMMs. These live at
//      different layers of the library and have different needs and responsibilities,
//      so we may eventually want to give them each materials specialized to their
//      purpose.
// Note that both of these will be breaking changes to any customers building
// custom implementations of keyrings or CMMs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct EncryptionMaterials {
    //= aws-encryption-sdk-specification/framework/structures.md#structure-2
    //= type=implication
    //# This structure MUST include the following fields:
    //#
    //# - [Algorithm Suite](#algorithm-suite)
    //# - [Encrypted Data Keys](#encrypted-data-keys)
    //# - [Encryption Context](#encryption-context-1)
    //# - [Required Encryption Context Keys](#required-encryption-context-keys)
    pub algorithm_suite: AlgorithmSuite,
    pub encryption_context: EncryptionContext,
    pub encrypted_data_keys: Vec<EncryptedDataKey>,
    pub required_encryption_context_keys: Vec<String>,

    //= aws-encryption-sdk-specification/framework/structures.md#structure-2
    //= type=implication
    //# This structure MAY include any of the following fields:
    //#
    //# - [Plaintext Data Key](#plaintext-data-key)
    //# - [Signing Key](#signing-key)
    pub plaintext_data_key: Option<Secret>,
    pub signing_key: Option<Secret>,
    //= aws-encryption-sdk-specification/framework/structures.md#symmetric-signing-keys
    //= type=implication
    //# The value of keys in this list MUST be kept secret.
    pub symmetric_signing_keys: Vec<Secret>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct DecryptionMaterials {
    //= aws-encryption-sdk-specification/framework/structures.md#fields
    //= type=implication
    //# This structure MUST include the following fields:
    //#
    //# - [Algorithm Suite](#algorithm-suite-1)
    //# - [Encryption Context](#encryption-context-2)
    //# - [Required Encryption Context Keys](#required-encryption-context-keys-1)
    pub algorithm_suite: AlgorithmSuite,
    pub encryption_context: EncryptionContext,
    pub required_encryption_context_keys: Vec<String>,

    //= aws-encryption-sdk-specification/framework/structures.md#fields
    //= type=implication
    //# This structure MAY include any of the following fields:
    //#
    //# - [Plaintext Data Key](#plaintext-data-key-1)
    //# - [Verification Key](#verification-key)
    pub plaintext_data_key: Option<Secret>,

    pub verification_key: Option<Secret>,

    //= aws-encryption-sdk-specification/framework/structures.md#symmetric-signing-key
    //= type=implication
    //# This value MUST be kept secret.
    pub symmetric_signing_key: Option<Secret>,
}

//= aws-encryption-sdk-specification/framework/structures.md#structure
//= type=implication
//# An encrypted data key is comprised of the following fields:
//#
//# - [Key Provider ID](#key-provider-id)
//# - [Key Provider Information](#key-provider-information)
//# - [Ciphertext](#ciphertext)
//#
//# Note: "Encrypted" is a misnomer here, as the process by which a key provider may obtain the plaintext data key
//# from the ciphertext and vice versa does not have to be an encryption and decryption cipher.
//# This specification uses the terms "encrypt" and "decrypt" for simplicity,
//# but the actual process by which a key provider obtains the plaintext data key from the ciphertext
//# and vice versa MAY be any reversible operation, though we expect that most will use encryption.
#[derive(Debug, PartialEq, Eq, Clone, Default, Hash)]
#[non_exhaustive]
pub struct EncryptedDataKey {
    // The spec defines keyProviderId in 2 places,
    // and, while they are not identical,
    // they do not disagree.
    // data-format/message-header.md#encrypted-data-key-entries ::
    // UTF-8 encoded bytes
    // framework/keyring-interface.md#key-provider-id ::
    // The key provider ID MUST be a binary value and SHOULD be equal to a UTF-8 encoding of the key namespace.
    pub key_provider_id: String,

    // The key provider info MUST be a binary value and SHOULD be equal to a UTF-8 encoding of the key name.
    pub key_provider_info: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

impl EncryptedDataKey {
    pub fn new(
        key_provider_id: impl Into<String>,
        key_provider_info: impl Into<Vec<u8>>,
        ciphertext: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            key_provider_id: key_provider_id.into(),
            key_provider_info: key_provider_info.into(),
            ciphertext: ciphertext.into(),
        }
    }
}
