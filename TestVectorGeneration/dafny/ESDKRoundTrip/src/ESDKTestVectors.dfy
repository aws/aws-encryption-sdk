include "possibly_smithygenerated/ESDKTestVectorsSmithyGenerated.dfy"
include "../../../../AwsEncryptionSDK/dafny/AwsEncryptionSdk/Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "../../Keyrings/src/CreateRawAESKeyringInput.dfy"

module EmptyPlaintextPartitioning refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt
    import AwsCryptographyEncryptionSdkTypes
    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType): bool
    {
        x == []
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [[]];
    }
}

module NonemptyPlaintextPartitioning refines TestVectorGeneration.Partition
{
    import opened StandardLibrary.UInt
    import AwsCryptographyEncryptionSdkTypes
    type ValueType = seq<uint8>

    function method IsInPartition(x: ValueType): bool
    {
        x != []
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [[0x61, 0x62, 0x63]]; // "abc" in UTF-8
    }
}

module PlaintextPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import opened StandardLibrary.UInt
    import AwsCryptographyEncryptionSdkTypes
    type ValueType = seq<uint8>
    import Base64

    import EmptyPlaintextPartitioning
    import NonemptyPlaintextPartitioning

    // 2 partitions: 
    // 1) Empty plaintext
    // 2) Non-empty plaintext
    // This is basic, but easy to extend or plug in fuzzed values.
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [EmptyPlaintextPartitioning.IsValidMember(x),
         NonemptyPlaintextPartitioning.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := EmptyPlaintextPartitioning.GetValues();
        var values2 := NonemptyPlaintextPartitioning.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        String(Base64.Encode(x))
    }
}

module EmptyEncryptionContextPartition refines TestVectorGeneration.Partition
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = AwsCryptographyMaterialProvidersTypes.EncryptionContext

    function method IsInPartition(x: ValueType): bool
    {
        |x.Keys| == 0
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [map[]];
    }
}

module NomemptyEncryptionContextPartition refines TestVectorGeneration.Partition
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = AwsCryptographyMaterialProvidersTypes.EncryptionContext

    import UTF8

    function method IsInPartition(x: ValueType): bool
    {
        |x.Keys| > 0
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        // TODO not this
        var aResult := UTF8.Encode("abc");
        var bResult := UTF8.Encode("def");
        if aResult.Success? && bResult.Success? {
            var a := aResult.value;
            var b := bResult.value;
            output := [map[a := b]];
        } else {
            output := [];
        }
    }
}

module EncryptionContextPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = AwsCryptographyMaterialProvidersTypes.EncryptionContext

    import EmptyEncryptionContextPartition
    import NomemptyEncryptionContextPartition

    // 2 partitions: 
    // 1) Empty encryption context
    // 2) Non-empty encryption context
    // This is basic, but easy to extend or plug in fuzzed values.
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [EmptyEncryptionContextPartition.IsValidMember(x),
         NomemptyEncryptionContextPartition.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := EmptyEncryptionContextPartition.GetValues();
        var values2 := NomemptyEncryptionContextPartition.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        String("TODO implement ToJSON for EncryptionContext")
    }
}

module ValidKeyIdentifierPruningConfiguration refines TestVectorGeneration.PruningConfiguration
{
    import AwsCryptographyMaterialProvidersTypes
    type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

    import ValidRawAesKeyNamespacePartition
    import KeyNamePartitioningScheme

    function method MatchesPruningConfiguration(x: ValueType): bool
    {
        && ValidRawAesKeyNamespacePartition.IsValidMember(x.keyNamespace)
        && KeyNamePartitioningScheme.IsValidMember(x.keyName)
    }

    method RepresentativeValue(input: ValueType) returns (output: ValueType)
    {
        var keyName := KeyNamePartitioningScheme.SomeValue();
        var keyNamespace := ValidRawAesKeyNamespacePartition.SomeValue();
        output := AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput(
            wrappingKey := input.wrappingKey,
            keyNamespace := keyNamespace,
            keyName := keyName,
            wrappingAlg := input.wrappingAlg
        );
    }
}

module InvalidKeyIdentifierPruningConfiguration refines TestVectorGeneration.PruningConfiguration
{
    import AwsCryptographyMaterialProvidersTypes
    type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

    import InvalidRawAesKeyNamespacePartition
    import KeyNamePartitioningScheme

    function method MatchesPruningConfiguration(x: ValueType): bool
    {
        && InvalidRawAesKeyNamespacePartition.IsValidMember(x.keyNamespace)
        && KeyNamePartitioningScheme.IsValidMember(x.keyName)
    }

    method RepresentativeValue(input: ValueType) returns (output: ValueType)
    {
        var keyName := KeyNamePartitioningScheme.SomeValue();
        var keyNamespace := InvalidRawAesKeyNamespacePartition.SomeValue();
        output := AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput(
            wrappingKey := input.wrappingKey,
            keyNamespace := keyNamespace,
            keyName := keyName,
            wrappingAlg := input.wrappingAlg
        );
    }
}

module KeyringPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    type ValueType = AwsCryptographyEncryptionSdkTestRoundtripTypes.SupportedKeyringCreateInputs

    import CreateRawAesKeyringInput

    // 1 partition:
    // 1) Raw AES keyring input
    // (This can either be valid or invalid, but we don't care about that here.)
    const PartitionCount := 1

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [CreateRawAesKeyringInput.IsValidMember(x.RawAes)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var res1 := CreateRawAesKeyringInput.GetValues();
        var out1 := [];
        for i := 0 to |res1|
        {
            out1 := out1 + [AwsCryptographyEncryptionSdkTestRoundtripTypes.RawAes(
                res1[i]
            )];
        }
        assert |out1| == |res1| by {
            assume {:axiom} false; // TODO prove length equality...
        }
        output := out1;
    }

    function method ToJSON(x: ValueType): JSON
    {
        match x
        case RawAes(_) =>
            Object(
                [("RawAes", CreateRawAesKeyringInput.ToJSON(x.RawAes))]
            )
    }
}

module ESDKAlgorithmSuitePartitioning refines TestVectorGeneration.EnumeratedValueSpace
{
    import opened AwsCryptographyMaterialProvidersTypes
    type ValueType = AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId

    function method ValueSpaceList(): seq<ValueType>
    {
        [
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF,
            ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_NO_KDF,
            ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_NO_KDF,
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256,
            ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256,
            ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256,
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256,
            ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384,
            ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384,
            ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY,
            ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384
        ]
    }

    function method ValueSpaceEqualityList(x: ValueType): seq<bool>
    {
        assert CountTrues([
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_NO_KDF(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_NO_KDF(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY(),
            ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF() == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384()
        ]) == 1;
        assert CountTrues(ValueSpaceEqualityList(x)) == 1 by {
            assume {:axiom} false;
        }
        [
            x == ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF(),
            x == ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_NO_KDF(),
            x == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_NO_KDF(),
            x == ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256(),
            x == ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256(),
            x == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256(),
            x == ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256(),
            x == ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384(),
            x == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384(),
            x == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY(),
            x == ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384()
        ]

    }

    function method ToJSON(x: ValueType): JSON
    {
        match x
        case ALG_AES_128_GCM_IV12_TAG16_NO_KDF =>
            String("ALG_AES_128_GCM_IV12_TAG16_NO_KDF")
        case ALG_AES_192_GCM_IV12_TAG16_NO_KDF =>
            String("ALG_AES_192_GCM_IV12_TAG16_NO_KDF")
        case ALG_AES_256_GCM_IV12_TAG16_NO_KDF =>
            String("ALG_AES_256_GCM_IV12_TAG16_NO_KDF")
        case ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256 =>
            String("ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256")
        case ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256 =>
            String("ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256")
        case ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256 =>
            String("ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256")
        case ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256 =>
            String("ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256")
        case ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 =>
            String("ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384")
        case ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384 =>
            String("ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384")
        case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY =>
            String("ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY")
        case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384 =>
            String("ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384")
    }
}

module MinimumFrameLengthPartition refines TestVectorGeneration.Partition
{
    import AwsCryptographyEncryptionSdkTypes
    type ValueType = AwsCryptographyEncryptionSdkTypes.FrameLength

    function method IsInPartition(x: ValueType): bool
    {
        x == 1
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [1];
    }
}

module NonminimumFrameLengthPartition refines TestVectorGeneration.Partition
{
    import AwsCryptographyEncryptionSdkTypes
    type ValueType = AwsCryptographyEncryptionSdkTypes.FrameLength

    function method IsInPartition(x: ValueType): bool
    {
        x != 1
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [10];
    }
}

module FrameLengthPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTypes
    type ValueType = AwsCryptographyEncryptionSdkTypes.FrameLength

    import MinimumFrameLengthPartition
    import NonminimumFrameLengthPartition

    // 2 partitions: 
    // 1) Minimum frame length (1)
    // 2) Anything other than the minimum frame length
    // We should extend this as defined in spec:
    // https://github.com/awslabs/aws-encryption-sdk-specification/blob/master/framework/test-vectors/esdk-test-vector-enumeration.md#representative-frame-sizes
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [MinimumFrameLengthPartition.IsValidMember(x),
         NonminimumFrameLengthPartition.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := MinimumFrameLengthPartition.GetValues();
        var values2 := NonminimumFrameLengthPartition.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        String("TODO implement ToJSON for FrameLengthPartitioning")
    }
}

module OptionalFrameLengthNonePartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyEncryptionSdkTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyEncryptionSdkTypes.FrameLength>

    function method IsInPartition(x: ValueType): bool
    {
        x.None?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [Option.None];
    }
}

module OptionalFrameLengthSomePartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyEncryptionSdkTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyEncryptionSdkTypes.FrameLength>

    import FrameLengthPartitioning

    function method IsInPartition(x: ValueType): bool
    {
        x.Some?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := FrameLengthPartitioning.GetValues();
        var values := values1;
        var temp: seq<ValueType> := [];
        var i := 0;

        while i < |values|
            invariant 0 <= i <= |values|
            invariant |temp| == i
            invariant forall j :: 0 <= j < i ==> temp[j] == Option.Some(values[j])
        {
            temp := temp + [Option.Some(values[i])];
            i := i + 1;
        }

        output := temp;
    }
}


module OptionalFrameLengthPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyEncryptionSdkTypes.FrameLength>

    import OptionalFrameLengthSomePartitioning
    import OptionalFrameLengthNonePartitioning

    // 2 partitions: Option<Some> and Option<None>
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [OptionalFrameLengthSomePartitioning.IsValidMember(x),
         OptionalFrameLengthNonePartitioning.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := OptionalFrameLengthSomePartitioning.GetValues();
        var values2 := OptionalFrameLengthNonePartitioning.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        if x.None? then
            Object(
                [("None", Null)]
            )
        else
            Object(
                [("Some", Number(Int(x.value as int)))]
            )
    }
}

module OptionNoneEncryptionContextPartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyMaterialProvidersTypes.EncryptionContext>

    function method IsInPartition(x: ValueType): bool
    {
        x.None?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [Option.None];
    }
}

module OptionSomeEncryptionContextPartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyMaterialProvidersTypes.EncryptionContext>

    import EncryptionContextPartitioning

    function method IsInPartition(x: ValueType): bool
    {
        x.Some?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := EncryptionContextPartitioning.GetValues();
        var values := values1;
        var temp: seq<ValueType> := [];
        var i := 0;

        while i < |values|
            invariant 0 <= i <= |values|
            invariant |temp| == i
            invariant forall j :: 0 <= j < i ==> temp[j] == Option.Some(values[j])
        {
            temp := temp + [Option.Some(values[i])];
            i := i + 1;
        }

        output := temp;
    }
}

module OptionalEncryptionContextPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyMaterialProvidersTypes.EncryptionContext>
    import SortedSets
    import Seq
    import UTF8

    import OptionNoneEncryptionContextPartitioning
    import OptionSomeEncryptionContextPartitioning

    // 2 partitions: Option<Some> and Option<None>
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [OptionNoneEncryptionContextPartitioning.IsValidMember(x),
         OptionSomeEncryptionContextPartitioning.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := OptionNoneEncryptionContextPartitioning.GetValues();
        var values2 := OptionSomeEncryptionContextPartitioning.GetValues();
        output := values1 + values2;
        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method EncryptionContextToJson(key: string, m: AwsCryptographyMaterialProvidersTypes.EncryptionContext)
        : Result<seq<(string, JSON)>, string>
    {
        var keys := SortedSets.ComputeSetToOrderedSequence2(m.Keys, (a, b) => a < b);
        var pairsBytes
        :- Seq.MapWithResult(
            k requires k in m.Keys =>
                var key :- UTF8.Decode(k);
                var value :- UTF8.Decode(m[k]);
                Success((key, String(value))),
            keys);
        Success([(key, Object(pairsBytes))])
    }

    function method ToJSON(x: ValueType): JSON
    {
        if x.None? then
            Object(
                [("None", Null)]
            )
        else
            match EncryptionContextToJson("Some", x.value)
            case Success(pairs) => Object(pairs)
            case Failure(err) => Object([("Error", String(err))])
    }
}

module OptionNoneKeyringPartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyEncryptionSdkTestRoundtripTypes.SupportedKeyringCreateInputs>

    function method IsInPartition(x: ValueType): bool
    {
        x.None?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [Option.None];
    }
}

module OptionSomeKeyringPartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyEncryptionSdkTestRoundtripTypes.SupportedKeyringCreateInputs>

    import KeyringPartitioning

    function method IsInPartition(x: ValueType): bool
    {
        x.Some?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := KeyringPartitioning.GetValues();
        var values := values1;
        var temp: seq<ValueType> := [];
        var i := 0;

        while i < |values|
            invariant 0 <= i <= |values|
            invariant |temp| == i
            invariant forall j :: 0 <= j < i ==> temp[j] == Option.Some(values[j])
        {
            temp := temp + [Option.Some(values[i])];
            i := i + 1;
        }

        output := temp;
    }
}

module OptionalKeyringPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyEncryptionSdkTestRoundtripTypes.SupportedKeyringCreateInputs>

    import OptionNoneKeyringPartitioning
    import OptionSomeKeyringPartitioning
    import KeyringPartitioning

    // 2 partitions: Option<Some> and Option<None>
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [OptionNoneKeyringPartitioning.IsValidMember(x),
         OptionSomeKeyringPartitioning.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := OptionNoneKeyringPartitioning.GetValues();
        var values2 := OptionSomeKeyringPartitioning.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        if x.None? then
            Object(
                [("None", Null)]
            )
        else
            Object(
                [("Some", KeyringPartitioning.ToJSON(x.value))]
            )
    }
}

module OptionNoneAlgorithmSuitePartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId>

    function method IsInPartition(x: ValueType): bool
    {
        x.None?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [Option.None];
    }
}

module OptionSomeAlgorithmSuitePartitioning refines TestVectorGeneration.Partition
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId>

    import ESDKAlgorithmSuitePartitioning

    function method IsInPartition(x: ValueType): bool
    {
        x.Some?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := ESDKAlgorithmSuitePartitioning.GetValues();
        var values := values1;
        var temp: seq<ValueType> := [];
        var i := 0;

        while i < |values|
            invariant 0 <= i <= |values|
            invariant |temp| == i
            invariant forall j :: 0 <= j < i ==> temp[j] == Option.Some(values[j])
        {
            temp := temp + [Option.Some(values[i])];
            i := i + 1;
        }

        output := temp;
    }
}

module OptionalESDKAlgorithmSuitePartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyMaterialProvidersTypes
    import opened Wrappers
    type ValueType = Option<AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId>

    import OptionNoneAlgorithmSuitePartitioning
    import OptionSomeAlgorithmSuitePartitioning
    import ESDKAlgorithmSuitePartitioning

    // 2 partitions: Option<Some> and Option<None>
    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [OptionNoneAlgorithmSuitePartitioning.IsValidMember(x),
         OptionSomeAlgorithmSuitePartitioning.IsValidMember(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := OptionNoneAlgorithmSuitePartitioning.GetValues();
        var values2 := OptionSomeAlgorithmSuitePartitioning.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        if x.None? then
            Object(
                [("None", Null)]
            )
        else
            Object(
                [("Some", ESDKAlgorithmSuitePartitioning.ToJSON(x.value))]
            )
    }
}

module EncryptInputPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    type ValueType = AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripEncryptInput

    import PlaintextPartitioning
    import OptionalEncryptionContextPartitioning
    import OptionalKeyringPartitioning
    import OptionalESDKAlgorithmSuitePartitioning
    import OptionalFrameLengthPartitioning

    // 1 partition containing both valid and invalid values.
    const PartitionCount := 1

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [
            && PlaintextPartitioning.IsValidMember(x.plaintext)
            && OptionalEncryptionContextPartitioning.IsValidMember(x.encryptionContext)
            && OptionalKeyringPartitioning.IsValidMember(x.keyring)
            && OptionalESDKAlgorithmSuitePartitioning.IsValidMember(x.algorithmSuiteId)
            && OptionalFrameLengthPartitioning.IsValidMember(x.frameLength)
        ]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := PlaintextPartitioning.GetValues();
        print "|PlaintextPartitioning.GetValues()| = ";
        print |values1|;
        print "\n";
        var values2 := OptionalEncryptionContextPartitioning.GetValues();
        print "|OptionalEncryptionContextPartitioning.GetValues()| = ";
        print |values2|;
        print "\n";
        var values3 := OptionalKeyringPartitioning.GetValues();
        print "|OptionalKeyringPartitioning.GetValues()| = ";
        print |values3|;
        print "\n";
        var values4 := OptionalESDKAlgorithmSuitePartitioning.GetValues();
        print "|OptionalESDKAlgorithmSuitePartitioning.GetValues()| = ";
        print |values4|;
        print "\n";
        var values5 := OptionalFrameLengthPartitioning.GetValues();
        print "|OptionalFrameLengthPartitioning.GetValues()| = ";
        print |values5|;
        print "\n";

        output := [];
        for i := 0 to |values1|
        {
            for j := 0 to |values2|
            {
                for k := 0 to |values3|
                {
                    for l := 0 to |values4|
                    {
                        for m := 0 to |values5|
                        {
                            output := output + [AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripEncryptInput(
                                plaintext := values1[i],
                                encryptionContext := values2[j],
                                keyring := values3[k],
                                algorithmSuiteId := values4[l],
                                frameLength := values5[m]
                            )];

                            if |output| >= 100 {
                                return output; // Limit the number of representative values
                            }
                        }
                    }
                }
            }
        }

        assert |output| > 0 by {
            assume {:axiom} false;
        }
    }

    function method ToJSON(x: ValueType): JSON
    {
        Object(
            [("plaintext", PlaintextPartitioning.ToJSON(x.plaintext)),
             ("encryptionContext", OptionalEncryptionContextPartitioning.ToJSON(x.encryptionContext)),
             ("keyring", OptionalKeyringPartitioning.ToJSON(x.keyring)),
             ("algorithmSuiteId", OptionalESDKAlgorithmSuitePartitioning.ToJSON(x.algorithmSuiteId)),
             ("frameLength", OptionalFrameLengthPartitioning.ToJSON(x.frameLength))]
        )
    }
}

module DecryptInputPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    type ValueType = AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripDecryptInput

    import OptionalEncryptionContextPartitioning
    import OptionalKeyringPartitioning

    // 1 partition containing all values.
    const PartitionCount := 1

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [
            && OptionalEncryptionContextPartitioning.IsValidMember(x.encryptionContext)
            && OptionalKeyringPartitioning.IsValidMember(x.keyring)
        ]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values2 := OptionalEncryptionContextPartitioning.GetValues();
        print "|OptionalEncryptionContextPartitioning.GetValues()| = ";
        print |values2|;
        print "\n";
        var values3 := OptionalKeyringPartitioning.GetValues();
        print "|OptionalKeyringPartitioning.GetValues()| = ";
        print |values3|;
        print "\n";

        output := [];
        for j := 0 to |values2|
        {
            for k := 0 to |values3|
            {
                output := output + [AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripDecryptInput(
                    encryptionContext := values2[j],
                    keyring := values3[k]
                )];

                if |output| >= 100 {
                    return output; // Limit the number of representative values
                }
            }
        }

        assert |output| > 0 by {
            assume {:axiom} false;
        }
    }

    function method ToJSON(x: ValueType): JSON
    {
        Object(
            [("encryptionContext", OptionalEncryptionContextPartitioning.ToJSON(x.encryptionContext)),
             ("keyring", OptionalKeyringPartitioning.ToJSON(x.keyring))]
        )
    }
}

module TestCrossLanguageRoundTripInputPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import AwsCryptographyEncryptionSdkTestRoundtripTypes
    type ValueType = AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripInput

    import EncryptInputPartitioning
    import DecryptInputPartitioning
    import opened Wrappers

    import InvalidKeyIdentifierPruningConfiguration
    import ValidKeyIdentifierPruningConfiguration

    // 1 partition containing all values.
    const PartitionCount := 1

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [
            && EncryptInputPartitioning.IsValidMember(x.encryptInput)
            && DecryptInputPartitioning.IsValidMember(x.decryptInput)
        ]
    }


    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := EncryptInputPartitioning.GetValues();
        var values2 := DecryptInputPartitioning.GetValues();

        output := [];
        for i := 0 to |values1|
        {
            for j := 0 to |values2|
            {
                
                // TODO: Proof should fail because I hardcoded Python...
                var val := AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripInput(
                    encryptLanguage := AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.PYTHON,
                    decryptLanguage := AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.PYTHON,
                    encryptInput := values1[i],
                    decryptInput := values2[j]
                );

                output := MaybeAddValToVals(val, output);
            }
        }

        assert |output| > 0 by {
            assume {:axiom} false;
        }
    }

    method ReplaceList(x: ValueType) returns (output: ValueType)
    {
        output := x;
        if (output.encryptInput.keyring.Some?
            && ValidKeyIdentifierPruningConfiguration.MatchesPruningConfiguration(output.encryptInput.keyring.value.RawAes))
        {
            var newRawAes := ValidKeyIdentifierPruningConfiguration.RepresentativeValue(output.encryptInput.keyring.value.RawAes);
            // var newKeyring := output.encryptInput.keyring;
            output := AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripInput(
                encryptLanguage := output.encryptLanguage,
                decryptLanguage := output.decryptLanguage,
                encryptInput := AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripEncryptInput(
                    plaintext := output.encryptInput.plaintext,
                    encryptionContext := output.encryptInput.encryptionContext,
                    keyring := Option.Some(AwsCryptographyEncryptionSdkTestRoundtripTypes.RawAes(newRawAes)),
                    algorithmSuiteId := output.encryptInput.algorithmSuiteId,
                    frameLength := output.encryptInput.frameLength
                ),
                decryptInput := output.decryptInput
            );
        }
        if (output.decryptInput.keyring.Some?
            && ValidKeyIdentifierPruningConfiguration.MatchesPruningConfiguration(output.decryptInput.keyring.value.RawAes))
        {
            var newRawAes := ValidKeyIdentifierPruningConfiguration.RepresentativeValue(output.decryptInput.keyring.value.RawAes);
            // var newKeyring := output.encryptInput.keyring;
            output := AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripInput(
                encryptLanguage := output.encryptLanguage,
                decryptLanguage := output.decryptLanguage,
                encryptInput := output.encryptInput,
                decryptInput := AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripDecryptInput(
                    encryptionContext := output.encryptInput.encryptionContext,
                    keyring := Option.Some(AwsCryptographyEncryptionSdkTestRoundtripTypes.RawAes(newRawAes))
                )
            );
        }
        if (output.encryptInput.keyring.Some?
        && InvalidKeyIdentifierPruningConfiguration.MatchesPruningConfiguration(output.encryptInput.keyring.value.RawAes))
        {
            var newRawAes := InvalidKeyIdentifierPruningConfiguration.RepresentativeValue(output.encryptInput.keyring.value.RawAes);
            output := AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripInput(
                encryptLanguage := output.encryptLanguage,
                decryptLanguage := output.decryptLanguage,
                encryptInput := AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripEncryptInput(
                    plaintext := output.encryptInput.plaintext,
                    encryptionContext := output.encryptInput.encryptionContext,
                    keyring := Option.Some(AwsCryptographyEncryptionSdkTestRoundtripTypes.RawAes(newRawAes)),
                    algorithmSuiteId := output.encryptInput.algorithmSuiteId,
                    frameLength := output.encryptInput.frameLength
                ),
                decryptInput := output.decryptInput
            );
        }
        else
        {
            return output;
        }
    }

    function method ToJSON(x: ValueType): JSON
    {
        Object(
            [("encryptLanguage", String("PYTHON")),
             ("decryptLanguage", String("PYTHON")),
             ("encrypt", EncryptInputPartitioning.ToJSON(x.encryptInput)),
             ("decrypt", DecryptInputPartitioning.ToJSON(x.decryptInput))]
        )
    }

    method MaybeAddValToVals(x: ValueType, vals: seq<ValueType>) returns (output: seq<ValueType>)
    {
        var replace := ReplaceList(x);
        if x == replace {
            // If the representative value is the same as the input, we can return it directly.
            return vals + [x];
        } else {
            return vals;
        }
    }}

module TestCrossLanguageRoundTrip refines AbstractTestCrossLanguageRoundTrip
{
    import TestCrossLanguageRoundTripInputPartitioning

    method DomainRepresentativeValues() returns (output: seq<Domain>)
    {
        output := TestCrossLanguageRoundTripInputPartitioning.GetValues();
    }

    method EvaluationRule(x: Domain) returns (output: Range)
    {
        if (x.encryptInput.encryptionContext != x.decryptInput.encryptionContext) {
            return AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripOutput(status:="encryption context mismatch");
        } else {
            return AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripOutput(status:="ok");
        }
    }

    function method InputToJSON(x: Domain): JSON
    {
        TestCrossLanguageRoundTripInputPartitioning.ToJSON(x)
    }

    function method OutputToJSON(x: Range): JSON
    {
        String(x.status)
    }
}

module ESDKTestRoundTrip refines AbstractESDKTestRoundTrip
{
    import TestCrossLanguageRoundTrip

    method GenerateOperationsJSON() returns (operationsJSON: JSON)
    {
        var TestCrossLanguageRoundTripJSON := TestCrossLanguageRoundTrip.GenerateJSON();
        operationsJSON := Array([
            TestCrossLanguageRoundTripJSON
        ]);
    }
}

module TestsRTT {
    import TestCrossLanguageRoundTrip
    import TestVectorGeneration

    method {:test} TestRTTVectors()
    {
        var outputJSON := TestCrossLanguageRoundTrip.GenerateJSON();
        var out := TestVectorGeneration.WriteVectorsFile("TestCrossLanguageRoundTrip.json", outputJSON);
    }
}
