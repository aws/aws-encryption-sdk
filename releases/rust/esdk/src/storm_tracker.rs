// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

#[allow(non_snake_case)]
pub mod internal_StormTrackingCMC {
    use crate::_StormTracker_Compile::CacheState::*;
    use crate::*;
    use std::sync::Mutex;
    use std::time::Duration;

    pub struct StormTrackingCMC {
        wrapped: Mutex<::dafny_runtime::Object<_StormTracker_Compile::StormTracker>>,
    }
    impl StormTrackingCMC {
        pub fn _allocate_object(
            cmc: ::dafny_runtime::Object<_StormTracker_Compile::StormTracker>,
        ) -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::Object::new(StormTrackingCMC {
                wrapped: Mutex::new(cmc),
            })
        }
    }

    impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for StormTrackingCMC {
        ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
    }

    impl ::dafny_runtime::UpcastObject<dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache>
    for StormTrackingCMC {
    ::dafny_runtime::UpcastObjectFn!(dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache);
}

    impl crate::software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache for StormTrackingCMC {
    fn r#_PutCacheEntry_k(&self, input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput>)
    -> std::rc::Rc<crate::_Wrappers_Compile::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>>
    {
        self.wrapped.lock().unwrap().as_mut().PutCacheEntry(input)
    }

    fn r#_UpdateUsageMetadata_k(&self, input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::UpdateUsageMetadataInput>)
    -> std::rc::Rc<crate::_Wrappers_Compile::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>>
    {
        self.wrapped.lock().unwrap().as_mut().UpdateUsageMetadata(input)
    }

    fn r#_GetCacheEntry_k(&self, input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput>)
    -> std::rc::Rc<crate::_Wrappers_Compile::Result<std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput>, std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>>
    {
        let max_in_flight = crate::Time::_default::CurrentRelativeTimeMilli() + unsafe { *(*self.wrapped.lock().unwrap()).as_ref().inFlightTTL.get() };
        let sleep_milli = unsafe { *(*self.wrapped.lock().unwrap()).as_ref().sleepMilli.get() };
        let sleep_time = Duration::from_millis(sleep_milli as u64);
        loop {
            let result = self.wrapped.lock().unwrap().as_mut().GetFromCache(input);
            match &*result {
                crate::_Wrappers_Compile::Result::Failure{error} => {return std::rc::Rc::new(crate::_Wrappers_Compile::Result::Failure{error : error.clone()});}
                crate::_Wrappers_Compile::Result::Success{value} => {
                    match &**value {
                        Full { data } => { return std::rc::Rc::new(crate::_Wrappers_Compile::Result::Success{value : data.clone()}); }
                        EmptyFetch {} => {
                            return std::rc::Rc::new(crate::_Wrappers_Compile::Result::Failure{error :
                                std::rc::Rc::new(crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error::EntryDoesNotExist { message:
                                    dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
                                        "Entry does not exist"
                                    )
                                },
                            )});
                        }
                        EmptyWait {} => {
                            if crate::Time::_default::CurrentRelativeTimeMilli() <= max_in_flight {
                                std::thread::sleep(sleep_time);
                            } else {
                                return std::rc::Rc::new(crate::_Wrappers_Compile::Result::Failure{error :
                                    std::rc::Rc::new(crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error::InFlightTTLExceeded { message:
                                        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
                                            "Storm cache inFlightTTL exceeded"
                                        )
                                    },
                                )});
                            }
                        }
                    }
                }
            }
        }
      }

    fn r#_DeleteCacheEntry_k(&self, input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput>) -> std::rc::Rc<crate::_Wrappers_Compile::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
        self.wrapped.lock().unwrap().as_mut().DeleteCacheEntry(input)
    }
  }
}
