namespace aws.cryptography.materialProviders.testvectors.keyrings.rawaes

// Example Smithy model for testing keyrings.
// The shapes defined here aren't used right now, but is here to demonstrate that this is possible.
// (The ESDKRoundTrip project defines a Smithy model that is actually used.)

use aws.cryptography.materialProviders#CreateRawAesKeyringInput

structure CreateRawAesKeyringSmokeTestOutput {}

// Shape definition for a test that creates a raw AES keyring
// and asserts the expected output of its creation.
operation CreateRawAesKeyringSmokeTest{
    input: CreateRawAesKeyringInput,
    output: CreateRawAesKeyringSmokeTestOutput
}

// I'd prefer to not need to define a localService to avoid taking a Polymorph V1 dependency,
// but Polymorph V1 requires a service to be defined to generate Smithy-Dafny shapes.
// The test vectors generate test vectors for this service just for demonstration,
// but ideally we wouldn't do this to avoid the localService dependency.
structure KeyringsTestConfig {}

@aws.polymorph#localService(
  sdkId: "KeyringsTest",
  config: KeyringsTestConfig,
)
service KeyringsTestService {
    version: "2021-11-01",
    operations: [
        CreateRawAesKeyringSmokeTest,
    ],
}