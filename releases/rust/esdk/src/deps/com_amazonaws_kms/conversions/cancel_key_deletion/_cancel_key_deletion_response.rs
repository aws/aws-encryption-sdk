// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::cancel_key_deletion::CancelKeyDeletionOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CancelKeyDeletionResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CancelKeyDeletionResponse::CancelKeyDeletionResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CancelKeyDeletionResponse,
    >,
) -> aws_sdk_kms::operation::cancel_key_deletion::CancelKeyDeletionOutput {
    aws_sdk_kms::operation::cancel_key_deletion::CancelKeyDeletionOutput::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .build()
}
