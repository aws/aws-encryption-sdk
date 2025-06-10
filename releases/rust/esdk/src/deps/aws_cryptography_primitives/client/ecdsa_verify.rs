// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`ECDSAVerify`](crate::operation::ecdsa_verify::builders::EcdsaVerifyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::set_message): (undocumented)<br>
    ///   - [`signature(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::signature) / [`set_signature(Option<::aws_smithy_types::Blob>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::set_signature): (undocumented)<br>
    ///   - [`signature_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::signature_algorithm) / [`set_signature_algorithm(Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::set_signature_algorithm): (undocumented)<br>
    ///   - [`verification_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::verification_key) / [`set_verification_key(Option<::aws_smithy_types::Blob>)`](crate::operation::ecdsa_verify::builders::ECDSAVerifyFluentBuilder::set_verification_key): (undocumented)<br>
    /// - On success, responds with [`EcdsaVerifyOutput`](crate::operation::ecdsa_verify::EcdsaVerifyOutput) with field(s):
    ///   - [`success(Option<::std::primitive::bool>)`](crate::operation::ecdsa_verify::ECDSAVerifyOutput::success): (undocumented)
    /// - On failure, responds with [`SdkError<EcdsaVerifyError>`](crate::operation::ecdsa_verify::EcdsaVerifyError)
    pub fn ecdsa_verify(&self) -> crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::builders::EcdsaVerifyFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::ecdsa_verify::builders::EcdsaVerifyFluentBuilder::new(self.clone())
    }
}
