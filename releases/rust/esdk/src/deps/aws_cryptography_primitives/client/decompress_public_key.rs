// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`DecompressPublicKey`](crate::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`compressed_public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder::compressed_public_key) / [`set_compressed_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder::set_compressed_public_key): (undocumented)<br>
    ///   - [`ecc_curve(impl Into<Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>>)`](crate::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder::ecc_curve) / [`set_ecc_curve(Option<crate::deps::aws_cryptography_primitives::types::EcdhCurveSpec>)`](crate::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder::set_ecc_curve): (undocumented)<br>
    /// - On success, responds with [`DecompressPublicKeyOutput`](crate::operation::decompress_public_key::DecompressPublicKeyOutput) with field(s):
    ///   - [`public_key(Option<crate::deps::aws_cryptography_primitives::types::EccPublicKey>)`](crate::operation::decompress_public_key::DecompressPublicKeyOutput::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<DecompressPublicKeyError>`](crate::operation::decompress_public_key::DecompressPublicKeyError)
    pub fn decompress_public_key(&self) -> crate::deps::aws_cryptography_primitives::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::decompress_public_key::builders::DecompressPublicKeyFluentBuilder::new(self.clone())
    }
}
