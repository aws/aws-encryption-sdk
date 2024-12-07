// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use aws_smithy_types::error::operation::BuildError;

#[derive(::std::clone::Clone, ::std::fmt::Debug, ::std::cmp::PartialEq)]
#[allow(missing_docs)]
pub struct Client {
    pub(crate) dafny_client: ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::encryptionsdk::internaldafny::types::IAwsEncryptionSdkClient>
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    #[track_caller]
    pub fn from_conf(
        conf: crate::types::aws_encryption_sdk_config::AwsEncryptionSdkConfig,
    ) -> Result<Self, crate::types::error::Error> {
        let inner =
            crate::software::amazon::cryptography::encryptionsdk::internaldafny::_default::ESDK(
                &crate::conversions::aws_encryption_sdk_config::_aws_encryption_sdk_config::to_dafny(conf),
            );
        if matches!(
            inner.as_ref(),
            crate::_Wrappers_Compile::Result::Failure { .. }
        ) {
            return Err(crate::conversions::error::from_dafny(inner.as_ref().error().clone()));
        }
        Ok(Self {
            dafny_client: ::dafny_runtime::upcast_object()(inner.Extract())
        })
    }
}

mod encrypt;

mod decrypt;
