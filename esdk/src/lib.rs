// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
//! # AWS Encryption SDK for Rust
//!
//! A native Rust implementation of the AWS Encryption SDK, providing
//! client-side encryption using envelope encryption. Data is encrypted
//! with a unique data key, which is itself encrypted by one or more
//! wrapping keys (key providers / keyrings).
//!
//! The two primary operations are:
//! - [`encrypt()`](fn@crate::encrypt) — encrypts plaintext and serializes an encrypted message
//! - [`decrypt()`](fn@crate::decrypt) — deserializes an encrypted message and decrypts the plaintext
//!
//! The SDK supports both **framed** and **nonframed** message body formats,
//! as well as **V1** and **V2** message format versions (V2 adds key commitment).
//!
#![cfg_attr(docsrs, feature(doc_cfg))]
// Project lint policy. We opt in to several rustc/clippy lint groups plus
// individually-selected clippy::restriction lints. Lint definitions live at
// https://doc.rust-lang.org/rustc/lints/listing/index.html and
// https://rust-lang.github.io/rust-clippy/master/index.html.
#![warn(
    // --- Forward-compatibility (catch idioms that future editions will break) ---
    deprecated_in_future,
    keyword_idents,
    keyword_idents_2024,
    rust_2018_idioms,           // sub-lints like elided_lifetimes_in_paths still fire on edition 2024

    // --- Public API hygiene ---
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    unreachable_pub,

    // --- Bug prevention (constructs that are usually mistakes) ---
    let_underscore_drop,        // `let _ = guard;` drops the guard immediately
    noop_method_call,           // .clone()/.borrow() that does nothing
    trivial_casts,              // `x as T` where T is x's type
    trivial_numeric_casts,      // numeric `as` cast that is a no-op
    unit_bindings,              // `let x = ();` is almost always a bug

    // --- Cleanup / explicitness ---
    absolute_paths_not_starting_with_crate,
    explicit_outlives_requirements,
    impl_trait_redundant_captures,
    macro_use_extern_crate,
    non_ascii_idents,
    redundant_imports,
    redundant_lifetimes,
    single_use_lifetimes,
    unused_extern_crates,
    unused_lifetimes,
    unused_qualifications,

    // --- Safety review ---
    unsafe_op_in_unsafe_fn,     // edition 2024 makes this warn-by-default; listed for explicit project policy

    // --- Clippy lint groups ---
    clippy::style,
    clippy::pedantic,
    clippy::nursery,            // unstable; review surprise warnings on toolchain bumps
    clippy::cargo,

    // --- Selected clippy::restriction lints. We do NOT enable
    // clippy::restriction wholesale; most of its lints are situational. ---
    clippy::create_dir,                       // require create_dir_all
    clippy::exhaustive_enums,                 // public enums must be #[non_exhaustive]
    clippy::exhaustive_structs,               // public structs must be #[non_exhaustive]
    clippy::little_endian_bytes,              // big-endian preferred (matches spec wire format)
    clippy::missing_asserts_for_indexing,     // catch panic-prone indexing
    clippy::mixed_read_write_in_expression,   // avoid order-of-eval surprises
    clippy::partial_pub_fields,               // all-or-nothing field visibility
    clippy::redundant_type_annotations,
    clippy::renamed_function_params,          // trait impls keep parameter names
    clippy::rest_pat_in_fully_bound_structs,  // explicit `..` only when needed
    clippy::same_name_method,                 // avoid inherent vs trait shadowing
    clippy::string_add,                       // prefer push_str over `s + s`
    clippy::string_lit_chars_any,             // prefer matches! over .chars().any()
    clippy::tests_outside_test_module,
    clippy::unneeded_field_pattern,

    // Intentionally not enabled (with rationale):
    //   clippy::panic                  — MPL enums are #[non_exhaustive], forcing panic!()
    //   clippy::indexing_slicing       — too noisy in serialization code
    //   clippy::wildcard_enum_match_arm — see allow(...) below
    //   clippy::restriction            — opt in to selected lints individually
    //   non_exhaustive_omitted_patterns — unstable
    //   rust_2021_incompatible_closure_captures
    //   rust_2021_incompatible_or_patterns
    //   rust_2021_prefixes_incompatible_syntax
    //   rust_2021_prelude_collisions  — all four are silent on edition 2024 (the things
    //                                   they warn about are compile errors here, not lintable)
)]
// Project disagreements with the warn set above:
#![allow(clippy::wildcard_enum_match_arm)] // too noisy against #[non_exhaustive] MPL enums
#![allow(clippy::multiple_crate_versions)] // transitively unavoidable
#![allow(clippy::option_if_let_else)]      // disagree: `if let` is often clearer
#![allow(clippy::redundant_pub_crate)]     // false positive: conflicts with unreachable_pub
#![allow(clippy::too_many_lines)]          // disagree: arbitrary line-count threshold
#![allow(unused_crate_dependencies)]       // false positives across workspace dev-deps

mod error;
pub use error::*;
mod encrypt;
pub use encrypt::*;
mod decrypt;
pub use decrypt::*;
#[cfg(feature = "test_vectors")]
#[cfg_attr(docsrs, doc(cfg(feature = "test_vectors")))]
/// Cross-implementation test-vector tooling. Gated behind the `test_vectors` feature.
pub mod test_vectors;
mod types;
pub use types::*;
mod client;
pub use client::*;

pub(crate) mod key_derivation;
pub(crate) mod legacy_compat;
pub(crate) mod materials;
pub(crate) mod message;

/// Internal items exposed solely so integration tests under `tests/` can
/// construct and assert on exact byte outputs of non-public serialization
/// helpers. Not part of the public API. Do not use.
///
/// Everything in this module is exempt from semantic versioning and may be
/// changed, renamed, or removed at any time.
#[doc(hidden)]
pub mod __test_internals {
    pub use crate::message::body::body_aad::{body_aad, BodyAADContent};
}
