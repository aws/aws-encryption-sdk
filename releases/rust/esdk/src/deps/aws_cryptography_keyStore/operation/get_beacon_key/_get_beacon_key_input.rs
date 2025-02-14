// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for getting a Beacon Key
pub struct GetBeaconKeyInput {
    /// The identifier of the Branch Key the Beacon Key is associated with.
    pub branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl GetBeaconKeyInput {
    /// The identifier of the Branch Key the Beacon Key is associated with.
    pub fn branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
}
impl GetBeaconKeyInput {
    /// Creates a new builder-style object to manufacture [`GetBeaconKeyInput`](crate::operation::get_beacon_key::builders::GetBeaconKeyInput).
    pub fn builder() -> crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::builders::GetBeaconKeyInputBuilder{
        crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::builders::GetBeaconKeyInputBuilder::default()
    }
}

/// A builder for [`GetBeaconKeyInput`](crate::operation::operation::GetBeaconKeyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBeaconKeyInputBuilder {
    pub(crate) branch_key_identifier: ::std::option::Option<::std::string::String>,
}
impl GetBeaconKeyInputBuilder {
    /// The identifier of the Branch Key the Beacon Key is associated with.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier of the Branch Key the Beacon Key is associated with.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_identifier = input;
        self
    }
    /// The identifier of the Branch Key the Beacon Key is associated with.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_identifier
    }
    /// Consumes the builder and constructs a [`GetBeaconKeyInput`](crate::operation::operation::GetBeaconKeyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyInput {
                branch_key_identifier: self.branch_key_identifier,
            },
        )
    }
}
