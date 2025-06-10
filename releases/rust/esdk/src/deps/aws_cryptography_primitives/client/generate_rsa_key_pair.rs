// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`GenerateRSAKeyPair`](crate::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`length_bits(impl Into<Option<::std::primitive::i32>>)`](crate::operation::generate_rsa_key_pair::builders::GenerateRSAKeyPairFluentBuilder::length_bits) / [`set_length_bits(Option<::std::primitive::i32>)`](crate::operation::generate_rsa_key_pair::builders::GenerateRSAKeyPairFluentBuilder::set_length_bits): (undocumented)<br>
    /// - On success, responds with [`GenerateRsaKeyPairOutput`](crate::operation::generate_rsa_key_pair::GenerateRsaKeyPairOutput) with field(s):
    ///   - [`private_key(Option<crate::deps::aws_cryptography_primitives::types::RsaPrivateKey>)`](crate::operation::generate_rsa_key_pair::GenerateRSAKeyPairOutput::private_key): (undocumented)
    ///   - [`public_key(Option<crate::deps::aws_cryptography_primitives::types::RsaPublicKey>)`](crate::operation::generate_rsa_key_pair::GenerateRSAKeyPairOutput::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateRsaKeyPairError>`](crate::operation::generate_rsa_key_pair::GenerateRsaKeyPairError)
    pub fn generate_rsa_key_pair(&self) -> crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::generate_rsa_key_pair::builders::GenerateRsaKeyPairFluentBuilder::new(self.clone())
    }
}
