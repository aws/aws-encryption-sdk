// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::sign::SignOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::SignResponse,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::SignResponse::SignResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 Signature: crate::standard_library_conversions::oblob_to_dafny(&value.signature),
 SigningAlgorithm: ::dafny_runtime::Rc::new(match &value.signing_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::signing_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::SignResponse,
    >,
) -> aws_sdk_kms::operation::sign::SignOutput {
    aws_sdk_kms::operation::sign::SignOutput::builder()
        .set_key_id(crate::standard_library_conversions::ostring_from_dafny(
            dafny_value.KeyId().clone(),
        ))
        .set_signature(crate::standard_library_conversions::oblob_from_dafny(
            dafny_value.Signature().clone(),
        ))
        .set_signing_algorithm(match &**dafny_value.SigningAlgorithm() {
            crate::r#_Wrappers_Compile::Option::Some { value } => Some(
                crate::deps::com_amazonaws_kms::conversions::signing_algorithm_spec::from_dafny(
                    value,
                ),
            ),
            _ => None,
        })
        .build()
}
