// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`GetPublicKeyFromPrivateKey`](crate::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ecc_curve(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder::ecc_curve) / [`set_ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder::set_ecc_curve): (undocumented)<br>
    ///   - [`private_key(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>>)`](crate::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder::private_key) / [`set_private_key(Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>)`](crate::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder::set_private_key): (undocumented)<br>
    /// - On success, responds with [`GetPublicKeyFromPrivateKeyOutput`](crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyOutput) with field(s):
    ///   - [`ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyOutput::ecc_curve): (undocumented)
    ///   - [`private_key(Option<crate::deps::aws_cryptography_primitives::types::EccPrivateKey>)`](crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyOutput::private_key): (undocumented)
    ///   - [`public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyOutput::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GetPublicKeyFromPrivateKeyError>`](crate::operation::get_public_key_from_private_key::GetPublicKeyFromPrivateKeyError)
    pub fn get_public_key_from_private_key(&self) -> crate::deps::aws_cryptography_primitives::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::get_public_key_from_private_key::builders::GetPublicKeyFromPrivateKeyFluentBuilder::new(self.clone())
    }
}
