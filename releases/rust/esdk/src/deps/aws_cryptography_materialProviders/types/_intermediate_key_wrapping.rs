// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct IntermediateKeyWrapping {
    #[allow(missing_docs)]
    pub key_encryption_key_kdf: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    #[allow(missing_docs)]
    pub mac_key_kdf: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    #[allow(missing_docs)]
    pub pdk_encrypt_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>,
}
impl IntermediateKeyWrapping {
    #[allow(missing_docs)]
    pub fn key_encryption_key_kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.key_encryption_key_kdf
    }
    #[allow(missing_docs)]
    pub fn mac_key_kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.mac_key_kdf
    }
    #[allow(missing_docs)]
    pub fn pdk_encrypt_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>
    {
        &self.pdk_encrypt_algorithm
    }
}
impl IntermediateKeyWrapping {
    /// Creates a new builder-style object to manufacture [`IntermediateKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::IntermediateKeyWrappingBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::IntermediateKeyWrappingBuilder::default()
    }
}

/// A builder for [`IntermediateKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct IntermediateKeyWrappingBuilder {
    pub(crate) key_encryption_key_kdf: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    pub(crate) mac_key_kdf: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    >,
    pub(crate) pdk_encrypt_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>,
}
impl IntermediateKeyWrappingBuilder {
    #[allow(missing_docs)]
    pub fn key_encryption_key_kdf(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.key_encryption_key_kdf = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key_encryption_key_kdf(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.key_encryption_key_kdf = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_key_encryption_key_kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.key_encryption_key_kdf
    }
    #[allow(missing_docs)]
    pub fn mac_key_kdf(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.mac_key_kdf = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_mac_key_kdf(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
        >,
    ) -> Self {
        self.mac_key_kdf = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_mac_key_kdf(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DerivationAlgorithm,
    > {
        &self.mac_key_kdf
    }
    #[allow(missing_docs)]
    pub fn pdk_encrypt_algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::Encrypt,
        >,
    ) -> Self {
        self.pdk_encrypt_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_pdk_encrypt_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::Encrypt,
        >,
    ) -> Self {
        self.pdk_encrypt_algorithm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_pdk_encrypt_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Encrypt>
    {
        &self.pdk_encrypt_algorithm
    }
    /// Consumes the builder and constructs a [`IntermediateKeyWrapping`](crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::IntermediateKeyWrapping {
                key_encryption_key_kdf: self.key_encryption_key_kdf,
                mac_key_kdf: self.mac_key_kdf,
                pdk_encrypt_algorithm: self.pdk_encrypt_algorithm,
            },
        )
    }
}
