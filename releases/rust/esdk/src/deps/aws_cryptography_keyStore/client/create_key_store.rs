// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_keyStore::client::Client {
    /// Constructs a fluent builder for the [`CreateKeyStore`](crate::operation::create_key_store::builders::CreateKeyStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`CreateKeyStoreOutput`](crate::operation::create_key_store::CreateKeyStoreOutput) with field(s):
    ///   - [`table_arn(Option<::std::string::String>)`](crate::operation::create_key_store::CreateKeyStoreOutput::table_arn): (undocumented)
    /// - On failure, responds with [`SdkError<CreateKeyStoreError>`](crate::operation::create_key_store::CreateKeyStoreError)
    pub fn create_key_store(&self) -> crate::deps::aws_cryptography_keyStore::operation::create_key_store::builders::CreateKeyStoreFluentBuilder{
        crate::deps::aws_cryptography_keyStore::operation::create_key_store::builders::CreateKeyStoreFluentBuilder::new(self.clone())
    }
}
