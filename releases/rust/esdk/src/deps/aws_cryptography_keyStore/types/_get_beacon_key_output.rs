// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Outputs for getting a Beacon Key
pub struct GetBeaconKeyOutput {
    /// The materials for the Beacon Key.
    pub beacon_key_materials:
        ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials>,
}
impl GetBeaconKeyOutput {
    /// The materials for the Beacon Key.
    pub fn beacon_key_materials(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials>
    {
        &self.beacon_key_materials
    }
}
impl GetBeaconKeyOutput {
    /// Creates a new builder-style object to manufacture [`GetBeaconKeyOutput`](crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::GetBeaconKeyOutputBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::GetBeaconKeyOutputBuilder::default(
        )
    }
}

/// A builder for [`GetBeaconKeyOutput`](crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBeaconKeyOutputBuilder {
    pub(crate) beacon_key_materials:
        ::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials>,
}
impl GetBeaconKeyOutputBuilder {
    /// The materials for the Beacon Key.
    pub fn beacon_key_materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
        >,
    ) -> Self {
        self.beacon_key_materials = ::std::option::Option::Some(input.into());
        self
    }
    /// The materials for the Beacon Key.
    pub fn set_beacon_key_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
        >,
    ) -> Self {
        self.beacon_key_materials = input;
        self
    }
    /// The materials for the Beacon Key.
    pub fn get_beacon_key_materials(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials>
    {
        &self.beacon_key_materials
    }
    /// Consumes the builder and constructs a [`GetBeaconKeyOutput`](crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::GetBeaconKeyOutput {
                beacon_key_materials: self.beacon_key_materials,
            },
        )
    }
}
