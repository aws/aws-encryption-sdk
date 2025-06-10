// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`ValidatePublicKey`](crate::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ecc_curve(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder::ecc_curve) / [`set_ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder::set_ecc_curve): (undocumented)<br>
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`ValidatePublicKeyOutput`](crate::operation::validate_public_key::ValidatePublicKeyOutput) with field(s):
    ///   - [`success(Option<::std::primitive::bool>)`](crate::operation::validate_public_key::ValidatePublicKeyOutput::success): (undocumented)
    /// - On failure, responds with [`SdkError<ValidatePublicKeyError>`](crate::operation::validate_public_key::ValidatePublicKeyError)
    pub fn validate_public_key(&self) -> crate::deps::aws_cryptography_primitives::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::validate_public_key::builders::ValidatePublicKeyFluentBuilder::new(self.clone())
    }
}
