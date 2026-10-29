# Changelog

All notable changes to this project will be documented in this file.

## 1.3.0 (2026-10-28)

### Features

- Encrypt now rejects plaintext larger than 128 MiB (134,217,728 bytes), returning an invalid-field error for `plaintext` (#896, #903)

### Maintenance

- bump aws-lc-rs from 1.15.4 to 1.18.0, aws-lc-sys from 0.39 to 0.46, and aws-lc-fips-sys from 0.13 to 0.14.1, and de-duplicate aws-lc-sys (#927, #930, #951, #961, #969)

## 1.2.4 (2026-04-01)

### Maintenance

- bump aws-lc-sys to 0.39 to mitigate false positive code scanners
- run clippy on CI and allow large error for Clippy

## 1.2.3 (2026-03-10)

### Maintenance

- bump aws-lc-sys to 0.38 to mitigate false positive code scanners

## 0.2.0 (2024-12-18)

### Fixes

- Support input validation

### Maintenance

- Add version branch key example

## 0.1.0 (2024-12-09)

Initial launch of the AWS Encryption SDK for Rust.
