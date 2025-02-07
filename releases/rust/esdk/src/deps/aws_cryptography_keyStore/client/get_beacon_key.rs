// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_keyStore::client::Client {
    /// Constructs a fluent builder for the [`GetBeaconKey`](crate::operation::get_beacon_key::builders::GetBeaconKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`branch_key_identifier(impl Into<Option<::std::string::String>>)`](crate::operation::get_beacon_key::builders::GetBeaconKeyFluentBuilder::branch_key_identifier) / [`set_branch_key_identifier(Option<::std::string::String>)`](crate::operation::get_beacon_key::builders::GetBeaconKeyFluentBuilder::set_branch_key_identifier): (undocumented)<br>
    /// - On success, responds with [`GetBeaconKeyOutput`](crate::operation::get_beacon_key::GetBeaconKeyOutput) with field(s):
    ///   - [`beacon_key_materials(Option<crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials>)`](crate::operation::get_beacon_key::GetBeaconKeyOutput::beacon_key_materials): (undocumented)
    /// - On failure, responds with [`SdkError<GetBeaconKeyError>`](crate::operation::get_beacon_key::GetBeaconKeyError)
    pub fn get_beacon_key(&self) -> crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::builders::GetBeaconKeyFluentBuilder{
        crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::builders::GetBeaconKeyFluentBuilder::new(self.clone())
    }
}
