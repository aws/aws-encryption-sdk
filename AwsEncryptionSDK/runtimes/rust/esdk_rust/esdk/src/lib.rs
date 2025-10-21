//! This is an awesome crate
//!

#![warn(
    absolute_paths_not_starting_with_crate,
    deprecated_in_future,
    explicit_outlives_requirements,
    keyword_idents,
    impl_trait_redundant_captures,
    keyword_idents_2024,
    let_underscore_drop,
    macro_use_extern_crate,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    non_ascii_idents,
    // non_exhaustive_omitted_patterns, unstable
    noop_method_call,
    redundant_imports,
    redundant_lifetimes,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_numeric_casts,
    trivial_casts,
    unit_bindings,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_lifetimes,
    unused_qualifications,

    clippy::style,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,

    // clippy::restriction,
    clippy::create_dir,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::little_endian_bytes,
    clippy::missing_asserts_for_indexing,
    clippy::mixed_read_write_in_expression,
    // clippy::panic, because mpl enums are stupid
    clippy::partial_pub_fields,
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::string_add,
    clippy::string_lit_chars_any,
    clippy::tests_outside_test_module,
)]
#![allow(clippy::cargo_common_metadata)] // REMOVE
#![allow(clippy::multiple_crate_versions)] // nothing to be done
#![allow(clippy::option_if_let_else)] // disagree
#![allow(clippy::cast_possible_truncation)] // REMOVE
#![allow(clippy::missing_panics_doc)] // REMOVE
#![allow(clippy::cast_sign_loss)] // REMOVE
#![allow(clippy::redundant_pub_crate)] // broken, conflicts with unreachable_pub
#![allow(clippy::wildcard_imports)] // REMOVE
#![allow(clippy::missing_errors_doc)] // REMOVE
#![allow(clippy::too_many_lines)] // disagree
#![allow(unused_crate_dependencies)] // broken

// #[allow(lint_name, reason = "Your explanation here")]
// #[warn(clippy::allow_attributes_without_reason)] // REMOVE

mod client;
pub use client::*;
mod error;
pub use error::*;
mod esdk_operations;
#[cfg(feature = "test_vectors")]
mod test_vectors;
#[cfg(feature = "test_vectors")]
pub use test_vectors::*;
mod types;
pub use types::*;

pub(crate) mod alloc;
pub(crate) mod encrypt_decrypt;
pub(crate) mod key_derivation;
pub(crate) mod message_body;
pub(crate) mod serialize;

pub use error::Error;

// pub use aws_mpl_rs::aws_cryptography_keyStore as key_store;
