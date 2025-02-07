// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct MaterialProvidersConfig {}
impl MaterialProvidersConfig {}
impl MaterialProvidersConfig {
    /// Creates a new builder-style object to manufacture [`MaterialProvidersConfig`](crate::deps::aws_cryptography_materialProviders::types::MaterialProvidersConfig).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfigBuilder{
        crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfigBuilder::default()
    }
}

/// A builder for [`MaterialProvidersConfig`](crate::deps::aws_cryptography_materialProviders::types::MaterialProvidersConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MaterialProvidersConfigBuilder {}
impl MaterialProvidersConfigBuilder {
    /// Consumes the builder and constructs a [`MaterialProvidersConfig`](crate::deps::aws_cryptography_materialProviders::types::MaterialProvidersConfig).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::material_providers_config::MaterialProvidersConfig {

        })
    }
}
