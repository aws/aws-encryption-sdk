// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`AESDecrypt`](crate::operation::aes_decrypt::builders::AesDecryptFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aad(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::aad) / [`set_aad(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::set_aad): (undocumented)<br>
    ///   - [`auth_tag(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::auth_tag) / [`set_auth_tag(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::set_auth_tag): (undocumented)<br>
    ///   - [`cipher_txt(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::cipher_txt) / [`set_cipher_txt(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::set_cipher_txt): (undocumented)<br>
    ///   - [`enc_alg(impl Into<Option<crate::deps::aws_cryptography_primitives::types::AesGcm>>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::enc_alg) / [`set_enc_alg(Option<crate::deps::aws_cryptography_primitives::types::AesGcm>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::set_enc_alg): (undocumented)<br>
    ///   - [`iv(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::iv) / [`set_iv(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::set_iv): (undocumented)<br>
    ///   - [`key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::key) / [`set_key(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_decrypt::builders::AESDecryptFluentBuilder::set_key): (undocumented)<br>
    /// - On success, responds with [`AesDecryptOutput`](crate::operation::aes_decrypt::AesDecryptOutput) with field(s):
    ///   - [`plaintext(Option<::aws_smithy_types::Blob>)`](crate::operation::aes_decrypt::AESDecryptOutput::plaintext): (undocumented)
    /// - On failure, responds with [`SdkError<AesDecryptError>`](crate::operation::aes_decrypt::AesDecryptError)
    pub fn aes_decrypt(&self) -> crate::deps::aws_cryptography_primitives::operation::aes_decrypt::builders::AesDecryptFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::aes_decrypt::builders::AesDecryptFluentBuilder::new(self.clone())
    }
}
