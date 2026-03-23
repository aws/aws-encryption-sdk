# Changelog

## [5.0.0](https://github.com/aws/aws-encryption-sdk-dafny/compare/v4.1.0...v5.0.0) (2026-03-23)

### ⚠ BREAKING CHANGES

- This version now explicitly depends on AWS SDK V4, if you need to use AWS SDK V3 you MUST use a previous version of this library.

### Features

- Bump to MPL V2 ([#869](https://github.com/aws/aws-encryption-sdk-dafny/issues/869)) ([9ddae5c](https://github.com/aws/aws-encryption-sdk-dafny/commit/9ddae5cdf6a6668cbaf43e551958100533d46fae))

### Fixes

- **.NET:** Improve Collection of Errors string ([#728](https://github.com/aws/aws-encryption-sdk-dafny/issues/728)) ([d66248b](https://github.com/aws/aws-encryption-sdk-dafny/commit/d66248b956e5bce553fd64ba46189a5ac6eb69ee))

### Maintenance

- add C to list of languages checked for interoperability ([#773](https://github.com/aws/aws-encryption-sdk-dafny/issues/773)) ([3b9297a](https://github.com/aws/aws-encryption-sdk-dafny/commit/3b9297aa54dc984885e426236987d55197cffc9f))
- **TestVectors:** add DefaultCmm Tests ([#698](https://github.com/aws/aws-encryption-sdk-dafny/issues/698)) ([a287d45](https://github.com/aws/aws-encryption-sdk-dafny/commit/a287d45c25688dfc07b3b94587bc1f0f152b810e))

## 4.1.1

### Notes

- Update .csproj files to prevent use of AWS-SDK-NET V4

**Full Changelog**: <https://github.com/aws/aws-encryption-sdk/compare/v4.1.0...v4.1.1>

## 4.1.0

### Notes

- [(#646)](https://github.com/aws/aws-encryption-sdk-dafny/commit/10daadfa19db0e43fc0cc6d7b989f1fb477a22b0) Enforces input constraints.

Prior to this fix, the AWS Encryption SDK in .NET (ESDK-NET) failed to enforce user input constraints. Input shapes without required members set would always result in a `NullReferenceException`.
Now, the ESDK-NET will throw its own Exceptions when illegal user input
is submitted.

### Fixes

- fix: throw an exception when MemoryStream instance has an empty backing array [(#633)](https://github.com/aws/aws-encryption-sdk-dafny/commit/550c714743e84f93d09900b3338f59d0a54bb3ce)

### Features

- feat: enforce input constraints [(#646)](https://github.com/aws/aws-encryption-sdk-dafny/commit/10daadfa19db0e43fc0cc6d7b989f1fb477a22b0)

### Maintenance

- fix(CI): Daily CI uses correct workflow [(#641)](https://github.com/aws/aws-encryption-sdk-dafny/commit/771835e22f6ef3c3b34d0891fb61cb1a49bcf855)
- chore(ci): fix role to assume [(#622)](https://github.com/aws/aws-encryption-sdk-dafny/commit/c1f04fc41093593748f16da80d893c2ec5325545)
- chore(CI/CD): add semantic release automation [(#647)](https://github.com/aws/aws-encryption-sdk-dafny/commit/e7b5392ccc18f502a5517580a27bce5980e1913d)
- chore: Adopt SmithyDafnyMakefile.mk, fix nightly build [(#638)](https://github.com/aws/aws-encryption-sdk-dafny/commit/cd199795003d91984e24f1c04d9a84ae9c445372)
- chore(CI): add interop tests to daily ci [(#640)](https://github.com/aws/aws-encryption-sdk-dafny/commit/c9ad0181b544b258d66bf7b7e8d0b2be4cec7af9)
- chore: only run net48 on windows and use node 17 to run integration-node [(#639)](https://github.com/aws/aws-encryption-sdk-dafny/commit/d6c62fb68d974b47eb9d6cf9d8fbf249d6889b54)
- chore(.NET): Add ESDK-Net v4.0.1 generated vectors[(#636)](https://github.com/aws/aws-encryption-sdk-dafny/commit/efef49720c55a28cb422133385f8ece5ebc1da9c)
- chore(NET-SupportPolicy): Mark 3.x as Support [(#631)](https://github.com/aws/aws-encryption-sdk-dafny/commit/3c36f7a4a19646a8dfa6073be04676394502ef23)
- chore: Add manual trigger for nightly_dafny.yml [(#629)](https://github.com/aws/aws-encryption-sdk-dafny/commit/419b1cbfb4a5d85c03d0ad8c555a89108f199b98)
- chore: split vc gen on some methods to migrate to Dafny 4.4 [(#627)](https://github.com/aws/aws-encryption-sdk-dafny/commit/fdc65ca7495402b5b51017655015413eba846e7f)
- test: restore CODEOWNERS and daily CI [(#624)](https://github.com/aws/aws-encryption-sdk-dafny/commit/ff823ac918b822db548e703307d2ce462e79eef7)
- chore: update template to point to public repo [(#626)](https://github.com/aws/aws-encryption-sdk-dafny/commit/2b07a391208cb2a0508d1d915ae800e5de212d0e)
- chore: remove unused release step in test-prod [(#623)](https://github.com/aws/aws-encryption-sdk-dafny/commit/98839331a2e1154913d6ba4c88b0f4cba7322233)

## 4.0.1

### Fixes

The ESDK-NET’s Message Header AAD
incorrectly appended two empty bytes
when using the DefaultCMM.
The HKDF invocation of non-committing algorithm suites
failed to include the Message ID in the info parameter.

Neither of these issues
effect the security of messages
written by the 4.0.0 release.

However,
these messages diverge
from the Encryption SDK Message Specification.
Thus:

- ESDK-NET v4.0.0 writes messages that only ESDK-NET v4.0.0 and greater can read.
- ESDK-NET v4.0.0 is ONLY able to read messages that are written by ESDK-NET v4.0.0

These issues are fixed in 4.0.1,
which writes messages according to the Encryption SDK Message Specification,
and are interoperable with other implementations of this library.

The option NetV4_RetryPolicy can be use to decrypt v4.0.0 messages.
See [NetV4_0_0Example.cs](Examples/NetV4_0_0Example.cs) on how to use the NetV4_RetryPolicy
and details on distributed applications.

## 4.0.0

### BREAKING CHANGES

- AWS Encryption SDK for .NET now directly depends on the AWS Cryptographic Material Providers Library for .NET
- Required Encryption Context CMM generates messages that the Encryption SDK for .NET < 4.0.0 cannot read
- This feature does not yet exist in other Encryption SDKs, as such, messages written using this feature are not interoperable
  with other runtimes.
- AWS Encryption SDK now only supports .NET 6.0 and later, and .NET Framework 4.8.0 and later.

### Features

- Required Encryption Context CMM
- AWS KMS RSA Keyring
- AWS KMS Hierarchical Keyring

### NuGet Rename

_Added on October 16th, 2023_
As of version 4.0.0, the AWS Encryption SDK for .NET is on NuGet as [AWS.Cryptography.EncryptionSDK](https://www.nuget.org/packages/AWS.Cryptography.EncryptionSDK).

Prior versions are under [AWS.EncryptionSDK](https://www.nuget.org/packages/AWS.EncryptionSDK).

## 3.1.0

### Fixes

- chore: pack README for display on NuGet page (<https://github.com/aws/aws-encryption-sdk-dafny/pull/585>)
- fix: add DiscoveryFilter to MRK Discovery Keyring example (<https://github.com/aws/aws-encryption-sdk-dafny/pull/581>)
- docs: fix .NET ESDK link in README (<https://github.com/aws/aws-encryption-sdk-dafny/pull/589>)
- docs: fix .NET ESDK package name in README (<https://github.com/aws/aws-encryption-sdk-dafny/pull/600>)
- docs: link to macOS setup wiki in README (<https://github.com/aws/aws-encryption-sdk-dafny/pull/601>)

### Maintenance

- chore: update generated KMS code (<https://github.com/aws/aws-encryption-sdk-dafny/pull/580>)
- chore: use public spec URL for submodule (<https://github.com/aws/aws-encryption-sdk-dafny/pull/586>)
- fix: use renamed directories for Duvet report (<https://github.com/aws/aws-encryption-sdk-dafny/pull/587>)
- chore: bump Newtonsoft.Json in test vector projects (<https://github.com/aws/aws-encryption-sdk-dafny/pull/595>)
- feat: add user agent to default KMS clients (<https://github.com/aws/aws-encryption-sdk-dafny/pull/598>)
- chore: address potential unsoundness (dafny-lang/dafny#2500) (<https://github.com/aws/aws-encryption-sdk-dafny/pull/599>)
- ci: use .NET 6.0 for release buildspecs (<https://github.com/aws/aws-encryption-sdk-dafny/pull/602>)

## 3.0.0 (2022-05-17)

Initial launch of the AWS Encryption SDK for .NET.
