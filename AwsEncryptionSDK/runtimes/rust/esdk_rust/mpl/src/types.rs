use crate::suites::AlgorithmSuite;
use std::fmt::Debug;
use std::fmt::Formatter;
use zeroize::Zeroize;

#[derive(Clone, Default, PartialEq, Eq)]
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

pub type KmsKeyId = String;
pub type Region = String;
pub type AccountId = String;
pub type EncryptionContextKey = String;

//= aws-encryption-sdk-specification/framework/structures.md#symmetric-signing-keys
//= type=implication
//# The value of keys in this list MUST be kept secret.
pub type SymmetricSigningKey = Secret;

//= aws-encryption-sdk-specification/framework/structures.md#structure-1
//= type=implication
//# The encryption context is a key-value mapping of arbitrary, non-secret, UTF-8 encoded strings.
//# It is used during [encryption](../client-apis/encrypt.md) and [decryption](../client-apis/decrypt.md) to provide additional authenticated data (AAD).
pub type EncryptionContext = std::collections::HashMap<EncryptionContextKey, String>;

// Values come from: https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/framework/raw-rsa-keyring.md#supported-padding-schemes
#[derive(Debug, PartialEq, Copy, Clone, Default, Eq)]
#[non_exhaustive]
pub enum PaddingScheme {
    Pkcs1,
    OaepSha1Mgf1,
    OaepSha256Mgf1,
    OaepSha384Mgf1,
    #[default]
    OaepSha512Mgf1,
}

#[derive(Debug, PartialEq, Copy, Clone, Default, Eq)]
#[non_exhaustive]
pub enum AesWrappingAlg {
    AlgAes128GcmIv12Tag16,
    AlgAes192GcmIv12Tag16,
    #[default]
    AlgAes256GcmIv12Tag16,
}

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
    pub required_encryption_context_keys: Vec<EncryptionContextKey>,

    //= aws-encryption-sdk-specification/framework/structures.md#structure-2
    //= type=implication
    //# This structure MAY include any of the following fields:
    //#
    //# - [Plaintext Data Key](#plaintext-data-key)
    //# - [Signing Key](#signing-key)
    pub plaintext_data_key: Option<Secret>,
    pub signing_key: Option<Secret>,
    pub symmetric_signing_keys: Vec<SymmetricSigningKey>,
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
    pub required_encryption_context_keys: Vec<EncryptionContextKey>,

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
#[derive(Clone, PartialEq, Eq, Debug)]
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
