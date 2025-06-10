// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`DeriveSharedSecret`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ecc_curve(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::ecc_curve) / [`set_ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_ecc_curve): (undocumented)<br>
    ///   - [`private_key(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::private_key) / [`set_private_key(Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_private_key): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::public_key) / [`set_public_key(Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>)`](crate::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`DeriveSharedSecretOutput`](crate::operation::derive_shared_secret::DeriveSharedSecretOutput) with field(s):
    ///   - [`shared_secret(Option<::aws_smithy_types::Blob>)`](crate::operation::derive_shared_secret::DeriveSharedSecretOutput::shared_secret): (undocumented)
    /// - On failure, responds with [`SdkError<DeriveSharedSecretError>`](crate::operation::derive_shared_secret::DeriveSharedSecretError)
    pub fn derive_shared_secret(&self) -> crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::derive_shared_secret::builders::DeriveSharedSecretFluentBuilder::new(self.clone())
    }
}
