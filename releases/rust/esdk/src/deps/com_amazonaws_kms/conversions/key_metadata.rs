// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::KeyMetadata,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyMetadata,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyMetadata::KeyMetadata {
        AWSAccountId: crate::standard_library_conversions::ostring_to_dafny(&value.aws_account_id),
 KeyId: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&value.key_id),
 Arn: crate::standard_library_conversions::ostring_to_dafny(&value.arn),
 CreationDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.creation_date),
 Enabled: crate::standard_library_conversions::obool_to_dafny(&Some(value.enabled)),
 Description: crate::standard_library_conversions::ostring_to_dafny(&value.description),
 KeyUsage: ::dafny_runtime::Rc::new(match &value.key_usage {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_usage_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeyState: ::dafny_runtime::Rc::new(match &value.key_state {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_state::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 DeletionDate: crate::standard_library_conversions::otimestamp_to_dafny(&value.deletion_date),
 ValidTo: crate::standard_library_conversions::otimestamp_to_dafny(&value.valid_to),
 Origin: ::dafny_runtime::Rc::new(match &value.origin {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::origin_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 CustomKeyStoreId: crate::standard_library_conversions::ostring_to_dafny(&value.custom_key_store_id),
 CloudHsmClusterId: crate::standard_library_conversions::ostring_to_dafny(&value.cloud_hsm_cluster_id),
 ExpirationModel: ::dafny_runtime::Rc::new(match &value.expiration_model {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::expiration_model_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeyManager: ::dafny_runtime::Rc::new(match &value.key_manager {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::key_manager_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
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
 MultiRegion: crate::standard_library_conversions::obool_to_dafny(&value.multi_region),
 MultiRegionConfiguration: ::dafny_runtime::Rc::new(match &value.multi_region_configuration {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::multi_region_configuration::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 PendingDeletionWindowInDays: crate::standard_library_conversions::oint_to_dafny(value.pending_deletion_window_in_days),
 MacAlgorithms: ::dafny_runtime::Rc::new(match &value.mac_algorithms {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::mac_algorithm_spec::to_dafny(e.clone()),
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 XksKeyConfiguration: ::dafny_runtime::Rc::new(match &value.xks_key_configuration {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::xks_key_configuration_type::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::KeyMetadata,
    >,
) -> aws_sdk_kms::types::KeyMetadata {
    aws_sdk_kms::types::KeyMetadata::builder()
          .set_aws_account_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.AWSAccountId().clone()))
 .set_key_id(Some( dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(dafny_value.KeyId()) ))
 .set_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Arn().clone()))
 .set_creation_date(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.CreationDate().clone()))
 .set_enabled(crate::standard_library_conversions::obool_from_dafny(dafny_value.Enabled().clone()))
 .set_description(crate::standard_library_conversions::ostring_from_dafny(dafny_value.Description().clone()))
 .set_key_usage(match &**dafny_value.KeyUsage() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_usage_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_state(match &**dafny_value.KeyState() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_state::from_dafny(value)
    ),
    _ => None,
}
)
 .set_deletion_date(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.DeletionDate().clone()))
 .set_valid_to(crate::standard_library_conversions::otimestamp_from_dafny(dafny_value.ValidTo().clone()))
 .set_origin(match &**dafny_value.Origin() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::origin_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_custom_key_store_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CustomKeyStoreId().clone()))
 .set_cloud_hsm_cluster_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.CloudHsmClusterId().clone()))
 .set_expiration_model(match &**dafny_value.ExpirationModel() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::expiration_model_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_manager(match &**dafny_value.KeyManager() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::key_manager_type::from_dafny(value)
    ),
    _ => None,
}
)
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
 .set_multi_region(crate::standard_library_conversions::obool_from_dafny(dafny_value.MultiRegion().clone()))
 .set_multi_region_configuration(match (*dafny_value.MultiRegionConfiguration()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::multi_region_configuration::from_dafny(value.clone())),
    _ => None,
}
)
 .set_pending_deletion_window_in_days(crate::standard_library_conversions::oint_from_dafny(dafny_value.PendingDeletionWindowInDays().clone()))
 .set_mac_algorithms(match (*dafny_value.MacAlgorithms()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::MacAlgorithmSpec>| crate::deps::com_amazonaws_kms::conversions::mac_algorithm_spec::from_dafny(e),
            )
        ),
    _ => None
}
)
 .set_xks_key_configuration(match (*dafny_value.XksKeyConfiguration()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::xks_key_configuration_type::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
          .unwrap()
}
