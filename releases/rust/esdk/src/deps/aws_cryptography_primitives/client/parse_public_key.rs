// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`ParsePublicKey`](crate::operation::parse_public_key::builders::ParsePublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::parse_public_key::builders::ParsePublicKeyFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::parse_public_key::builders::ParsePublicKeyFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`ParsePublicKeyOutput`](crate::operation::parse_public_key::ParsePublicKeyOutput) with field(s):
    ///   - [`public_key(Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>)`](crate::operation::parse_public_key::ParsePublicKeyOutput::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<ParsePublicKeyError>`](crate::operation::parse_public_key::ParsePublicKeyError)
    pub fn parse_public_key(&self) -> crate::deps::aws_cryptography_primitives::operation::parse_public_key::builders::ParsePublicKeyFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::parse_public_key::builders::ParsePublicKeyFluentBuilder::new(self.clone())
    }
}
