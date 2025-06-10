// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(missing_docs)]
pub enum TimeUnits {
    Seconds,
    Milliseconds,
}

impl ::std::fmt::Display for TimeUnits {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            TimeUnits::Seconds => write!(f, "Seconds"),
            TimeUnits::Milliseconds => write!(f, "Milliseconds"),
        }
    }
}
