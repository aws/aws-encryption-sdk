// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct BeaconKeyMaterials {
    #[allow(missing_docs)]
    pub beacon_key: ::std::option::Option<::aws_smithy_types::Blob>,
    #[allow(missing_docs)]
    pub beacon_key_identifier: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)]
    pub encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[allow(missing_docs)]
    pub hmac_keys: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
    >,
}
impl BeaconKeyMaterials {
    #[allow(missing_docs)]
    pub fn beacon_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.beacon_key
    }
    #[allow(missing_docs)]
    pub fn beacon_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.beacon_key_identifier
    }
    #[allow(missing_docs)]
    pub fn encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        &self.encryption_context
    }
    #[allow(missing_docs)]
    pub fn hmac_keys(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
    > {
        &self.hmac_keys
    }
}
impl BeaconKeyMaterials {
    /// Creates a new builder-style object to manufacture [`BeaconKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_keyStore::types::builders::BeaconKeyMaterialsBuilder {
        crate::deps::aws_cryptography_keyStore::types::builders::BeaconKeyMaterialsBuilder::default(
        )
    }
}

/// A builder for [`BeaconKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BeaconKeyMaterialsBuilder {
    pub(crate) beacon_key: ::std::option::Option<::aws_smithy_types::Blob>,
    pub(crate) beacon_key_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_context: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub(crate) hmac_keys: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
    >,
}
impl BeaconKeyMaterialsBuilder {
    #[allow(missing_docs)]
    pub fn beacon_key(
        mut self,
        input: impl ::std::convert::Into<::aws_smithy_types::Blob>,
    ) -> Self {
        self.beacon_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_beacon_key(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::Blob>,
    ) -> Self {
        self.beacon_key = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_beacon_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.beacon_key
    }
    #[allow(missing_docs)]
    pub fn beacon_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.beacon_key_identifier = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_beacon_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.beacon_key_identifier = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_beacon_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.beacon_key_identifier
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
    #[allow(missing_docs)]
    pub fn hmac_keys(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
        >,
    ) -> Self {
        self.hmac_keys = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_hmac_keys(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
        >,
    ) -> Self {
        self.hmac_keys = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_hmac_keys(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::aws_smithy_types::Blob>,
    > {
        &self.hmac_keys
    }
    /// Consumes the builder and constructs a [`BeaconKeyMaterials`](crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials {
                beacon_key: self.beacon_key,
                beacon_key_identifier: self.beacon_key_identifier,
                encryption_context: self.encryption_context,
                hmac_keys: self.hmac_keys,
            },
        )
    }
}
