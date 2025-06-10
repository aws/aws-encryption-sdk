// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`Hkdf`](crate::operation::hkdf::builders::HkdfFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`expected_length(impl Into<Option<::std::primitive::i32>>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::expected_length) / [`set_expected_length(Option<::std::primitive::i32>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::set_expected_length): (undocumented)<br>
    ///   - [`ikm(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::ikm) / [`set_ikm(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::set_ikm): (undocumented)<br>
    ///   - [`info(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::info) / [`set_info(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::set_info): (undocumented)<br>
    ///   - [`salt(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::salt) / [`set_salt(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf::builders::HkdfFluentBuilder::set_salt): (undocumented)<br>
    /// - On success, responds with [`HkdfOutput`](crate::operation::hkdf::HkdfOutput) with field(s):
    ///   - [`okm(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf::HkdfOutput::okm): (undocumented)
    /// - On failure, responds with [`SdkError<HkdfError>`](crate::operation::hkdf::HkdfError)
    pub fn hkdf(
        &self,
    ) -> crate::deps::aws_cryptography_primitives::operation::hkdf::builders::HkdfFluentBuilder
    {
        crate::deps::aws_cryptography_primitives::operation::hkdf::builders::HkdfFluentBuilder::new(
            self.clone(),
        )
    }
}
