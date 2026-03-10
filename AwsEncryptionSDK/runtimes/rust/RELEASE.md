To publish a new version of the aws-esdk for version N.N.N

1. Acquire the appropriate AWS permissions to run the tests
1. Ensure that `dafny --version` refers to the appropriate version for Rust
   1. This is most likely the version defined [here](https://github.com/aws/aws-encryption-sdk/blob/mainline/.github/workflows/library_rust_tests.yml#L59). Consult a Subject Matter Expert if you're not sure.
1. Ensure git checkout of main is fresh and clean
1. ./start_release.sh N.N.N
1. `cd ../../../releases/rust/esdk`
1. Create a PR with all changed or added files
1. Within the PR, make sure you also:
   1. Update the `CHANGELOG.md` inside AwsEncryptionSDK/runtimes/rust/ with the changes
   1. If this is a major version bump, update the `SUPPORT_POLICY.rst` for Rust
   1. Get the PR reviewed by a teammate
   1. Before merging the PR, publish the new version of the `aws-esdk` crate and test the published crate (documented in next steps)
1. Run `cargo publish`
1. `cd ../../../AwsEncryptionSDK/runtimes/rust/` # i.e. return here
1. ./test_published.sh N.N.N
1. Ignore/stash the changes in `AwsEncryptionSDK/runtimes/rust/test_examples/Cargo.toml` which adds the `aws-esdk` crate
1. Merge the release PR
