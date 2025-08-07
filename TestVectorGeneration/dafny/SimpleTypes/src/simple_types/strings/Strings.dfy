include "../../../../Core/Index.dfy"
include "../../../../../../mpl/StandardLibrary/src/Index.dfy"

// Example basic partitioning scheme: string length
// 1) Nonempty strings
// 2) Empty strings
// This exhausts the value space of strings; i.e. all strings are either empty or nonempty.
module EmptyStringPartition refines TestVectorGeneration.Partition
{
    type ValueType = string

    function method IsInPartition(x: ValueType): bool
    {
        |x| == 0
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [""];
    }
}

module NonemptyStringPartition refines TestVectorGeneration.Partition
{
    type ValueType = string

    function method IsInPartition(x: ValueType): bool
    {
        |x| > 0
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := ["abc"];
    }
}

module StringLengthPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import EmptyStringPartition
    import NonemptyStringPartition

    type ValueType = string

    const PartitionCount := 2

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [EmptyStringPartition.IsInPartition(x),
         NonemptyStringPartition.IsInPartition(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>) {
        var values1 := EmptyStringPartition.GetValues();
        var values2 := NonemptyStringPartition.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        // TODO: rewrite proofs to remove this
        assert PartitionMemberships(values1[0]) == [true, false];
        assert PartitionMemberships(values2[0]) == [false, true];
        assert values1[0] in output;
        assert values2[0] in output;
    }

    function DomainValueToString(input: ValueType): string
    {
        input
    }

    function method ToJSON(x: ValueType): JSON
    {
        String(x)
    }
}

// More useful partitioning scheme for strings:
// 1) Empty strings
// 2) ASCII strings
// 3) Non-ASCII strings
// This exhausts the value space of strings; i.e. all strings are either empty,
// contain only ASCII characters, or contain some non-ASCII characters.
module AsciiPartition refines TestVectorGeneration.Partition
{
    type ValueType = string

    function method IsInPartition(x: ValueType): bool
    {
        |x| > 0 && forall i :: 0 <= i < |x| ==> x[i] as int <= 127
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := ["abc"];
    }
}

module NonAsciiPartition refines TestVectorGeneration.Partition
{        
    type ValueType = string

    function method IsInPartition(x: ValueType): bool
    {
        |x| > 0 && exists i :: 0 <= i < |x| && x[i] as int > 127 && x[i] as int <= 65535
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var value := "Hëllø \uD83C\uDF0D — こんにちは世界 — مرحبا بالعالم — Здравствуй, мир!";
        assert value[1] as int > 127 && value[1] as int <= 65535;
        output := [value];
    }
}

module BasicStringPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import EmptyStringPartition
    import AsciiPartition
    import NonAsciiPartition

    type ValueType = string

    const PartitionCount := 3

    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        // (I have no idea why this assert is necessary and would like to be able to remove it)
        assert CountTrues(
            [EmptyStringPartition.IsInPartition(""),
                AsciiPartition.IsInPartition(""),
                NonAsciiPartition.IsInPartition("")]
        ) == 1;

        [EmptyStringPartition.IsInPartition(x),
         AsciiPartition.IsInPartition(x),
         NonAsciiPartition.IsInPartition(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := EmptyStringPartition.GetValues();
        var values2 := AsciiPartition.GetValues();
        var values3 := NonAsciiPartition.GetValues();
        output := values1 + values2 + values3;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        // TODO: rewrite proofs to remove this
        assert PartitionMemberships(values1[0]) == [true, false, false];
        assert PartitionMemberships(values2[0]) == [false, true, false];
        assert PartitionMemberships(values3[0]) == [false, false, true];
        assert values1[0] in output;
        assert values2[0] in output;
        assert values3[0] in output;
    }

    function method ToJSON(x: ValueType): JSON
    {
        String(x)
    }
}


// Partitioning scheme for Option<string> values:
// 1) None
// 2) Some with representative values from BasicStringPartitioning
module OptionStringNone refines TestVectorGeneration.Partition
{
    import opened Wrappers
    import opened JSON.Values

    type ValueType = Option<string>

    function method IsInPartition(x: ValueType): bool
    {
        x.None?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        output := [Option.None];
    }
}

module OptionStringSome refines TestVectorGeneration.Partition
{
    import opened Wrappers
    import StringLengthPartitioning
    import BasicStringPartitioning
    import opened JSON.Values

    type ValueType = Option<string>

    function method IsInPartition(x: ValueType): bool
    {
        x.Some?
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
    {
        var values1 := StringLengthPartitioning.GetValues();
        var values2 := BasicStringPartitioning.GetValues();
        var values := values1 + values2;
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

module OptionStringPartitioning refines TestVectorGeneration.PartitioningScheme
{
    import opened Wrappers
    import OptionStringNone
    import OptionStringSome

    type ValueType = Option<string>

    const PartitionCount := 2

    // Two partitions: Option.None and Option.Some.
    function method PartitionMemberships(x: ValueType): seq<bool>
    {
        [OptionStringNone.IsInPartition(x),
         OptionStringSome.IsInPartition(x)]
    }

    method RepresentativeValues() returns (output: seq<ValueType>) {
        var values1 := OptionStringNone.GetValues();
        var values2 := OptionStringSome.GetValues();
        output := values1 + values2;

        // I don't know why I need these asserts,
        // but Dafny wants them to prove that the values exhaust all partitions.
        // TODO: rewrite proofs to remove this
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
                [("Some", String(x.value))]
            )
    }
}

// // Part of these modules could be generated with Smithy-Dafny
// // to automate specifying ValueType force it to be specified according to the model,
// // but that would take effort and doesn't provide significant value.
// // Separating out the "Smithy-generatable" parts for demonstration.
// abstract module AbstractGetStringInput refines TestVectorGeneration.PartitioningScheme {
//     import SimpleTypesSmithyStringTypes
//     type ValueType = SimpleTypesSmithyStringTypes.GetStringInput
// }

// module GetStringInputPartitioning refines AbstractGetStringInput
// {
//     import OptionStringPartitioning

//     const PartitionCount := 1 // One partition

//     // One partition: Option<string>
//     function method PartitionMemberships(x: ValueType): seq<bool>
//     {
//         var optionString := OptionStringPartitioning.PartitionMemberships(
//             x.value
//         );
//         [optionString[0] || optionString[1]]
//     }

//     method RepresentativeValues() returns (output: seq<ValueType>)
//     {
//         var values := OptionStringPartitioning.GetValues();
//         var temp: seq<ValueType> := [];
//         var i := 0;

//         while i < |values|
//             invariant 0 <= i <= |values|
//             invariant |temp| == i
//         {
//             var t := SimpleTypesSmithyStringTypes.GetStringInput(
//                 value := values[i]
//             );
//             temp := temp + [t];
//             assert t in temp;
//             i := i + 1;
//         }
//         output := temp;
//     }

//     function method ToJSON(x: ValueType): JSON
//     {
//         Object(
//             [("value", OptionStringPartitioning.ToJSON(x.value))]
//         )
//     }
// }


// // This abstract module could be Smithy-generated.
// abstract module AbstractGetStringOperation refines TestVectorGeneration.OperationModel
// {
//     import SimpleTypesSmithyStringTypes

//     type Domain = SimpleTypesSmithyStringTypes.GetStringInput
//     type Range = SimpleTypesSmithyStringTypes.GetStringOutput

//     const operationName := "GetString"
// }

// module GetStringOperationImpl refines AbstractGetStringOperation
// {
//     import GetStringInputPartitioning
//     import OptionStringPartitioning

//     method DomainRepresentativeValues() returns (output: seq<Domain>)
//     {
//         output := GetStringInputPartitioning.GetValues();
//     }
    
//     method EvaluationRule(x: Domain) returns (output: Range)
//     {
//         // This is where we would write a rule for the expected result of an operation
//         // if we wanted to store its expected result in the generated JSON.
//         output := SimpleTypesSmithyStringTypes.GetStringOutput(value := x.value);
//     }

//     function method InputToJSON(x: Domain): JSON
//     {
//         GetStringInputPartitioning.ToJSON(x)
//     }

//     function method OutputToJSON(x: Range): JSON
//     {
//         Object(
//             [("value", OptionStringPartitioning.ToJSON(x.value))]
//         )
//     }
// }

// // This abstract module could be Smithy-generated.
// abstract module AbstractSimpleStringServiceModel refines TestVectorGeneration.ServiceModel
// {
//     const serviceName := "SimpleString"
//     const serviceNamespace := "simple.types.smithyString"
// }

// // 
// module SimpleStringServiceModel refines AbstractSimpleStringServiceModel
// {
//     import GetStringOperationImpl

//     method GenerateGetStringJSON() returns (json: JSON)
//     {
//         json := GetStringOperationImpl.GenerateJSON();
//     }

//     method GenerateOperationsJSON() returns (operationsJSON: JSON)
//     {
//         // There is only one operation right now, but we would add more operations here if we want. 
//         var GetStringJSON := GenerateGetStringJSON();
//         operationsJSON := Array([
//             GetStringJSON
//         ]);
//     }
// }

// module StringsTests {
//     import GetStringOperationImpl
//     import SimpleString
//     import opened SimpleTypesSmithyStringTypes
//     import opened Wrappers
//     import UTF8
//     import SimpleStringServiceModel
//     import TestVectorGeneration
//     import opened JSON.Values
//     import GetStringInputPartitioning

//     method {:test} TestVectorsV2()
//     {
//         // We can generate the JSON for any shape defined here,
//         // depending on what we want to test.
//         // ex. JSON for all GetStringInput values:
//         var getStringInputJSON := GetStringInputPartitioning.GenerateJSON();
//         var out := TestVectorGeneration.WriteVectorsFile("GetStringInput.json", getStringInputJSON);
//         // ex. JSON for just the GetString operation:
//         var getStringJSON := GetStringOperationImpl.GenerateJSON();
//         out := TestVectorGeneration.WriteVectorsFile("GetString.json", getStringJSON);
//         // ex. JSON for the whole service model:
//         var outputJSON := SimpleStringServiceModel.GenerateJSON();
//         out := TestVectorGeneration.WriteVectorsFile("SimpleStringService.json", outputJSON);
//     }
// }
