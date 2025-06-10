// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`KdfCounterMode`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`expected_length(impl Into<Option<::std::primitive::i32>>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::expected_length) / [`set_expected_length(Option<::std::primitive::i32>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::set_expected_length): (undocumented)<br>
    ///   - [`ikm(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::ikm) / [`set_ikm(Option<::aws_smithy_types::Blob>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::set_ikm): (undocumented)<br>
    ///   - [`nonce(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::nonce) / [`set_nonce(Option<::aws_smithy_types::Blob>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::set_nonce): (undocumented)<br>
    ///   - [`purpose(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::purpose) / [`set_purpose(Option<::aws_smithy_types::Blob>)`](crate::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::set_purpose): (undocumented)<br>
    /// - On success, responds with [`KdfCtrOutput`](crate::operation::kdf_counter_mode::KdfCtrOutput) with field(s):
    ///   - [`okm(Option<::aws_smithy_types::Blob>)`](crate::operation::kdf_counter_mode::KdfCtrOutput::okm): (undocumented)
    /// - On failure, responds with [`SdkError<KdfCounterModeError>`](crate::operation::kdf_counter_mode::KdfCounterModeError)
    pub fn kdf_counter_mode(&self) -> crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::kdf_counter_mode::builders::KdfCounterModeFluentBuilder::new(self.clone())
    }
}
