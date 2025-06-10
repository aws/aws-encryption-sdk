// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct EncryptedDataKey {
    #[allow(missing_docs)]
    pub ciphertext: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub key_provider_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)]
    pub key_provider_info: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EncryptedDataKey {
    #[allow(missing_docs)]
    pub fn ciphertext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.ciphertext
    }
    #[allow(missing_docs)]
    pub fn key_provider_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_provider_id
    }
    #[allow(missing_docs)]
    pub fn key_provider_info(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.key_provider_info
    }
}
impl EncryptedDataKey {
    /// Creates a new builder-style object to manufacture [`EncryptedDataKey`](crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::EncryptedDataKeyBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::EncryptedDataKeyBuilder::default()
    }
}

/// A builder for [`EncryptedDataKey`](crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncryptedDataKeyBuilder {
    pub(crate) ciphertext: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) key_provider_id: ::std::option::Option<::std::string::String>,
    pub(crate) key_provider_info: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl EncryptedDataKeyBuilder {
    #[allow(missing_docs)]
    pub fn ciphertext(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.ciphertext = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_ciphertext(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.ciphertext = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_ciphertext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.ciphertext
    }
    #[allow(missing_docs)]
    pub fn key_provider_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.key_provider_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key_provider_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.key_provider_id = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_key_provider_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_provider_id
    }
    #[allow(missing_docs)]
    pub fn key_provider_info(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.key_provider_info = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_key_provider_info(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.key_provider_info = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_key_provider_info(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.key_provider_info
    }
    /// Consumes the builder and constructs a [`EncryptedDataKey`](crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::EncryptedDataKey {
                ciphertext: self.ciphertext,
                key_provider_id: self.key_provider_id,
                key_provider_info: self.key_provider_info,
            },
        )
    }
}
