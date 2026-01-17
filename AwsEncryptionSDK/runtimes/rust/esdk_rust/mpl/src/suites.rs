// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(unused)]

use crate::error::*;
use aws_mpl_primitives::AesGcm;
use aws_mpl_primitives::EcdsaSignatureAlgorithm;
use std::collections::HashMap;
use std::sync::LazyLock;

const BITS256: u32 = 32;
const BITS192: u32 = 24;
const BITS128: u32 = 16;
const TAG_LEN: u32 = 16;
const LV_LEN: u32 = 12;

// Algorithm Suites

// For now, the actual properties of algorithm suites are only used by internal
// components and are not actually customer facing. If and when we make them
// customer facing, we will need to either model the AlgorithmSuiteProperties
// as a separate structure (with an associated resource/operation for translating
// from name to properties) or use more advanced custom traits which allow us to
// model all properties of the algorithm suite in one structure.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct AlgorithmSuite {
    pub id: AlgorithmSuiteId,
    pub binary_id: [u8; 2],
    pub message_version: u32,
    pub encrypt: Encrypt,
    pub kdf: DerivationAlgorithm,
    pub commitment: DerivationAlgorithm,
    pub signature: SignatureAlgorithm,
    pub symmetric_signature: SymmetricSignatureAlgorithm,
    pub edk_wrapping: EdkWrappingAlgorithm,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Encrypt {
    //= aws-encryption-sdk-specification/framework/algorithm-suites.md#gcm
    //= type=implication
    //# If specified to use GCM, the AWS Encryption SDK MUST use GCM with the following specifics:
    //# - The internal block cipher is the encryption algorithm specified by the algorithm suite.
    AesGcm(AesGcm),
}
impl Default for Encrypt {
    fn default() -> Self {
        Self::AesGcm(AesGcm::default())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct Hkdf {
    pub hmac: aws_mpl_primitives::DigestAlg,
    pub salt_length: u32,
    pub input_key_length: u32,
    pub output_key_length: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum DerivationAlgorithm {
    Hkdf(Hkdf),
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

const fn hkdf_sha_256(key_length: u32) -> DerivationAlgorithm {
    DerivationAlgorithm::Hkdf(Hkdf {
        hmac: aws_mpl_primitives::DigestAlg::Sha256,
        salt_length: 0,
        input_key_length: key_length,
        output_key_length: key_length,
    })
}

const fn hkdf_sha_384(key_length: u32) -> DerivationAlgorithm {
    DerivationAlgorithm::Hkdf(Hkdf {
        hmac: aws_mpl_primitives::DigestAlg::Sha384,
        salt_length: 0,
        input_key_length: key_length,
        output_key_length: key_length,
    })
}

const fn hkdf_sha_512(key_length: u32) -> DerivationAlgorithm {
    DerivationAlgorithm::Hkdf(Hkdf {
        hmac: aws_mpl_primitives::DigestAlg::Sha512,
        salt_length: 32,
        input_key_length: key_length,
        output_key_length: key_length,
    })
}

//= aws-encryption-sdk-specification/framework/algorithm-suites.md#asymmetric-signature-algorithm
//= type=implication
//# This field is OPTIONAL.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    Ecdsa(EcdsaSignatureAlgorithm),
    #[default]
    None,
}

//= aws-encryption-sdk-specification/framework/algorithm-suites.md#symmetric-signature-algorithm
//# This field is OPTIONAL.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum SymmetricSignatureAlgorithm {
    Hmac(aws_mpl_primitives::DigestAlg),
    #[default]
    None,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum EdkWrappingAlgorithm {
    #[default]
    DirectKeyWrapping,
    IntermediateKeyWrapping(IntermediateKeyWrapping),
}

const EDK_INTERMEDIATE_WRAPPING_AES_GCM_256_HKDF_SHA_512: EdkWrappingAlgorithm =
    EdkWrappingAlgorithm::IntermediateKeyWrapping(IntermediateKeyWrapping {
        key_encryption_key_kdf: hkdf_sha_512(BITS256),
        mac_key_kdf: hkdf_sha_512(BITS256),
        pdk_encrypt_algorithm: Encrypt::AesGcm(AesGcm::Aes256Gcm),
    });

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct IntermediateKeyWrapping {
    pub key_encryption_key_kdf: DerivationAlgorithm,
    pub mac_key_kdf: DerivationAlgorithm,
    pub pdk_encrypt_algorithm: Encrypt,
}

pub fn get_algorithm_suite_info(binary_id: [u8; 2]) -> Result<&'static AlgorithmSuite, Error> {
    match binary_id {
        [0x00, 0x14] => Ok(&ESDK_ALG_AES_128_GCM_IV12_TAG16_NO_KDF),
        [0x00, 0x46] => Ok(&ESDK_ALG_AES_192_GCM_IV12_TAG16_NO_KDF),
        [0x00, 0x78] => Ok(&ESDK_ALG_AES_256_GCM_IV12_TAG16_NO_KDF),
        [0x01, 0x14] => Ok(&ESDK_ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256),
        [0x01, 0x46] => Ok(&ESDK_ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256),
        [0x01, 0x78] => Ok(&ESDK_ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256),
        [0x02, 0x14] => Ok(&ESDK_ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256),
        [0x03, 0x46] => Ok(&ESDK_ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384),
        [0x03, 0x78] => Ok(&ESDK_ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384),
        [0x04, 0x78] => Ok(&ESDK_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY),
        [0x05, 0x78] => Ok(&ESDK_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384),
        [0x67, 0x00] => Ok(&DBE_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384),
        [0x67, 0x01] => {
            Ok(&DBE_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384)
        }
        _ => Err(mpl_err("Invalid BinaryId")),
    }
}
const fn implies(a: bool, b: bool) -> bool {
    !a || b
}
// Invariants for all supported Algorithm Suites
const fn valid_suite(a: &AlgorithmSuite) -> bool {
    true
    /*
    && KeyDerivationAlgorithm?(a.kdf)
    && CommitmentDerivationAlgorithm?(a.commitment)
    && EdkWrappingAlgorithm?(a.edkWrapping)

    // If there is a KDF, the output length MUST match the encrypt length
    && (a.kdf.HKDF? ==>
          && a.kdf.HKDF.outputKeyLength == a.encrypt.AES_GCM.keyLength)
       // If there is a signature, there MUST be a KDF
    && (a.signature.ECDSA? ==> a.kdf.HKDF?)
       // If there is commitment, the KDF MUST match
    && (a.commitment.HKDF? ==>
          && a.commitment.HKDF.saltLength == 32
          && a.commitment == a.kdf)
       // If there is a IntermediateKeyWrapping, the KDFs MUST match
    && (a.edkWrapping.IntermediateKeyWrapping? ==>
          && a.kdf.HKDF?
          && a.edkWrapping.IntermediateKeyWrapping.keyEncryptionKeyKdf == a.kdf
          && a.edkWrapping.IntermediateKeyWrapping.macKeyKdf == a.kdf)
       // If there is a KDF and no commitment then salt MUST be 0
    && (a.kdf.HKDF? && a.commitment.None? ==> a.kdf.HKDF.saltLength == 0)
       //= aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-signature-settings
       //= type=implication
       //# An algorithm suite with a symmetric signature algorithm MUST use [intermediate key wrapping](#intermediate-key-wrapping).
       //
       // If the algorithm suite includes an symmetric signature algorithm:
       //= aws-encryption-sdk-specification/framework/algorithm-suites.md#symmetric-signature-algorithm
       //# - The algorithm suite MUST also use [Intermediate Key Wrapping](#intermediate-key-wrapping).
    && (!a.symmetricSignature.None? ==>
          && a.edkWrapping.IntermediateKeyWrapping?)
          */
}

const fn valid_esdk_suite(suite: &AlgorithmSuite, a: EsdkAlgorithmSuiteId) -> bool {
    // Adheres to general Algorithm Suite constraints
    valid_suite(suite)
    /*
        // Adheres to constraints for all algorithm suites
        && AlgorithmSuiteInfo?(a)
           // All ESDK encrypt with AES_GCM
        && SupportedESDKEncrypt?(a.encrypt)

        // Specification for each supported ESDK Algorithm Suite
        && match a.id.ESDK
           // Legacy non-KDF suites

           case ALG_AES_128_GCM_IV12_TAG16_NO_KDF() =>
             && a.binaryId == [0x00, 0x14]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 16
             && a.kdf.IDENTITY?
             && a.signature.None?
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_192_GCM_IV12_TAG16_NO_KDF() =>
             && a.binaryId == [0x00, 0x46]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 24
             && a.kdf.IDENTITY?
             && a.signature.None?
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_256_GCM_IV12_TAG16_NO_KDF() =>
             && a.binaryId == [0x00, 0x78]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.IDENTITY?
             && a.signature.None?
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?

           // HKDF suites

           case ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256() =>
             && a.binaryId == [0x01, 0x14]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 16
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_256
             && a.signature.None?
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256() =>
             && a.binaryId == [0x01, 0x46]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 24
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_256
             && a.signature.None?
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256() =>
             && a.binaryId == [0x01, 0x78]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_256
             && a.signature.None?
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?

           // Signature suites

           case ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256() =>
             && a.binaryId == [0x02, 0x14]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 16
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_256
             && a.signature.ECDSA?
             && a.signature.ECDSA.curve == AwsCryptographyPrimitivesTypes.ECDSA_P256
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384() =>
             && a.binaryId == [0x03, 0x46]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 24
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_384
             && a.signature.ECDSA?
             && a.signature.ECDSA.curve == AwsCryptographyPrimitivesTypes.ECDSA_P384
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384() =>
             && a.binaryId == [0x03, 0x78]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_384
             && a.signature.ECDSA?
             && a.signature.ECDSA.curve == AwsCryptographyPrimitivesTypes.ECDSA_P384
             && a.commitment.None?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?

           // Suites with key commitment

           case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY() =>
             && a.binaryId == [0x04, 0x78]
             && a.messageVersion == 2
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_512
             && a.signature.None?
             && a.commitment.HKDF?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
           case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384() =>
             && a.binaryId == [0x05, 0x78]
             && a.messageVersion == 2
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_512
             && a.signature.ECDSA?
             && a.signature.ECDSA.curve == AwsCryptographyPrimitivesTypes.ECDSA_P384
             && a.commitment.HKDF?
             && a.symmetricSignature.None?
             && a.edkWrapping.DIRECT_KEY_WRAPPING?
    */
}

const fn valid_dbe_suite(suite: &AlgorithmSuite, a: DbeAlgorithmSuiteId) -> bool {
    // Adheres to general Algorithm Suite constraints
    valid_suite(suite)
    /*
        // DBE only supports suites with AES_GCM 256
        && SupportedDBEEncrypt?(a.encrypt)

        // DBE only supports suites with intermediate provider wrapping keys
        && SupportedDBEEDKWrapping?(a.edkWrapping)

        // Specification for each supported DBE Algorithm Suite
        && match a.id.DBE
           case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384() =>
             && a.binaryId == [0x67, 0x00]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_512
             && a.signature.None?
             && a.commitment.HKDF?
             && a.symmetricSignature.HMAC?
             && a.symmetricSignature.HMAC == AwsCryptographyPrimitivesTypes.SHA_384
             && a.edkWrapping.IntermediateKeyWrapping?
             && a.edkWrapping.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM?
             && a.edkWrapping.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM.keyLength == 32
           case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384() =>
             && a.binaryId == [0x67, 0x01]
             && a.messageVersion == 1
             && a.encrypt.AES_GCM?
             && a.encrypt.AES_GCM.keyLength == 32
             && a.kdf.HKDF?
             && a.kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_512
             && a.signature.ECDSA?
             && a.signature.ECDSA.curve == AwsCryptographyPrimitivesTypes.ECDSA_P384
             && a.commitment.HKDF?
             && a.symmetricSignature.HMAC?
             && a.symmetricSignature.HMAC == AwsCryptographyPrimitivesTypes.SHA_384
             && a.edkWrapping.IntermediateKeyWrapping?
             && a.edkWrapping.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM?
             && a.edkWrapping.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM.keyLength == 32
    */
}

pub fn valid_algorithm_suite_info(suite: &AlgorithmSuite) -> Result<(), Error> {
    let valid = match suite.id {
        AlgorithmSuiteId::Esdk(a) => valid_esdk_suite(suite, a),
        AlgorithmSuiteId::Dbe(a) => valid_dbe_suite(suite, a),
    };
    if valid {
        Ok(())
    } else {
        Err(err(
            ErrorKind::InvalidAlgorithmSuiteInfo,
            "Invalid AlgorithmSuiteInfo",
        ))
    }
}

const ESDK_ALG_AES_128_GCM_IV12_TAG16_NO_KDF: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf),
    binary_id: [0x00, 0x14],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes128Gcm),
    kdf: DerivationAlgorithm::Identity,
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_192_GCM_IV12_TAG16_NO_KDF: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf),
    binary_id: [0x00, 0x46],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes192Gcm),
    kdf: DerivationAlgorithm::Identity,
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_256_GCM_IV12_TAG16_NO_KDF: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf),
    binary_id: [0x00, 0x78],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
    kdf: DerivationAlgorithm::Identity,
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256),
    binary_id: [0x01, 0x14],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes128Gcm),
    kdf: hkdf_sha_256(BITS128),
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256),
    binary_id: [0x01, 0x46],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes192Gcm),
    kdf: hkdf_sha_256(BITS192),
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256),
    binary_id: [0x01, 0x78],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
    kdf: hkdf_sha_256(BITS256),
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

//Signature KDF suites
const ESDK_ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256),
    binary_id: [0x02, 0x14],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes128Gcm),
    kdf: hkdf_sha_256(BITS128),
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::Ecdsa(EcdsaSignatureAlgorithm::EcdsaP256),
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384),
    encrypt: Encrypt::AesGcm(AesGcm::Aes192Gcm),
    binary_id: [0x03, 0x46],
    message_version: 1,
    kdf: hkdf_sha_384(BITS192),
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::Ecdsa(EcdsaSignatureAlgorithm::EcdsaP384),
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384),
    binary_id: [0x03, 0x78],
    message_version: 1,
    encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
    kdf: hkdf_sha_384(BITS256),
    commitment: DerivationAlgorithm::None,
    signature: SignatureAlgorithm::Ecdsa(EcdsaSignatureAlgorithm::EcdsaP384),
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

// Commitment Suites
const ESDK_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey),
    binary_id: [0x04, 0x78],
    message_version: 2,
    encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
    kdf: hkdf_sha_512(BITS256),
    commitment: hkdf_sha_512(BITS256),
    signature: SignatureAlgorithm::None,
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const ESDK_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384: AlgorithmSuite = AlgorithmSuite {
    id: AlgorithmSuiteId::Esdk(EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384),
    binary_id: [0x05, 0x78],
    message_version: 2,
    encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
    kdf: hkdf_sha_512(BITS256),
    commitment: hkdf_sha_512(BITS256),
    signature: SignatureAlgorithm::Ecdsa(EcdsaSignatureAlgorithm::EcdsaP384),
    symmetric_signature: SymmetricSignatureAlgorithm::None,
    edk_wrapping: EdkWrappingAlgorithm::DirectKeyWrapping,
};

const DBE_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384: AlgorithmSuite =
    AlgorithmSuite {
        id: AlgorithmSuiteId::Dbe(
            DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384,
        ),
        binary_id: [0x67, 0x00],
        message_version: 1,
        encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
        kdf: hkdf_sha_512(BITS256),
        commitment: hkdf_sha_512(BITS256),
        signature: SignatureAlgorithm::Ecdsa(EcdsaSignatureAlgorithm::EcdsaP384),
        symmetric_signature: SymmetricSignatureAlgorithm::None,
        edk_wrapping: EDK_INTERMEDIATE_WRAPPING_AES_GCM_256_HKDF_SHA_512,
    };

const DBE_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384: AlgorithmSuite =
    AlgorithmSuite {
        id: AlgorithmSuiteId::Dbe(
            DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384,
        ),
        binary_id: [0x67, 0x01],
        message_version: 1,
        encrypt: Encrypt::AesGcm(AesGcm::Aes256Gcm),
        kdf: hkdf_sha_512(BITS256),
        commitment: hkdf_sha_512(BITS256),
        signature: SignatureAlgorithm::Ecdsa(EcdsaSignatureAlgorithm::EcdsaP384),
        symmetric_signature: SymmetricSignatureAlgorithm::Hmac(
            aws_mpl_primitives::DigestAlg::Sha384,
        ),
        edk_wrapping: EDK_INTERMEDIATE_WRAPPING_AES_GCM_256_HKDF_SHA_512,
    };

const fn get_dbe_suite(id: DbeAlgorithmSuiteId) -> &'static AlgorithmSuite {
    match id {
        DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => {
            &DBE_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_SYMSIG_HMAC_SHA384
        }
        DbeAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => {
            &DBE_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384
        }
    }
}

const fn get_esdk_suite(id: EsdkAlgorithmSuiteId) -> &'static AlgorithmSuite {
    match id {
        EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => &ESDK_ALG_AES_128_GCM_IV12_TAG16_NO_KDF,
        EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => &ESDK_ALG_AES_192_GCM_IV12_TAG16_NO_KDF,
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => &ESDK_ALG_AES_256_GCM_IV12_TAG16_NO_KDF,
        EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => {
            &ESDK_ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256
        }
        EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => {
            &ESDK_ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256
        }
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => {
            &ESDK_ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256
        }
        EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            &ESDK_ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256
        }
        EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            &ESDK_ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384
        }
        EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            &ESDK_ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384
        }
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
            &ESDK_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY
        }
        EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            &ESDK_ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384
        }
    }
}

const fn get_suite(id: AlgorithmSuiteId) -> &'static AlgorithmSuite {
    match id {
        AlgorithmSuiteId::Dbe(e) => get_dbe_suite(e),
        AlgorithmSuiteId::Esdk(e) => get_esdk_suite(e),
    }
}

/*


  // Invariants for Esdk Encrypt
  predicate method SupportedEsdkEncrypt?(e: Encrypt) {
    && e.AES_GCM?
    && (
         || e.AES_GCM.keyLength == 32
         || e.AES_GCM.keyLength == 24
         || e.AES_GCM.keyLength == 16)
    && e.AES_GCM.tagLength == 16
    && e.AES_GCM.ivLength == 12
  }

  // Invariants for DBE Encrypt
  predicate method SupportedDBEEncrypt?(e: Encrypt) {
    && e.AES_GCM?
    && e.AES_GCM.keyLength == 32
    && e.AES_GCM.tagLength == 16
    && e.AES_GCM.ivLength == 12
  }

  // Invariants for DBE EDK Wrapping Algorithms
  predicate method SupportedDBEsdk_wrapping?(p: EdkWrappingAlgorithm) {
    && p.IntermediateKeyWrapping?
    && p.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM?
    && p.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM.keyLength == 32
    && p.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM.tagLength == 16
    && p.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM.ivLength == 12
    && p.IntermediateKeyWrapping.macKeyKdf.HKDF?
    && p.IntermediateKeyWrapping.keyEncryptionKeyKdf.HKDF?
  }

  // Invariants for all supported KDFs
  predicate method KeyDerivationAlgorithm?(kdf: DerivationAlgorithm) {
    && (
         && kdf.HKDF?
         ==>
           && kdf.HKDF.inputKeyLength == kdf.HKDF.outputKeyLength
           && (kdf.HKDF.hmac == AwsCryptographyPrimitivesTypes.SHA_512 ==> kdf.HKDF.inputKeyLength == 32))
    && !kdf.None?
  }

  // Invariants for all supported Commitment Derivation Algorithms
  predicate method CommitmentDerivationAlgorithm?(kdf: DerivationAlgorithm) {
    && (
         && kdf.HKDF?
         ==>
           && kdf.HKDF.hmac.SHA_512?
           && kdf.HKDF.saltLength == 32
           && kdf.HKDF.inputKeyLength == 32
           && kdf.HKDF.outputKeyLength == 32)
    && !kdf.IDENTITY?
  }

  // Invariants for all supported Provider Wrapping Algorithms
  predicate method EdkWrappingAlgorithm?(alg: EdkWrappingAlgorithm) {
    || (
         && alg.IntermediateKeyWrapping?
         && alg.IntermediateKeyWrapping.keyEncryptionKeyKdf.HKDF?
         && alg.IntermediateKeyWrapping.macKeyKdf.HKDF?
         && alg.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM?
         && alg.IntermediateKeyWrapping.pdkEncryptAlgorithm.AES_GCM.keyLength == 32
       )
    || alg.DIRECT_KEY_WRAPPING?
  }

  // Invariants for all supported Algorithm Suites
  predicate method AlgorithmSuite?(a: AlgorithmSuite) {
    && KeyDerivationAlgorithm?(a.kdf)
    && CommitmentDerivationAlgorithm?(a.commitment)
    && EdkWrappingAlgorithm?(a.edk_wrapping)

    // If there is a KDF, the output length MUST match the encrypt length
    && (a.kdf.HKDF? ==>
          && a.kdf.HKDF.outputKeyLength == a.encrypt.AES_GCM.keyLength)
       // If there is a signature, there MUST be a KDF
    && (a.signature.ECDSA? ==> a.kdf.HKDF?)
       // If there is commitment, the KDF MUST match
    && (a.commitment.HKDF? ==>
          && a.commitment.HKDF.saltLength == 32
          && a.commitment == a.kdf)
       // If there is a IntermediateKeyWrapping, the KDFs MUST match
    && (a.edk_wrapping.IntermediateKeyWrapping? ==>
          && a.kdf.HKDF?
          && a.edk_wrapping.IntermediateKeyWrapping.keyEncryptionKeyKdf == a.kdf
          && a.edk_wrapping.IntermediateKeyWrapping.macKeyKdf == a.kdf)
       // If there is a KDF and no commitment then salt MUST be 0
    && (a.kdf.HKDF? && a.commitment.None? ==> a.kdf.HKDF.saltLength == 0)
       //= aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-signature-settings
       //= type=implication
       //# An algorithm suite with a symmetric signature algorithm MUST use [intermediate key wrapping](#intermediate-key-wrapping).
       //
       // If the algorithm suite includes an symmetric signature algorithm:
       //= aws-encryption-sdk-specification/framework/algorithm-suites.md#symmetric-signature-algorithm
       //# - The algorithm suite MUST also use [Intermediate Key Wrapping](#intermediate-key-wrapping).
    && (!a.symmetric_signature.None? ==>
          && a.edk_wrapping.IntermediateKeyWrapping?)
  }

  const AES_128_GCM_IV12_TAG16 : Encrypt.AES_GCM(AwsCryptographyPrimitivesTypes.AES_GCM(
                                                    keyLength : BITS128,
                                                    tagLength : TagLen,
                                                    ivLength : IvLen
                                                  ))
  const AES_192_GCM_IV12_TAG16 : Encrypt.AES_GCM(AwsCryptographyPrimitivesTypes.AES_GCM(
                                                    keyLength : BITS192,
                                                    tagLength : TagLen,
                                                    ivLength : IvLen
                                                  ))
  const AES_256_GCM_IV12_TAG16 : Encrypt.AES_GCM(AwsCryptographyPrimitivesTypes.AES_GCM(
                                                    keyLength : BITS256,
                                                    tagLength : TagLen,
                                                    ivLength : IvLen
                                                  ))

  function method GetEncryptKeyLength(a: AlgorithmSuite)
    : (output: int32)
    ensures
      && AwsCryptographyPrimitivesTypes.IsValid_PositiveInteger(output)
      && AwsCryptographyPrimitivesTypes.IsValid_SymmetricKeyLength(output)
    ensures a.encrypt.AES_GCM? ==> output == a.encrypt.AES_GCM.keyLength
  {
    match a.encrypt
    case AES_GCM(e) => e.keyLength
  }

  function method GetEncryptTagLength(a: AlgorithmSuite)
    : (output: int32)
    ensures
      && AwsCryptographyPrimitivesTypes.IsValid_PositiveInteger(output)
      && AwsCryptographyPrimitivesTypes.IsValid_Uint8Bytes(output)
    ensures a.encrypt.AES_GCM? ==> output == a.encrypt.AES_GCM.tagLength
  {
    match a.encrypt
    case AES_GCM(e) => e.tagLength
  }

  function method GetEncryptIvLength(a: AlgorithmSuite)
    : (output: int32)
    ensures
      && AwsCryptographyPrimitivesTypes.IsValid_PositiveInteger(output)
      && AwsCryptographyPrimitivesTypes.IsValid_Uint8BITS(output)
    ensures a.encrypt.AES_GCM? ==> output == a.encrypt.AES_GCM.ivLength
  {
    match a.encrypt
    case AES_GCM(e) => e.ivLength
  }
}
 */
