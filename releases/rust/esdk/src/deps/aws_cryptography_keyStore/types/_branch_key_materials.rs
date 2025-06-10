// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct BranchKeyMaterials {
    #[allow(missing_docs)]
    pub branch_key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)]
    pub branch_key_version: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)]
    pub encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl BranchKeyMaterials {
    #[allow(missing_docs)]
    pub fn branch_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.branch_key
    }
    #[allow(missing_docs)]
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    #[allow(missing_docs)]
    pub fn branch_key_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_version
    }
    #[allow(missing_docs)]
    pub fn encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
}
impl BranchKeyMaterials {
    /// Creates a new builder-style object to manufacture [`BranchKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::BranchKeyMaterialsBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::BranchKeyMaterialsBuilder::default(
        )
    }
}

/// A builder for [`BranchKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BranchKeyMaterialsBuilder {
    pub(crate) branch_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) branch_key_version: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl BranchKeyMaterialsBuilder {
    #[allow(missing_docs)]
    pub fn branch_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.branch_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_branch_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.branch_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_branch_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.branch_key
    }
    #[allow(missing_docs)]
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    #[allow(missing_docs)]
    pub fn branch_key_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_version = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_branch_key_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_version = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_branch_key_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_version
    }
    #[allow(missing_docs)]
    pub fn encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.encryption_context = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
    /// Consumes the builder and constructs a [`BranchKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials {
                branch_key: self.branch_key,
                branch_key_identifier: self.branch_key_identifier,
                branch_key_version: self.branch_key_version,
                encryption_context: self.encryption_context,
            },
        )
    }
}
