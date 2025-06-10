// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`HkdfExtract`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`digest_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::digest_algorithm) / [`set_digest_algorithm(Option<crate::deps::aws_cryptography_primitives::types::DigestAlgorithm>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::set_digest_algorithm): (undocumented)<br>
    ///   - [`ikm(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::ikm) / [`set_ikm(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::set_ikm): (undocumented)<br>
    ///   - [`salt(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::salt) / [`set_salt(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::set_salt): (undocumented)<br>
    /// - On success, responds with [`HkdfExtractOutput`](crate::operation::hkdf_extract::HkdfExtractOutput) with field(s):
    ///   - [`prk(Option<::aws_smithy_types::Blob>)`](crate::operation::hkdf_extract::HkdfExtractOutput::prk): (undocumented)
    /// - On failure, responds with [`SdkError<HkdfExtractError>`](crate::operation::hkdf_extract::HkdfExtractError)
    pub fn hkdf_extract(&self) -> crate::deps::aws_cryptography_primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::hkdf_extract::builders::HkdfExtractFluentBuilder::new(self.clone())
    }
}
