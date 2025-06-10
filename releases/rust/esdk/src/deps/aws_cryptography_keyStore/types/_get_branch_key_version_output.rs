// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for getting a version of a Branch Key.
pub struct GetBranchKeyVersionOutput {
    /// The materials for the Branch Key.
    pub branch_key_materials:
        ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials>,
}
impl GetBranchKeyVersionOutput {
    /// The materials for the Branch Key.
    pub fn branch_key_materials(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials>
    {
        &self.branch_key_materials
    }
}
impl GetBranchKeyVersionOutput {
    /// Creates a new builder-style object to manufacture [`GetBranchKeyVersionOutput`](crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::GetBranchKeyVersionOutputBuilder
    {
        crate::deps::aws_cryptography_keyStore::types::builders::GetBranchKeyVersionOutputBuilder::default()
    }
}

/// A builder for [`GetBranchKeyVersionOutput`](crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBranchKeyVersionOutputBuilder {
    pub(crate) branch_key_materials:
        ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials>,
}
impl GetBranchKeyVersionOutputBuilder {
    /// The materials for the Branch Key.
    pub fn branch_key_materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
        >,
    ) -> Self {
        self.branch_key_materials = ::std::option::Option::Some(input.into());
        self
    }
    /// The materials for the Branch Key.
    pub fn set_branch_key_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials,
        >,
    ) -> Self {
        self.branch_key_materials = input;
        self
    }
    /// The materials for the Branch Key.
    pub fn get_branch_key_materials(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials>
    {
        &self.branch_key_materials
    }
    /// Consumes the builder and constructs a [`GetBranchKeyVersionOutput`](crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::GetBranchKeyVersionOutput {
                branch_key_materials: self.branch_key_materials,
            },
        )
    }
}
