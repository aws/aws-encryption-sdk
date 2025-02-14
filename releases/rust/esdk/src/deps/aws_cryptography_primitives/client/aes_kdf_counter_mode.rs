// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`AesKdfCounterMode`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`expected_length(impl Into<Option<::std::primitive::i32>>)`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::expected_length) / [`set_expected_length(Option<::std::primitive::i32>)`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::set_expected_length): (undocumented)<br>
    ///   - [`ikm(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::ikm) / [`set_ikm(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::set_ikm): (undocumented)<br>
    ///   - [`nonce(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::nonce) / [`set_nonce(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::set_nonce): (undocumented)<br>
    /// - On success, responds with [`AesKdfCtrOutput`](crate::operation::aes_kdf_counter_mode::AesKdfCtrOutput) with field(s):
    ///   - [`okm(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_kdf_counter_mode::AesKdfCtrOutput::okm): (undocumented)
    /// - On failure, responds with [`SdkError<AesKdfCounterModeError>`](crate::operation::aes_kdf_counter_mode::AesKdfCounterModeError)
    pub fn aes_kdf_counter_mode(&self) -> crate::deps::aws_cryptography_primitives::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::aes_kdf_counter_mode::builders::AesKdfCounterModeFluentBuilder::new(self.clone())
    }
}
