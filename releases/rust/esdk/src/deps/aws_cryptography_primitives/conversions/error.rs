// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Wraps up an arbitrary Rust Error value as a Dafny Error
pub fn to_opaque_error(
    value: String,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error,
> {
    let error_msg = value.clone();
    let error_msg =
        ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
            &error_msg,
        );
    let error_obj: ::dafny_runtime::Object<::dafny_runtime::DynAny> =
        ::dafny_runtime::Object(Some(::dafny_runtime::Rc::new(
            ::dafny_runtime::UnsafeCell::new(value),
        )));
    ::dafny_runtime::Rc::new(
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::OpaqueWithText {
            obj: error_obj,
	    objMessage: error_msg
        },
    )
}

/// Wraps up an arbitrary Rust Error value as a Dafny Result<T, Error>.Failure
pub fn to_opaque_error_result<T: ::dafny_runtime::DafnyType>(
    value: String,
) -> ::dafny_runtime::Rc<
    crate::_Wrappers_Compile::Result<
        T,
        ::dafny_runtime::Rc<
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error,
        >,
    >,
> {
    ::dafny_runtime::Rc::new(crate::_Wrappers_Compile::Result::Failure {
        error: to_opaque_error(value),
    })
}
pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::types::error::Error,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error,
> {
    ::dafny_runtime::Rc::new(match value {
        crate::deps::aws_cryptography_primitives::types::error::Error::AwsCryptographicPrimitivesError { message } =>
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::AwsCryptographicPrimitivesError {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
    },
        crate::deps::aws_cryptography_primitives::types::error::Error::CollectionOfErrors { list, message } =>
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::CollectionOfErrors {
                message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&message),
                list: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&list, |e| to_dafny(e.clone()))
            },
        crate::deps::aws_cryptography_primitives::types::error::Error::ValidationError(inner) =>
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque {
                obj: {
                    let rc = ::dafny_runtime::Rc::new(inner) as ::dafny_runtime::Rc<::dafny_runtime::DynAny>;
                    // safety: `rc` is new, ensuring it has refcount 1 and is uniquely owned.
                    // we should use `dafny_runtime_conversions::rc_struct_to_dafny_class` once it
                    // accepts unsized types (https://github.com/dafny-lang/dafny/pull/5769)
                    unsafe { ::dafny_runtime::Object::from_rc(rc) }
                },
            },
            crate::deps::aws_cryptography_primitives::types::error::Error::Opaque { obj } =>
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque {
                obj: ::dafny_runtime::Object(obj.0)
            },
            crate::deps::aws_cryptography_primitives::types::error::Error::OpaqueWithText { obj, objMessage } =>
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object(obj.0),
                objMessage: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&objMessage),
            },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error,
    >,
) -> crate::deps::aws_cryptography_primitives::types::error::Error {
    match ::std::borrow::Borrow::borrow(&dafny_value) {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::AwsCryptographicPrimitivesError { message } =>
    crate::deps::aws_cryptography_primitives::types::error::Error::AwsCryptographicPrimitivesError {
        message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
    },
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::CollectionOfErrors { list, message } =>
            crate::deps::aws_cryptography_primitives::types::error::Error::CollectionOfErrors {
                message: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&message),
                list: ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(&list, |e| from_dafny(e.clone()))
            },
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque { obj } =>
            crate::deps::aws_cryptography_primitives::types::error::Error::Opaque {
                obj: obj.clone()
            },
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::Opaque { obj } =>
            {
                use ::std::any::Any;
                if ::dafny_runtime::is_object!(obj, crate::deps::aws_cryptography_primitives::types::error::ValidationError) {
                    let typed = ::dafny_runtime::cast_object!(obj.clone(), crate::deps::aws_cryptography_primitives::types::error::ValidationError);
                    crate::deps::aws_cryptography_primitives::types::error::Error::ValidationError(
                        // safety: dafny_class_to_struct will increment ValidationError's Rc
                        unsafe {
                            ::dafny_runtime::dafny_runtime_conversions::object::dafny_class_to_struct(typed)
                        }
                    )
                } else {
                    crate::deps::aws_cryptography_primitives::types::error::Error::Opaque {
                        obj: obj.clone()
                    }
                }
            },
            crate::r#software::amazon::cryptography::primitives::internaldafny::types::Error::OpaqueWithText { obj, objMessage } =>
            {
                use ::std::any::Any;
                if ::dafny_runtime::is_object!(obj, crate::deps::aws_cryptography_primitives::types::error::ValidationError) {
                    let typed = ::dafny_runtime::cast_object!(obj.clone(), crate::deps::aws_cryptography_primitives::types::error::ValidationError);
                    crate::deps::aws_cryptography_primitives::types::error::Error::ValidationError(
                        // safety: dafny_class_to_struct will increment ValidationError's Rc
                        unsafe {
                            ::dafny_runtime::dafny_runtime_conversions::object::dafny_class_to_struct(typed)
                        }
                    )
                } else {
                    crate::deps::aws_cryptography_primitives::types::error::Error::OpaqueWithText {
                        obj: obj.clone(),
                        objMessage: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(&objMessage),
                    }
                }
            },
    }
}
