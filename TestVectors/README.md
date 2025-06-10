# AWS Encryption SDK Test Vectors

This project contains code encrypts and decrypts a suite unstructured data.
This validates the Encryption SDK's cross compatability between major versions
of the Encryption SDK and runtimes.

## Getting Started

### Development Requirements

- Dafny 4.9.0: https://github.com/dafny-lang/dafny

  The code that executes the test vectors is written in Dafny.
  You must install the Dafny runtime to compile the Dafny tests into Java.

- A .NET 6.0 TargetFramework or newer development environment

### Building and Running

#### For .NET

1. Start in the root `./TestVectors` directory
2. Run `make transpile_net`
3. Run `make test_net_mac_intel` if running on a MacOS environment or
   `make test_net` if running on a Windows or Linux environment.

#### For Rust

1. Start in the root `./TestVectors` directory
2. Run `make polymorph_rust`
3. Run `make transpile_rust`
4. Run `make test_rust`

Note: If you run into a stack overflow error while running `make test_rust`, run `export RUST_MIN_STACK=104857600`, and re-run `make test_rust`.

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This project is licensed under the Apache-2.0 License.
