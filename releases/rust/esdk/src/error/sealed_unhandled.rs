// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use std::any::Any;

use dafny_runtime::UpcastObject;

/// This struct is not intended to be used.
///
/// This struct holds information about an unhandled error,
/// but that information should be obtained by using the
/// [`ProvideErrorMetadata`](::aws_smithy_types::error::metadata::ProvideErrorMetadata) trait
/// on the error type.
///
/// This struct intentionally doesn't yield any useful information itself.
#[deprecated(
    note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
variable wildcard pattern and check `.code()`:
 \
&nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
 \
See [`ProvideErrorMetadata`](::aws_smithy_types::error::metadata::ProvideErrorMetadata) for what information is available for the error."
)]
#[derive(Debug)]
pub struct Unhandled {
    pub(crate) source: ::aws_smithy_runtime_api::box_error::BoxError,
    pub(crate) meta: ::aws_smithy_types::error::metadata::ErrorMetadata,
}

impl UpcastObject<dyn Any> for Unhandled {
    ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
}
