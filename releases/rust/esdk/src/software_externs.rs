// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]
#![allow(non_snake_case)]

pub mod software {
    pub mod amazon {
        pub mod cryptography {
            pub mod internaldafny {
                pub mod StormTrackingCMC {
                    pub use crate::storm_tracker::internal_StormTrackingCMC::*;
                }
                pub mod SynchronizedLocalCMC {
                    pub use crate::local_cmc::internal_SynchronizedLocalCMC::*;
                }
            }
        }
    }
}
