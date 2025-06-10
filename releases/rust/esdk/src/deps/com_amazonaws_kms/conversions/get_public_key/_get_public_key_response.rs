// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::get_public_key::GetPublicKeyOutput
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyResponse,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyResponse::GetPublicKeyResponse {
        KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
 PublicKey: crate::standard_library_conversions::oblob_to_dafny(&value.public_key),
 CustomerMasterKeySpec: ::dafny_runtime::Rc::new(match &value.customer_master_key_spec {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::customer_master_key_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeySpec: ::dafny_runtime::Rc::new(match &value.key_spec {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeyUsage: ::dafny_runtime::Rc::new(match &value.key_usage {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_usage_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 EncryptionAlgorithms: ::dafny_runtime::Rc::new(match &value.encryption_algorithms {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::to_dafny(e.clone()),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 SigningAlgorithms: ::dafny_runtime::Rc::new(match &value.signing_algorithms {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::signing_algorithm_spec::to_dafny(e.clone()),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 KeyAgreementAlgorithms: ::dafny_runtime::Rc::new(match &value.key_agreement_algorithms {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::key_agreement_algorithm_spec::to_dafny(e.clone()),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyResponse,
    >,
) -> aws_sdk_kms::operation::get_public_key::GetPublicKeyOutput {
    aws_sdk_kms::operation::get_public_key::GetPublicKeyOutput::builder()
          .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
 .set_public_key(crate::standard_library_conversions::oblob_from_dafny(dafny_value.PublicKey().clone()))
 .set_customer_master_key_spec(match &**dafny_value.CustomerMasterKeySpec() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::customer_master_key_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_spec(match &**dafny_value.KeySpec() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_usage(match &**dafny_value.KeyUsage() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_usage_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_encryption_algorithms(match (*dafny_value.EncryptionAlgorithms()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::EncryptionAlgorithmSpec>| crate::deps::com_amazonaws_kms::conversions::encryption_algorithm_spec::from_dafny(e),
            )
        ),
    _ => None
}
)
 .set_signing_algorithms(match (*dafny_value.SigningAlgorithms()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::SigningAlgorithmSpec>| crate::deps::com_amazonaws_kms::conversions::signing_algorithm_spec::from_dafny(e),
            )
        ),
    _ => None
}
)
 .set_key_agreement_algorithms(match (*dafny_value.KeyAgreementAlgorithms()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::KeyAgreementAlgorithmSpec>| crate::deps::com_amazonaws_kms::conversions::key_agreement_algorithm_spec::from_dafny(e),
            )
        ),
    _ => None
}
)
          .build()
}
