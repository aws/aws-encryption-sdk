include "possibly_smithygenerated/CreateRawAESKeyringInput.dfy"
include "../../SimpleTypes/src/simple_types/strings/Strings.dfy"

// AES wrapping key partitioning scheme:
// 1) AES-128 wrapping keys
// 2) AES-192 wrapping keys
// 3) AES-256 wrapping keys
// 4) Invalid AES wrapping keys (not 128, 192, or 256 bits)
module Aes128WrappingKeyPartition refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt

    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType) : bool
    {
        // MUST test some key with length == 16
        |x| == 16
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [
            [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f]
        ];
    }
}

module Aes192WrappingKeyPartition refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt

    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType) : bool
    {
        // MUST test some key with length == 24
        |x| == 24
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [
            [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
             0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17]
        ];
    }
}

module Aes256WrappingKeyPartition refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt

    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType) : bool
    {
        // MUST test some key with length == 32
        |x| == 32
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [
            [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
             0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
             0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
             0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f]
        ];
    }
}

module ValidAesWrappingKeyPartition refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt
    import Aes128WrappingKeyPartition
    import Aes192WrappingKeyPartition
    import Aes256WrappingKeyPartition

    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType) : bool
    {
        // All valid AES keys are either 128-, 192-, or 256-bit keys
        || Aes128WrappingKeyPartition.IsInPartition(x)
        || Aes192WrappingKeyPartition.IsInPartition(x)
        || Aes256WrappingKeyPartition.IsInPartition(x)
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var validAes128 := Aes128WrappingKeyPartition.GetValues();
        var validAes192 := Aes192WrappingKeyPartition.GetValues();
        var validAes256 := Aes256WrappingKeyPartition.GetValues();
        output := validAes128 +  validAes192 + validAes256;
    }
}

module InvalidAesWrappingKeyPartition refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt
    import Aes128WrappingKeyPartition
    import Aes192WrappingKeyPartition
    import Aes256WrappingKeyPartition

    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType) : bool
    {
        // MUST test some key with a length that does not match any other wrapping key length
        !(Aes128WrappingKeyPartition.IsInPartition(x) ||
        Aes192WrappingKeyPartition.IsInPartition(x) ||
        Aes256WrappingKeyPartition.IsInPartition(x))
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        // One empty, one invalid
        output := [[], [0x00]];
    }
}

module AesWrappingKeyPartitioningScheme refines TestVectorGeneration.PartitioningScheme
{
    import opened StandardLibrary.UInt
    import ValidAesWrappingKeyPartition
    import InvalidAesWrappingKeyPartition
    import Base64

    type ValueType = seq<uint8>

    // 2 partitions: 
    // All AES keys are either valid or invalid;
    // The TestVectorGenerator module asserts completeness of the partitioning scheme
    // over the value space
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType) : seq<bool>
    {
        [ValidAesWrappingKeyPartition.IsInPartition(x),
        InvalidAesWrappingKeyPartition.IsInPartition(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var validAes := ValidAesWrappingKeyPartition.GetValues();
        var invalidAes := InvalidAesWrappingKeyPartition.GetValues();
        output := validAes + invalidAes;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(validAes[0]) == [true, false];
        assert PartitionMemberships(invalidAes[0]) == [false, true];
        assert validAes[0] in output;
        assert invalidAes[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        String(Base64.Encode(x))
    }
}

module WrappingAlgValues refines TestVectorGeneration.EnumeratedValueSpace
{
    import MPLTypes = AwsCryptographyMaterialProvidersTypes
    type ValueType = MPLTypes.AesWrappingAlg

    function method ValueSpaceList(): seq<ValueType>
    {
        // MUST test all specified wrapping algorithms
        // The TestVectorGenerator module asserts that this list contains every value in the value space
        [
            MPLTypes.ALG_AES128_GCM_IV12_TAG16(),
            MPLTypes.ALG_AES192_GCM_IV12_TAG16(),
            MPLTypes.ALG_AES256_GCM_IV12_TAG16()
        ]
    }

    function method ToJSON(x: ValueType): JSON
    {
        if x == MPLTypes.ALG_AES128_GCM_IV12_TAG16() then
            String("ALG_AES128_GCM_IV12_TAG16")
        else if x == MPLTypes.ALG_AES192_GCM_IV12_TAG16() then
            String("ALG_AES192_GCM_IV12_TAG16")
        else if x == MPLTypes.ALG_AES256_GCM_IV12_TAG16() then
            String("ALG_AES256_GCM_IV12_TAG16")
        else
            String("UnknownWrappingAlg")
    }

    // TODO: Remove this; derive it from ValueSpaceList in the core module
    // This isn't actually used in test vector value generation but is used in proofs
    function method ValueSpaceEqualityList(x: ValueType): seq<bool>
    {
        assert CountTrues([
            MPLTypes.ALG_AES128_GCM_IV12_TAG16() == MPLTypes.ALG_AES128_GCM_IV12_TAG16(),
            MPLTypes.ALG_AES128_GCM_IV12_TAG16() == MPLTypes.ALG_AES192_GCM_IV12_TAG16(),
            MPLTypes.ALG_AES128_GCM_IV12_TAG16() == MPLTypes.ALG_AES256_GCM_IV12_TAG16()
        ]) == 1;
        [
            x == MPLTypes.ALG_AES128_GCM_IV12_TAG16(),
            x == MPLTypes.ALG_AES192_GCM_IV12_TAG16(),
            x == MPLTypes.ALG_AES256_GCM_IV12_TAG16()
        ]
    }
}

module KeyNamespacePartitioningScheme refines TestVectorGeneration.PartitioningScheme
{
    import InvalidRawAesKeyNamespacePartition
    import ValidRawAesKeyNamespacePartition

    type ValueType = string

    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool> {
        // 2 partitions: 1) invalid key namespace, 2) valid key namespace
        [InvalidRawAesKeyNamespacePartition.IsInPartition(x),
        ValidRawAesKeyNamespacePartition.IsInPartition(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>) {
        var invalidValues := InvalidRawAesKeyNamespacePartition.GetValues();
        var validValues := ValidRawAesKeyNamespacePartition.GetValues();
        output := invalidValues + validValues;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(invalidValues[0]) == [true, false];
        assert PartitionMemberships(validValues[0]) == [false, true];
        assert invalidValues[0] in output;
        assert validValues[0] in output;
    }

    function method ToJSON(x: ValueType): JSON {
        String(x)
    }
}

module InvalidRawAesKeyNamespacePartition refines TestVectorGeneration.Partition
{
    // An invalid raw AES key namespace is just one that is "aws-kms";
    // All other namespaces are valid
    type ValueType = string

    function method IsInPartition(x: ValueType): bool {
        // MUST test with "aws-kms"
        x == "aws-kms"
    }

    method RepresentativeValues() returns (output: seq<ValueType>) {
        output := ["aws-kms"];
    }
}

module ValidRawAesKeyNamespacePartition refines TestVectorGeneration.Partition
{
    import BasicStringPartitioning

    type ValueType = string

    function method IsInPartition(x: ValueType): bool {
        // MUST test with strings that are not "aws-kms"
        x != "aws-kms"
    }

    function method FilterNonAwsKms(s: seq<ValueType>): seq<ValueType>
        decreases |s|
        ensures forall i :: 0 <= i < |FilterNonAwsKms(s)| ==> FilterNonAwsKms(s)[i] != "aws-kms"
    {
        if |s| == 0 then []
        else if s[0] == "aws-kms" then FilterNonAwsKms(s[1..])
        else [s[0]] + FilterNonAwsKms(s[1..])
    }

    method RepresentativeValues() returns (output: seq<ValueType>) {
        var values := BasicStringPartitioning.GetValues();
        output := FilterNonAwsKms(values);
        if |output| == 0 {
            // If every single value in the representative strings is "aws-kms"
            // return some valid default value
            output := ["default-namespace"];
        }
    }
}

module KeyNamePartitioningScheme refines TestVectorGeneration.PartitioningScheme
{
    // All key names are valid; use representative strings
    import BasicStringPartitioning

    type ValueType = string

    const PartitionCount := 1

    function method PartitionMemberships(x: ValueType): seq<bool> {
        // MUST test with representative strings
        // Only one partition since all key names are valid
        [BasicStringPartitioning.IsInExactlyOnePartition(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>) {
        output := BasicStringPartitioning.GetValues();
    }

    function method ToJSON(x: ValueType): JSON {
        String(x)
    }
}

module InvalidAes128PruningConfiguration refines TestVectorGeneration.PruningConfiguration
{
    import MPLTypes = AwsCryptographyMaterialProvidersTypes
    import Aes128WrappingKeyPartition
    import ValidRawAesKeyNamespacePartition
    import KeyNamePartitioningScheme
    import Aes192WrappingKeyPartition

    type ValueType = MPLTypes.CreateRawAesKeyringInput

    function method MatchesPruningConfiguration(x: ValueType): bool
    {
        x.wrappingAlg == MPLTypes.ALG_AES128_GCM_IV12_TAG16() &&
        !Aes128WrappingKeyPartition.IsInPartition(x.wrappingKey)
    }

    method RepresentativeValue(input: ValueType) returns (output: ValueType)
    {
        // change to valid (not NonAwsKmsKeyNamespacePartition) to make clearer?
        var representativeKeyNamespace := ValidRawAesKeyNamespacePartition.SomeValue();
        var representativeKeyName := KeyNamePartitioningScheme.SomeValue();
        var wrappingKey := Aes192WrappingKeyPartition.SomeValue();
    
        output := MPLTypes.CreateRawAesKeyringInput(
            wrappingAlg := MPLTypes.ALG_AES128_GCM_IV12_TAG16(),
            wrappingKey := wrappingKey,
            keyNamespace := representativeKeyNamespace,
            keyName := representativeKeyName
        );
    }
}

module InvalidAes192PruningConfiguration refines TestVectorGeneration.PruningConfiguration
{
    import MPLTypes = AwsCryptographyMaterialProvidersTypes
    import Aes192WrappingKeyPartition
    import ValidRawAesKeyNamespacePartition
    import KeyNamePartitioningScheme
    import InvalidAesWrappingKeyPartition

    type ValueType = MPLTypes.CreateRawAesKeyringInput

    function method MatchesPruningConfiguration(x: ValueType): bool
    {
        x.wrappingAlg == MPLTypes.ALG_AES192_GCM_IV12_TAG16() &&
        !Aes192WrappingKeyPartition.IsInPartition(x.wrappingKey)
    }

    method RepresentativeValue(input: ValueType) returns (output: ValueType)
    {
        // change to valid (not NonAwsKmsKeyNamespacePartition) to make clearer?
        var representativeKeyNamespace := ValidRawAesKeyNamespacePartition.SomeValue();
        var representativeKeyName := KeyNamePartitioningScheme.SomeValue();
        var wrappingKey := InvalidAesWrappingKeyPartition.SomeValue();
    
        output := MPLTypes.CreateRawAesKeyringInput(
            wrappingAlg := MPLTypes.ALG_AES192_GCM_IV12_TAG16(),
            wrappingKey := wrappingKey,
            keyNamespace := representativeKeyNamespace,
            keyName := representativeKeyName
        );
    }
}

module InvalidAes256PruningConfiguration refines TestVectorGeneration.PruningConfiguration
{
    import MPLTypes = AwsCryptographyMaterialProvidersTypes
    import Aes256WrappingKeyPartition
    import ValidRawAesKeyNamespacePartition
    import KeyNamePartitioningScheme
    import Aes192WrappingKeyPartition

    type ValueType = MPLTypes.CreateRawAesKeyringInput

    function method MatchesPruningConfiguration(x: ValueType): bool
    {
        x.wrappingAlg == MPLTypes.ALG_AES256_GCM_IV12_TAG16() &&
        !Aes256WrappingKeyPartition.IsInPartition(x.wrappingKey)
    }

    method RepresentativeValue(input: ValueType) returns (output: ValueType)
    {
        var representativeKeyNamespace := ValidRawAesKeyNamespacePartition.SomeValue();
        var representativeKeyName := KeyNamePartitioningScheme.SomeValue();
        var wrappingKey := Aes192WrappingKeyPartition.SomeValue();
    
        output := MPLTypes.CreateRawAesKeyringInput(
            wrappingAlg := MPLTypes.ALG_AES256_GCM_IV12_TAG16(),
            wrappingKey := wrappingKey,
            keyNamespace := representativeKeyNamespace,
            keyName := representativeKeyName
        );
    }
}

// Enumerates representative values for CreateRawAesKeyringInput.
// This shape (or any shape) can be fully enumerated on its own,
// if implementer doesn't want to set up a test service/operations
// and doesn't care about expecting particular output values in the JSON,
// and just wants to generate a set of exhuastive inputs.
module CreateRawAesKeyringInput refines AbstractCreateRawAesKeyringInput
{
    import AesWrappingKeyPartitioningScheme
    import WrappingAlgValues
    import KeyNamespacePartitioningScheme
    import KeyNamePartitioningScheme
    import MPLTypes = AwsCryptographyMaterialProvidersTypes
    import InvalidAes128PruningConfiguration
    import InvalidAes192PruningConfiguration
    import InvalidAes256PruningConfiguration

    const PartitionCount := 1

    function method PartitionMemberships(x: ValueType) : seq<bool>
    {
        // 1 partition.
        // If we wanted, we could make this 2 partitions:
        // 1) Valid raw AES keyring inputs
        // 2) Invalid raw AES keyring inputs
        // There's no real benefit to that beyond conceptual clarity.
        [AesWrappingKeyPartitioningScheme.IsValidMember(x.wrappingKey) &&
         WrappingAlgValues.IsValidMember(x.wrappingAlg) &&
         KeyNamespacePartitioningScheme.IsValidMember(x.keyNamespace) &&
         KeyNamePartitioningScheme.IsValidMember(x.keyName)]
    }

    // Any shape can specify a list of pruning configurations.
    // This is a list of rules that remove particular values from the enumerated value space.
    // In this case, this removes redundant invalid configurations to reduce the number of
    // "uninteresting" test cases.
    method PruningConfigurationList(x: ValueType) returns (output: seq<(bool, ValueType)>)
    {
        var invalidAes128 := InvalidAes128PruningConfiguration.PruningConfiguration(x);
        var invalidAes192 := InvalidAes192PruningConfiguration.PruningConfiguration(x);
        var invalidAes256 := InvalidAes256PruningConfiguration.PruningConfiguration(x);

        return [
            invalidAes128,
            invalidAes192,
            invalidAes256
        ];
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var wrappingKeys := AesWrappingKeyPartitioningScheme.GetValues();
        var wrappingAlgs := WrappingAlgValues.GetValues();
        var keyNamespaces := KeyNamespacePartitioningScheme.GetValues();
        var keyNames := KeyNamePartitioningScheme.GetValues();

        output := [];

        assert |wrappingAlgs| > 0;
        assert |wrappingKeys| > 0;
        assert |keyNamespaces| > 0;
        assert |keyNames| > 0;

        for i := 0 to |wrappingAlgs| {
            for j := 0 to |wrappingKeys| {
                for k := 0 to |keyNamespaces| {
                    for l := 0 to |keyNames| {
                        var keyring := MPLTypes.CreateRawAesKeyringInput(
                            wrappingKey := wrappingKeys[j],
                            wrappingAlg := wrappingAlgs[i],
                            keyNamespace := keyNamespaces[k],
                            keyName := keyNames[l]
                        );

                        output := AddIfNotPruned(keyring, output);
                    }
                }
            }
        }

       if |output| == 0 {
            // If every single value in the representative strings is pruned (in practice, never)
            // return some default value
            output := [MPLTypes.CreateRawAesKeyringInput(
                wrappingKey := [0x00],
                wrappingAlg := MPLTypes.ALG_AES128_GCM_IV12_TAG16(),
                keyNamespace := "default",
                keyName := "default"
            )];
        }
    }

    function method ToJSON(x: ValueType): JSON
    {
        Object(
            [("wrappingKey", AesWrappingKeyPartitioningScheme.ToJSON(x.wrappingKey)),
             ("wrappingAlg", WrappingAlgValues.ToJSON(x.wrappingAlg)),
             ("keyNamespace", KeyNamespacePartitioningScheme.ToJSON(x.keyNamespace)),
             ("keyName", KeyNamePartitioningScheme.ToJSON(x.keyName))]
        )
    }
}

// Enumerates representative inputs and outputs for the test CreateRawAesKeyringSmokeTest operation.
// To model the operation's inputs and outputs in JSON, we need to enumerate representative inputs
// and evaluate the expected outputs for those inputs.
// This operation (or any operation) can be fully enumerated on its own,
// if implementer doesn't want to set up a test service
// and just wants to generate a set of exhuastive inputs and outputs.
module CreateRawAesKeyringSmokeTest refines AbstractCreateRawAesKeyringSmokeTest
{
    import CreateRawAesKeyringInput
    import AesWrappingKeyPartitioningScheme
    import ValidAesWrappingKeyPartition
    import Aes128WrappingKeyPartition
    import Aes192WrappingKeyPartition
    import Aes256WrappingKeyPartition
    import WrappingAlgValues
    import KeyNamespacePartitioningScheme
    import KeyNamePartitioningScheme
    import MPLTypes = AwsCryptographyMaterialProvidersTypes
    import Base64
    import OptionStringPartitioning

    // Computes expected output to write to JSON for a given input.
    // Note that the operation under test is NOT CreateRawAesKeyringInput,
    // but rather a test operation that wraps CreateRawAesKeyringInput.
    // This test operation can interpret any errors from CreateRawAesKeyringInput
    // as a failure, and match on the string output.
    // (There are other ways to handle errors; this is intentionally left flexible)
    method EvaluationRule(x: Domain) returns (output: Range)
    {
        // TODO: real duvet
        // The length of the wrapping key MUST be 128, 192, or 256.
        if (!ValidAesWrappingKeyPartition.IsInPartition(x.wrappingKey)) {
            output := "Invalid wrapping key length";
            return;
        }
        // Initialization MUST fail if the length of the wrapping key does not match the length specified by the wrapping algorithm.
        if (x.wrappingAlg == MPLTypes.ALG_AES128_GCM_IV12_TAG16() && !Aes128WrappingKeyPartition.IsInPartition(x.wrappingKey)) {
            output := "Invalid wrapping key length";
            return;
        } else if (x.wrappingAlg == MPLTypes.ALG_AES192_GCM_IV12_TAG16() && !Aes192WrappingKeyPartition.IsInPartition(x.wrappingKey)) {
            output := "Invalid wrapping key length";
            return;
        } else if (x.wrappingAlg == MPLTypes.ALG_AES256_GCM_IV12_TAG16() && !Aes256WrappingKeyPartition.IsInPartition(x.wrappingKey)) {
            output := "Invalid wrapping key length";
            return;
        }
        // This value MUST NOT be or start with "aws-kms" unless this encrypted data key was produced by one of the AWS KMS Keyrings.
        if (x.keyNamespace == "aws-kms") {
            output := "Invalid key namespace for raw AES keyring";
            return;
        }
        output := "ok";
    }

    function method InputToJSON(x: Domain): JSON
    {
        CreateRawAesKeyringInput.ToJSON(x)
    }

    function method OutputToJSON(x: Range): JSON
    {
       String(x)
    }

    method DomainRepresentativeValues() returns (output: seq<Domain>)
    {
        output := CreateRawAesKeyringInput.GetValues();
    }
}

// Enumerates representative inputs and outputs for operations in the CreateRawAesKeyringTestService.
// This refers to the vector generator for its only operation.
module CreateRawAesKeyringTestService refines AbstractCreateRawAesKeyringTestService
{
    import CreateRawAesKeyringInput
    import CreateRawAesKeyringSmokeTest

    method GenerateOperationsJSON() returns (operationsJSON: JSON)
    {
        var CreateRawAesKeyringSmokeTestJSON := CreateRawAesKeyringSmokeTest.GenerateJSON();
        operationsJSON := Array([
            CreateRawAesKeyringSmokeTestJSON
        ]);
    }
}

module CreateRawAesKeyringInputTests {
    import CreateRawAesKeyringInput
    import CreateRawAesKeyringSmokeTest
    import CreateRawAesKeyringTestService
    import TestVectorGeneration

    method {:test} TestVectorsV2()
    {
        // We can generate exhaustive inputs for just the CreateRawAesKeyringInput;
        var CreateRawAesKeyringInputJSON := CreateRawAesKeyringInput.GenerateJSON();
        var out := TestVectorGeneration.WriteVectorsFile("CreateRawAesKeyringInput.json", CreateRawAesKeyringInputJSON);

        // We can generate exhaustive inputs and outputs for the CreateRawAesKeyringSmokeTest operation;
        var CreateRawAesKeyringSmokeTestJSON := CreateRawAesKeyringSmokeTest.GenerateJSON();
        var out2 := TestVectorGeneration.WriteVectorsFile("CreateRawAesKeyringSmokeTest.json", CreateRawAesKeyringSmokeTestJSON);

        // We can generate exhaustive inputs and outputs for all operations in CreateRawAesKeyringTestService.
        var CreateRawAesKeyringTestServiceJSON := CreateRawAesKeyringTestService.GenerateJSON();
        var out3 := TestVectorGeneration.WriteVectorsFile("CreateRawAesKeyringTestService.json", CreateRawAesKeyringTestServiceJSON);
    }
}