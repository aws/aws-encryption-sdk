// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct HkdfExtractInput {
    #[allow(missing_docs)]
    pub digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    #[allow(missing_docs)]
    pub ikm: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub salt: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExtractInput {
    #[allow(missing_docs)]
    pub fn digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.digest_algorithm
    }
    #[allow(missing_docs)]
    pub fn ikm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.ikm
    }
    #[allow(missing_docs)]
    pub fn salt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.salt
    }
}
impl HkdfExtractInput {
    /// Creates a new builder-style object to manufacture [`HkdfExtractInput`](crate::deps::aws_cryptography_primitives::types::HkdfExtractInput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_primitives::types::builders::HkdfExtractInputBuilder {
        crate::deps::aws_cryptography_primitives::types::builders::HkdfExtractInputBuilder::default(
        )
    }
}

/// A builder for [`HkdfExtractInput`](crate::deps::aws_cryptography_primitives::types::HkdfExtractInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct HkdfExtractInputBuilder {
    pub(crate) digest_algorithm:
        ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>,
    pub(crate) ikm: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) salt: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl HkdfExtractInputBuilder {
    #[allow(missing_docs)]
    pub fn digest_algorithm(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.digest_algorithm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_digest_algorithm(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_primitives::types::DigestAlgorithm,
        >,
    ) -> Self {
        self.digest_algorithm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_digest_algorithm(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>
    {
        &self.digest_algorithm
    }
    #[allow(missing_docs)]
    pub fn ikm(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.ikm = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_ikm(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.ikm = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_ikm(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.ikm
    }
    #[allow(missing_docs)]
    pub fn salt(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
        self.salt = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_salt(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.salt = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_salt(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.salt
    }
    /// Consumes the builder and constructs a [`HkdfExtractInput`](crate::deps::aws_cryptography_primitives::types::HkdfExtractInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_primitives::types::HkdfExtractInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_primitives::types::HkdfExtractInput {
                digest_algorithm: self.digest_algorithm,
                ikm: self.ikm,
                salt: self.salt,
            },
        )
    }
}
