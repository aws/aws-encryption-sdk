// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

pub fn ostring_to_dafny(
    input: &Option<String>,
) -> ::std::rc::Rc<
    crate::_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>,
> {
    let dafny_value = match input {
    Some(b) => crate::_Wrappers_Compile::Option::Some { value:
        dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&b)
        },
    None => crate::_Wrappers_Compile::Option::None {},
};
    ::std::rc::Rc::new(dafny_value)
}

pub fn ostring_from_dafny(
    input: ::std::rc::Rc<
        crate::_Wrappers_Compile::Option<
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        >,
    >,
) -> Option<String> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                &input.Extract(),
            ),
        )
    } else {
        None
    }
}

pub fn obool_to_dafny(
    input: &Option<bool>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<bool>> {
    let dafny_value = match input {
        Some(b) => crate::_Wrappers_Compile::Option::Some { value: *b },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn obool_from_dafny(
    input: ::std::rc::Rc<crate::_Wrappers_Compile::Option<bool>>,
) -> Option<bool> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(input.Extract())
    } else {
        None
    }
}

pub fn oint_to_dafny(input: Option<i32>) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<i32>> {
    let dafny_value = match input {
        Some(b) => crate::_Wrappers_Compile::Option::Some { value: b },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn oint_from_dafny(input: ::std::rc::Rc<crate::_Wrappers_Compile::Option<i32>>) -> Option<i32> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(input.Extract())
    } else {
        None
    }
}

pub fn olong_to_dafny(input: &Option<i64>) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<i64>> {
    let dafny_value = match input {
        Some(b) => crate::_Wrappers_Compile::Option::Some { value: *b },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn olong_from_dafny(
    input: ::std::rc::Rc<crate::_Wrappers_Compile::Option<i64>>,
) -> Option<i64> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(input.Extract())
    } else {
        None
    }
}

pub fn blob_to_dafny(input: &::aws_smithy_types::Blob) -> ::dafny_runtime::Sequence<u8> {
    ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&input.clone().into_inner(), |x| *x)
}

pub fn oblob_to_dafny(
    input: &Option<::aws_smithy_types::Blob>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Sequence<u8>>> {
    let dafny_value = match input {
        Some(b) => crate::_Wrappers_Compile::Option::Some {
            value: blob_to_dafny(&b),
        },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn blob_from_dafny(input: ::dafny_runtime::Sequence<u8>) -> ::aws_smithy_types::Blob {
    ::aws_smithy_types::Blob::new(
        ::std::rc::Rc::try_unwrap(input.to_array()).unwrap_or_else(|rc| (*rc).clone()),
    )
}

pub fn oblob_from_dafny(
    input: ::std::rc::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Sequence<u8>>>,
) -> Option<::aws_smithy_types::Blob> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(blob_from_dafny(input.Extract()))
    } else {
        None
    }
}

pub fn double_to_dafny(input: f64) -> ::dafny_runtime::Sequence<u8> {
    ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(
        &f64::to_be_bytes(input).to_vec(),
        |x| *x,
    )
}

pub fn odouble_to_dafny(
    input: &Option<f64>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Sequence<u8>>> {
    let dafny_value = match input {
        Some(f) => crate::_Wrappers_Compile::Option::Some {
            value: double_to_dafny(*f),
        },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn double_from_dafny(input: &::dafny_runtime::Sequence<u8>) -> f64 {
    let v = ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(input, |x| *x);
    f64::from_be_bytes(v.try_into().expect("Error converting Sequence to f64"))
}

pub fn odouble_from_dafny(
    input: ::std::rc::Rc<crate::_Wrappers_Compile::Option<::dafny_runtime::Sequence<u8>>>,
) -> Option<f64> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(double_from_dafny(&input.Extract()))
    } else {
        None
    }
}

pub fn timestamp_to_dafny(
    input: &::aws_smithy_types::DateTime,
) -> ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16> {
    ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
        &input.to_string(),
    )
}

pub fn otimestamp_to_dafny(
    input: &Option<::aws_smithy_types::DateTime>,
) -> ::std::rc::Rc<
    crate::_Wrappers_Compile::Option<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>,
> {
    let dafny_value = match input {
        Some(f) => crate::_Wrappers_Compile::Option::Some {
            value: timestamp_to_dafny(f),
        },
        None => crate::_Wrappers_Compile::Option::None {},
    };
    ::std::rc::Rc::new(dafny_value)
}

pub fn timestamp_from_dafny(
    input: ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
) -> ::aws_smithy_types::DateTime {
    let s = dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
        &input,
    );
    ::aws_smithy_types::DateTime::from_str(&s, aws_smithy_types::date_time::Format::DateTime)
        .unwrap()
}

pub fn otimestamp_from_dafny(
    input: ::std::rc::Rc<
        crate::_Wrappers_Compile::Option<
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        >,
    >,
) -> Option<::aws_smithy_types::DateTime> {
    if matches!(
        input.as_ref(),
        crate::_Wrappers_Compile::Option::Some { .. }
    ) {
        Some(timestamp_from_dafny(input.Extract()))
    } else {
        None
    }
}

pub fn option_from_dafny<T: ::dafny_runtime::DafnyType, TR>(
    input: ::std::rc::Rc<crate::_Wrappers_Compile::Option<T>>,
    converter: fn(&T) -> TR,
) -> Option<TR> {
    match &*input {
        crate::_Wrappers_Compile::Option::Some { value } => Some(converter(value)),
        crate::_Wrappers_Compile::Option::None {} => None,
    }
}

pub fn option_to_dafny<T: ::dafny_runtime::DafnyType, TR>(
    input: &Option<TR>,
    converter: fn(&TR) -> T,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<T>> {
    match input {
        Some(value) => ::std::rc::Rc::new(crate::_Wrappers_Compile::Option::Some {
            value: converter(&value),
        }),
        None => ::std::rc::Rc::new(crate::_Wrappers_Compile::Option::None {}),
    }
}

pub fn result_from_dafny<T: ::dafny_runtime::DafnyType, TR, E: ::dafny_runtime::DafnyType, ER>(
    input: ::std::rc::Rc<crate::_Wrappers_Compile::Result<T, E>>,
    converter_t: fn(&T) -> TR,
    converter_e: fn(&E) -> ER,
) -> Result<TR, ER> {
    match &*input {
        crate::_Wrappers_Compile::Result::Success { value } => Ok(converter_t(value)),
        crate::_Wrappers_Compile::Result::Failure { error } => Err(converter_e(error)),
    }
}

pub fn result_to_dafny<T: ::dafny_runtime::DafnyType, TR, E: ::dafny_runtime::DafnyType, ER>(
    input: &Result<TR, ER>,
    converter_t: fn(&TR) -> T,
    converter_e: fn(&ER) -> E,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Result<T, E>> {
    match input {
        Ok(value) => ::std::rc::Rc::new(crate::_Wrappers_Compile::Result::Success {
            value: converter_t(&value),
        }),
        Err(error) => ::std::rc::Rc::new(crate::_Wrappers_Compile::Result::Failure {
            error: converter_e(&error),
        }),
    }
}
