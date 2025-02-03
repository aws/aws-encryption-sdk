// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_primitives::client::Client {
    /// Constructs a fluent builder for the [`GetRSAKeyModulusLength`](crate::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`public_key(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::get_rsa_key_modulus_length::builders::GetRSAKeyModulusLengthFluentBuilder::public_key) / [`set_public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::get_rsa_key_modulus_length::builders::GetRSAKeyModulusLengthFluentBuilder::set_public_key): (undocumented)<br>
    /// - On success, responds with [`GetRsaKeyModulusLengthOutput`](crate::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthOutput) with field(s):
    ///   - [`length(Option<::std::primitive::i32>)`](crate::operation::get_rsa_key_modulus_length::GetRSAKeyModulusLengthOutput::length): (undocumented)
    /// - On failure, responds with [`SdkError<GetRsaKeyModulusLengthError>`](crate::operation::get_rsa_key_modulus_length::GetRsaKeyModulusLengthError)
    pub fn get_rsa_key_modulus_length(&self) -> crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthFluentBuilder{
        crate::deps::aws_cryptography_primitives::operation::get_rsa_key_modulus_length::builders::GetRsaKeyModulusLengthFluentBuilder::new(self.clone())
    }
}
