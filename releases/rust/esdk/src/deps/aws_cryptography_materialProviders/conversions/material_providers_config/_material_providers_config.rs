// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::MaterialProvidersConfig,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::MaterialProvidersConfig,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig{
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::MaterialProvidersConfig{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::MaterialProvidersConfig::MaterialProvidersConfig {

    }
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::MaterialProvidersConfig,
) -> crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig{
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::MaterialProvidersConfig::MaterialProvidersConfig {..} =>
            crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig::builder()

                .build()
                .unwrap()
    }
}
