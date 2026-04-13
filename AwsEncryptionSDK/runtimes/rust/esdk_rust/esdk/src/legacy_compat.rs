//! Legacy type conversion layer between aws_mpl_legacy::dafny::types and aws_mpl_legacy::suites.

use crate::*;
use aws_mpl_legacy::DecryptionMaterials;
use aws_mpl_legacy::EncryptionMaterials;
use aws_mpl_legacy::Secret;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) fn from_dafny_esdk_suite_id(
    legacy: aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId,
) -> aws_mpl_legacy::suites::EsdkAlgorithmSuiteId {
    use aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId as New;
    match legacy {
        Old::AlgAes128GcmIv12Tag16NoKdf => New::AlgAes128GcmIv12Tag16NoKdf,
        Old::AlgAes192GcmIv12Tag16NoKdf => New::AlgAes192GcmIv12Tag16NoKdf,
        Old::AlgAes256GcmIv12Tag16NoKdf => New::AlgAes256GcmIv12Tag16NoKdf,
        Old::AlgAes128GcmIv12Tag16HkdfSha256 => New::AlgAes128GcmIv12Tag16HkdfSha256,
        Old::AlgAes192GcmIv12Tag16HkdfSha256 => New::AlgAes192GcmIv12Tag16HkdfSha256,
        Old::AlgAes256GcmIv12Tag16HkdfSha256 => New::AlgAes256GcmIv12Tag16HkdfSha256,
        Old::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => New::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256,
        Old::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => New::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384,
        Old::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => New::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384,
        Old::AlgAes256GcmHkdfSha512CommitKey => New::AlgAes256GcmHkdfSha512CommitKey,
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
        _ => panic!("Unknown ESDK algorithm suite"),
    }
}

pub(crate) fn from_dafny_dbe_suite_id(
    legacy: aws_mpl_legacy::dafny::types::DbeAlgorithmSuiteId,
) -> aws_mpl_legacy::suites::DbeAlgorithmSuiteId {
    use aws_mpl_legacy::dafny::types::DbeAlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::DbeAlgorithmSuiteId as New;
    match legacy {
        Old::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => New::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384,
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384,
        _ => panic!("Unknown DBE algorithm suite"),
    }
}

pub(crate) fn from_dafny_suite_id(
    legacy: &aws_mpl_legacy::dafny::types::AlgorithmSuiteId,
) -> Result<aws_mpl_legacy::suites::AlgorithmSuiteId, Error> {
    use aws_mpl_legacy::dafny::types::AlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::AlgorithmSuiteId as New;
    match legacy {
        Old::Esdk(x) => Ok(New::Esdk(from_dafny_esdk_suite_id(*x))),
        Old::Dbe(x) => Ok(New::Dbe(from_dafny_dbe_suite_id(*x))),
        _ => Err(val_err("Unrecognized legacy AlgorithmSuiteId")),
    }
}

pub(crate) fn convert_commit(
    x: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> aws_mpl_legacy::dafny::types::CommitmentPolicy {
    match x {
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => {
            aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
            )
        }
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => {
            aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt,
            )
        }
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => {
            aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
            )
        }
        _ => panic!(),
    }
}

pub(crate) fn convert_edk(x: &aws_mpl_legacy::EncryptedDataKey) -> aws_mpl_legacy::dafny::types::EncryptedDataKey {
    aws_mpl_legacy::dafny::types::EncryptedDataKey::builder()
        .key_provider_id(x.key_provider_id.clone())
        .key_provider_info(x.key_provider_info.clone())
        .ciphertext(x.ciphertext.clone())
        .build()
        .unwrap()
}

pub(crate) fn convert_edks(
    x: &[aws_mpl_legacy::EncryptedDataKey],
) -> Vec<aws_mpl_legacy::dafny::types::EncryptedDataKey> {
    x.iter().map(convert_edk).collect()
}

pub(crate) fn convert_esdk_alg(
    x: aws_mpl_legacy::suites::EsdkAlgorithmSuiteId,
) -> aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId {
    match x {
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
        _ => panic!(),
    }
}

pub(crate) fn convert_alg(x: aws_mpl_legacy::suites::AlgorithmSuiteId) -> aws_mpl_legacy::dafny::types::AlgorithmSuiteId {
    match x {
        aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(a) => {
            aws_mpl_legacy::dafny::types::AlgorithmSuiteId::Esdk(convert_esdk_alg(a))
        }
        _ => panic!(),
    }
}

pub(crate) fn from_legacy_aes(
    x: &aws_mpl_legacy::dafny::aws_cryptography_primitives::types::AesGcm,
) -> aws_mpl_legacy::primitives::AesGcm {
    match x.key_length().unwrap() {
        16 => aws_mpl_legacy::primitives::AesGcm::Aes128Gcm,
        24 => aws_mpl_legacy::primitives::AesGcm::Aes192Gcm,
        32 => aws_mpl_legacy::primitives::AesGcm::Aes256Gcm,
        _ => panic!(),
    }
}

pub(crate) fn from_legacy_encrypt(x: aws_mpl_legacy::dafny::types::Encrypt) -> aws_mpl_legacy::primitives::AesGcm {
    match x {
        aws_mpl_legacy::dafny::types::Encrypt::AesGcm(x) => from_legacy_aes(&x),
        _ => panic!(),
    }
}

pub(crate) const fn from_legacy_hmac(
    x: aws_mpl_legacy::dafny::aws_cryptography_primitives::types::DigestAlgorithm,
) -> aws_mpl_legacy::primitives::DigestAlg {
    use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::DigestAlgorithm as Old;
    use aws_mpl_legacy::primitives::DigestAlg as New;
    match x {
        Old::Sha256 => New::Sha256,
        Old::Sha384 => New::Sha384,
        Old::Sha512 => New::Sha512,
    }
}

pub(crate) fn from_legacy_hkdf(x: &aws_mpl_legacy::dafny::types::Hkdf) -> aws_mpl_legacy::suites::Hkdf {
    let mut n = aws_mpl_legacy::suites::Hkdf::default();
    n.hmac = from_legacy_hmac(x.hmac.unwrap());
    n.salt_length = x.salt_length.unwrap() as u32;
    n.input_key_length = x.input_key_length.unwrap() as u32;
    n.output_key_length = x.output_key_length.unwrap() as u32;
    n
}

pub(crate) fn from_legacy_da(
    x: aws_mpl_legacy::dafny::types::DerivationAlgorithm,
) -> aws_mpl_legacy::suites::DerivationAlgorithm {
    use aws_mpl_legacy::dafny::types::DerivationAlgorithm as Old;
    use aws_mpl_legacy::suites::DerivationAlgorithm as New;
    match x {
        Old::Hkdf(x) => New::Hkdf(from_legacy_hkdf(&x)),
        Old::Identity(_x) => New::Identity,
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

pub(crate) const fn from_legacy_ecdsa(
    x: &aws_mpl_legacy::dafny::types::Ecdsa,
) -> aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm {
    use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm as Old;
    use aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm as New;
    match x.curve.unwrap() {
        Old::EcdsaP256 => New::EcdsaP256,
        Old::EcdsaP384 => New::EcdsaP384,
    }
}

pub(crate) fn from_legacy_sig(
    x: aws_mpl_legacy::dafny::types::SignatureAlgorithm,
) -> aws_mpl_legacy::suites::SignatureAlgorithm {
    use aws_mpl_legacy::dafny::types::SignatureAlgorithm as Old;
    use aws_mpl_legacy::suites::SignatureAlgorithm as New;
    match x {
        Old::Ecdsa(x) => New::Ecdsa(from_legacy_ecdsa(&x)),
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

pub(crate) fn from_legacy_ssig(
    x: aws_mpl_legacy::dafny::types::SymmetricSignatureAlgorithm,
) -> aws_mpl_legacy::suites::SymmetricSignatureAlgorithm {
    use aws_mpl_legacy::dafny::types::SymmetricSignatureAlgorithm as Old;
    use aws_mpl_legacy::suites::SymmetricSignatureAlgorithm as New;
    match x {
        Old::Hmac(x) => New::Hmac(from_legacy_hmac(x)),
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

pub(crate) fn from_legacy_inter_wrap(
    x: aws_mpl_legacy::dafny::types::IntermediateKeyWrapping,
) -> aws_mpl_legacy::suites::IntermediateKeyWrapping {
    let mut r = aws_mpl_legacy::suites::IntermediateKeyWrapping::default();
    r.key_encryption_key_kdf = from_legacy_da(x.key_encryption_key_kdf.unwrap());
    r.mac_key_kdf = from_legacy_da(x.mac_key_kdf.unwrap());
    r.pdk_encrypt_algorithm =
        aws_mpl_legacy::suites::Encrypt::AesGcm(from_legacy_encrypt(x.pdk_encrypt_algorithm.unwrap()));
    r
}

pub(crate) fn from_legacy_wrap(
    x: aws_mpl_legacy::dafny::types::EdkWrappingAlgorithm,
) -> aws_mpl_legacy::suites::EdkWrappingAlgorithm {
    use aws_mpl_legacy::dafny::types::EdkWrappingAlgorithm as Old;
    use aws_mpl_legacy::suites::EdkWrappingAlgorithm as New;
    match x {
        Old::DirectKeyWrapping(_x) => New::DirectKeyWrapping,
        Old::IntermediateKeyWrapping(x) => New::IntermediateKeyWrapping(from_legacy_inter_wrap(x)),
        _ => panic!(),
    }
}

pub(crate) fn from_legacy_as(x: aws_mpl_legacy::dafny::types::AlgorithmSuiteInfo) -> Result<AlgorithmSuite, Error> {
    let mut s = AlgorithmSuite::default();
    s.id = from_dafny_suite_id(&x.id.unwrap())?;
    s.binary_id = x.binary_id.unwrap().as_ref()[0..2].try_into().unwrap();
    s.message_version = x.message_version.unwrap() as u32;
    s.encrypt = aws_mpl_legacy::suites::Encrypt::AesGcm(from_legacy_encrypt(x.encrypt.unwrap()));
    s.kdf = from_legacy_da(x.kdf.unwrap());
    s.commitment = from_legacy_da(x.commitment.unwrap());
    s.signature = from_legacy_sig(x.signature.as_ref().unwrap().clone());
    s.symmetric_signature = from_legacy_ssig(x.symmetric_signature.unwrap());
    s.edk_wrapping = from_legacy_wrap(x.edk_wrapping.unwrap());
    Ok(s)
}

pub(crate) fn from_legacy_em(
    x: aws_mpl_legacy::dafny::types::EncryptionMaterials,
) -> Result<EncryptionMaterials, Error> {
    let mut ret = EncryptionMaterials::default();
    ret.symmetric_signing_keys = match x.symmetric_signing_keys.as_ref() {
        None => vec![],
        Some(x) => x.iter().map(|x| Secret(x.clone().into_inner())).collect(),
    };
    ret.encrypted_data_keys = x
        .encrypted_data_keys()
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| {
            aws_mpl_legacy::EncryptedDataKey::new(
                x.key_provider_id().as_ref().unwrap().clone(),
                x.key_provider_info().as_ref().unwrap().clone(),
                x.ciphertext().as_ref().unwrap().clone(),
            )
        })
        .collect();
    ret.algorithm_suite = from_legacy_as(x.algorithm_suite.unwrap())?;
    ret.encryption_context = x.encryption_context.unwrap();
    ret.required_encryption_context_keys = x.required_encryption_context_keys.unwrap();
    ret.plaintext_data_key = x.plaintext_data_key.map(|y| Secret(y.into_inner()));
    ret.signing_key = x.signing_key.map(|y| Secret(y.into_inner()));
    Ok(ret)
}

pub(crate) fn from_legacy_dm(
    x: aws_mpl_legacy::dafny::types::DecryptionMaterials,
) -> Result<DecryptionMaterials, Error> {
    let mut ret = DecryptionMaterials::default();
    ret.algorithm_suite = from_legacy_as(x.algorithm_suite.unwrap())?;
    ret.encryption_context = x.encryption_context.unwrap();
    ret.required_encryption_context_keys = x.required_encryption_context_keys.unwrap();

    ret.plaintext_data_key = x.plaintext_data_key.map(|y| Secret(y.into_inner()));
    ret.verification_key = x.verification_key.map(|y| Secret(y.into_inner()));
    ret.symmetric_signing_key = x.symmetric_signing_key.map(|y| Secret(y.into_inner()));
    Ok(ret)
}
