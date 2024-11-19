// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use dafny_runtime::Sequence;
use dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16;

impl crate::r#_TestWrappedESDKMain_Compile::_default {
    #[allow(non_snake_case)]
    pub fn GetTestVectorExecutionDirectory() -> Sequence<DafnyCharUTF16>
    {
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string("../../")
    }
}
