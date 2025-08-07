namespace aws.cryptography.encryptionSdk.testvectors.roundtrip

// Smithy model for testing cross-language round trip encrypt/decrypt.
// This defines separate testing encrypt/decrypt shapes,
// since the encrypt/decrypt shapes defined in the ESDK can't be exhuastively enumerated:
// 1) DecryptInput requires a ciphertext, but this is only available after the encrypt operation.
//    The test vector generation framework would want to assert exhaustiveness of the decrypt input's values
//    but there is only one valid ciphertext for this test.
//    Work around this by defining a separate RoundTripDecryptInput that doesn't have a ciphertext member.
// 2) The ESDK inputs require keyrings/cmms, but I haven't implemented all keyrings/CMMs yet.
//    The test vector generation framework would want to assert exhaustiveness of the keyrings/CMMs.
//    Work around this by defining a separate RoundTripEncryptInput and RoundTripDecryptInput that only require a keyring.
//    (Once more keyrings/CMMs are implemented, we should remove the RoundTripEncryptInput in favor of the ESDK's EncryptInput.)

use aws.cryptography.encryptionSdk#AwsEncryptionSdk

// I'd prefer to not need to define a localService to avoid taking a Polymorph V1 dependency,
// but Polymorph V1 requires a service to be defined to generate Smithy-Dafny shapes.
// The test vectors generate test vectors for this service just for demonstration,
// but ideally we wouldn't do this to avoid the localService dependency.
// We can just generate test vectors for operations.
@aws.polymorph#localService(
  sdkId: "ESDKTestRoundTrip",
  config: ESDKTestRoundTripConfig,
)
service ESDKTestRoundTripService {
  version: "2021-11-01",
  operations: [
    TestCrossLanguageRoundTrip,
  ],
}

structure ESDKTestRoundTripConfig {}

// This operation's JSON would be intended to run against test servers hosting local implementations of the ESDK
// in different runtime languages, since it specifies the encrypt/decrypt language.
operation TestCrossLanguageRoundTrip{
  input: TestCrossLanguageRoundTripInput,
  output: TestCrossLanguageRoundTripOutput,
}

// This operation's JSON would be intended to run against a single local implementation of the ESDK.
operation TestRoundTrip {
  input: RoundTripEncryptInput,
  output: RoundTripDecryptInput,
}

@enum([
  {
    name: "JAVA",
    value: "JAVA",
  },
  {
    name: "PYTHON",
    value: "PYTHON",
  },
  {
    name: "DOTNET",
    value: "DOTNET",
  },
  {
    name: "GO",
    value: "GO",
  },
  {
    name: "JAVASCRIPT",
    value: "JAVASCRIPT",
  },
  {
    name: "RUST",
    value: "RUST",
  },
])
string ImplementationLanguage

structure TestCrossLanguageRoundTripInput {
  @required
  encryptLanguage: ImplementationLanguage,
  @required
  decryptLanguage: ImplementationLanguage,
  @required
  encryptInput: RoundTripEncryptInput,
  @required
  decryptInput: RoundTripDecryptInput,
  // We should add these to assert exhaustiveness of the localService configs.
  // @required
  // mplClientConfig: aws.cryptography.materialProviders#MaterialProvidersConfig,
  // @required
  // esdkClientConfig: aws.cryptography.encryptionSdk#AwsEncryptionSdkConfig,
}

structure TestCrossLanguageRoundTripOutput {
  @required
  status: String
}

union SupportedKeyringCreateInputs {
  RawAes: aws.cryptography.materialProviders#CreateRawAesKeyringInput
}

structure RoundTripEncryptInput {
  @required
  plaintext: Blob,

  encryptionContext: aws.cryptography.materialProviders#EncryptionContext,

  // One of keyring or CMM are required
  // materialsManager: aws.cryptography.materialProviders#CryptographicMaterialsManagerReference,
  keyring: SupportedKeyringCreateInputs

  algorithmSuiteId: aws.cryptography.materialProviders#ESDKAlgorithmSuiteId,

  frameLength: aws.cryptography.encryptionSdk#FrameLength
}

structure RoundTripDecryptInput {
  // No ciphertext in the RoundTripDecryptInput.
  // Ciphertext will be taken from the output of the encrypt operation in the test.

  encryptionContext: aws.cryptography.materialProviders#EncryptionContext,

  // One of keyring or CMM are required
  // materialsManager: aws.cryptography.materialProviders#CryptographicMaterialsManagerReference,
  keyring: SupportedKeyringCreateInputs
}