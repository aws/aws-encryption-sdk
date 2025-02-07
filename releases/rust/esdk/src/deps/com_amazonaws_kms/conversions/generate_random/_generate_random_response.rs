// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::generate_random::GenerateRandomOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomResponse::GenerateRandomResponse {
        Plaintext: crate::standard_library_conversions::oblob_to_dafny(&value.plaintext),
 CiphertextForRecipient: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_for_recipient),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomResponse,
    >,
) -> aws_sdk_kms::operation::generate_random::GenerateRandomOutput {
    aws_sdk_kms::operation::generate_random::GenerateRandomOutput::builder()
        .set_plaintext(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.Plaintext().clone(),
        ))
        .set_ciphertext_for_recipient(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.CiphertextForRecipient().clone(),
        ))
        .build()
}
