// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput::CreateAwsKmsHierarchicalKeyringInput {
        branchKeyId: crate::standard_library_conversions::ostring_to_dafny(&value.branch_key_id),
 branchKeyIdSupplier: ::dafny_runtime::Rc::new(match &value.branch_key_id_supplier {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::branch_key_id_supplier::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 keyStore: crate::deps::aws_cryptography_keyStore::conversions::client::to_dafny(&value.key_store.clone().unwrap())
,
 ttlSeconds: value.ttl_seconds.clone().unwrap(),
 cache: ::dafny_runtime::Rc::new(match &value.cache {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::cache_type::to_dafny(&x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 partitionId: crate::standard_library_conversions::ostring_to_dafny(&value.partition_id),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
>>>{
    ::dafny_runtime::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::dafny_runtime::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
) -> crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput::CreateAwsKmsHierarchicalKeyringInput {..} =>
            crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput::builder()
                .set_branch_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.branchKeyId().clone()))
 .set_branch_key_id_supplier(match (*dafny_value.branchKeyIdSupplier()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::branch_key_id_supplier::from_dafny(value.clone())),
    _ => None,
}
)
 .set_key_store(Some( crate::deps::aws_cryptography_keyStore::conversions::client::from_dafny(dafny_value.keyStore().clone())
 ))
 .set_ttl_seconds(Some( dafny_value.ttlSeconds() .clone() ))
 .set_cache(match (*dafny_value.cache()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::aws_cryptography_materialProviders::conversions::cache_type::from_dafny(value.clone())),
    _ => None,
}
)
 .set_partition_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.partitionId().clone()))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CreateAwsKmsHierarchicalKeyringInput,
    >>>,
) -> ::std::option::Option<
    crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsHierarchicalKeyringInput,
> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
