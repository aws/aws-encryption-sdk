// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM(
    input: &crate::deps::aws_cryptography_primitives::types::AesGcm,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAES__GCM_DkeyLength(&input.r#key_length)?;
    validate_aws_Pcryptography_Pprimitives_HAES__GCM_DtagLength(&input.r#tag_length)?;
    validate_aws_Pcryptography_Pprimitives_HAES__GCM_DivLength(&input.r#iv_length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM_DivLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "iv_length",
                "iv_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..=255).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "iv_length",
        "iv_length failed to satisfy constraint: Member must be between 0 and 255, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM_DkeyLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key_length",
                "key_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(1..=32).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "key_length",
        "key_length failed to satisfy constraint: Member must be between 1 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAES__GCM_DtagLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "tag_length",
                "tag_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..=32).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "tag_length",
        "tag_length failed to satisfy constraint: Member must be between 0 and 32, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput(
    input: &crate::deps::aws_cryptography_primitives::types::AesDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DencAlg(&input.r#enc_alg)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Dkey(&input.r#key)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DcipherTxt(&input.r#cipher_txt)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DauthTag(&input.r#auth_tag)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Div(&input.r#iv)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Daad(&input.r#aad)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_for_AwsCryptographicPrimitives_AESDecrypt(
    input: &crate::deps::aws_cryptography_primitives::operation::aes_decrypt::AesDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DencAlg(&input.r#enc_alg)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Dkey(&input.r#key)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DcipherTxt(&input.r#cipher_txt)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DauthTag(&input.r#auth_tag)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Div(&input.r#iv)?;
    validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Daad(&input.r#aad)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Daad(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "aad",
                "aad is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DauthTag(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "auth_tag",
                "auth_tag is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DcipherTxt(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "cipher_txt",
                "cipher_txt is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_DencAlg(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "enc_alg",
                "enc_alg is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HAES__GCM(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Div(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "iv",
                "iv is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptInput_Dkey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key",
                "key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESDecryptOutput_Dplaintext(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptOutput_for_AwsCryptographicPrimitives_AESDecrypt(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESDecryptOutput_Dplaintext(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESDecryptOutput_Dplaintext(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "plaintext",
                "plaintext is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput(
    input: &crate::deps::aws_cryptography_primitives::types::AesEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_DencAlg(&input.r#enc_alg)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Div(&input.r#iv)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Dkey(&input.r#key)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Dmsg(&input.r#msg)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Daad(&input.r#aad)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_for_AwsCryptographicPrimitives_AESEncrypt(
    input: &crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_DencAlg(&input.r#enc_alg)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Div(&input.r#iv)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Dkey(&input.r#key)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Dmsg(&input.r#msg)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Daad(&input.r#aad)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Daad(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "aad",
                "aad is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_DencAlg(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::AesGcm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "enc_alg",
                "enc_alg is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HAES__GCM(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Div(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "iv",
                "iv is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Dkey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key",
                "key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptInput_Dmsg(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "msg",
                "msg is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput(
    input: &crate::deps::aws_cryptography_primitives::types::AesEncryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_DcipherText(&input.r#cipher_text)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_DauthTag(&input.r#auth_tag)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_for_AwsCryptographicPrimitives_AESEncrypt(
    input: &crate::deps::aws_cryptography_primitives::operation::aes_encrypt::AesEncryptOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_DcipherText(&input.r#cipher_text)?;
    validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_DauthTag(&input.r#auth_tag)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_DauthTag(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "auth_tag",
                "auth_tag is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAESEncryptOutput_DcipherText(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "cipher_text",
                "cipher_text is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput(
    input: &crate::deps::aws_cryptography_primitives::types::AesKdfCtrInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_Dikm(&input.r#ikm)?;
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_DexpectedLength(
        &input.r#expected_length,
    )?;
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_Dnonce(&input.r#nonce)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_for_AwsCryptographicPrimitives_AesKdfCounterMode(
    input: &crate::deps::aws_cryptography_primitives::operation::aes_kdf_counter_mode::AesKdfCtrInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_Dikm(&input.r#ikm)?;
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_DexpectedLength(
        &input.r#expected_length,
    )?;
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_Dnonce(&input.r#nonce)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_DexpectedLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "expected_length",
                "expected_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expected_length",
        "expected_length failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_Dikm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ikm",
                "ikm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrInput_Dnonce(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrOutput_for_AwsCryptographicPrimitives_AesKdfCounterMode(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HAesKdfCtrOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HAesKdfCtrOutput_Dokm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "okm",
                "okm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput(
    input: &crate::deps::aws_cryptography_primitives::types::CompressPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_DpublicKey(&input.r#public_key)?;
    validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_DeccCurve(&input.r#ecc_curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_for_AwsCryptographicPrimitives_CompressPublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_DpublicKey(&input.r#public_key)?;
    validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_DeccCurve(&input.r#ecc_curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyInput_DpublicKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPublicKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyOutput(
    input: &crate::deps::aws_cryptography_primitives::types::CompressPublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyOutput_DcompressedPublicKey(
        &input.r#compressed_public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyOutput_for_AwsCryptographicPrimitives_CompressPublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::compress_public_key::CompressPublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyOutput_DcompressedPublicKey(
        &input.r#compressed_public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCompressPublicKeyOutput_DcompressedPublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "compressed_public_key",
                "compressed_public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HCryptoConfig(
    input: &crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput(
    input: &crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_DcompressedPublicKey(
        &input.r#compressed_public_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_DeccCurve(&input.r#ecc_curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_for_AwsCryptographicPrimitives_DecompressPublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_DcompressedPublicKey(
        &input.r#compressed_public_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_DeccCurve(&input.r#ecc_curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_DcompressedPublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "compressed_public_key",
                "compressed_public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyInput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyOutput(
    input: &crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyOutput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyOutput_for_AwsCryptographicPrimitives_DecompressPublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::decompress_public_key::DecompressPublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyOutput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDecompressPublicKeyOutput_DpublicKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPublicKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput(
    input: &crate::deps::aws_cryptography_primitives::types::DeriveSharedSecretInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DeccCurve(&input.r#ecc_curve)?;
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DprivateKey(
        &input.r#private_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_for_AwsCryptographicPrimitives_DeriveSharedSecret(
    input: &crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::DeriveSharedSecretInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DeccCurve(&input.r#ecc_curve)?;
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DprivateKey(
        &input.r#private_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DprivateKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "private_key",
                "private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPrivateKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretInput_DpublicKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPublicKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretOutput(
    input: &crate::deps::aws_cryptography_primitives::types::DeriveSharedSecretOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretOutput_DsharedSecret(
        &input.r#shared_secret,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretOutput_for_AwsCryptographicPrimitives_DeriveSharedSecret(
    input: &crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::DeriveSharedSecretOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretOutput_DsharedSecret(
        &input.r#shared_secret,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDeriveSharedSecretOutput_DsharedSecret(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "shared_secret",
                "shared_secret is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestInput(
    input: &crate::deps::aws_cryptography_primitives::types::DigestInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDigestInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HDigestInput_Dmessage(&input.r#message)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestInput_for_AwsCryptographicPrimitives_Digest(
    input: &crate::deps::aws_cryptography_primitives::operation::digest::DigestInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDigestInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HDigestInput_Dmessage(&input.r#message)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestInput_DdigestAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest_algorithm",
                "digest_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestInput_Dmessage(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "message",
                "message is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDigestOutput_Ddigest(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestOutput_for_AwsCryptographicPrimitives_Digest(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HDigestOutput_Ddigest(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HDigestOutput_Ddigest(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest",
                "digest is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECCPrivateKey(
    input: &crate::deps::aws_cryptography_primitives::types::EccPrivateKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECCPrivateKey_Dpem(&input.r#pem)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECCPrivateKey_Dpem(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "pem",
                "pem is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECCPublicKey(
    input: &crate::deps::aws_cryptography_primitives::types::EccPublicKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECCPublicKey_Dder(&input.r#der)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECCPublicKey_Dder(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "der",
                "der is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignInput(
    input: &crate::deps::aws_cryptography_primitives::types::EcdsaSignInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSASignInput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HECDSASignInput_DsigningKey(&input.r#signing_key)?;
    validate_aws_Pcryptography_Pprimitives_HECDSASignInput_Dmessage(&input.r#message)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignInput_for_AwsCryptographicPrimitives_ECDSASign(
    input: &crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::EcdsaSignInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSASignInput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HECDSASignInput_DsigningKey(&input.r#signing_key)?;
    validate_aws_Pcryptography_Pprimitives_HECDSASignInput_Dmessage(&input.r#message)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignInput_Dmessage(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "message",
                "message is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignInput_DsignatureAlgorithm(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature_algorithm",
                "signature_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignInput_DsigningKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signing_key",
                "signing_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSASignOutput_Dsignature(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignOutput_for_AwsCryptographicPrimitives_ECDSASign(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSASignOutput_Dsignature(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSASignOutput_Dsignature(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature",
                "signature is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput(
    input: &crate::deps::aws_cryptography_primitives::types::EcdsaVerifyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_DverificationKey(
        &input.r#verification_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_Dmessage(&input.r#message)?;
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_Dsignature(&input.r#signature)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_for_AwsCryptographicPrimitives_ECDSAVerify(
    input: &crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::EcdsaVerifyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_DverificationKey(
        &input.r#verification_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_Dmessage(&input.r#message)?;
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_Dsignature(&input.r#signature)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_Dmessage(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "message",
                "message is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_Dsignature(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature",
                "signature is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_DsignatureAlgorithm(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature_algorithm",
                "signature_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyInput_DverificationKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "verification_key",
                "verification_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyOutput(
    input: &::std::primitive::bool,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyOutput_Dsuccess(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyOutput_for_AwsCryptographicPrimitives_ECDSAVerify(
    input: &::std::primitive::bool,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HECDSAVerifyOutput_Dsuccess(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HECDSAVerifyOutput_Dsuccess(
    input: &::std::option::Option<::std::primitive::bool>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "success",
                "success is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairInput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairInput_DeccCurve(&input.r#ecc_curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairInput_for_AwsCryptographicPrimitives_GenerateECCKeyPair(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairInput_DeccCurve(&input.r#ecc_curve)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairInput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateEccKeyPairOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DeccCurve(&input.r#ecc_curve)?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DprivateKey(
        &input.r#private_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_for_AwsCryptographicPrimitives_GenerateECCKeyPair(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DeccCurve(&input.r#ecc_curve)?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DprivateKey(
        &input.r#private_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DprivateKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "private_key",
                "private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPrivateKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECCKeyPairOutput_DpublicKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPublicKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyInput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateEcdsaSignatureKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyInput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyInput_for_AwsCryptographicPrimitives_GenerateECDSASignatureKey(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyInput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyInput_DsignatureAlgorithm(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature_algorithm",
                "signature_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateEcdsaSignatureKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DverificationKey(
        &input.r#verification_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DsigningKey(
        &input.r#signing_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_for_AwsCryptographicPrimitives_GenerateECDSASignatureKey(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DsignatureAlgorithm(
        &input.r#signature_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DverificationKey(
        &input.r#verification_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DsigningKey(
        &input.r#signing_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DsignatureAlgorithm(
    input: &::std::option::Option<
        crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm,
    >,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signature_algorithm",
                "signature_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DsigningKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "signing_key",
                "signing_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateECDSASignatureKeyOutput_DverificationKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "verification_key",
                "verification_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesInput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateRandomBytesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesInput_Dlength(&input.r#length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesInput_for_AwsCryptographicPrimitives_GenerateRandomBytes(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_random_bytes::GenerateRandomBytesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesInput_Dlength(&input.r#length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesInput_Dlength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "length",
                "length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "length",
                "length failed to satisfy constraint: Member must be greater than or equal to 0",
            ),
        );
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesOutput_Ddata(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesOutput_for_AwsCryptographicPrimitives_GenerateRandomBytes(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesOutput_Ddata(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRandomBytesOutput_Ddata(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "data",
                "data is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairInput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairInput_DlengthBits(
        &input.r#length_bits,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairInput_for_AwsCryptographicPrimitives_GenerateRSAKeyPair(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairInput_DlengthBits(
        &input.r#length_bits,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairInput_DlengthBits(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "length_bits",
                "length_bits is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(81..=4096).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "length_bits",
        "length_bits failed to satisfy constraint: Member must be between 81 and 4096, inclusive",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput(
    input: &crate::deps::aws_cryptography_primitives::types::GenerateRsaKeyPairOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_DpublicKey(
        &input.r#public_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_DprivateKey(
        &input.r#private_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_for_AwsCryptographicPrimitives_GenerateRSAKeyPair(
    input: &crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_DpublicKey(
        &input.r#public_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_DprivateKey(
        &input.r#private_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_DprivateKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "private_key",
                "private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HRSAPrivateKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGenerateRSAKeyPairOutput_DpublicKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HRSAPublicKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput(
    input: &crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_DeccCurve(
        &input.r#ecc_curve,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_DprivateKey(
        &input.r#private_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_for_AwsCryptographicPrimitives_GetPublicKeyFromPrivateKey(
    input: &crate::deps::aws_cryptography_primitives::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_DeccCurve(
        &input.r#ecc_curve,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_DprivateKey(
        &input.r#private_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyInput_DprivateKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "private_key",
                "private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPrivateKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput(
    input: &crate::deps::aws_cryptography_primitives::types::GetPublicKeyFromPrivateKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DeccCurve(
        &input.r#ecc_curve,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DprivateKey(
        &input.r#private_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_for_AwsCryptographicPrimitives_GetPublicKeyFromPrivateKey(
    input: &crate::deps::aws_cryptography_primitives::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DeccCurve(
        &input.r#ecc_curve,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DprivateKey(
        &input.r#private_key,
    )?;
    validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DprivateKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "private_key",
                "private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPrivateKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetPublicKeyFromPrivateKeyOutput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthInput(
    input: &crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthInput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthInput_for_AwsCryptographicPrimitives_GetRSAKeyModulusLength(
    input: &crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthInput_DpublicKey(
        &input.r#public_key,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthInput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthOutput(
    input: &crate::deps::aws_cryptography_primitives::types::GetRsaKeyModulusLengthOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthOutput_Dlength(&input.r#length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthOutput_for_AwsCryptographicPrimitives_GetRSAKeyModulusLength(
    input: &crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthOutput_Dlength(&input.r#length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HGetRSAKeyModulusLengthOutput_Dlength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "length",
                "length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(81..).contains(input) {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "length",
                "length failed to satisfy constraint: Member must be greater than or equal to 81",
            ),
        );
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput(
    input: &crate::deps::aws_cryptography_primitives::types::HkdfExpandInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_Dprk(&input.r#prk)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_Dinfo(&input.r#info)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_DexpectedLength(
        &input.r#expected_length,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_for_AwsCryptographicPrimitives_HkdfExpand(
    input: &crate::deps::aws_cryptography_primitives::operation::hkdf_expand::HkdfExpandInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_Dprk(&input.r#prk)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_Dinfo(&input.r#info)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_DexpectedLength(
        &input.r#expected_length,
    )?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_DdigestAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest_algorithm",
                "digest_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_DexpectedLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "expected_length",
                "expected_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expected_length",
        "expected_length failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_Dinfo(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "info",
                "info is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandInput_Dprk(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "prk",
                "prk is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandOutput_for_AwsCryptographicPrimitives_HkdfExpand(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExpandOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExpandOutput_Dokm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "okm",
                "okm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput(
    input: &crate::deps::aws_cryptography_primitives::types::HkdfExtractInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_Dsalt(&input.r#salt)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_Dikm(&input.r#ikm)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_for_AwsCryptographicPrimitives_HkdfExtract(
    input: &crate::deps::aws_cryptography_primitives::operation::hkdf_extract::HkdfExtractInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_Dsalt(&input.r#salt)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_Dikm(&input.r#ikm)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_DdigestAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest_algorithm",
                "digest_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_Dikm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ikm",
                "ikm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractInput_Dsalt(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractOutput_Dprk(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractOutput_for_AwsCryptographicPrimitives_HkdfExtract(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfExtractOutput_Dprk(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfExtractOutput_Dprk(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "prk",
                "prk is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput(
    input: &crate::deps::aws_cryptography_primitives::types::HkdfInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_DdigestAlgorithm(&input.r#digest_algorithm)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dsalt(&input.r#salt)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dikm(&input.r#ikm)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dinfo(&input.r#info)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_DexpectedLength(&input.r#expected_length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput_for_AwsCryptographicPrimitives_Hkdf(
    input: &crate::deps::aws_cryptography_primitives::operation::hkdf::HkdfInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_DdigestAlgorithm(&input.r#digest_algorithm)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dsalt(&input.r#salt)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dikm(&input.r#ikm)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dinfo(&input.r#info)?;
    validate_aws_Pcryptography_Pprimitives_HHkdfInput_DexpectedLength(&input.r#expected_length)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput_DdigestAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest_algorithm",
                "digest_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput_DexpectedLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "expected_length",
                "expected_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expected_length",
        "expected_length failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dikm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ikm",
                "ikm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dinfo(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "info",
                "info is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfInput_Dsalt(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfOutput_for_AwsCryptographicPrimitives_Hkdf(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHkdfOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHkdfOutput_Dokm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "okm",
                "okm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacInput(
    input: &crate::deps::aws_cryptography_primitives::types::HMacInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHMacInput_DdigestAlgorithm(&input.r#digest_algorithm)?;
    validate_aws_Pcryptography_Pprimitives_HHMacInput_Dkey(&input.r#key)?;
    validate_aws_Pcryptography_Pprimitives_HHMacInput_Dmessage(&input.r#message)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacInput_for_AwsCryptographicPrimitives_HMac(
    input: &crate::deps::aws_cryptography_primitives::operation::h_mac::HMacInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHMacInput_DdigestAlgorithm(&input.r#digest_algorithm)?;
    validate_aws_Pcryptography_Pprimitives_HHMacInput_Dkey(&input.r#key)?;
    validate_aws_Pcryptography_Pprimitives_HHMacInput_Dmessage(&input.r#message)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacInput_DdigestAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest_algorithm",
                "digest_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacInput_Dkey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "key",
                "key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacInput_Dmessage(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "message",
                "message is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHMacOutput_Ddigest(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacOutput_for_AwsCryptographicPrimitives_HMac(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HHMacOutput_Ddigest(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HHMacOutput_Ddigest(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest",
                "digest is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput(
    input: &crate::deps::aws_cryptography_primitives::types::KdfCtrInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dikm(&input.r#ikm)?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_DexpectedLength(&input.r#expected_length)?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dpurpose(&input.r#purpose)?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dnonce(&input.r#nonce)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_for_AwsCryptographicPrimitives_KdfCounterMode(
    input: &crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::KdfCtrInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_DdigestAlgorithm(
        &input.r#digest_algorithm,
    )?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dikm(&input.r#ikm)?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_DexpectedLength(&input.r#expected_length)?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dpurpose(&input.r#purpose)?;
    validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dnonce(&input.r#nonce)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_DdigestAlgorithm(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "digest_algorithm",
                "digest_algorithm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_DexpectedLength(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "expected_length",
                "expected_length is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(0..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "expected_length",
        "expected_length failed to satisfy constraint: Member must be greater than or equal to 0",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dikm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ikm",
                "ikm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dnonce(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrInput_Dpurpose(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HKdfCtrOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrOutput_for_AwsCryptographicPrimitives_KdfCounterMode(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HKdfCtrOutput_Dokm(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HKdfCtrOutput_Dokm(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "okm",
                "okm is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HParsePublicKeyInput(
    input: &crate::deps::aws_cryptography_primitives::types::ParsePublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HParsePublicKeyInput_DpublicKey(&input.r#public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HParsePublicKeyInput_for_AwsCryptographicPrimitives_ParsePublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::parse_public_key::ParsePublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HParsePublicKeyInput_DpublicKey(&input.r#public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HParsePublicKeyInput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HParsePublicKeyOutput(
    input: &crate::deps::aws_cryptography_primitives::types::ParsePublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HParsePublicKeyOutput_DpublicKey(&input.r#public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HParsePublicKeyOutput_for_AwsCryptographicPrimitives_ParsePublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::parse_public_key::ParsePublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HParsePublicKeyOutput_DpublicKey(&input.r#public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HParsePublicKeyOutput_DpublicKey(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    validate_aws_Pcryptography_Pprimitives_HECCPublicKey(input)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptInput(
    input: &crate::deps::aws_cryptography_primitives::types::RsaDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_Dpadding(&input.r#padding)?;
    validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_DprivateKey(&input.r#private_key)?;
    validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_DcipherText(&input.r#cipher_text)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_for_AwsCryptographicPrimitives_RSADecrypt(
    input: &crate::deps::aws_cryptography_primitives::operation::rsa_decrypt::RsaDecryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_Dpadding(&input.r#padding)?;
    validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_DprivateKey(&input.r#private_key)?;
    validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_DcipherText(&input.r#cipher_text)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_DcipherText(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "cipher_text",
                "cipher_text is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_Dpadding(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "padding",
                "padding is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptInput_DprivateKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "private_key",
                "private_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSADecryptOutput_Dplaintext(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptOutput_for_AwsCryptographicPrimitives_RSADecrypt(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSADecryptOutput_Dplaintext(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSADecryptOutput_Dplaintext(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "plaintext",
                "plaintext is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput(
    input: &crate::deps::aws_cryptography_primitives::types::RsaEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_Dpadding(&input.r#padding)?;
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_DpublicKey(&input.r#public_key)?;
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_Dplaintext(&input.r#plaintext)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_for_AwsCryptographicPrimitives_RSAEncrypt(
    input: &crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_Dpadding(&input.r#padding)?;
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_DpublicKey(&input.r#public_key)?;
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_Dplaintext(&input.r#plaintext)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_Dpadding(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "padding",
                "padding is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_Dplaintext(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "plaintext",
                "plaintext is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptInput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptOutput(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptOutput_DcipherText(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptOutput_for_AwsCryptographicPrimitives_RSAEncrypt(
    input: &::aws_smithy_types::Blob,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSAEncryptOutput_DcipherText(&Some(input.clone()))?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAEncryptOutput_DcipherText(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "cipher_text",
                "cipher_text is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAPrivateKey(
    input: &crate::deps::aws_cryptography_primitives::types::RsaPrivateKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSAPrivateKey_DlengthBits(&input.r#length_bits)?;
    validate_aws_Pcryptography_Pprimitives_HRSAPrivateKey_Dpem(&input.r#pem)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAPrivateKey_DlengthBits(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "length_bits",
                "length_bits is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(81..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "length_bits",
        "length_bits failed to satisfy constraint: Member must be greater than or equal to 81",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAPrivateKey_Dpem(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "pem",
                "pem is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAPublicKey(
    input: &crate::deps::aws_cryptography_primitives::types::RsaPublicKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HRSAPublicKey_DlengthBits(&input.r#length_bits)?;
    validate_aws_Pcryptography_Pprimitives_HRSAPublicKey_Dpem(&input.r#pem)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAPublicKey_DlengthBits(
    input: &::std::option::Option<::std::primitive::i32>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "length_bits",
                "length_bits is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    if !(81..).contains(input) {
        return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::invalid_field(
        "length_bits",
        "length_bits failed to satisfy constraint: Member must be greater than or equal to 81",
    ));
    }
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HRSAPublicKey_Dpem(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "pem",
                "pem is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput(
    input: &crate::deps::aws_cryptography_primitives::types::ValidatePublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_DeccCurve(&input.r#ecc_curve)?;
    validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_DpublicKey(&input.r#public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_for_AwsCryptographicPrimitives_ValidatePublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_DeccCurve(&input.r#ecc_curve)?;
    validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_DpublicKey(&input.r#public_key)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_DeccCurve(
    input: &::std::option::Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "ecc_curve",
                "ecc_curve is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyInput_DpublicKey(
    input: &::std::option::Option<::aws_smithy_types::Blob>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "public_key",
                "public_key is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyOutput(
    input: &crate::deps::aws_cryptography_primitives::types::ValidatePublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyOutput_Dsuccess(&input.r#success)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyOutput_for_AwsCryptographicPrimitives_ValidatePublicKey(
    input: &crate::deps::aws_cryptography_primitives::operation::validate_public_key::ValidatePublicKeyOutput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyOutput_Dsuccess(&input.r#success)?;
    Ok(())
}
pub(crate) fn validate_aws_Pcryptography_Pprimitives_HValidatePublicKeyOutput_Dsuccess(
    input: &::std::option::Option<::std::primitive::bool>,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    if input.is_none() {
        return ::std::result::Result::Err(
            ::aws_smithy_types::error::operation::BuildError::missing_field(
                "success",
                "success is required but was not specified",
            ),
        );
    }
    if input.is_none() {
        return ::std::result::Result::Ok(());
    }
    let input = input.as_ref().unwrap();

    Ok(())
}
