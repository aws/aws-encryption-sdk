// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::RecipientInfo,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RecipientInfo>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RecipientInfo::RecipientInfo {
        KeyEncryptionAlgorithm: ::std::rc::Rc::new(match &value.key_encryption_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_encryption_mechanism::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 AttestationDocument: crate::standard_library_conversions::oblob_to_dafny(&value.attestation_document),
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RecipientInfo,
    >,
) -> aws_sdk_kms::types::RecipientInfo {
    aws_sdk_kms::types::RecipientInfo::builder()
          .set_key_encryption_algorithm(match &**dafny_value.KeyEncryptionAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_encryption_mechanism::from_dafny(value)
    ),
    _ => None,
}
)
 .set_attestation_document(crate::standard_library_conversions::oblob_from_dafny(dafny_value.AttestationDocument().clone()))
          .build()

}
