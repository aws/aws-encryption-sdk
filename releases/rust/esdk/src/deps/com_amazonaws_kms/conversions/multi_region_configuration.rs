// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::types::MultiRegionConfiguration,
) -> ::std::rc::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionConfiguration>{
  ::std::rc::Rc::new(
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionConfiguration::MultiRegionConfiguration {
        MultiRegionKeyType: ::std::rc::Rc::new(match &value.multi_region_key_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::multi_region_key_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 PrimaryKey: ::std::rc::Rc::new(match &value.primary_key {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::multi_region_key::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 ReplicaKeys: ::std::rc::Rc::new(match &value.replica_keys {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_kms::conversions::multi_region_key::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
} #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionConfiguration,
    >,
) -> aws_sdk_kms::types::MultiRegionConfiguration {
    aws_sdk_kms::types::MultiRegionConfiguration::builder()
          .set_multi_region_key_type(match &**dafny_value.MultiRegionKeyType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::multi_region_key_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_primary_key(match (*dafny_value.PrimaryKey()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_kms::conversions::multi_region_key::from_dafny(value.clone())),
    _ => None,
}
)
 .set_replica_keys(match (*dafny_value.ReplicaKeys()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::std::rc::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::MultiRegionKey>| crate::deps::com_amazonaws_kms::conversions::multi_region_key::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
          .build()

}
