// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`GenerateECCKeyPair`](crate::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ecc_curve(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::generate_ecc_key_pair::builders::GenerateECCKeyPairFluentBuilder::ecc_curve) / [`set_ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::generate_ecc_key_pair::builders::GenerateECCKeyPairFluentBuilder::set_ecc_curve): (undocumented)<br>
    /// - On success, responds with [`GenerateEccKeyPairOutput`](crate::operation::generate_ecc_key_pair::GenerateEccKeyPairOutput) with field(s):
    ///   - [`ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::generate_ecc_key_pair::GenerateECCKeyPairOutput::ecc_curve): (undocumented)
    ///   - [`private_key(Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>)`](crate::operation::generate_ecc_key_pair::GenerateECCKeyPairOutput::private_key): (undocumented)
    ///   - [`public_key(Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>)`](crate::operation::generate_ecc_key_pair::GenerateECCKeyPairOutput::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateEccKeyPairError>`](crate::operation::generate_ecc_key_pair::GenerateEccKeyPairError)
    pub fn generate_ecc_key_pair(&self) -> crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::generate_ecc_key_pair::builders::GenerateEccKeyPairFluentBuilder::new(self.clone())
    }
}
