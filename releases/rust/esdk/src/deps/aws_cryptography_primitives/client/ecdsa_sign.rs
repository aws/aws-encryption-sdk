// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`ECDSASign`](crate::operation::ecdsa_sign::builders::EcdsaSignFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`message(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::ecdsa_sign::builders::ECDSASignFluentBuilder::message) / [`set_message(Option<::aws_smithy_types::Blob>)`](crate::operation::ecdsa_sign::builders::ECDSASignFluentBuilder::set_message): (undocumented)<br>
    ///   - [`signature_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>>)`](crate::operation::ecdsa_sign::builders::ECDSASignFluentBuilder::signature_algorithm) / [`set_signature_algorithm(Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>)`](crate::operation::ecdsa_sign::builders::ECDSASignFluentBuilder::set_signature_algorithm): (undocumented)<br>
    ///   - [`signing_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::ecdsa_sign::builders::ECDSASignFluentBuilder::signing_key) / [`set_signing_key(Option<::aws_smithy_types::Blob>)`](crate::operation::ecdsa_sign::builders::ECDSASignFluentBuilder::set_signing_key): (undocumented)<br>
    /// - On success, responds with [`EcdsaSignOutput`](crate::operation::ecdsa_sign::EcdsaSignOutput) with field(s):
    ///   - [`signature(Option<::aws_smithy_types::Blob>)`](crate::operation::ecdsa_sign::ECDSASignOutput::signature): (undocumented)
    /// - On failure, responds with [`SdkError<EcdsaSignError>`](crate::operation::ecdsa_sign::EcdsaSignError)
    pub fn ecdsa_sign(&self) -> crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::builders::EcdsaSignFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::ecdsa_sign::builders::EcdsaSignFluentBuilder::new(self.clone())
    }
}
