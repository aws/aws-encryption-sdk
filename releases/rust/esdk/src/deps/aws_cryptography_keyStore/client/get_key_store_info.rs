// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_keyStore::client::Client {
    /// Constructs a fluent builder for the [`GetKeyStoreInfo`](crate::operation::get_key_store_info::builders::GetKeyStoreInfoFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`GetKeyStoreInfoOutput`](crate::operation::get_key_store_info::GetKeyStoreInfoOutput) with field(s):
    ///   - [`grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::get_key_store_info::GetKeyStoreInfoOutput::grant_tokens): (undocumented)
    ///   - [`key_store_id(Option<::std::string::String>)`](crate::operation::get_key_store_info::GetKeyStoreInfoOutput::key_store_id): (undocumented)
    ///   - [`key_store_name(Option<::std::string::String>)`](crate::operation::get_key_store_info::GetKeyStoreInfoOutput::key_store_name): (undocumented)
    ///   - [`kms_configuration(Option<crate::deps::aws_cryptography_keyStore::types::KmsConfiguration>)`](crate::operation::get_key_store_info::GetKeyStoreInfoOutput::kms_configuration): (undocumented)
    ///   - [`logical_key_store_name(Option<::std::string::String>)`](crate::operation::get_key_store_info::GetKeyStoreInfoOutput::logical_key_store_name): (undocumented)
    /// - On failure, responds with [`SdkError<GetKeyStoreInfoError>`](crate::operation::get_key_store_info::GetKeyStoreInfoError)
    pub fn get_key_store_info(&self) -> crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::builders::GetKeyStoreInfoFluentBuilder{
        crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::builders::GetKeyStoreInfoFluentBuilder::new(self.clone())
    }
}
