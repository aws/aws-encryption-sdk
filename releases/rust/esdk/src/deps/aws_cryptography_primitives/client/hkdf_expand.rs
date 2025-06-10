// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`HkdfExpand`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`expected_length(impl Into<Option<::std::primitive::i32>>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::expected_length) / [`set_expected_length(Option<::std::primitive::i32>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::set_expected_length): (undocumented)<br>
    ///   - [`info(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::info) / [`set_info(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::set_info): (undocumented)<br>
    ///   - [`prk(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::prk) / [`set_prk(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::set_prk): (undocumented)<br>
    /// - On success, responds with [`HkdfExpandOutput`](crate::operation::hkdf_expand::HkdfExpandOutput) with field(s):
    ///   - [`okm(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_expand::HkdfExpandOutput::okm): (undocumented)
    /// - On failure, responds with [`SdkError<HkdfExpandError>`](crate::operation::hkdf_expand::HkdfExpandError)
    pub fn hkdf_expand(&self) -> crate::deps::aws_cryptography_primitives::operation::hkdf_expand::builders::HkdfExpandFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::hkdf_expand::builders::HkdfExpandFluentBuilder::new(self.clone())
    }
}
