// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache,
>{
    ::dafny_runtime::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache,
) -> crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache{
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache::StormTrackingCache {
        entryCapacity: value.entry_capacity.clone().unwrap(),
 entryPruningTailSize: crate::standard_library_conversions::oint_to_dafny(value.entry_pruning_tail_size),
 gracePeriod: value.grace_period.clone().unwrap(),
 graceInterval: value.grace_interval.clone().unwrap(),
 fanOut: value.fan_out.clone().unwrap(),
 inFlightTTL: value.in_flight_ttl.clone().unwrap(),
 sleepMilli: value.sleep_milli.clone().unwrap(),
 timeUnits: ::dafny_runtime::Rc::new(match &value.time_units {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::aws_cryptography_materialProviders::conversions::time_units::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache>,
) -> ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
  crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache,
>>>{
    ::dafny_runtime::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::dafny_runtime::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache,
    >,
) -> crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache,
) -> crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache {
    match dafny_value {
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache::StormTrackingCache {..} =>
            crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache::builder()
                .set_entry_capacity(Some( dafny_value.entryCapacity() .clone() ))
 .set_entry_pruning_tail_size(crate::standard_library_conversions::oint_from_dafny(dafny_value.entryPruningTailSize().clone()))
 .set_grace_period(Some( dafny_value.gracePeriod() .clone() ))
 .set_grace_interval(Some( dafny_value.graceInterval() .clone() ))
 .set_fan_out(Some( dafny_value.fanOut() .clone() ))
 .set_in_flight_ttl(Some( dafny_value.inFlightTTL() .clone() ))
 .set_sleep_milli(Some( dafny_value.sleepMilli() .clone() ))
 .set_time_units(match &**dafny_value.timeUnits() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::aws_cryptography_materialProviders::conversions::time_units::from_dafny(value)
    ),
    _ => None,
}
)
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::dafny_runtime::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::StormTrackingCache,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache>
{
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
