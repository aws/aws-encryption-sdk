// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct OnDecryptInput {
    #[allow(missing_docs)]
    pub encrypted_data_keys: ::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
    #[allow(missing_docs)]
    pub materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl OnDecryptInput {
    #[allow(missing_docs)]
    pub fn encrypted_data_keys(
        &self,
    ) -> &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    > {
        &self.encrypted_data_keys
    }
    #[allow(missing_docs)]
    pub fn materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.materials
    }
}
impl OnDecryptInput {
    /// Creates a new builder-style object to manufacture [`OnDecryptInput`](crate::deps::aws_cryptography_materialProviders::types::OnDecryptInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::OnDecryptInputBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::OnDecryptInputBuilder::default()
    }
}

/// A builder for [`OnDecryptInput`](crate::deps::aws_cryptography_materialProviders::types::OnDecryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OnDecryptInputBuilder {
    pub(crate) encrypted_data_keys: ::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    >,
    pub(crate) materials: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    >,
}
impl OnDecryptInputBuilder {
    #[allow(missing_docs)]
    pub fn encrypted_data_keys(
        mut self,
        input: impl ::std::convert::Into<
            ::std::vec::Vec<
                crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
            >,
        >,
    ) -> Self {
        self.encrypted_data_keys = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encrypted_data_keys(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
            >,
        >,
    ) -> Self {
        self.encrypted_data_keys = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_encrypted_data_keys(
        &self,
    ) -> &::std::option::Option<
        ::std::vec::Vec<crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey>,
    > {
        &self.encrypted_data_keys
    }
    #[allow(missing_docs)]
    pub fn materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.materials = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
        >,
    ) -> Self {
        self.materials = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_materials(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DecryptionMaterials,
    > {
        &self.materials
    }
    /// Consumes the builder and constructs a [`OnDecryptInput`](crate::deps::aws_cryptography_materialProviders::types::OnDecryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::OnDecryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::OnDecryptInput {
                encrypted_data_keys: self.encrypted_data_keys,
                materials: self.materials,
            },
        )
    }
}
