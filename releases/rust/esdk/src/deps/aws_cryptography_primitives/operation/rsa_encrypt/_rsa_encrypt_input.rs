// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct RsaEncryptInput {
    #[allow(missing_docs)]
    pub padding:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>,
    #[allow(missing_docs)]
    pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaEncryptInput {
    #[allow(missing_docs)]
    pub fn padding(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>
    {
        &self.padding
    }
    #[allow(missing_docs)]
    pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext
    }
    #[allow(missing_docs)]
    pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
}
impl RsaEncryptInput {
    /// Creates a new builder-style object to manufacture [`RsaEncryptInput`](crate::operation::rsa_encrypt::builders::RsaEncryptInput).
    pub fn builder() -> crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::builders::RsaEncryptInputBuilder{
        crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::builders::RsaEncryptInputBuilder::default()
    }
}

/// A builder for [`RsaEncryptInput`](crate::operation::operation::RsaEncryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RsaEncryptInputBuilder {
    pub(crate) padding:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>,
    pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RsaEncryptInputBuilder {
    #[allow(missing_docs)]
    pub fn padding(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
        >,
    ) -> Self {
        self.padding = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_padding(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::RsaPaddingMode,
        >,
    ) -> Self {
        self.padding = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_padding(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::RsaPaddingMode>
    {
        &self.padding
    }
    #[allow(missing_docs)]
    pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.plaintext = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.plaintext = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.plaintext
    }
    #[allow(missing_docs)]
    pub fn public_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_public_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.public_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.public_key
    }
    /// Consumes the builder and constructs a [`RsaEncryptInput`](crate::operation::operation::RsaEncryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::operation::rsa_encrypt::RsaEncryptInput {
                padding: self.padding,
                plaintext: self.plaintext,
                public_key: self.public_key,
            },
        )
    }
}
