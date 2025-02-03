// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`GenerateECDSASignatureKey`](crate::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`signature_algorithm(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>>)`](crate::operation::generate_ecdsa_signature_key::builders::GenerateECDSASignatureKeyFluentBuilder::signature_algorithm) / [`set_signature_algorithm(Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>)`](crate::operation::generate_ecdsa_signature_key::builders::GenerateECDSASignatureKeyFluentBuilder::set_signature_algorithm): (undocumented)<br>
    /// - On success, responds with [`GenerateEcdsaSignatureKeyOutput`](crate::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyOutput) with field(s):
    ///   - [`signature_algorithm(Option<crate::deps::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm>)`](crate::operation::generate_ecdsa_signature_key::GenerateECDSASignatureKeyOutput::signature_algorithm): (undocumented)
    ///   - [`signing_key(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_ecdsa_signature_key::GenerateECDSASignatureKeyOutput::signing_key): (undocumented)
    ///   - [`verification_key(Option<::aws_smithy_types::Blob>)`](crate::operation::generate_ecdsa_signature_key::GenerateECDSASignatureKeyOutput::verification_key): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateEcdsaSignatureKeyError>`](crate::operation::generate_ecdsa_signature_key::GenerateEcdsaSignatureKeyError)
    pub fn generate_ecdsa_signature_key(&self) -> crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::generate_ecdsa_signature_key::builders::GenerateEcdsaSignatureKeyFluentBuilder::new(self.clone())
    }
}
