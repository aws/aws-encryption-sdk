# AWS Encryption SDK for .NET

AWS Encryption SDK for .NET

[Security issue notifications](./CONTRIBUTING.md#security-issue-notifications)

## Using the AWS Encryption SDK for .NET

The AWS Encryption SDK is available on [NuGet](https://www.nuget.org/) and can referenced from an existing `.csproj` through typical ways.

Using the dotnet CLI:

```shell
dotnet add <your-project-name>.csproj package AWS.Cryptography.EncryptionSDK
```

Alternatively, you may directly modify the `.csproj` and add the AWS Encryption SDK to `PackageReference` `ItemGroup`:

```xml
<PackageReference Include="AWS.Cryptography.EncryptionSDK" />
```

The AWS Encryption SDK targets [.NET](https://learn.microsoft.com/en-us/dotnet/fundamentals/implementations#net-5-and-later-versions) 6.0
and newer on all platforms, and [.NET Framework](https://docs.microsoft.com/en-us/dotnet/framework/) 4.8.0 and newer on Windows only.

### Additional setup for macOS only

If you are using macOS then you must install OpenSSL 1.1,
and the OpenSSL 1.1 `lib` directory must be on the dynamic linker path at runtime.
Also, if using an M1-based Mac, you must install OpenSSL and the .NET SDK for x86-64.
Please refer to [the wiki](https://github.com/aws/aws-encryption-sdk-dafny/wiki/Using-the-AWS-Encryption-SDK-for-.NET-on-macOS) for detailed instructions.

## Building the AWS Encryption SDK for .NET

To build, the AWS Encryption SDK requires the most up to date version of [Dafny](https://github.com/dafny-lang/dafny) on your PATH.

The AWS Encryption SDK targets frameworks [`net48` and `net6.0`](https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks).
To build and test the AWS Encryption SDK, you must install the following .NET tools:

- [.NET 6.0](https://dotnet.microsoft.com/en-us/download/dotnet/6.0) or newer
- [.NET Framework 4.8.0](https://docs.microsoft.com/en-us/dotnet/framework/install/) or newer (if on Windows)

You will also need to ensure that you fetch all submodules using either `git clone --recursive ...` when cloning the repository or `git submodule update --init` on an existing clone.

To build all source files into one dll:

```
# Transpile Dafny to .NET
cd AwsEncryptionSDK
make transpile_implementation_net
# Run dotnet restore
make setup_net
# Run dotnet build
dotnet build runtimes/net
```

### (Optional) Set up the AWS Encryption SDK to work with AWS KMS

If you set up the AWS Encryption SDK to use the AWS KMS Keyring,
the AWS Encryption SDK will make calls to AWS KMS on your behalf,
using the appropriate AWS SDK.

However, you must first set up AWS credentials for use with the AWS SDK.
Instructions for setting up AWS credentials are available in the [AWS Docs for the AWS SDK for .NET.](https://docs.aws.amazon.com/sdk-for-net/v3/developer-guide/net-dg-config-creds.html).

## Testing the AWS Encryption SDK for .NET

### Configure AWS credentials

To run the test suite you must first set up AWS credentials for use with the AWS SDK.
This is required in order to run the integration tests, which use a KMS Keyring against a publicly accessible KMS CMK.

Instructions for setting up AWS credentials are available in the [AWS Docs for the AWS SDK for .NET](https://docs.aws.amazon.com/sdk-for-net/v3/developer-guide/net-dg-config-creds.html).

### Run the tests

Run the test suite with:

```
cd AwsEncryptionSDK
make transpile_test_net
# Windows/Linux
make test_net
# On Mac
make test_net_mac_brew
```

Run tests on examples, to ensure they are up to date:

```
cd AwsEncryptionSDK/runtimes/net
dotnet test Examples
```

Please note that tests and test vectors require internet access and valid AWS credentials, since calls to KMS are made as part of the test workflow.

## Other Development advice

Most c# IDEs appreciate Solution files.
To generate one Solution file for all the projects here,
run:

```
cd AwsEncryptionSDK/runtimes/net
dotnet new sln --name ESDK
dotnet sln add $(find . -name '*.csproj')
```

Then ask your IDE to open `ESDK.sln`.

## License

This library is licensed under the Apache 2.0 License.
