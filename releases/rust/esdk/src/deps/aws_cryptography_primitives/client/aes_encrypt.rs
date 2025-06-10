// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`AESEncrypt`](crate::operation::aes_encrypt::builders::AesEncryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aad(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::aad) / [`set_aad(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::set_aad): (undocumented)<br>
    ///   - [`enc_alg(impl Into<Option<crate::deps::aws_cryptography_primitives::types::AesGcm>>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::enc_alg) / [`set_enc_alg(Option<crate::deps::aws_cryptography_primitives::types::AesGcm>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::set_enc_alg): (undocumented)<br>
    ///   - [`iv(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::iv) / [`set_iv(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::set_iv): (undocumented)<br>
    ///   - [`key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::key) / [`set_key(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::set_key): (undocumented)<br>
    ///   - [`msg(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::msg) / [`set_msg(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_encrypt::builders::AESEncryptFluentBuilder::set_msg): (undocumented)<br>
    /// - On success, responds with [`AesEncryptOutput`](crate::operation::aes_encrypt::AesEncryptOutput) with field(s):
    ///   - [`auth_tag(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_encrypt::AESEncryptOutput::auth_tag): (undocumented)
    ///   - [`cipher_text(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_encrypt::AESEncryptOutput::cipher_text): (undocumented)
    /// - On failure, responds with [`SdkError<AesEncryptError>`](crate::operation::aes_encrypt::AesEncryptError)
    pub fn aes_encrypt(&self) -> crate::deps::aws_cryptography_primitives::operation::aes_encrypt::builders::AesEncryptFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::aes_encrypt::builders::AesEncryptFluentBuilder::new(self.clone())
    }
}
