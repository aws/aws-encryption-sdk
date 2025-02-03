// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::re_encrypt::ReEncryptOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptResponse,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptResponse::ReEncryptResponse {
        CiphertextBlob: crate::standard_library_conversions::oblob_to_dafny(&value.ciphertext_blob),
 SourceKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.source_key_id),
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 SourceEncryptionAlgorithm: ::dafny_runtime::Rc::new(match &value.source_encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DestinationEncryptionAlgorithm: ::dafny_runtime::Rc::new(match &value.destination_encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptResponse,
    >,
) -> aws_sdk_kms::operation::re_encrypt::ReEncryptOutput {
    aws_sdk_kms::operation::re_encrypt::ReEncryptOutput::builder()
          .set_ciphertext_blob(crate::standard_library_conversions::oblob_from_dafny(dafny_value.CiphertextBlob().clone()))
 .set_source_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.SourceKeyId().clone()))
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_source_encryption_algorithm(match &**dafny_value.SourceEncryptionAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_destination_encryption_algorithm(match &**dafny_value.DestinationEncryptionAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
          .build()
}
