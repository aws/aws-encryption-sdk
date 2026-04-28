# aws-mpl-primitives

The `aws-mpl-primitives` crate wraps the [`aws-lc-rs`] crate, providing a more convenient interface.

This crate is intended for internal use by [`aws-esdk`], [`aws-db-esdk`] and [`aws-mpl-rs`].

[`aws-lc-rs`]: https://docs.rs/aws-lc-rs/
[`aws-esdk`]: https://docs.rs/aws-esdk/
[`aws-db-esdk`]: https://docs.rs/aws-db-esdk/
[`aws-mpl-rs`]: https://docs.rs/aws-mpl-rs/

## Features

### fips

enable the `fips` feature in the [`aws-lc-rs`] crate

### track

Enable the `#[global_allocator]` for use with [`memory_tracker::ResourceTracker`]
