// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! Legacy type conversion layer between `aws_mpl_legacy::dafny::types` and `aws_mpl_legacy::suites`.

use crate::{Error, val_err};
use aws_mpl_legacy::DecryptionMaterials;
use aws_mpl_legacy::EncryptionMaterials;
use aws_mpl_legacy::Secret;
use aws_mpl_legacy::suites::AlgorithmSuite;

pub(crate) const fn from_dafny_esdk_suite_id(
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
    }
}

pub(crate) const fn from_dafny_dbe_suite_id(
    legacy: aws_mpl_legacy::dafny::types::DbeAlgorithmSuiteId,
) -> aws_mpl_legacy::suites::DbeAlgorithmSuiteId {
    use aws_mpl_legacy::dafny::types::DbeAlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::DbeAlgorithmSuiteId as New;
    match legacy {
        Old::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => {
            New::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384
        }
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => {
            New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384
        }
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
) -> Result<aws_mpl_legacy::dafny::types::CommitmentPolicy, Error> {
    match x {
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => {
            Ok(aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
            ))
        }
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => {
            Ok(aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt,
            ))
        }
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => {
            Ok(aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
            ))
        }
        _ => Err(val_err("Unknown commitment policy variant")),
    }
}

pub(crate) fn convert_edk(
    x: &aws_mpl_legacy::EncryptedDataKey,
) -> Result<aws_mpl_legacy::dafny::types::EncryptedDataKey, Error> {
    aws_mpl_legacy::dafny::types::EncryptedDataKey::builder()
        .key_provider_id(x.key_provider_id.clone())
        .key_provider_info(x.key_provider_info.clone())
        .ciphertext(x.ciphertext.clone())
        .build()
        .map_err(|e| val_err(format!("Failed to build legacy EncryptedDataKey: {e}")))
}

pub(crate) fn convert_edks(
    x: &[aws_mpl_legacy::EncryptedDataKey],
) -> Result<Vec<aws_mpl_legacy::dafny::types::EncryptedDataKey>, Error> {
    x.iter().map(convert_edk).collect()
}

pub(crate) fn convert_esdk_alg(
    x: aws_mpl_legacy::suites::EsdkAlgorithmSuiteId,
) -> Result<aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId, Error> {
    match x {
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey)
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            Ok(aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384)
        }
        _ => Err(val_err("Unknown ESDK algorithm suite variant")),
    }
}

pub(crate) fn convert_alg(
    x: aws_mpl_legacy::suites::AlgorithmSuiteId,
) -> Result<aws_mpl_legacy::dafny::types::AlgorithmSuiteId, Error> {
    match x {
        aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(a) => Ok(
            aws_mpl_legacy::dafny::types::AlgorithmSuiteId::Esdk(convert_esdk_alg(a)?),
        ),
        _ => Err(val_err("Unknown algorithm suite variant for conversion")),
    }
}

pub(crate) fn from_legacy_aes(
    x: &aws_mpl_legacy::dafny::aws_cryptography_primitives::types::AesGcm,
) -> Result<aws_mpl_legacy::primitives::AesGcm, Error> {
    match x
        .key_length()
        .ok_or_else(|| val_err("Legacy AES-GCM missing key_length"))?
    {
        16 => Ok(aws_mpl_legacy::primitives::AesGcm::Aes128Gcm),
        24 => Ok(aws_mpl_legacy::primitives::AesGcm::Aes192Gcm),
        32 => Ok(aws_mpl_legacy::primitives::AesGcm::Aes256Gcm),
        other => Err(val_err(format!("Unknown AES-GCM key length: {other}"))),
    }
}

pub(crate) fn from_legacy_encrypt(
    x: aws_mpl_legacy::dafny::types::Encrypt,
) -> Result<aws_mpl_legacy::primitives::AesGcm, Error> {
    match x {
        aws_mpl_legacy::dafny::types::Encrypt::AesGcm(x) => from_legacy_aes(&x),
        _ => Err(val_err("Unknown legacy encrypt variant")),
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

pub(crate) fn from_legacy_hkdf(
    x: &aws_mpl_legacy::dafny::types::Hkdf,
) -> Result<aws_mpl_legacy::suites::Hkdf, Error> {
    let mut n = aws_mpl_legacy::suites::Hkdf::default();
    n.hmac = from_legacy_hmac(x.hmac.ok_or_else(|| val_err("Legacy HKDF missing hmac"))?);
    let salt_length = x
        .salt_length
        .ok_or_else(|| val_err("Legacy HKDF missing salt_length"))?;
    let Ok(salt_length) = u32::try_from(salt_length) else {
        return Err(val_err("negative value from MPL"));
    };
    n.salt_length = salt_length;
    let input_key_length = x
        .input_key_length
        .ok_or_else(|| val_err("Legacy HKDF missing input_key_length"))?;
    let Ok(input_key_length) = u32::try_from(input_key_length) else {
        return Err(val_err("negative value from MPL"));
    };
    n.input_key_length = input_key_length;
    let output_key_length = x
        .output_key_length
        .ok_or_else(|| val_err("Legacy HKDF missing output_key_length"))?;
    let Ok(output_key_length) = u32::try_from(output_key_length) else {
        return Err(val_err("negative value from MPL"));
    };
    n.output_key_length = output_key_length;
    Ok(n)
}

pub(crate) fn from_legacy_da(
    x: aws_mpl_legacy::dafny::types::DerivationAlgorithm,
) -> Result<aws_mpl_legacy::suites::DerivationAlgorithm, Error> {
    use aws_mpl_legacy::dafny::types::DerivationAlgorithm as Old;
    use aws_mpl_legacy::suites::DerivationAlgorithm as New;
    match x {
        Old::Hkdf(x) => Ok(New::Hkdf(from_legacy_hkdf(&x)?)),
        Old::Identity(_x) => Ok(New::Identity),
        Old::None(_x) => Ok(New::None),
        _ => Err(val_err("Unknown legacy DerivationAlgorithm variant")),
    }
}

pub(crate) fn from_legacy_ecdsa(
    x: &aws_mpl_legacy::dafny::types::Ecdsa,
) -> Result<aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm, Error> {
    use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm as Old;
    use aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm as New;
    match x
        .curve
        .ok_or_else(|| val_err("Legacy ECDSA missing curve"))?
    {
        Old::EcdsaP256 => Ok(New::EcdsaP256),
        Old::EcdsaP384 => Ok(New::EcdsaP384),
    }
}

pub(crate) fn from_legacy_sig(
    x: aws_mpl_legacy::dafny::types::SignatureAlgorithm,
) -> Result<aws_mpl_legacy::suites::SignatureAlgorithm, Error> {
    use aws_mpl_legacy::dafny::types::SignatureAlgorithm as Old;
    use aws_mpl_legacy::suites::SignatureAlgorithm as New;
    match x {
        Old::Ecdsa(x) => Ok(New::Ecdsa(from_legacy_ecdsa(&x)?)),
        Old::None(_x) => Ok(New::None),
        _ => Err(val_err("Unknown legacy SignatureAlgorithm variant")),
    }
}

pub(crate) fn from_legacy_ssig(
    x: aws_mpl_legacy::dafny::types::SymmetricSignatureAlgorithm,
) -> Result<aws_mpl_legacy::suites::SymmetricSignatureAlgorithm, Error> {
    use aws_mpl_legacy::dafny::types::SymmetricSignatureAlgorithm as Old;
    use aws_mpl_legacy::suites::SymmetricSignatureAlgorithm as New;
    match x {
        Old::Hmac(x) => Ok(New::Hmac(from_legacy_hmac(x))),
        Old::None(_x) => Ok(New::None),
        _ => Err(val_err(
            "Unknown legacy SymmetricSignatureAlgorithm variant",
        )),
    }
}

pub(crate) fn from_legacy_inter_wrap(
    x: aws_mpl_legacy::dafny::types::IntermediateKeyWrapping,
) -> Result<aws_mpl_legacy::suites::IntermediateKeyWrapping, Error> {
    let mut r = aws_mpl_legacy::suites::IntermediateKeyWrapping::default();
    r.key_encryption_key_kdf = from_legacy_da(x.key_encryption_key_kdf.ok_or_else(|| {
        val_err("Legacy IntermediateKeyWrapping missing key_encryption_key_kdf")
    })?)?;
    r.mac_key_kdf = from_legacy_da(
        x.mac_key_kdf
            .ok_or_else(|| val_err("Legacy IntermediateKeyWrapping missing mac_key_kdf"))?,
    )?;
    r.pdk_encrypt_algorithm = aws_mpl_legacy::suites::Encrypt::AesGcm(from_legacy_encrypt(
        x.pdk_encrypt_algorithm.ok_or_else(|| {
            val_err("Legacy IntermediateKeyWrapping missing pdk_encrypt_algorithm")
        })?,
    )?);
    Ok(r)
}

pub(crate) fn from_legacy_wrap(
    x: aws_mpl_legacy::dafny::types::EdkWrappingAlgorithm,
) -> Result<aws_mpl_legacy::suites::EdkWrappingAlgorithm, Error> {
    use aws_mpl_legacy::dafny::types::EdkWrappingAlgorithm as Old;
    use aws_mpl_legacy::suites::EdkWrappingAlgorithm as New;
    match x {
        Old::DirectKeyWrapping(_x) => Ok(New::DirectKeyWrapping),
        Old::IntermediateKeyWrapping(x) => {
            Ok(New::IntermediateKeyWrapping(from_legacy_inter_wrap(x)?))
        }
        _ => Err(val_err("Unknown legacy EdkWrappingAlgorithm variant")),
    }
}

pub(crate) fn from_legacy_as(
    x: aws_mpl_legacy::dafny::types::AlgorithmSuiteInfo,
) -> Result<AlgorithmSuite, Error> {
    let mut s = AlgorithmSuite::default();
    s.id = from_dafny_suite_id(
        &x.id
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing id"))?,
    )?;
    s.binary_id = x
        .binary_id
        .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing binary_id"))?
        .as_ref()[0..2]
        .try_into()
        .map_err(|_| val_err("Legacy AlgorithmSuiteInfo binary_id too short"))?;
    let message_version = x
        .message_version
        .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing message_version"))?;
    let Ok(message_version) = u32::try_from(message_version) else {
        return Err(val_err("negative value from MPL"));
    };
    s.message_version = message_version;
    s.encrypt = aws_mpl_legacy::suites::Encrypt::AesGcm(from_legacy_encrypt(
        x.encrypt
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing encrypt"))?,
    )?);
    s.kdf = from_legacy_da(
        x.kdf
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing kdf"))?,
    )?;
    s.commitment = from_legacy_da(
        x.commitment
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing commitment"))?,
    )?;
    s.signature = from_legacy_sig(
        x.signature
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing signature"))?,
    )?;
    s.symmetric_signature = from_legacy_ssig(
        x.symmetric_signature
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing symmetric_signature"))?,
    )?;
    s.edk_wrapping = from_legacy_wrap(
        x.edk_wrapping
            .ok_or_else(|| val_err("Legacy AlgorithmSuiteInfo missing edk_wrapping"))?,
    )?;
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
        .ok_or_else(|| val_err("Legacy EncryptionMaterials missing encrypted_data_keys"))?
        .iter()
        .map(|x| {
            Ok(aws_mpl_legacy::EncryptedDataKey::new(
                x.key_provider_id()
                    .as_ref()
                    .ok_or_else(|| val_err("Legacy EDK missing key_provider_id"))?
                    .clone(),
                x.key_provider_info()
                    .as_ref()
                    .ok_or_else(|| val_err("Legacy EDK missing key_provider_info"))?
                    .clone(),
                x.ciphertext()
                    .as_ref()
                    .ok_or_else(|| val_err("Legacy EDK missing ciphertext"))?
                    .clone(),
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    ret.algorithm_suite = from_legacy_as(
        x.algorithm_suite
            .ok_or_else(|| val_err("Legacy EncryptionMaterials missing algorithm_suite"))?,
    )?;
    ret.encryption_context = x
        .encryption_context
        .ok_or_else(|| val_err("Legacy EncryptionMaterials missing encryption_context"))?;
    ret.required_encryption_context_keys = x.required_encryption_context_keys.ok_or_else(|| {
        val_err("Legacy EncryptionMaterials missing required_encryption_context_keys")
    })?;
    ret.plaintext_data_key = x.plaintext_data_key.map(|y| Secret(y.into_inner()));
    ret.signing_key = x.signing_key.map(|y| Secret(y.into_inner()));
    Ok(ret)
}

pub(crate) fn from_legacy_dm(
    x: aws_mpl_legacy::dafny::types::DecryptionMaterials,
) -> Result<DecryptionMaterials, Error> {
    let mut ret = DecryptionMaterials::default();
    ret.algorithm_suite = from_legacy_as(
        x.algorithm_suite
            .ok_or_else(|| val_err("Legacy DecryptionMaterials missing algorithm_suite"))?,
    )?;
    ret.encryption_context = x
        .encryption_context
        .ok_or_else(|| val_err("Legacy DecryptionMaterials missing encryption_context"))?;
    ret.required_encryption_context_keys = x.required_encryption_context_keys.ok_or_else(|| {
        val_err("Legacy DecryptionMaterials missing required_encryption_context_keys")
    })?;

    ret.plaintext_data_key = x.plaintext_data_key.map(|y| Secret(y.into_inner()));
    ret.verification_key = x.verification_key.map(|y| Secret(y.into_inner()));
    ret.symmetric_signing_key = x.symmetric_signing_key.map(|y| Secret(y.into_inner()));
    Ok(ret)
}
