// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(non_snake_case)]

impl crate::implementation_from_dafny::OsLang::_default {
    pub fn GetLanguageShort() -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
            "Rust",
        )
    }
    pub fn GetLanguageLong() -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
            "Rust",
        )
    }
    pub fn GetOsShort() -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
        let os = match std::env::consts::OS {
            "macos" => "MacOS",
            "windows" => "Windows",
            "openbsd" => "Unix",
            "linux" => "Unix",
            "freebsd" => "Unix",
            "netbsd" => "Unix",
            _ => "Other",
        };
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(os)
    }
    pub fn GetOsLong() -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
        let os = format!("{} {}", std::env::consts::OS, std::env::consts::ARCH);
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&os)
    }
}
