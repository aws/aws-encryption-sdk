// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::CacheType,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_materialProviders::types::CacheType::Default(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::Default {
        Default: crate::deps::aws_cryptography_materialProviders::conversions::default_cache::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::CacheType::No(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::No {
        No: crate::deps::aws_cryptography_materialProviders::conversions::no_cache::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::CacheType::SingleThreaded(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::SingleThreaded {
        SingleThreaded: crate::deps::aws_cryptography_materialProviders::conversions::single_threaded_cache::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::CacheType::MultiThreaded(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::MultiThreaded {
        MultiThreaded: crate::deps::aws_cryptography_materialProviders::conversions::multi_threaded_cache::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::CacheType::StormTracking(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::StormTracking {
        StormTracking: crate::deps::aws_cryptography_materialProviders::conversions::storm_tracking_cache::to_dafny(&x.clone())
,
    },
crate::deps::aws_cryptography_materialProviders::types::CacheType::Shared(x) =>
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::Shared {
        Shared: crate::deps::aws_cryptography_materialProviders::conversions::cryptographic_materials_cache::to_dafny(&x.clone())
,
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::CacheType {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::Default {
    Default: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CacheType::Default(crate::deps::aws_cryptography_materialProviders::conversions::default_cache::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::No {
    No: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CacheType::No(crate::deps::aws_cryptography_materialProviders::conversions::no_cache::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::SingleThreaded {
    SingleThreaded: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CacheType::SingleThreaded(crate::deps::aws_cryptography_materialProviders::conversions::single_threaded_cache::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::MultiThreaded {
    MultiThreaded: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CacheType::MultiThreaded(crate::deps::aws_cryptography_materialProviders::conversions::multi_threaded_cache::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::StormTracking {
    StormTracking: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CacheType::StormTracking(crate::deps::aws_cryptography_materialProviders::conversions::storm_tracking_cache::from_dafny(x.clone())
),
crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::CacheType::Shared {
    Shared: x @ _,
} => crate::deps::aws_cryptography_materialProviders::types::CacheType::Shared(crate::deps::aws_cryptography_materialProviders::conversions::cryptographic_materials_cache::from_dafny(x.clone())
),
    }
}
