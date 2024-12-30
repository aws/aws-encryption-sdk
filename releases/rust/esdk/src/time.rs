// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use crate::*;
use std::convert::TryFrom;
use std::time::SystemTime;

impl crate::Time::_default {
    #[allow(non_snake_case)]
    pub fn CurrentRelativeTime() -> i64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs() as i64,
            Err(_) => 0,
        }
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn CurrentRelativeTimeMilli() -> i64 {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_millis() as i64,
            Err(_) => 0,
        }
    }

    #[allow(non_snake_case)]
    #[allow(dead_code)]
    pub fn GetProcessCpuTimeMillis() -> i64 {
        i64::try_from(cpu_time::ProcessTime::now().as_duration().as_millis())
            .expect("CPU millisecond didn't fit in an i64")
    }

    #[allow(non_snake_case)]
    pub fn GetCurrentTimeStamp() -> ::std::rc::Rc<
        _Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        >,
    > {
        let now_utc = chrono::Utc::now();
        let formatted = format!("{}", now_utc.format("%Y-%m-%dT%H:%M:%S%.6fZ"));
        ::std::rc::Rc::new(
                _Wrappers_Compile::Result::Success{value :
                dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&formatted)
                }
            )
    }
}
