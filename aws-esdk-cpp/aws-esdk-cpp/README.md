# aws-esdk-cpp

A C++ shim over the Rust implementation of the AWS Encryption SDK.

This crate exposes the Rust AWS Encryption SDK (`aws-esdk`) to C++ callers through
an idiomatic, generator-agnostic C++ API. Consumers program against
`Aws::Esdk::` types and `#include <aws/esdk/EncryptionSDK.h>`; the underlying
Rust bindings (generated with `cxx`) are an implementation detail hidden behind
the facade.

The shim's own behavioral contract — type translation, resource lifetimes, error
propagation, delegation to the core library — is specified in
[`spec/shim/shim.md`](spec/shim/shim.md) (generic) and
[`spec/shim/esdk-shim.md`](spec/shim/esdk-shim.md) (ESDK-specific), and tracked
with Duvet annotations in the source.

## Status

Work in progress.
