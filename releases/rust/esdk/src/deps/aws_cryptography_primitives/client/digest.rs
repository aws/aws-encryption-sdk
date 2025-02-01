// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`Digest`](crate::operation::digest::builders::DigestFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>>)`](crate::operation::digest::builders::DigestFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>)`](crate::operation::digest::builders::DigestFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::digest::builders::DigestFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::digest::builders::DigestFluentBuilder::set_message): (undocumented)<br>
    /// - On success, responds with [`DigestOutput`](crate::operation::digest::DigestOutput) with field(s):
    ///   - [`digest(Option<::aws_smithy_types::Blob>)`](crate::operation::digest::DigestOutput::digest): (undocumented)
    /// - On failure, responds with [`SdkError<DigestError>`](crate::operation::digest::DigestError)
    pub fn digest(
        &self,
    ) -> crate::deps::aws_cryptography_primitives::operation::digest::builders::DigestFluentBuilder
    {
        crate::deps::aws_cryptography_primitives::operation::digest::builders::DigestFluentBuilder::new(self.clone())
    }
}
