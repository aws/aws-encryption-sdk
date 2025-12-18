#![allow(unused)]

use crate::error::*;

// Algorithm Suites

// For now, the actual properties of algorithm suites are only used by internal
// components and are not actually customer facing. If and when we make them
// customer facing, we will need to either model the AlgorithmSuiteProperties
// as a separate structure (with an associated resource/operation for translating
// from name to properties) or use more advanced custom traits which allow us to
// model all properties of the algorithm suite in one structure.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum EsdkAlgorithmSuiteId {
    AlgAes128GcmIv12Tag16NoKdf = 0x0014,
    AlgAes192GcmIv12Tag16NoKdf = 0x0046,
    AlgAes256GcmIv12Tag16NoKdf = 0x0078,
    AlgAes128GcmIv12Tag16HkdfSha256 = 0x0114,
    AlgAes192GcmIv12Tag16HkdfSha256 = 0x0146,
    AlgAes256GcmIv12Tag16HkdfSha256 = 0x0178,
    AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 = 0x0214,
    AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 = 0x0346,
    AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 = 0x0378,
    AlgAes256GcmHkdfSha512CommitKey = 0x0478,
    #[default]
    AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 = 0x0578,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum DbeAlgorithmSuiteId {
    AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 = 0x6700,
    #[default]
    AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 = 0x6701,
}

//= aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum
//= type=implication
//# The Material Providers Library MUST provide
//# an ENUM that is the super set of all the [supported format algorithm suites enum](#supported-format-algorithm-suites-enum)
//# called the Algorithm Suite ENUM.
//
//= aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum
//= type=implication
//# This means that different formats MAY have duplicate Format Algorithm Suite ENUM.
//
//= aws-encryption-sdk-specification/framework/algorithm-suites.md#overview
//= type=implication
//# The algorithm suite defines the behaviors [supported formats](#supported-formats) MUST follow for cryptographic operations.
#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum AlgorithmSuiteId {
    Esdk(EsdkAlgorithmSuiteId),
    Dbe(DbeAlgorithmSuiteId),
}
impl Default for AlgorithmSuiteId {
    fn default() -> Self {
        Self::Esdk(EsdkAlgorithmSuiteId::default())
    }
}

//= aws-encryption-sdk-specification/framework/algorithm-suites.md#structure
//= type=implication
//# The fields described below are REQUIRED to be specified by algorithm suites, unless otherwise specified.
#[derive(Debug, PartialEq, Clone, Default)]
#[non_exhaustive]
pub struct AlgorithmSuiteInfo {
    id: AlgorithmSuiteId,
    binary_id: Vec<u8>,
    message_version: u32,
    encrypt: Encrypt,
    kdf: DerivationAlgorithm,
    commitment: DerivationAlgorithm,
    signature: SignatureAlgorithm,
    symmetric_signature: SymmetricSignatureAlgorithm,
    edk_wrapping: EdkWrappingAlgorithm,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum Encrypt {
    //= aws-encryption-sdk-specification/framework/algorithm-suites.md#gcm
    //= type=implication
    //# If specified to use GCM, the AWS Encryption SDK MUST use GCM with the following specifics:
    //# - The internal block cipher is the encryption algorithm specified by the algorithm suite.
    AesGcm(aws_mpl_primitives::AesGcm),
}
impl Default for Encrypt {
    fn default() -> Self {
        Self::AesGcm(aws_mpl_primitives::AesGcm::default())
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum DerivationAlgorithm {
    Hkdf(aws_mpl_primitives::DigestAlg),
    // We are using both `IDENTITY` and `None` here
    // to model the fact that deriving
    // the data encryption key and the commitment key
    // MUST be the same.
    // The specification treats NO_KDF as an identity operation.
    // So this naming convention mirrors the specification.
    Identity,
    #[default]
    None,
}

//= aws-encryption-sdk-specification/framework/algorithm-suites.md#asymmetric-signature-algorithm
//= type=implication
//# This field is OPTIONAL.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    Ecdsa(aws_mpl_primitives::EcdsaSignatureAlgorithm),
    #[default]
    None,
}

//= aws-encryption-sdk-specification/framework/algorithm-suites.md#symmetric-signature-algorithm
//# This field is OPTIONAL.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum SymmetricSignatureAlgorithm {
    Hmac(aws_mpl_primitives::DigestAlg),
    #[default]
    None,
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
#[non_exhaustive]
pub enum EdkWrappingAlgorithm {
    #[default]
    DirectKeyWrapping,
    IntermediateKeyWrapping(IntermediateKeyWrapping),
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub struct IntermediateKeyWrapping {
    key_encryption_key_kdf: DerivationAlgorithm,
    mac_key_kdf: DerivationAlgorithm,
    pdk_encrypt_algorithm: Encrypt,
}

pub fn get_algorithm_suite_info(binary_id: &[u8]) -> Result<AlgorithmSuiteInfo, Error> {
    Err(mpl_err("foo"))
}

pub fn valid_algorithm_suite_info(suite: &AlgorithmSuiteInfo) -> Result<(), Error> {
    Err(mpl_err("InvalidAlgorithmSuiteInfo"))
}
