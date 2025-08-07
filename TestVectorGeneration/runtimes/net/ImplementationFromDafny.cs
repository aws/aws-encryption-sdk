// Dafny program the_program compiled into C#
// To recompile, you will need the libraries
//     System.Runtime.Numerics.dll System.Collections.Immutable.dll
// but the 'dotnet' tool in .NET should pick those up automatically.
// Optionally, you may want to include compiler switches like
//     /debug /nowarn:162,164,168,183,219,436,1717,1718

using System;
using System.Numerics;
using System.Collections;
[assembly: DafnyAssembly.DafnySourceAttribute(@"// dafny 4.10.0.0
// Command-line arguments: translate cs --stdin --no-verify --cores 2 --optimize-erasable-datatype-wrapper false --unicode-char false --function-syntax 3 --output runtimes/net/ImplementationFromDafny --allow-warnings --compile-suffix --legacy-module-names --allow-external-contracts --library /Users/lucmcdon/workplace/aws-encryption-sdk/mpl/StandardLibrary/src/Index.dfy --library /Users/lucmcdon/workplace/aws-encryption-sdk/mpl/AwsCryptographyPrimitives/src/Index.dfy --library /Users/lucmcdon/workplace/aws-encryption-sdk/mpl/ComAmazonawsKms/src/Index.dfy --library /Users/lucmcdon/workplace/aws-encryption-sdk/mpl/ComAmazonawsDynamodb/src/Index.dfy --library /Users/lucmcdon/workplace/aws-encryption-sdk/mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders/src/Index.dfy --library /Users/lucmcdon/workplace/aws-encryption-sdk/mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore/src/Index.dfy --library /Users/lucmcdon/workplace/aws-encryption-sdk/AwsEncryptionSDK/dafny/AwsEncryptionSdk/src/Index.dfy
// the_program


















































































































































module ValidAes128WrappingKeyPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| == 16
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened UInt = StandardLibrary.UInt

  type ValueType = seq<uint8>
}

module ValidAes192WrappingKeyPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| == 24
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened UInt = StandardLibrary.UInt

  type ValueType = seq<uint8>
}

module ValidAes256WrappingKeyPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| == 32
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened UInt = StandardLibrary.UInt

  type ValueType = seq<uint8>
}

module ValidAesWrappingKeyPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    ValidAes128WrappingKeyPartition.IsInPartition(x) || ValidAes192WrappingKeyPartition.IsInPartition(x) || ValidAes256WrappingKeyPartition.IsInPartition(x)
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    var validAes128 := ValidAes128WrappingKeyPartition.RepresentativeValues();
    var validAes192 := ValidAes192WrappingKeyPartition.RepresentativeValues();
    var validAes256 := ValidAes256WrappingKeyPartition.RepresentativeValues();
    output := validAes128 + validAes192 + validAes256;
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened UInt = StandardLibrary.UInt

  import ValidAes128WrappingKeyPartition

  import ValidAes192WrappingKeyPartition

  import ValidAes256WrappingKeyPartition

  type ValueType = seq<uint8>
}

module InvalidAesWrappingKeyPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    !(ValidAes128WrappingKeyPartition.IsInPartition(x) || ValidAes192WrappingKeyPartition.IsInPartition(x) || ValidAes256WrappingKeyPartition.IsInPartition(x))
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [[], [1]];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened UInt = StandardLibrary.UInt

  import ValidAes128WrappingKeyPartition

  import ValidAes192WrappingKeyPartition

  import ValidAes256WrappingKeyPartition

  type ValueType = seq<uint8>
}

module AesWrappingKeyPartitioningScheme refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    [ValidAesWrappingKeyPartition.IsInPartition(x), InvalidAesWrappingKeyPartition.IsInPartition(x)]
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var validAes := ValidAesWrappingKeyPartition.RepresentativeValues();
    var invalidAes := InvalidAesWrappingKeyPartition.RepresentativeValues();
    output := validAes + invalidAes;
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    String(Base64.Encode(x))
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened UInt = StandardLibrary.UInt

  import ValidAesWrappingKeyPartition

  import InvalidAesWrappingKeyPartition

  import Base64

  type ValueType = seq<uint8>

  import opened Values = JSON.Values
}

module WrappingAlgValues refines TestVectorsNameTBDModule.EnumeratedValueSpace {
  function method ValueSpaceList(): seq<ValueType>
    ensures |ValueSpaceList()| > 0
    ensures forall i: int {:trigger ValueSpaceList()[i]} :: 0 <= i < |ValueSpaceList()| ==> IsValidMember(ValueSpaceList()[i])
  {
    [MPLTypes.ALG_AES128_GCM_IV12_TAG16(), MPLTypes.ALG_AES192_GCM_IV12_TAG16(), MPLTypes.ALG_AES256_GCM_IV12_TAG16()]
  }

  function method ValueSpaceEqualityList(x: ValueType): seq<bool>
    ensures |ValueSpaceEqualityList(x)| > 0
    ensures CountTrues(ValueSpaceEqualityList(x)) == 1
    decreases x
  {
    assert CountTrues([MPLTypes.ALG_AES128_GCM_IV12_TAG16() == MPLTypes.ALG_AES128_GCM_IV12_TAG16(), MPLTypes.ALG_AES128_GCM_IV12_TAG16() == MPLTypes.ALG_AES192_GCM_IV12_TAG16(), MPLTypes.ALG_AES128_GCM_IV12_TAG16() == MPLTypes.ALG_AES256_GCM_IV12_TAG16()]) == 1;
    [x == MPLTypes.ALG_AES128_GCM_IV12_TAG16(), x == MPLTypes.ALG_AES192_GCM_IV12_TAG16(), x == MPLTypes.ALG_AES256_GCM_IV12_TAG16()]
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    if x == MPLTypes.ALG_AES128_GCM_IV12_TAG16() then
      String(""ALG_AES128_GCM_IV12_TAG16"")
    else if x == MPLTypes.ALG_AES192_GCM_IV12_TAG16() then
      String(""ALG_AES192_GCM_IV12_TAG16"")
    else if x == MPLTypes.ALG_AES256_GCM_IV12_TAG16() then
      String(""ALG_AES256_GCM_IV12_TAG16"")
    else
      String(""UnknownWrappingAlg"")
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsExactlyOneValueInValueSpace(x: ValueType): bool
    ensures IsExactlyOneValueInValueSpace(x) ==> exists i: int {:trigger ValueSpaceEqualityList(x)[i]} :: 0 <= i < |ValueSpaceEqualityList(x)| && ValueSpaceEqualityList(x)[i] && forall j: int {:trigger ValueSpaceEqualityList(x)[j]} :: 0 <= j < |ValueSpaceEqualityList(x)| && j != i ==> !ValueSpaceEqualityList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := ValueSpaceEqualityList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsExactlyOneValueInValueSpace(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := ValueSpaceList();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import MPLTypes = AwsCryptographyMaterialProvidersTypes

  type ValueType = MPLTypes.AesWrappingAlg

  import opened Values = JSON.Values
}

module KeyNamespacePartitioningScheme refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    [InvalidRawAesKeyNamespacePartition.IsInPartition(x), ValidRawAesKeyNamespacePartition.IsInPartition(x)]
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var values1 := InvalidRawAesKeyNamespacePartition.RepresentativeValues();
    var values2 := ValidRawAesKeyNamespacePartition.RepresentativeValues();
    output := values1 + values2;
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    String(x)
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import InvalidRawAesKeyNamespacePartition

  import ValidRawAesKeyNamespacePartition

  type ValueType = string

  import opened Values = JSON.Values
}

module InvalidRawAesKeyNamespacePartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    x == ""aws-kms""
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [""aws-kms""];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  type ValueType = string
}

module ValidRawAesKeyNamespacePartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    x != ""aws-kms""
  }

  function method FilterNonAwsKms(s: seq<ValueType>): seq<ValueType>
    ensures forall i: int {:trigger FilterNonAwsKms(s)[i]} :: 0 <= i < |FilterNonAwsKms(s)| ==> FilterNonAwsKms(s)[i] != ""aws-kms""
    decreases |s|
  {
    if |s| == 0 then
      []
    else if s[0] == ""aws-kms"" then
      FilterNonAwsKms(s[1..])
    else
      [s[0]] + FilterNonAwsKms(s[1..])
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    var values := CharacterValuePartitioning.RepresentativeValues();
    output := FilterNonAwsKms(values);
    assume {:axiom} |output| > 0;
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import CharacterValuePartitioning

  type ValueType = string
}

module KeyNamePartitioningScheme refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    [CharacterValuePartitioning.IsInExactlyOnePartition(x)]
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    output := CharacterValuePartitioning.RepresentativeValues();
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    String(x)
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import CharacterValuePartitioning

  type ValueType = string

  import opened Values = JSON.Values
}

module CreateRawAesKeyringInput refines AbstractCreateRawAesKeyringInput {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    [AesWrappingKeyPartitioningScheme.IsValidMember(x.wrappingKey) && WrappingAlgValues.IsValidMember(x.wrappingAlg) && KeyNamespacePartitioningScheme.IsValidMember(x.keyNamespace) && KeyNamePartitioningScheme.IsValidMember(x.keyName)]
  }

  function method MatchesSomePruningConfiguration(x: ValueType): bool
    decreases x
  {
    InvalidAes128PruningConfiguration.MatchesPruningConfiguration(x) || InvalidAes192PruningConfiguration.MatchesPruningConfiguration(x) || InvalidAes256PruningConfiguration.MatchesPruningConfiguration(x)
  }

  method ReplaceList(x: ValueType) returns (output: ValueType)
    decreases x
  {
    if InvalidAes128PruningConfiguration.MatchesPruningConfiguration(x) {
      output := InvalidAes128PruningConfiguration.RepresentativeValue(x);
      return output;
    } else if InvalidAes192PruningConfiguration.MatchesPruningConfiguration(x) {
      output := InvalidAes192PruningConfiguration.RepresentativeValue(x);
      return output;
    } else if InvalidAes256PruningConfiguration.MatchesPruningConfiguration(x) {
      output := InvalidAes256PruningConfiguration.RepresentativeValue(x);
      return output;
    } else {
      return x;
    }
  }

  method MaybeAddValToVals(x: ValueType, vals: seq<ValueType>) returns (output: seq<ValueType>)
    decreases x, vals
  {
    var replace := ReplaceList(x);
    if x == replace {
      return vals + [x];
    } else {
      return vals;
    }
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var wrappingKeys := AesWrappingKeyPartitioningScheme.RepresentativeValues();
    var wrappingAlgs := WrappingAlgValues.ValueSpaceList();
    var keyNamespaces := KeyNamespacePartitioningScheme.RepresentativeValues();
    var keyNames := KeyNamePartitioningScheme.RepresentativeValues();
    output := [];
    assert |wrappingAlgs| > 0;
    assert |wrappingKeys| > 0;
    assert |keyNamespaces| > 0;
    assert |keyNames| > 0;
    for i: int := 0 to |wrappingAlgs| {
      for j: int := 0 to |wrappingKeys| {
        for k: int := 0 to |keyNamespaces| {
          for l: int := 0 to |keyNames| {
            var keyring := MPLTypes.CreateRawAesKeyringInput(wrappingKey := wrappingKeys[j], wrappingAlg := wrappingAlgs[i], keyNamespace := keyNamespaces[k], keyName := keyNames[l]);
            output := MaybeAddValToVals(keyring, output);
          }
        }
      }
    }
    assert |output| > 0 by {
      assume false;
    }
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    Object([(""wrappingKey"", AesWrappingKeyPartitioningScheme.ToJSON(x.wrappingKey)), (""wrappingAlg"", WrappingAlgValues.ToJSON(x.wrappingAlg)), (""keyNamespace"", KeyNamespacePartitioningScheme.ToJSON(x.keyNamespace)), (""keyName"", KeyNamePartitioningScheme.ToJSON(x.keyName))])
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import AesWrappingKeyPartitioningScheme

  import WrappingAlgValues

  import KeyNamespacePartitioningScheme

  import KeyNamePartitioningScheme

  import MPLTypes = AwsCryptographyMaterialProvidersTypes

  import InvalidAes128PruningConfiguration

  import InvalidAes192PruningConfiguration

  import InvalidAes256PruningConfiguration

  import AwsCryptographyMaterialProvidersTypes

  type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  import opened Values = JSON.Values
}

module InvalidAes128PruningConfiguration refines TestVectorsNameTBDModule.PruningConfiguration {
  function method MatchesPruningConfiguration(x: ValueType): bool
    decreases x
  {
    x.wrappingAlg == AwsCryptographyMaterialProvidersTypes.ALG_AES128_GCM_IV12_TAG16() &&
    !ValidAes128WrappingKeyPartition.IsInPartition(x.wrappingKey)
  }

  method RepresentativeValue(input: ValueType) returns (output: ValueType)
    requires MatchesPruningConfiguration(input)
    ensures MatchesPruningConfiguration(output)
    decreases input
  {
    var representativeKeyNamespace := ValidRawAesKeyNamespacePartition.RepresentativeValues();
    var representativeKeyName := KeyNamePartitioningScheme.RepresentativeValues();
    output := AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput(wrappingAlg := input.wrappingAlg, wrappingKey := input.wrappingKey, keyNamespace := representativeKeyNamespace[0], keyName := representativeKeyName[0]);
  }

  method MaybeReplaceWithRepresentativeValue(x: ValueType) returns (output: ValueType)
    ensures !MatchesPruningConfiguration(x) ==> output == x
    ensures MatchesPruningConfiguration(x) ==> MatchesPruningConfiguration(output)
    decreases x
  {
    if MatchesPruningConfiguration(x) {
      output := RepresentativeValue(x);
    } else {
      output := x;
    }
  }

  import AwsCryptographyMaterialProvidersTypes

  type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  import ValidAes128WrappingKeyPartition

  import ValidRawAesKeyNamespacePartition

  import KeyNamePartitioningScheme
}

module InvalidAes192PruningConfiguration refines TestVectorsNameTBDModule.PruningConfiguration {
  function method MatchesPruningConfiguration(x: ValueType): bool
    decreases x
  {
    x.wrappingAlg == AwsCryptographyMaterialProvidersTypes.ALG_AES192_GCM_IV12_TAG16() &&
    !ValidAes192WrappingKeyPartition.IsInPartition(x.wrappingKey)
  }

  method RepresentativeValue(input: ValueType) returns (output: ValueType)
    requires MatchesPruningConfiguration(input)
    ensures MatchesPruningConfiguration(output)
    decreases input
  {
    var representativeKeyNamespace := ValidRawAesKeyNamespacePartition.RepresentativeValues();
    var representativeKeyName := KeyNamePartitioningScheme.RepresentativeValues();
    output := AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput(wrappingAlg := input.wrappingAlg, wrappingKey := input.wrappingKey, keyNamespace := representativeKeyNamespace[0], keyName := representativeKeyName[0]);
  }

  method MaybeReplaceWithRepresentativeValue(x: ValueType) returns (output: ValueType)
    ensures !MatchesPruningConfiguration(x) ==> output == x
    ensures MatchesPruningConfiguration(x) ==> MatchesPruningConfiguration(output)
    decreases x
  {
    if MatchesPruningConfiguration(x) {
      output := RepresentativeValue(x);
    } else {
      output := x;
    }
  }

  import AwsCryptographyMaterialProvidersTypes

  type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  import ValidAes192WrappingKeyPartition

  import ValidRawAesKeyNamespacePartition

  import KeyNamePartitioningScheme
}

module InvalidAes256PruningConfiguration refines TestVectorsNameTBDModule.PruningConfiguration {
  function method MatchesPruningConfiguration(x: ValueType): bool
    decreases x
  {
    x.wrappingAlg == AwsCryptographyMaterialProvidersTypes.ALG_AES256_GCM_IV12_TAG16() &&
    !ValidAes256WrappingKeyPartition.IsInPartition(x.wrappingKey)
  }

  method RepresentativeValue(input: ValueType) returns (output: ValueType)
    requires MatchesPruningConfiguration(input)
    ensures MatchesPruningConfiguration(output)
    decreases input
  {
    var representativeKeyNamespace := ValidRawAesKeyNamespacePartition.RepresentativeValues();
    var representativeKeyName := KeyNamePartitioningScheme.RepresentativeValues();
    output := AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput(wrappingAlg := input.wrappingAlg, wrappingKey := input.wrappingKey, keyNamespace := representativeKeyNamespace[0], keyName := representativeKeyName[0]);
  }

  method MaybeReplaceWithRepresentativeValue(x: ValueType) returns (output: ValueType)
    ensures !MatchesPruningConfiguration(x) ==> output == x
    ensures MatchesPruningConfiguration(x) ==> MatchesPruningConfiguration(output)
    decreases x
  {
    if MatchesPruningConfiguration(x) {
      output := RepresentativeValue(x);
    } else {
      output := x;
    }
  }

  import AwsCryptographyMaterialProvidersTypes

  type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  import ValidAes256WrappingKeyPartition

  import ValidRawAesKeyNamespacePartition

  import KeyNamePartitioningScheme
}

module CreateRawAesKeyringSmokeTest refines AbstractCreateRawAesKeyringSmokeTest {
  method EvaluationRule(x: Domain) returns (output: Range)
    decreases x
  {
    if !ValidAesWrappingKeyPartition.IsInPartition(x.wrappingKey) {
      output := ""Invalid wrapping key length"";
      return;
    }
    if x.wrappingAlg == MPLTypes.ALG_AES128_GCM_IV12_TAG16() && !ValidAes128WrappingKeyPartition.IsInPartition(x.wrappingKey) {
      output := ""Invalid wrapping key length"";
      return;
    } else if x.wrappingAlg == MPLTypes.ALG_AES192_GCM_IV12_TAG16() && !ValidAes192WrappingKeyPartition.IsInPartition(x.wrappingKey) {
      output := ""Invalid wrapping key length"";
      return;
    } else if x.wrappingAlg == MPLTypes.ALG_AES256_GCM_IV12_TAG16() && !ValidAes256WrappingKeyPartition.IsInPartition(x.wrappingKey) {
      output := ""Invalid wrapping key length"";
      return;
    }
    if x.keyNamespace == ""aws-kms"" {
      output := ""Invalid key namespace for raw AES keyring"";
      return;
    }
    output := ""ok"";
  }

  function method InputToJSON(x: Domain): JSON
    decreases x
  {
    CreateRawAesKeyringInput.ToJSON(x)
  }

  function method OutputToJSON(x: Range): JSON
    decreases x
  {
    String(x)
  }

  method DomainRepresentativeValues() returns (output: seq<Domain>)
    ensures |output| > 0
  {
    output := CreateRawAesKeyringInput.AllValues();
  }

  const operationName: string := ""CreateRawAesKeyringSmokeTest""

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := [];
    var inputValues := DomainRepresentativeValues();
    for i: int := 0 to |inputValues| {
      var input := inputValues[i];
      var json := JSONToWriteForOperationInputValue(input);
      operationsJSON := operationsJSON + [json];
    }
    outputJSON := Object([(operationName, Array(operationsJSON))]);
  }

  method JSONToWriteForOperationInputValue(input: Domain) returns (output: JSON)
    decreases input
  {
    var outputValue := EvaluationRule(input);
    var inputJSON := InputToJSON(input);
    var outputJSON := OutputToJSON(outputValue);
    output := Object([(""input"", inputJSON), (""output"", outputJSON)]);
  }

  import CreateRawAesKeyringInput

  import AesWrappingKeyPartitioningScheme

  import ValidAesWrappingKeyPartition

  import ValidAes128WrappingKeyPartition

  import ValidAes192WrappingKeyPartition

  import ValidAes256WrappingKeyPartition

  import WrappingAlgValues

  import KeyNamespacePartitioningScheme

  import KeyNamePartitioningScheme

  import MPLTypes = AwsCryptographyMaterialProvidersTypes

  import Base64

  import OptionStringDomainModel

  import AwsCryptographyMaterialProvidersTypes

  import Wrappers

  type Domain = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  type Range = string

  import opened Values = JSON.Values
}

module CreateRawAesKeyringTestService refines AbstractCreateRawAesKeyringTestService {
  method GenerateOperationsJSON() returns (operationsJSON: JSON)
  {
    var CreateRawAesKeyringSmokeTestJSON := CreateRawAesKeyringSmokeTest.GenerateJSON();
    operationsJSON := Array([CreateRawAesKeyringSmokeTestJSON]);
  }

  const serviceName: string := ""CreateRawAesKeyringTestService""
  const serviceNamespace: string

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := GenerateOperationsJSON();
    outputJSON := Object([(serviceNamespace + "":"" + serviceName, operationsJSON)]);
  }

  import CreateRawAesKeyringInput

  import CreateRawAesKeyringSmokeTest

  import AwsCryptographyMaterialProvidersTypes

  import Wrappers

  import opened Values = JSON.Values
}

module Tests3 {
  method {:test} TestVectorsV2()
  {
    var keyringsInputJSON := CreateRawAesKeyringInput.GenerateJSON();
    var out := TestVectorsNameTBDModule.WriteVectorsFile(""outkeyringsonly.json"", keyringsInputJSON);
    var outputJSON := CreateRawAesKeyringTestService.GenerateJSON();
    var out2 := TestVectorsNameTBDModule.WriteVectorsFile(""outkeyrings.json"", outputJSON);
  }

  import CreateRawAesKeyringInput

  import CreateRawAesKeyringTestService

  import TestVectorsNameTBDModule
}

module EmptyStringPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| == 0
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [""""];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  type ValueType = string
}

module NonemptyStringPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| > 0
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [""abc""];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  type ValueType = string
}

module LengthPartitioning refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    [EmptyStringPartition.IsInPartition(x), NonemptyStringPartition.IsInPartition(x)]
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var values1 := EmptyStringPartition.RepresentativeValues();
    var values2 := NonemptyStringPartition.RepresentativeValues();
    output := values1 + values2;
  }

  function DomainValueToString(input: ValueType): string
    decreases input
  {
    input
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    String(x)
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import EmptyStringPartition

  import NonemptyStringPartition

  type ValueType = string

  import opened Values = JSON.Values
}

module AsciiPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| > 0 &&
    forall i: int {:trigger x[i]} :: 
      0 <= i < |x| ==>
        x[i] as int <= 127
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [""abc""];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  type ValueType = string
}

module BMPPartition refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    |x| > 0 &&
    exists i: int {:trigger x[i]} :: 
      0 <= i < |x| &&
      x[i] as int > 127 &&
      x[i] as int <= 65535
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    var value := ""Hëllø \uD83C\uDF0D — こんにちは世界 — مرحبا بالعالم — Здравствуй, мир!"";
    assert value[1] as int > 127 && value[1] as int <= 65535;
    output := [value];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  type ValueType = string
}

module CharacterValuePartitioning refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    assert CountTrues([EmptyStringPartition.IsInPartition(""""), AsciiPartition.IsInPartition(""""), BMPPartition.IsInPartition("""")]) == 1;
    [EmptyStringPartition.IsInPartition(x), AsciiPartition.IsInPartition(x), BMPPartition.IsInPartition(x)]
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var values1 := EmptyStringPartition.RepresentativeValues();
    var values2 := AsciiPartition.RepresentativeValues();
    var values3 := BMPPartition.RepresentativeValues();
    output := values1 + values2 + values3;
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    String(x)
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import EmptyStringPartition

  import AsciiPartition

  import BMPPartition

  type ValueType = string

  import opened Values = JSON.Values
}

module OptionStringNone refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    x.None?
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    output := [Option.None];
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened Wrappers

  import opened Values = JSON.Values

  type ValueType = Option<string>
}

module OptionStringSome refines TestVectorsNameTBDModule.Partition {
  function method IsInPartition(x: ValueType): bool
    decreases x
  {
    x.Some?
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
  {
    var values1 := LengthPartitioning.AllValues();
    var values2 := CharacterValuePartitioning.AllValues();
    var values := values1 + values2;
    var temp: seq<ValueType> := [];
    var i := 0;
    while i < |values|
      invariant 0 <= i <= |values|
      invariant |temp| == i
      invariant forall j: int {:trigger values[j]} {:trigger temp[j]} :: 0 <= j < i ==> temp[j] == Option.Some(values[j])
      decreases |values| - i
    {
      temp := temp + [Option.Some(values[i])];
      i := i + 1;
    }
    output := temp;
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    decreases x
  {
    IsInPartition(x)
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
  {
    output := RepresentativeValues();
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened Wrappers

  import LengthPartitioning

  import CharacterValuePartitioning

  import opened Values = JSON.Values

  type ValueType = Option<string>
}

module OptionStringDomainModel refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    [OptionStringNone.IsInPartition(x), OptionStringSome.IsInPartition(x)]
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var values1 := OptionStringNone.RepresentativeValues();
    var values2 := OptionStringSome.RepresentativeValues();
    output := values1 + values2;
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    if x.None? then
      Object([(""None"", Null)])
    else
      Object([(""Some"", String(x.value))])
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import opened Wrappers

  import OptionStringNone

  import OptionStringSome

  type ValueType = Option<string>

  import opened Values = JSON.Values
}

module GetStringInputPartitioning refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x
  {
    OptionStringDomainModel.IsInPartitionList(x.value)
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var values := OptionStringDomainModel.AllValues();
    var temp: seq<ValueType> := [];
    var i := 0;
    while i < |values|
      invariant 0 <= i <= |values|
      invariant |temp| == i
      decreases |values| - i
    {
      temp := temp + [SimpleTypesSmithyStringTypes.GetStringInput(value := values[i])];
      i := i + 1;
    }
    output := temp;
  }

  function method ToJSON(x: ValueType): JSON
    decreases x
  {
    Object([(""value"", OptionStringDomainModel.ToJSON(x.value))])
  }

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import SimpleTypesSmithyStringTypes

  import OptionStringDomainModel

  type ValueType = SimpleTypesSmithyStringTypes.GetStringInput

  import opened Values = JSON.Values
}

abstract module AbstractGetStringModel refines TestVectorsNameTBDModule.OperationModel {
  const operationName: string := ""GetString""

  method EvaluationRule(x: Domain) returns (output: Range)
    decreases x

  method DomainRepresentativeValues() returns (output: seq<Domain>)
    ensures |output| > 0

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := [];
    var inputValues := DomainRepresentativeValues();
    for i: int := 0 to |inputValues| {
      var input := inputValues[i];
      var json := JSONToWriteForOperationInputValue(input);
      operationsJSON := operationsJSON + [json];
    }
    outputJSON := Object([(operationName, Array(operationsJSON))]);
  }

  function method InputToJSON(x: Domain): JSON
    decreases x

  function method OutputToJSON(x: Range): JSON
    decreases x

  method JSONToWriteForOperationInputValue(input: Domain) returns (output: JSON)
    decreases input
  {
    var outputValue := EvaluationRule(input);
    var inputJSON := InputToJSON(input);
    var outputJSON := OutputToJSON(outputValue);
    output := Object([(""input"", inputJSON), (""output"", outputJSON)]);
  }

  import SimpleTypesSmithyStringTypes

  type Domain = SimpleTypesSmithyStringTypes.GetStringInput

  type Range = SimpleTypesSmithyStringTypes.GetStringOutput

  import opened Values = JSON.Values
}

module GetStringModelImpl refines AbstractGetStringModel {
  method DomainRepresentativeValues() returns (output: seq<Domain>)
    ensures |output| > 0
  {
    output := GetStringInputPartitioning.RepresentativeValues();
  }

  method EvaluationRule(x: Domain) returns (output: Range)
    decreases x
  {
    output := SimpleTypesSmithyStringTypes.GetStringOutput(value := x.value);
  }

  function method InputToJSON(x: Domain): JSON
    decreases x
  {
    GetStringInputPartitioning.ToJSON(x)
  }

  function method OutputToJSON(x: Range): JSON
    decreases x
  {
    Object([(""value"", OptionStringDomainModel.ToJSON(x.value))])
  }

  const operationName: string := ""GetString""

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := [];
    var inputValues := DomainRepresentativeValues();
    for i: int := 0 to |inputValues| {
      var input := inputValues[i];
      var json := JSONToWriteForOperationInputValue(input);
      operationsJSON := operationsJSON + [json];
    }
    outputJSON := Object([(operationName, Array(operationsJSON))]);
  }

  method JSONToWriteForOperationInputValue(input: Domain) returns (output: JSON)
    decreases input
  {
    var outputValue := EvaluationRule(input);
    var inputJSON := InputToJSON(input);
    var outputJSON := OutputToJSON(outputValue);
    output := Object([(""input"", inputJSON), (""output"", outputJSON)]);
  }

  import GetStringInputPartitioning

  import OptionStringDomainModel

  import SimpleTypesSmithyStringTypes

  type Domain = SimpleTypesSmithyStringTypes.GetStringInput

  type Range = SimpleTypesSmithyStringTypes.GetStringOutput

  import opened Values = JSON.Values
}

module Tests2 {
  method {:test} TestVectorsV2()
  {
    var outputJSON := SimpleStringServiceModel.GenerateJSON();
    var out := TestVectorsNameTBDModule.WriteVectorsFile(""out.json"", outputJSON);
  }

  import GetStringModelImpl

  import SimpleString

  import opened SimpleTypesSmithyStringTypes

  import opened Wrappers

  import UTF8

  import SimpleStringServiceModel

  import TestVectorsNameTBDModule

  import opened Values = JSON.Values
}

abstract module AbstractSimpleStringServiceModel refines TestVectorsNameTBDModule.ServiceModel {
  const serviceName: string := ""SimpleString""
  const serviceNamespace: string := ""simple.types.smithyString""

  method GenerateOperationsJSON() returns (operationsJSON: JSON)

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := GenerateOperationsJSON();
    outputJSON := Object([(serviceNamespace + "":"" + serviceName, operationsJSON)]);
  }

  import opened Values = JSON.Values
}

module SimpleStringServiceModel refines AbstractSimpleStringServiceModel {
  method GenerateOperationsJSON() returns (operationsJSON: JSON)
  {
    var GetStringJSON := GetStringModelImpl.GenerateJSON();
    operationsJSON := Array([GetStringJSON]);
  }

  const serviceName: string := ""SimpleString""
  const serviceNamespace: string := ""simple.types.smithyString""

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := GenerateOperationsJSON();
    outputJSON := Object([(serviceNamespace + "":"" + serviceName, operationsJSON)]);
  }

  import GetStringModelImpl

  import opened Values = JSON.Values
}

module SimpleString refines AbstractSimpleTypesSmithyStringService {
  function method DefaultSimpleStringConfig(): SimpleStringConfig
  {
    SimpleStringConfig
  }

  method SimpleString(config: SimpleStringConfig := DefaultSimpleStringConfig()) returns (res: Result<SimpleStringClient, Error>)
    ensures res.Success? ==> fresh(res.value) && fresh(res.value.Modifies) && fresh(res.value.History) && res.value.ValidState()
    decreases config
  {
    var client := new SimpleStringClient(Operations.Config);
    return Success(client);
  }

  function method CreateSuccessOfClient(client: ISimpleTypesStringClient): Result<ISimpleTypesStringClient, Error>
    decreases client
  {
    Success(client)
  }

  function method CreateFailureOfError(error: Error): Result<ISimpleTypesStringClient, Error>
    decreases error
  {
    Failure(error)
  }

  import Operations = SimpleStringImpl

  class SimpleStringClient ... {
    predicate ValidState()
      ensures ValidState() ==> Operations.ValidInternalConfig?(config) && History !in Operations.ModifiesInternalConfig(config) && Modifies == Operations.ModifiesInternalConfig(config) + {History}
    {
      Operations.ValidInternalConfig?(config) &&
      Modifies == Operations.ModifiesInternalConfig(config) + {History}
    }

    constructor (config: Operations.InternalConfig)
      requires Operations.ValidInternalConfig?(config)
      ensures ValidState() && fresh(History) && this.config == config
      decreases config
    {
      this.config := config;
      History := new ISimpleTypesStringClientCallHistory();
      Modifies := Operations.ModifiesInternalConfig(config) + {History};
    }

    const config: Operations.InternalConfig

    predicate GetStringEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
      decreases input, output
    {
      Operations.GetStringEnsuresPublicly(input, output)
    }

    method GetString(input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetString
      ensures true && ValidState()
      ensures GetStringEnsuresPublicly(input, output)
      ensures History.GetString == old(History.GetString) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetString(config, input);
      History.GetString := History.GetString + [DafnyCallEvent(input, output)];
    }

    predicate GetStringKnownValueEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
      decreases input, output
    {
      Operations.GetStringKnownValueEnsuresPublicly(input, output)
    }

    method GetStringKnownValue(input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringKnownValue
      ensures true && ValidState()
      ensures GetStringKnownValueEnsuresPublicly(input, output)
      ensures History.GetStringKnownValue == old(History.GetStringKnownValue) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetStringKnownValue(config, input);
      History.GetStringKnownValue := History.GetStringKnownValue + [DafnyCallEvent(input, output)];
    }

    predicate GetStringUTF8EnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
      decreases input, output
    {
      Operations.GetStringUTF8EnsuresPublicly(input, output)
    }

    method GetStringUTF8(input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringUTF8
      ensures true && ValidState()
      ensures GetStringUTF8EnsuresPublicly(input, output)
      ensures History.GetStringUTF8 == old(History.GetStringUTF8) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetStringUTF8(config, input);
      History.GetStringUTF8 := History.GetStringUTF8 + [DafnyCallEvent(input, output)];
    }

    predicate GetStringUTF8KnownValueEnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
      decreases input, output
    {
      Operations.GetStringUTF8KnownValueEnsuresPublicly(input, output)
    }

    method GetStringUTF8KnownValue(input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringUTF8KnownValue
      ensures true && ValidState()
      ensures GetStringUTF8KnownValueEnsuresPublicly(input, output)
      ensures History.GetStringUTF8KnownValue == old(History.GetStringUTF8KnownValue) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetStringUTF8KnownValue(config, input);
      History.GetStringUTF8KnownValue := History.GetStringUTF8KnownValue + [DafnyCallEvent(input, output)];
    }
  }

  import opened Wrappers

  import opened UInt = StandardLibrary.UInt

  import opened UTF8

  import opened Types = SimpleTypesSmithyStringTypes
}

module SimpleStringImpl refines AbstractSimpleTypesSmithyStringOperations {
  predicate ValidInternalConfig?(config: InternalConfig)
    decreases config
  {
    true
  }

  function ModifiesInternalConfig(config: InternalConfig): set<object>
    decreases config
  {
    {}
  }

  predicate GetStringEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
    decreases input, output
  {
    true
  }

  predicate GetStringKnownValueEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
    decreases input, output
  {
    true
  }

  predicate GetStringUTF8EnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
    decreases input, output
  {
    true
  }

  predicate GetStringUTF8KnownValueEnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
    decreases input, output
  {
    true
  }

  method GetString(config: InternalConfig, input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringEnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)
  {
    var res := GetStringOutput(value := input.value);
    return Success(res);
  }

  method GetStringKnownValue(config: InternalConfig, input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringKnownValueEnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)
  {
    expect input.value.Some?, ""expectation violation"";
    expect input.value.value == ""TEST_SIMPLE_STRING_KNOWN_VALUE"", ""expectation violation"";
    var res := GetStringOutput(value := input.value);
    return Success(res);
  }

  method GetStringUTF8(config: InternalConfig, input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringUTF8EnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)
  {
    expect input.value.Some?, ""expectation violation"";
    var res := GetStringUTF8Output(value := input.value);
    return Success(res);
  }

  method GetStringUTF8KnownValue(config: InternalConfig, input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringUTF8KnownValueEnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)
  {
    expect input.value.Some?, ""expectation violation"";
    var expected := [72, 101, 108, 108, 111];
    expect input.value.value == expected, ""expectation violation"";
    var res := GetStringUTF8Output(value := input.value);
    return Success(res);
  }

  datatype Config = Config

  type InternalConfig = Config

  import opened Wrappers

  import opened UInt = StandardLibrary.UInt

  import opened UTF8

  import opened Types = SimpleTypesSmithyStringTypes
}

module SimpleTypesSmithyStringTypes {
  predicate method IsDummySubsetType(x: int)
    decreases x
  {
    0 < x
  }

  import opened Wrappers

  import opened UInt = StandardLibrary.UInt

  import opened UTF8

  datatype DafnyCallEvent<I, O> = DafnyCallEvent(input: I, output: O)

  datatype GetStringInput = GetStringInput(nameonly value: Option<string> := Option.None)

  datatype GetStringOutput = GetStringOutput(nameonly value: Option<string> := Option.None)

  datatype GetStringUTF8Input = GetStringUTF8Input(nameonly value: Option<UTF8Bytes> := Option.None)

  datatype GetStringUTF8Output = GetStringUTF8Output(nameonly value: Option<UTF8Bytes> := Option.None)

  datatype SimpleStringConfig = SimpleStringConfig

  class ISimpleTypesStringClientCallHistory {
    ghost constructor ()
    {
      GetString := [];
      GetStringKnownValue := [];
      GetStringUTF8 := [];
      GetStringUTF8KnownValue := [];
    }

    ghost var GetString: seq<DafnyCallEvent<GetStringInput, Result<GetStringOutput, Error>>>
    ghost var GetStringKnownValue: seq<DafnyCallEvent<GetStringInput, Result<GetStringOutput, Error>>>
    ghost var GetStringUTF8: seq<DafnyCallEvent<GetStringUTF8Input, Result<GetStringUTF8Output, Error>>>
    ghost var GetStringUTF8KnownValue: seq<DafnyCallEvent<GetStringUTF8Input, Result<GetStringUTF8Output, Error>>>
  }

  trait {:termination false} ISimpleTypesStringClient {
    ghost const Modifies: set<object>

    predicate ValidState()
      ensures ValidState() ==> History in Modifies

    ghost const History: ISimpleTypesStringClientCallHistory

    predicate GetStringEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
      decreases input, output

    method GetString(input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetString
      ensures true && ValidState()
      ensures GetStringEnsuresPublicly(input, output)
      ensures History.GetString == old(History.GetString) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}

    predicate GetStringKnownValueEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
      decreases input, output

    method GetStringKnownValue(input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringKnownValue
      ensures true && ValidState()
      ensures GetStringKnownValueEnsuresPublicly(input, output)
      ensures History.GetStringKnownValue == old(History.GetStringKnownValue) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}

    predicate GetStringUTF8EnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
      decreases input, output

    method GetStringUTF8(input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringUTF8
      ensures true && ValidState()
      ensures GetStringUTF8EnsuresPublicly(input, output)
      ensures History.GetStringUTF8 == old(History.GetStringUTF8) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}

    predicate GetStringUTF8KnownValueEnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
      decreases input, output

    method GetStringUTF8KnownValue(input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringUTF8KnownValue
      ensures true && ValidState()
      ensures GetStringUTF8KnownValueEnsuresPublicly(input, output)
      ensures History.GetStringUTF8KnownValue == old(History.GetStringUTF8KnownValue) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
  }

  type UTF8Bytes = ValidUTF8Bytes

  datatype Error = CollectionOfErrors(list: seq<Error>, nameonly message: string) | Opaque(obj: object) | OpaqueWithText(obj: object, objMessage: string)

  type OpaqueError = e: Error
    | e.Opaque? || e.OpaqueWithText?
    witness *

  type DummySubsetType = x: int
    | IsDummySubsetType(x)
    witness 1
}

abstract module AbstractSimpleTypesSmithyStringService {
  function method DefaultSimpleStringConfig(): SimpleStringConfig

  method SimpleString(config: SimpleStringConfig := DefaultSimpleStringConfig()) returns (res: Result<SimpleStringClient, Error>)
    ensures res.Success? ==> fresh(res.value) && fresh(res.value.Modifies) && fresh(res.value.History) && res.value.ValidState()
    decreases config

  function method CreateSuccessOfClient(client: ISimpleTypesStringClient): Result<ISimpleTypesStringClient, Error>
    decreases client
  {
    Success(client)
  }

  function method CreateFailureOfError(error: Error): Result<ISimpleTypesStringClient, Error>
    decreases error
  {
    Failure(error)
  }

  import opened Wrappers

  import opened UInt = StandardLibrary.UInt

  import opened UTF8

  import opened Types = SimpleTypesSmithyStringTypes

  import Operations : AbstractSimpleTypesSmithyStringOperations

  class SimpleStringClient extends ISimpleTypesStringClient {
    constructor (config: Operations.InternalConfig)
      requires Operations.ValidInternalConfig?(config)
      ensures ValidState() && fresh(History) && this.config == config

    const config: Operations.InternalConfig

    predicate ValidState()
      ensures ValidState() ==> Operations.ValidInternalConfig?(config) && History !in Operations.ModifiesInternalConfig(config) && Modifies == Operations.ModifiesInternalConfig(config) + {History}

    predicate GetStringEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
      decreases input, output
    {
      Operations.GetStringEnsuresPublicly(input, output)
    }

    method GetString(input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetString
      ensures true && ValidState()
      ensures GetStringEnsuresPublicly(input, output)
      ensures History.GetString == old(History.GetString) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetString(config, input);
      History.GetString := History.GetString + [DafnyCallEvent(input, output)];
    }

    predicate GetStringKnownValueEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
      decreases input, output
    {
      Operations.GetStringKnownValueEnsuresPublicly(input, output)
    }

    method GetStringKnownValue(input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringKnownValue
      ensures true && ValidState()
      ensures GetStringKnownValueEnsuresPublicly(input, output)
      ensures History.GetStringKnownValue == old(History.GetStringKnownValue) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetStringKnownValue(config, input);
      History.GetStringKnownValue := History.GetStringKnownValue + [DafnyCallEvent(input, output)];
    }

    predicate GetStringUTF8EnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
      decreases input, output
    {
      Operations.GetStringUTF8EnsuresPublicly(input, output)
    }

    method GetStringUTF8(input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringUTF8
      ensures true && ValidState()
      ensures GetStringUTF8EnsuresPublicly(input, output)
      ensures History.GetStringUTF8 == old(History.GetStringUTF8) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetStringUTF8(config, input);
      History.GetStringUTF8 := History.GetStringUTF8 + [DafnyCallEvent(input, output)];
    }

    predicate GetStringUTF8KnownValueEnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
      decreases input, output
    {
      Operations.GetStringUTF8KnownValueEnsuresPublicly(input, output)
    }

    method GetStringUTF8KnownValue(input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
      requires true && ValidState()
      modifies Modifies - {History}, History`GetStringUTF8KnownValue
      ensures true && ValidState()
      ensures GetStringUTF8KnownValueEnsuresPublicly(input, output)
      ensures History.GetStringUTF8KnownValue == old(History.GetStringUTF8KnownValue) + [DafnyCallEvent(input, output)]
      decreases Modifies - {History}
    {
      output := Operations.GetStringUTF8KnownValue(config, input);
      History.GetStringUTF8KnownValue := History.GetStringUTF8KnownValue + [DafnyCallEvent(input, output)];
    }
  }
}

abstract module AbstractSimpleTypesSmithyStringOperations {
  predicate ValidInternalConfig?(config: InternalConfig)

  function ModifiesInternalConfig(config: InternalConfig): set<object>

  predicate GetStringEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
    decreases input, output

  method GetString(config: InternalConfig, input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringEnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)

  predicate GetStringKnownValueEnsuresPublicly(input: GetStringInput, output: Result<GetStringOutput, Error>)
    decreases input, output

  method GetStringKnownValue(config: InternalConfig, input: GetStringInput) returns (output: Result<GetStringOutput, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringKnownValueEnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)

  predicate GetStringUTF8EnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
    decreases input, output

  method GetStringUTF8(config: InternalConfig, input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringUTF8EnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)

  predicate GetStringUTF8KnownValueEnsuresPublicly(input: GetStringUTF8Input, output: Result<GetStringUTF8Output, Error>)
    decreases input, output

  method GetStringUTF8KnownValue(config: InternalConfig, input: GetStringUTF8Input) returns (output: Result<GetStringUTF8Output, Error>)
    requires true && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures true && ValidInternalConfig?(config)
    ensures GetStringUTF8KnownValueEnsuresPublicly(input, output)
    decreases ModifiesInternalConfig(config)

  import opened Wrappers

  import opened UInt = StandardLibrary.UInt

  import opened UTF8

  import opened Types = SimpleTypesSmithyStringTypes

  type InternalConfig
}

module TestVectorsNameTBDModule {
  method WriteVectorsFile(location: string, json: JSON) returns (output: Result<(), string>)
    decreases location, json
  {
    var jsonBytes :- API.Serialize(json).MapFailure((e: Errors.SerializationError) => e.ToString());
    output := FileIO.WriteBytesToFile(location, jsonBytes);
  }

  import FileIO

  import opened Wrappers

  import opened UInt = StandardLibrary.UInt

  import opened Values = JSON.Values

  import API = JSON.API

  import Errors = JSON.Errors

  abstract module ValueSpace {
    function method IsValidMember(x: ValueType): bool
      ensures IsValidMember(x) ==> true && x is ValueType

    method AllValues() returns (output: seq<ValueType>)
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])

    method SomeValue() returns (output: ValueType)
      ensures IsValidMember(output)
    {
      var allValues := AllValues();
      output := allValues[0];
    }

    type ValueType
  }

  abstract module EnumeratedValueSpace refines ValueSpace {
    function method ValueSpaceList(): seq<ValueType>
      ensures |ValueSpaceList()| > 0
      ensures forall i: int {:trigger ValueSpaceList()[i]} :: 0 <= i < |ValueSpaceList()| ==> IsValidMember(ValueSpaceList()[i])

    function method CountTrues(partitions: seq<bool>): int
      ensures 0 <= CountTrues(partitions) <= |partitions|
      ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
      ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
      ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
      decreases partitions
    {
      if |partitions| == 0 then
        0
      else if partitions[0] then
        1 + CountTrues(partitions[1..])
      else
        CountTrues(partitions[1..])
    }

    function method ValueSpaceEqualityList(x: ValueType): seq<bool>
      ensures |ValueSpaceEqualityList(x)| > 0
      ensures CountTrues(ValueSpaceEqualityList(x)) == 1

    function method IsExactlyOneValueInValueSpace(x: ValueType): bool
      ensures IsExactlyOneValueInValueSpace(x) ==> exists i: int {:trigger ValueSpaceEqualityList(x)[i]} :: 0 <= i < |ValueSpaceEqualityList(x)| && ValueSpaceEqualityList(x)[i] && forall j: int {:trigger ValueSpaceEqualityList(x)[j]} :: 0 <= j < |ValueSpaceEqualityList(x)| && j != i ==> !ValueSpaceEqualityList(x)[j]
    {
      var partitionList: seq<bool> := ValueSpaceEqualityList(x);
      CountTrues(partitionList) == 1
    }

    function method IsValidMember(x: ValueType): bool
      ensures IsValidMember(x) ==> true && x is ValueType
    {
      x is ValueType &&
      IsExactlyOneValueInValueSpace(x)
    }

    method AllValues() returns (output: seq<ValueType>)
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    {
      output := ValueSpaceList();
    }

    method SomeValue() returns (output: ValueType)
      ensures IsValidMember(output)
    {
      var allValues := AllValues();
      output := allValues[0];
    }

    import opened Values = JSON.Values

    type ValueType(==,!new)
  }

  abstract module Partition refines ValueSpace {
    function method IsInPartition(x: ValueType): bool

    function method IsValidMember(x: ValueType): bool
      ensures IsValidMember(x) ==> true && x is ValueType
      ensures IsValidMember(x) ==> x is ValueType && IsInPartition(x)
    {
      IsInPartition(x)
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])

    method AllValues() returns (output: seq<ValueType>)
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    {
      output := RepresentativeValues();
    }

    method SomeValue() returns (output: ValueType)
      ensures IsValidMember(output)
    {
      var allValues := AllValues();
      output := allValues[0];
    }

    type ValueType
  }

  abstract module PartitioningScheme refines ValueSpace {
    function method IsInPartitionList(x: ValueType): seq<bool>
      ensures CountTrues(IsInPartitionList(x)) == 1

    function method CountTrues(partitions: seq<bool>): int
      ensures 0 <= CountTrues(partitions) <= |partitions|
      ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
      ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
      ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
      decreases partitions
    {
      if |partitions| == 0 then
        0
      else if partitions[0] then
        1 + CountTrues(partitions[1..])
      else
        CountTrues(partitions[1..])
    }

    function method IsInExactlyOnePartition(x: ValueType): bool
      ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    {
      var partitionList: seq<bool> := IsInPartitionList(x);
      CountTrues(partitionList) == 1
    }

    function method IsValidMember(x: ValueType): bool
      ensures IsValidMember(x) ==> true && x is ValueType
    {
      x is ValueType &&
      IsInExactlyOnePartition(x)
    }

    function method ToJSON(x: ValueType): JSON

    method GenerateJSON() returns (outputJSON: JSON)
    {
      var values := AllValues();
      var out := [];
      for i: int := 0 to |values| {
        var value := values[i];
        var json := ToJSON(value);
        out := out + [json];
      }
      outputJSON := Array(out);
    }

    method RepresentativeValues() returns (output: seq<ValueType>)
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])

    method AllValues() returns (output: seq<ValueType>)
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
    {
      var raw := RepresentativeValues();
      var seen := {};
      output := [];
      for i: int := 0 to |raw| {
        if raw[i] !in seen {
          output := output + [raw[i]];
          seen := seen + {raw[i]};
        }
      }
      assert |output| > 0 by {
        assert |raw| > 0;
        assert |seen| > 0 by {
          assume {:axiom} raw[0] in seen;
        }
        assert |seen| == |output| by {
          assume {:axiom} |output| == |seen|;
        }
      }
    }

    method SomeValue() returns (output: ValueType)
      ensures IsValidMember(output)
    {
      var allValues := AllValues();
      output := allValues[0];
    }

    import opened Values = JSON.Values

    type ValueType(==)
  }

  abstract module PruningConfiguration {
    function method MatchesPruningConfiguration(x: ValueType): bool

    method RepresentativeValue(input: ValueType) returns (output: ValueType)
      requires MatchesPruningConfiguration(input)
      ensures MatchesPruningConfiguration(output)

    method MaybeReplaceWithRepresentativeValue(x: ValueType) returns (output: ValueType)
      ensures !MatchesPruningConfiguration(x) ==> output == x
      ensures MatchesPruningConfiguration(x) ==> MatchesPruningConfiguration(output)
    {
      if MatchesPruningConfiguration(x) {
        output := RepresentativeValue(x);
      } else {
        output := x;
      }
    }

    type ValueType(==)
  }

  abstract module ParametrizedDomainPartition {
    function method IsInPartition<T>(x: Domain<T>): bool

    method RepresentativeValues<T>(input: seq<T>) returns (output: seq<Domain<T>>)
      requires |input| > 0
      ensures |output| > 0
      ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInPartition(output[i])
      decreases input

    type Domain<T>
  }

  abstract module ParametrizedCompleteDomainPartitioning {
    function method IsInPartitionList<T>(x: Domain<T>): seq<bool>

    import opened Values = JSON.Values

    type Domain<T>
  }

  abstract module OperationModel {
    const operationName: string

    method EvaluationRule(x: Domain) returns (output: Range)

    method DomainRepresentativeValues() returns (output: seq<Domain>)
      ensures |output| > 0

    method GenerateJSON() returns (outputJSON: JSON)
    {
      var operationsJSON := [];
      var inputValues := DomainRepresentativeValues();
      for i: int := 0 to |inputValues| {
        var input := inputValues[i];
        var json := JSONToWriteForOperationInputValue(input);
        operationsJSON := operationsJSON + [json];
      }
      outputJSON := Object([(operationName, Array(operationsJSON))]);
    }

    function method InputToJSON(x: Domain): JSON

    function method OutputToJSON(x: Range): JSON

    method JSONToWriteForOperationInputValue(input: Domain) returns (output: JSON)
    {
      var outputValue := EvaluationRule(input);
      var inputJSON := InputToJSON(input);
      var outputJSON := OutputToJSON(outputValue);
      output := Object([(""input"", inputJSON), (""output"", outputJSON)]);
    }

    import opened Values = JSON.Values

    type Domain

    type Range
  }

  abstract module ServiceModel {
    const serviceName: string
    const serviceNamespace: string

    method GenerateOperationsJSON() returns (operationsJSON: JSON)

    method GenerateJSON() returns (outputJSON: JSON)
    {
      var operationsJSON := GenerateOperationsJSON();
      outputJSON := Object([(serviceNamespace + "":"" + serviceName, operationsJSON)]);
    }

    import opened Values = JSON.Values
  }
}

abstract module AbstractCreateRawAesKeyringInput refines TestVectorsNameTBDModule.PartitioningScheme {
  function method IsInPartitionList(x: ValueType): seq<bool>
    ensures CountTrues(IsInPartitionList(x)) == 1
    decreases x

  function method CountTrues(partitions: seq<bool>): int
    ensures 0 <= CountTrues(partitions) <= |partitions|
    ensures CountTrues(partitions) == 0 ==> forall i: int {:trigger partitions[i]} :: 0 <= i < |partitions| ==> !partitions[i]
    ensures CountTrues(partitions) == 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i] && forall j: int {:trigger partitions[j]} :: 0 <= j < |partitions| && j != i ==> !partitions[j]
    ensures CountTrues(partitions) >= 1 ==> exists i: int {:trigger partitions[i]} :: 0 <= i < |partitions| && partitions[i]
    decreases partitions
  {
    if |partitions| == 0 then
      0
    else if partitions[0] then
      1 + CountTrues(partitions[1..])
    else
      CountTrues(partitions[1..])
  }

  function method IsInExactlyOnePartition(x: ValueType): bool
    ensures IsInExactlyOnePartition(x) ==> exists i: int {:trigger IsInPartitionList(x)[i]} :: 0 <= i < |IsInPartitionList(x)| && IsInPartitionList(x)[i] && forall j: int {:trigger IsInPartitionList(x)[j]} :: 0 <= j < |IsInPartitionList(x)| && j != i ==> !IsInPartitionList(x)[j]
    decreases x
  {
    var partitionList: seq<bool> := IsInPartitionList(x);
    CountTrues(partitionList) == 1
  }

  function method IsValidMember(x: ValueType): bool
    ensures IsValidMember(x) ==> true && x is ValueType
    decreases x
  {
    x is ValueType &&
    IsInExactlyOnePartition(x)
  }

  function method ToJSON(x: ValueType): JSON
    decreases x

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var values := AllValues();
    var out := [];
    for i: int := 0 to |values| {
      var value := values[i];
      var json := ToJSON(value);
      out := out + [json];
    }
    outputJSON := Array(out);
  }

  method RepresentativeValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])

  method AllValues() returns (output: seq<ValueType>)
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsValidMember(output[i])
    ensures |output| > 0
    ensures forall i: int {:trigger output[i]} :: 0 <= i < |output| ==> IsInExactlyOnePartition(output[i])
  {
    var raw := RepresentativeValues();
    var seen := {};
    output := [];
    for i: int := 0 to |raw| {
      if raw[i] !in seen {
        output := output + [raw[i]];
        seen := seen + {raw[i]};
      }
    }
    assert |output| > 0 by {
      assert |raw| > 0;
      assert |seen| > 0 by {
        assume {:axiom} raw[0] in seen;
      }
      assert |seen| == |output| by {
        assume {:axiom} |output| == |seen|;
      }
    }
  }

  method SomeValue() returns (output: ValueType)
    ensures IsValidMember(output)
  {
    var allValues := AllValues();
    output := allValues[0];
  }

  import AwsCryptographyMaterialProvidersTypes

  type ValueType = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  import opened Values = JSON.Values
}

abstract module AbstractCreateRawAesKeyringSmokeTest refines TestVectorsNameTBDModule.OperationModel {
  const operationName: string := ""CreateRawAesKeyringSmokeTest""

  method EvaluationRule(x: Domain) returns (output: Range)
    decreases x

  method DomainRepresentativeValues() returns (output: seq<Domain>)
    ensures |output| > 0

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := [];
    var inputValues := DomainRepresentativeValues();
    for i: int := 0 to |inputValues| {
      var input := inputValues[i];
      var json := JSONToWriteForOperationInputValue(input);
      operationsJSON := operationsJSON + [json];
    }
    outputJSON := Object([(operationName, Array(operationsJSON))]);
  }

  function method InputToJSON(x: Domain): JSON
    decreases x

  function method OutputToJSON(x: Range): JSON
    decreases x

  method JSONToWriteForOperationInputValue(input: Domain) returns (output: JSON)
    decreases input
  {
    var outputValue := EvaluationRule(input);
    var inputJSON := InputToJSON(input);
    var outputJSON := OutputToJSON(outputValue);
    output := Object([(""input"", inputJSON), (""output"", outputJSON)]);
  }

  import AwsCryptographyMaterialProvidersTypes

  import Wrappers

  type Domain = AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput

  type Range = string

  import opened Values = JSON.Values
}

abstract module AbstractCreateRawAesKeyringTestService refines TestVectorsNameTBDModule.ServiceModel {
  const serviceName: string := ""CreateRawAesKeyringTestService""
  const serviceNamespace: string

  method GenerateOperationsJSON() returns (operationsJSON: JSON)

  method GenerateJSON() returns (outputJSON: JSON)
  {
    var operationsJSON := GenerateOperationsJSON();
    outputJSON := Object([(serviceNamespace + "":"" + serviceName, operationsJSON)]);
  }

  import AwsCryptographyMaterialProvidersTypes

  import Wrappers

  import opened Values = JSON.Values
}


")]

namespace Dafny {
  internal class ArrayHelpers {
    public static T[] InitNewArray1<T>(T z, BigInteger size0) {
      int s0 = (int)size0;
      T[] a = new T[s0];
      for (int i0 = 0; i0 < s0; i0++) {
        a[i0] = z;
      }
      return a;
    }
  }
} // end of namespace Dafny
internal static class FuncExtensions {
  public static Func<U, UResult> DowncastClone<T, TResult, U, UResult>(this Func<T, TResult> F, Func<U, T> ArgConv, Func<TResult, UResult> ResConv) {
    return arg => ResConv(F(ArgConv(arg)));
  }
  public static Func<UResult> DowncastClone<TResult, UResult>(this Func<TResult> F, Func<TResult, UResult> ResConv) {
    return () => ResConv(F());
  }
  public static Func<U1, U2, UResult> DowncastClone<T1, T2, TResult, U1, U2, UResult>(this Func<T1, T2, TResult> F, Func<U1, T1> ArgConv1, Func<U2, T2> ArgConv2, Func<TResult, UResult> ResConv) {
    return (arg1, arg2) => ResConv(F(ArgConv1(arg1), ArgConv2(arg2)));
  }
  public static Func<U1, U2, U3, U4, U5, UResult> DowncastClone<T1, T2, T3, T4, T5, TResult, U1, U2, U3, U4, U5, UResult>(this Func<T1, T2, T3, T4, T5, TResult> F, Func<U1, T1> ArgConv1, Func<U2, T2> ArgConv2, Func<U3, T3> ArgConv3, Func<U4, T4> ArgConv4, Func<U5, T5> ArgConv5, Func<TResult, UResult> ResConv) {
    return (arg1, arg2, arg3, arg4, arg5) => ResConv(F(ArgConv1(arg1), ArgConv2(arg2), ArgConv3(arg3), ArgConv4(arg4), ArgConv5(arg5)));
  }
  public static Func<U1, U2, U3, UResult> DowncastClone<T1, T2, T3, TResult, U1, U2, U3, UResult>(this Func<T1, T2, T3, TResult> F, Func<U1, T1> ArgConv1, Func<U2, T2> ArgConv2, Func<U3, T3> ArgConv3, Func<TResult, UResult> ResConv) {
    return (arg1, arg2, arg3) => ResConv(F(ArgConv1(arg1), ArgConv2(arg2), ArgConv3(arg3)));
  }
  public static Func<U1, U2, U3, U4, UResult> DowncastClone<T1, T2, T3, T4, TResult, U1, U2, U3, U4, UResult>(this Func<T1, T2, T3, T4, TResult> F, Func<U1, T1> ArgConv1, Func<U2, T2> ArgConv2, Func<U3, T3> ArgConv3, Func<U4, T4> ArgConv4, Func<TResult, UResult> ResConv) {
    return (arg1, arg2, arg3, arg4) => ResConv(F(ArgConv1(arg1), ArgConv2(arg2), ArgConv3(arg3), ArgConv4(arg4)));
  }
  public static Func<U1, U2, U3, U4, U5, U6, UResult> DowncastClone<T1, T2, T3, T4, T5, T6, TResult, U1, U2, U3, U4, U5, U6, UResult>(this Func<T1, T2, T3, T4, T5, T6, TResult> F, Func<U1, T1> ArgConv1, Func<U2, T2> ArgConv2, Func<U3, T3> ArgConv3, Func<U4, T4> ArgConv4, Func<U5, T5> ArgConv5, Func<U6, T6> ArgConv6, Func<TResult, UResult> ResConv) {
    return (arg1, arg2, arg3, arg4, arg5, arg6) => ResConv(F(ArgConv1(arg1), ArgConv2(arg2), ArgConv3(arg3), ArgConv4(arg4), ArgConv5(arg5), ArgConv6(arg6)));
  }
  public static Func<U1, U2, U3, U4, U5, U6, U7, UResult> DowncastClone<T1, T2, T3, T4, T5, T6, T7, TResult, U1, U2, U3, U4, U5, U6, U7, UResult>(this Func<T1, T2, T3, T4, T5, T6, T7, TResult> F, Func<U1, T1> ArgConv1, Func<U2, T2> ArgConv2, Func<U3, T3> ArgConv3, Func<U4, T4> ArgConv4, Func<U5, T5> ArgConv5, Func<U6, T6> ArgConv6, Func<U7, T7> ArgConv7, Func<TResult, UResult> ResConv) {
    return (arg1, arg2, arg3, arg4, arg5, arg6, arg7) => ResConv(F(ArgConv1(arg1), ArgConv2(arg2), ArgConv3(arg3), ArgConv4(arg4), ArgConv5(arg5), ArgConv6(arg6), ArgConv7(arg7)));
  }
}
// end of class FuncExtensions
namespace TestVectorsNameTBDModule_Compile {

  public partial class __default {
    public static Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> WriteVectorsFile(Dafny.ISequence<char> location, JSON_mValues_Compile._IJSON json)
    {
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> output = Wrappers_Compile.Result<_System._ITuple0, Dafny.ISequence<char>>.Default(_System.Tuple0.Default());
      Wrappers_Compile._IResult<Dafny.ISequence<byte>, Dafny.ISequence<char>> _0_valueOrError0 = Wrappers_Compile.Result<Dafny.ISequence<byte>, Dafny.ISequence<char>>.Default(Dafny.Sequence<byte>.Empty);
      _0_valueOrError0 = Wrappers_Compile.Result<Dafny.ISequence<byte>, JSON_mErrors_Compile._ISerializationError>.MapFailure<Dafny.ISequence<char>>(JSON_mAPI_Compile.__default.Serialize(json), ((System.Func<JSON_mErrors_Compile._ISerializationError, Dafny.ISequence<char>>)((_1_e) => {
        return (_1_e)._ToString();
      })));
      if ((_0_valueOrError0).IsFailure()) {
        output = (_0_valueOrError0).PropagateFailure<_System._ITuple0>();
        return output;
      }
      Dafny.ISequence<byte> _2_jsonBytes;
      _2_jsonBytes = (_0_valueOrError0).Extract();
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _out0;
      _out0 = FileIO_Compile.__default.WriteBytesToFile(location, _2_jsonBytes);
      output = _out0;
      return output;
    }
  }
} // end of namespace TestVectorsNameTBDModule_Compile
namespace ValidAes128WrappingKeyPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<byte> x) {
      return (new BigInteger((x).Count)) == (new BigInteger(16));
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<byte>>.FromElements(Dafny.Sequence<byte>.FromElements((byte)(0), (byte)(1), (byte)(2), (byte)(3), (byte)(4), (byte)(5), (byte)(6), (byte)(7), (byte)(8), (byte)(9), (byte)(10), (byte)(11), (byte)(12), (byte)(13), (byte)(14), (byte)(15)));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<byte> x) {
      return ValidAes128WrappingKeyPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes128WrappingKeyPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<byte> SomeValue()
    {
      Dafny.ISequence<byte> output = Dafny.Sequence<byte>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes128WrappingKeyPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace ValidAes128WrappingKeyPartition_Compile
namespace ValidAes192WrappingKeyPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<byte> x) {
      return (new BigInteger((x).Count)) == (new BigInteger(24));
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<byte>>.FromElements(Dafny.Sequence<byte>.FromElements((byte)(0), (byte)(1), (byte)(2), (byte)(3), (byte)(4), (byte)(5), (byte)(6), (byte)(7), (byte)(8), (byte)(9), (byte)(10), (byte)(11), (byte)(12), (byte)(13), (byte)(14), (byte)(15), (byte)(16), (byte)(17), (byte)(18), (byte)(19), (byte)(20), (byte)(21), (byte)(22), (byte)(23)));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<byte> x) {
      return ValidAes192WrappingKeyPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes192WrappingKeyPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<byte> SomeValue()
    {
      Dafny.ISequence<byte> output = Dafny.Sequence<byte>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes192WrappingKeyPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace ValidAes192WrappingKeyPartition_Compile
namespace ValidAes256WrappingKeyPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<byte> x) {
      return (new BigInteger((x).Count)) == (new BigInteger(32));
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<byte>>.FromElements(Dafny.Sequence<byte>.FromElements((byte)(0), (byte)(1), (byte)(2), (byte)(3), (byte)(4), (byte)(5), (byte)(6), (byte)(7), (byte)(8), (byte)(9), (byte)(10), (byte)(11), (byte)(12), (byte)(13), (byte)(14), (byte)(15), (byte)(16), (byte)(17), (byte)(18), (byte)(19), (byte)(20), (byte)(21), (byte)(22), (byte)(23), (byte)(24), (byte)(25), (byte)(26), (byte)(27), (byte)(28), (byte)(29), (byte)(30), (byte)(31)));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<byte> x) {
      return ValidAes256WrappingKeyPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes256WrappingKeyPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<byte> SomeValue()
    {
      Dafny.ISequence<byte> output = Dafny.Sequence<byte>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes256WrappingKeyPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace ValidAes256WrappingKeyPartition_Compile
namespace ValidAesWrappingKeyPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<byte> x) {
      return ((ValidAes128WrappingKeyPartition_Compile.__default.IsInPartition(x)) || (ValidAes192WrappingKeyPartition_Compile.__default.IsInPartition(x))) || (ValidAes256WrappingKeyPartition_Compile.__default.IsInPartition(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_validAes128;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAes128WrappingKeyPartition_Compile.__default.RepresentativeValues();
      _0_validAes128 = _out0;
      Dafny.ISequence<Dafny.ISequence<byte>> _1_validAes192;
      Dafny.ISequence<Dafny.ISequence<byte>> _out1;
      _out1 = ValidAes192WrappingKeyPartition_Compile.__default.RepresentativeValues();
      _1_validAes192 = _out1;
      Dafny.ISequence<Dafny.ISequence<byte>> _2_validAes256;
      Dafny.ISequence<Dafny.ISequence<byte>> _out2;
      _out2 = ValidAes256WrappingKeyPartition_Compile.__default.RepresentativeValues();
      _2_validAes256 = _out2;
      output = Dafny.Sequence<Dafny.ISequence<byte>>.Concat(Dafny.Sequence<Dafny.ISequence<byte>>.Concat(_0_validAes128, _1_validAes192), _2_validAes256);
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<byte> x) {
      return ValidAesWrappingKeyPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAesWrappingKeyPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<byte> SomeValue()
    {
      Dafny.ISequence<byte> output = Dafny.Sequence<byte>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAesWrappingKeyPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace ValidAesWrappingKeyPartition_Compile
namespace InvalidAesWrappingKeyPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<byte> x) {
      return !(((ValidAes128WrappingKeyPartition_Compile.__default.IsInPartition(x)) || (ValidAes192WrappingKeyPartition_Compile.__default.IsInPartition(x))) || (ValidAes256WrappingKeyPartition_Compile.__default.IsInPartition(x)));
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<byte>>.FromElements(Dafny.Sequence<byte>.FromElements(), Dafny.Sequence<byte>.FromElements((byte)(1)));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<byte> x) {
      return InvalidAesWrappingKeyPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = InvalidAesWrappingKeyPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<byte> SomeValue()
    {
      Dafny.ISequence<byte> output = Dafny.Sequence<byte>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = InvalidAesWrappingKeyPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace InvalidAesWrappingKeyPartition_Compile
namespace AesWrappingKeyPartitioningScheme_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(Dafny.ISequence<byte> x) {
      return Dafny.Sequence<bool>.FromElements(ValidAesWrappingKeyPartition_Compile.__default.IsInPartition(x), InvalidAesWrappingKeyPartition_Compile.__default.IsInPartition(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_validAes;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = ValidAesWrappingKeyPartition_Compile.__default.RepresentativeValues();
      _0_validAes = _out0;
      Dafny.ISequence<Dafny.ISequence<byte>> _1_invalidAes;
      Dafny.ISequence<Dafny.ISequence<byte>> _out1;
      _out1 = InvalidAesWrappingKeyPartition_Compile.__default.RepresentativeValues();
      _1_invalidAes = _out1;
      output = Dafny.Sequence<Dafny.ISequence<byte>>.Concat(_0_validAes, _1_invalidAes);
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(Dafny.ISequence<byte> x) {
      return JSON_mValues_Compile.JSON.create_String(Base64_Compile.__default.Encode(x));
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(Dafny.ISequence<byte> x) {
      Dafny.ISequence<bool> _0_partitionList = AesWrappingKeyPartitioningScheme_Compile.__default.IsInPartitionList(x);
      return (AesWrappingKeyPartitioningScheme_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(Dafny.ISequence<byte> x) {
      return (true) && (AesWrappingKeyPartitioningScheme_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<Dafny.ISequence<byte>> _0_values;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = AesWrappingKeyPartitioningScheme_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        Dafny.ISequence<byte> _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = AesWrappingKeyPartitioningScheme_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<Dafny.ISequence<byte>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<byte>> output = Dafny.Sequence<Dafny.ISequence<byte>>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_raw;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = AesWrappingKeyPartitioningScheme_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<Dafny.ISequence<byte>> _1_seen;
      _1_seen = Dafny.Set<Dafny.ISequence<byte>>.FromElements();
      output = Dafny.Sequence<Dafny.ISequence<byte>>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<Dafny.ISequence<byte>>.Concat(output, Dafny.Sequence<Dafny.ISequence<byte>>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<Dafny.ISequence<byte>>.Union(_1_seen, Dafny.Set<Dafny.ISequence<byte>>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static Dafny.ISequence<byte> SomeValue()
    {
      Dafny.ISequence<byte> output = Dafny.Sequence<byte>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = AesWrappingKeyPartitioningScheme_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace AesWrappingKeyPartitioningScheme_Compile
namespace WrappingAlgValues_Compile {

  public partial class __default {
    public static Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg> ValueSpaceList() {
      return Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg>.FromElements(software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES128__GCM__IV12__TAG16(), software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES192__GCM__IV12__TAG16(), software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES256__GCM__IV12__TAG16());
    }
    public static Dafny.ISequence<bool> ValueSpaceEqualityList(software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg x) {
      return Dafny.Sequence<bool>.FromElements(object.Equals(x, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES128__GCM__IV12__TAG16()), object.Equals(x, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES192__GCM__IV12__TAG16()), object.Equals(x, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES256__GCM__IV12__TAG16()));
    }
    public static JSON_mValues_Compile._IJSON ToJSON(software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg x) {
      if (object.Equals(x, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES128__GCM__IV12__TAG16())) {
        return JSON_mValues_Compile.JSON.create_String(Dafny.Sequence<char>.FromString("ALG_AES128_GCM_IV12_TAG16"));
      } else if (object.Equals(x, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES192__GCM__IV12__TAG16())) {
        return JSON_mValues_Compile.JSON.create_String(Dafny.Sequence<char>.FromString("ALG_AES192_GCM_IV12_TAG16"));
      } else if (object.Equals(x, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES256__GCM__IV12__TAG16())) {
        return JSON_mValues_Compile.JSON.create_String(Dafny.Sequence<char>.FromString("ALG_AES256_GCM_IV12_TAG16"));
      } else {
        return JSON_mValues_Compile.JSON.create_String(Dafny.Sequence<char>.FromString("UnknownWrappingAlg"));
      }
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsExactlyOneValueInValueSpace(software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg x) {
      Dafny.ISequence<bool> _0_partitionList = WrappingAlgValues_Compile.__default.ValueSpaceEqualityList(x);
      return (WrappingAlgValues_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg x) {
      return (true) && (WrappingAlgValues_Compile.__default.IsExactlyOneValueInValueSpace(x));
    }
    public static Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg> AllValues()
    {
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg> output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg>.Empty;
      output = WrappingAlgValues_Compile.__default.ValueSpaceList();
      return output;
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg SomeValue()
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg output = software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.Default();
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg> _0_allValues;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg> _out0;
      _out0 = WrappingAlgValues_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace WrappingAlgValues_Compile
namespace InvalidRawAesKeyNamespacePartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<char> x) {
      return (x).Equals(Dafny.Sequence<char>.FromString("aws-kms"));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements(Dafny.Sequence<char>.FromString("aws-kms"));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return InvalidRawAesKeyNamespacePartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = InvalidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = InvalidRawAesKeyNamespacePartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace InvalidRawAesKeyNamespacePartition_Compile
namespace EmptyStringPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<char> x) {
      return (new BigInteger((x).Count)).Sign == 0;
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements(Dafny.Sequence<char>.FromString(""));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return EmptyStringPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = EmptyStringPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = EmptyStringPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace EmptyStringPartition_Compile
namespace AsciiPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<char> x) {
      return ((new BigInteger((x).Count)).Sign == 1) && (Dafny.Helpers.Id<Func<Dafny.ISequence<char>, bool>>((_0_x) => Dafny.Helpers.Quantifier<BigInteger>(Dafny.Helpers.IntegerRange(BigInteger.Zero, new BigInteger((_0_x).Count)), true, (((_forall_var_0) => {
        BigInteger _1_i = (BigInteger)_forall_var_0;
        return !(((_1_i).Sign != -1) && ((_1_i) < (new BigInteger((_0_x).Count)))) || ((new BigInteger((_0_x).Select(_1_i))) <= (new BigInteger(127)));
      }))))(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements(Dafny.Sequence<char>.FromString("abc"));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return AsciiPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = AsciiPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = AsciiPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace AsciiPartition_Compile
namespace BMPPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<char> x) {
      return ((new BigInteger((x).Count)).Sign == 1) && (Dafny.Helpers.Id<Func<Dafny.ISequence<char>, bool>>((_0_x) => Dafny.Helpers.Quantifier<BigInteger>(Dafny.Helpers.IntegerRange(BigInteger.Zero, new BigInteger((_0_x).Count)), false, (((_exists_var_0) => {
        BigInteger _1_i = (BigInteger)_exists_var_0;
        return ((((_1_i).Sign != -1) && ((_1_i) < (new BigInteger((_0_x).Count)))) && ((new BigInteger((_0_x).Select(_1_i))) > (new BigInteger(127)))) && ((new BigInteger((_0_x).Select(_1_i))) <= (new BigInteger(65535)));
      }))))(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<char> _0_value;
      _0_value = Dafny.Sequence<char>.FromString("Hëllø \uD83C\uDF0D — こんにちは世界 — مرحبا بالعالم — Здравствуй, мир!");
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements(_0_value);
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return BMPPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = BMPPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = BMPPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace BMPPartition_Compile
namespace CharacterValuePartitioning_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(Dafny.ISequence<char> x) {
      return Dafny.Sequence<bool>.FromElements(EmptyStringPartition_Compile.__default.IsInPartition(x), AsciiPartition_Compile.__default.IsInPartition(x), BMPPartition_Compile.__default.IsInPartition(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_values1;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = EmptyStringPartition_Compile.__default.RepresentativeValues();
      _0_values1 = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_values2;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = AsciiPartition_Compile.__default.RepresentativeValues();
      _1_values2 = _out1;
      Dafny.ISequence<Dafny.ISequence<char>> _2_values3;
      Dafny.ISequence<Dafny.ISequence<char>> _out2;
      _out2 = BMPPartition_Compile.__default.RepresentativeValues();
      _2_values3 = _out2;
      output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(Dafny.Sequence<Dafny.ISequence<char>>.Concat(_0_values1, _1_values2), _2_values3);
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(Dafny.ISequence<char> x) {
      return JSON_mValues_Compile.JSON.create_String(x);
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(Dafny.ISequence<char> x) {
      Dafny.ISequence<bool> _0_partitionList = CharacterValuePartitioning_Compile.__default.IsInPartitionList(x);
      return (CharacterValuePartitioning_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return (true) && (CharacterValuePartitioning_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_values;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = CharacterValuePartitioning_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        Dafny.ISequence<char> _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = CharacterValuePartitioning_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_raw;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = CharacterValuePartitioning_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<Dafny.ISequence<char>> _1_seen;
      _1_seen = Dafny.Set<Dafny.ISequence<char>>.FromElements();
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(output, Dafny.Sequence<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<Dafny.ISequence<char>>.Union(_1_seen, Dafny.Set<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = CharacterValuePartitioning_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace CharacterValuePartitioning_Compile
namespace ValidRawAesKeyNamespacePartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<char> x) {
      return !(x).Equals(Dafny.Sequence<char>.FromString("aws-kms"));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> FilterNonAwsKms(Dafny.ISequence<Dafny.ISequence<char>> s) {
      Dafny.ISequence<Dafny.ISequence<char>> _0___accumulator = Dafny.Sequence<Dafny.ISequence<char>>.FromElements();
    TAIL_CALL_START: ;
      if ((new BigInteger((s).Count)).Sign == 0) {
        return Dafny.Sequence<Dafny.ISequence<char>>.Concat(_0___accumulator, Dafny.Sequence<Dafny.ISequence<char>>.FromElements());
      } else if (((s).Select(BigInteger.Zero)).Equals(Dafny.Sequence<char>.FromString("aws-kms"))) {
        Dafny.ISequence<Dafny.ISequence<char>> _in0 = (s).Drop(BigInteger.One);
        s = _in0;
        goto TAIL_CALL_START;
      } else {
        _0___accumulator = Dafny.Sequence<Dafny.ISequence<char>>.Concat(_0___accumulator, Dafny.Sequence<Dafny.ISequence<char>>.FromElements((s).Select(BigInteger.Zero)));
        Dafny.ISequence<Dafny.ISequence<char>> _in1 = (s).Drop(BigInteger.One);
        s = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_values;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = CharacterValuePartitioning_Compile.__default.RepresentativeValues();
      _0_values = _out0;
      output = ValidRawAesKeyNamespacePartition_Compile.__default.FilterNonAwsKms(_0_values);
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return ValidRawAesKeyNamespacePartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = ValidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = ValidRawAesKeyNamespacePartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace ValidRawAesKeyNamespacePartition_Compile
namespace KeyNamespacePartitioningScheme_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(Dafny.ISequence<char> x) {
      return Dafny.Sequence<bool>.FromElements(InvalidRawAesKeyNamespacePartition_Compile.__default.IsInPartition(x), ValidRawAesKeyNamespacePartition_Compile.__default.IsInPartition(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_values1;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = InvalidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      _0_values1 = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_values2;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = ValidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      _1_values2 = _out1;
      output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(_0_values1, _1_values2);
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(Dafny.ISequence<char> x) {
      return JSON_mValues_Compile.JSON.create_String(x);
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(Dafny.ISequence<char> x) {
      Dafny.ISequence<bool> _0_partitionList = KeyNamespacePartitioningScheme_Compile.__default.IsInPartitionList(x);
      return (KeyNamespacePartitioningScheme_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return (true) && (KeyNamespacePartitioningScheme_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_values;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = KeyNamespacePartitioningScheme_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        Dafny.ISequence<char> _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = KeyNamespacePartitioningScheme_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_raw;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = KeyNamespacePartitioningScheme_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<Dafny.ISequence<char>> _1_seen;
      _1_seen = Dafny.Set<Dafny.ISequence<char>>.FromElements();
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(output, Dafny.Sequence<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<Dafny.ISequence<char>>.Union(_1_seen, Dafny.Set<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = KeyNamespacePartitioningScheme_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace KeyNamespacePartitioningScheme_Compile
namespace KeyNamePartitioningScheme_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(Dafny.ISequence<char> x) {
      return Dafny.Sequence<bool>.FromElements(CharacterValuePartitioning_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = CharacterValuePartitioning_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(Dafny.ISequence<char> x) {
      return JSON_mValues_Compile.JSON.create_String(x);
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(Dafny.ISequence<char> x) {
      Dafny.ISequence<bool> _0_partitionList = KeyNamePartitioningScheme_Compile.__default.IsInPartitionList(x);
      return (KeyNamePartitioningScheme_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return (true) && (KeyNamePartitioningScheme_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_values;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = KeyNamePartitioningScheme_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        Dafny.ISequence<char> _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = KeyNamePartitioningScheme_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_raw;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = KeyNamePartitioningScheme_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<Dafny.ISequence<char>> _1_seen;
      _1_seen = Dafny.Set<Dafny.ISequence<char>>.FromElements();
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(output, Dafny.Sequence<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<Dafny.ISequence<char>>.Union(_1_seen, Dafny.Set<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = KeyNamePartitioningScheme_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace KeyNamePartitioningScheme_Compile
namespace InvalidAes128PruningConfiguration_Compile {

  public partial class __default {
    public static bool MatchesPruningConfiguration(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return (object.Equals((x).dtor_wrappingAlg, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES128__GCM__IV12__TAG16())) && (!(ValidAes128WrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey)));
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput RepresentativeValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput input)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_representativeKeyNamespace;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = ValidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      _0_representativeKeyNamespace = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_representativeKeyName;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = KeyNamePartitioningScheme_Compile.__default.RepresentativeValues();
      _1_representativeKeyName = _out1;
      output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.create((_0_representativeKeyNamespace).Select(BigInteger.Zero), (_1_representativeKeyName).Select(BigInteger.Zero), (input).dtor_wrappingKey, (input).dtor_wrappingAlg);
      return output;
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput MaybeReplaceWithRepresentativeValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      if (InvalidAes128PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out0;
        _out0 = InvalidAes128PruningConfiguration_Compile.__default.RepresentativeValue(x);
        output = _out0;
      } else {
        output = x;
      }
      return output;
    }
  }
} // end of namespace InvalidAes128PruningConfiguration_Compile
namespace InvalidAes192PruningConfiguration_Compile {

  public partial class __default {
    public static bool MatchesPruningConfiguration(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return (object.Equals((x).dtor_wrappingAlg, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES192__GCM__IV12__TAG16())) && (!(ValidAes192WrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey)));
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput RepresentativeValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput input)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_representativeKeyNamespace;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = ValidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      _0_representativeKeyNamespace = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_representativeKeyName;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = KeyNamePartitioningScheme_Compile.__default.RepresentativeValues();
      _1_representativeKeyName = _out1;
      output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.create((_0_representativeKeyNamespace).Select(BigInteger.Zero), (_1_representativeKeyName).Select(BigInteger.Zero), (input).dtor_wrappingKey, (input).dtor_wrappingAlg);
      return output;
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput MaybeReplaceWithRepresentativeValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      if (InvalidAes192PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out0;
        _out0 = InvalidAes192PruningConfiguration_Compile.__default.RepresentativeValue(x);
        output = _out0;
      } else {
        output = x;
      }
      return output;
    }
  }
} // end of namespace InvalidAes192PruningConfiguration_Compile
namespace InvalidAes256PruningConfiguration_Compile {

  public partial class __default {
    public static bool MatchesPruningConfiguration(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return (object.Equals((x).dtor_wrappingAlg, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES256__GCM__IV12__TAG16())) && (!(ValidAes256WrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey)));
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput RepresentativeValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput input)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_representativeKeyNamespace;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = ValidRawAesKeyNamespacePartition_Compile.__default.RepresentativeValues();
      _0_representativeKeyNamespace = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_representativeKeyName;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = KeyNamePartitioningScheme_Compile.__default.RepresentativeValues();
      _1_representativeKeyName = _out1;
      output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.create((_0_representativeKeyNamespace).Select(BigInteger.Zero), (_1_representativeKeyName).Select(BigInteger.Zero), (input).dtor_wrappingKey, (input).dtor_wrappingAlg);
      return output;
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput MaybeReplaceWithRepresentativeValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      if (InvalidAes256PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out0;
        _out0 = InvalidAes256PruningConfiguration_Compile.__default.RepresentativeValue(x);
        output = _out0;
      } else {
        output = x;
      }
      return output;
    }
  }
} // end of namespace InvalidAes256PruningConfiguration_Compile
namespace CreateRawAesKeyringInput_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return Dafny.Sequence<bool>.FromElements((((AesWrappingKeyPartitioningScheme_Compile.__default.IsValidMember((x).dtor_wrappingKey)) && (WrappingAlgValues_Compile.__default.IsValidMember((x).dtor_wrappingAlg))) && (KeyNamespacePartitioningScheme_Compile.__default.IsValidMember((x).dtor_keyNamespace))) && (KeyNamePartitioningScheme_Compile.__default.IsValidMember((x).dtor_keyName)));
    }
    public static bool MatchesSomePruningConfiguration(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return ((InvalidAes128PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) || (InvalidAes192PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x))) || (InvalidAes256PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x));
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput ReplaceList(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x)
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      if (InvalidAes128PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out0;
        _out0 = InvalidAes128PruningConfiguration_Compile.__default.RepresentativeValue(x);
        output = _out0;
        output = output;
        return output;
      } else if (InvalidAes192PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out1;
        _out1 = InvalidAes192PruningConfiguration_Compile.__default.RepresentativeValue(x);
        output = _out1;
        output = output;
        return output;
      } else if (InvalidAes256PruningConfiguration_Compile.__default.MatchesPruningConfiguration(x)) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out2;
        _out2 = InvalidAes256PruningConfiguration_Compile.__default.RepresentativeValue(x);
        output = _out2;
        output = output;
        return output;
      } else {
        output = x;
        return output;
      }
      return output;
    }
    public static Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> MaybeAddValToVals(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x, Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> vals)
    {
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Empty;
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _0_replace;
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _out0;
      _out0 = CreateRawAesKeyringInput_Compile.__default.ReplaceList(x);
      _0_replace = _out0;
      if (object.Equals(x, _0_replace)) {
        output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Concat(vals, Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.FromElements(x));
        return output;
      } else {
        output = vals;
        return output;
      }
      return output;
    }
    public static Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> RepresentativeValues()
    {
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Empty;
      Dafny.ISequence<Dafny.ISequence<byte>> _0_wrappingKeys;
      Dafny.ISequence<Dafny.ISequence<byte>> _out0;
      _out0 = AesWrappingKeyPartitioningScheme_Compile.__default.RepresentativeValues();
      _0_wrappingKeys = _out0;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._IAesWrappingAlg> _1_wrappingAlgs;
      _1_wrappingAlgs = WrappingAlgValues_Compile.__default.ValueSpaceList();
      Dafny.ISequence<Dafny.ISequence<char>> _2_keyNamespaces;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = KeyNamespacePartitioningScheme_Compile.__default.RepresentativeValues();
      _2_keyNamespaces = _out1;
      Dafny.ISequence<Dafny.ISequence<char>> _3_keyNames;
      Dafny.ISequence<Dafny.ISequence<char>> _out2;
      _out2 = KeyNamePartitioningScheme_Compile.__default.RepresentativeValues();
      _3_keyNames = _out2;
      output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.FromElements();
      BigInteger _hi0 = new BigInteger((_1_wrappingAlgs).Count);
      for (BigInteger _4_i = BigInteger.Zero; _4_i < _hi0; _4_i++) {
        BigInteger _hi1 = new BigInteger((_0_wrappingKeys).Count);
        for (BigInteger _5_j = BigInteger.Zero; _5_j < _hi1; _5_j++) {
          BigInteger _hi2 = new BigInteger((_2_keyNamespaces).Count);
          for (BigInteger _6_k = BigInteger.Zero; _6_k < _hi2; _6_k++) {
            BigInteger _hi3 = new BigInteger((_3_keyNames).Count);
            for (BigInteger _7_l = BigInteger.Zero; _7_l < _hi3; _7_l++) {
              software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _8_keyring;
              _8_keyring = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.create((_2_keyNamespaces).Select(_6_k), (_3_keyNames).Select(_7_l), (_0_wrappingKeys).Select(_5_j), (_1_wrappingAlgs).Select(_4_i));
              Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _out3;
              _out3 = CreateRawAesKeyringInput_Compile.__default.MaybeAddValToVals(_8_keyring, output);
              output = _out3;
            }
          }
        }
      }
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("wrappingKey"), AesWrappingKeyPartitioningScheme_Compile.__default.ToJSON((x).dtor_wrappingKey)), _System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("wrappingAlg"), WrappingAlgValues_Compile.__default.ToJSON((x).dtor_wrappingAlg)), _System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("keyNamespace"), KeyNamespacePartitioningScheme_Compile.__default.ToJSON((x).dtor_keyNamespace)), _System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("keyName"), KeyNamePartitioningScheme_Compile.__default.ToJSON((x).dtor_keyName))));
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      Dafny.ISequence<bool> _0_partitionList = CreateRawAesKeyringInput_Compile.__default.IsInPartitionList(x);
      return (CreateRawAesKeyringInput_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return (true) && (CreateRawAesKeyringInput_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _0_values;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _out0;
      _out0 = CreateRawAesKeyringInput_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = CreateRawAesKeyringInput_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> AllValues()
    {
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Empty;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _0_raw;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _out0;
      _out0 = CreateRawAesKeyringInput_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _1_seen;
      _1_seen = Dafny.Set<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.FromElements();
      output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Concat(output, Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Union(_1_seen, Dafny.Set<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput SomeValue()
    {
      software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput output = software.amazon.cryptography.materialproviders.internaldafny.types.CreateRawAesKeyringInput.Default();
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _0_allValues;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _out0;
      _out0 = CreateRawAesKeyringInput_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace CreateRawAesKeyringInput_Compile
namespace OptionStringNone_Compile {

  public partial class __default {
    public static bool IsInPartition(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      return (x).is_None;
    }
    public static Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> RepresentativeValues()
    {
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Empty;
      output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements(Wrappers_Compile.Option<Dafny.ISequence<char>>.create_None());
      return output;
    }
    public static bool IsValidMember(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      return OptionStringNone_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> AllValues()
    {
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Empty;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringNone_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Wrappers_Compile._IOption<Dafny.ISequence<char>> SomeValue()
    {
      Wrappers_Compile._IOption<Dafny.ISequence<char>> output = Wrappers_Compile.Option<Dafny.ISequence<char>>.Default();
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_allValues;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringNone_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace OptionStringNone_Compile
namespace NonemptyStringPartition_Compile {

  public partial class __default {
    public static bool IsInPartition(Dafny.ISequence<char> x) {
      return (new BigInteger((x).Count)).Sign == 1;
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements(Dafny.Sequence<char>.FromString("abc"));
      return output;
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return NonemptyStringPartition_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = NonemptyStringPartition_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = NonemptyStringPartition_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace NonemptyStringPartition_Compile
namespace LengthPartitioning_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(Dafny.ISequence<char> x) {
      return Dafny.Sequence<bool>.FromElements(EmptyStringPartition_Compile.__default.IsInPartition(x), NonemptyStringPartition_Compile.__default.IsInPartition(x));
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> RepresentativeValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_values1;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = EmptyStringPartition_Compile.__default.RepresentativeValues();
      _0_values1 = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_values2;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = NonemptyStringPartition_Compile.__default.RepresentativeValues();
      _1_values2 = _out1;
      output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(_0_values1, _1_values2);
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(Dafny.ISequence<char> x) {
      return JSON_mValues_Compile.JSON.create_String(x);
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(Dafny.ISequence<char> x) {
      Dafny.ISequence<bool> _0_partitionList = LengthPartitioning_Compile.__default.IsInPartitionList(x);
      return (LengthPartitioning_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(Dafny.ISequence<char> x) {
      return (true) && (LengthPartitioning_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<Dafny.ISequence<char>> _0_values;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = LengthPartitioning_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        Dafny.ISequence<char> _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = LengthPartitioning_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<Dafny.ISequence<char>> AllValues()
    {
      Dafny.ISequence<Dafny.ISequence<char>> output = Dafny.Sequence<Dafny.ISequence<char>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_raw;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = LengthPartitioning_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<Dafny.ISequence<char>> _1_seen;
      _1_seen = Dafny.Set<Dafny.ISequence<char>>.FromElements();
      output = Dafny.Sequence<Dafny.ISequence<char>>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<Dafny.ISequence<char>>.Concat(output, Dafny.Sequence<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<Dafny.ISequence<char>>.Union(_1_seen, Dafny.Set<Dafny.ISequence<char>>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static Dafny.ISequence<char> SomeValue()
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_allValues;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = LengthPartitioning_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace LengthPartitioning_Compile
namespace OptionStringSome_Compile {

  public partial class __default {
    public static bool IsInPartition(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      return (x).is_Some;
    }
    public static Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> RepresentativeValues()
    {
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Empty;
      Dafny.ISequence<Dafny.ISequence<char>> _0_values1;
      Dafny.ISequence<Dafny.ISequence<char>> _out0;
      _out0 = LengthPartitioning_Compile.__default.AllValues();
      _0_values1 = _out0;
      Dafny.ISequence<Dafny.ISequence<char>> _1_values2;
      Dafny.ISequence<Dafny.ISequence<char>> _out1;
      _out1 = CharacterValuePartitioning_Compile.__default.AllValues();
      _1_values2 = _out1;
      Dafny.ISequence<Dafny.ISequence<char>> _2_values;
      _2_values = Dafny.Sequence<Dafny.ISequence<char>>.Concat(_0_values1, _1_values2);
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _3_temp;
      _3_temp = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements();
      BigInteger _4_i;
      _4_i = BigInteger.Zero;
      while ((_4_i) < (new BigInteger((_2_values).Count))) {
        _3_temp = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Concat(_3_temp, Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements(Wrappers_Compile.Option<Dafny.ISequence<char>>.create_Some((_2_values).Select(_4_i))));
        _4_i = (_4_i) + (BigInteger.One);
      }
      output = _3_temp;
      return output;
    }
    public static bool IsValidMember(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      return OptionStringSome_Compile.__default.IsInPartition(x);
    }
    public static Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> AllValues()
    {
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Empty;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringSome_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static Wrappers_Compile._IOption<Dafny.ISequence<char>> SomeValue()
    {
      Wrappers_Compile._IOption<Dafny.ISequence<char>> output = Wrappers_Compile.Option<Dafny.ISequence<char>>.Default();
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_allValues;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringSome_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace OptionStringSome_Compile
namespace OptionStringDomainModel_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      return Dafny.Sequence<bool>.FromElements(OptionStringNone_Compile.__default.IsInPartition(x), OptionStringSome_Compile.__default.IsInPartition(x));
    }
    public static Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> RepresentativeValues()
    {
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Empty;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_values1;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringNone_Compile.__default.RepresentativeValues();
      _0_values1 = _out0;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _1_values2;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out1;
      _out1 = OptionStringSome_Compile.__default.RepresentativeValues();
      _1_values2 = _out1;
      output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Concat(_0_values1, _1_values2);
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      if ((x).is_None) {
        return JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("None"), JSON_mValues_Compile.JSON.create_Null())));
      } else {
        return JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("Some"), JSON_mValues_Compile.JSON.create_String((x).dtor_value))));
      }
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      Dafny.ISequence<bool> _0_partitionList = OptionStringDomainModel_Compile.__default.IsInPartitionList(x);
      return (OptionStringDomainModel_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(Wrappers_Compile._IOption<Dafny.ISequence<char>> x) {
      return (true) && (OptionStringDomainModel_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_values;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringDomainModel_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        Wrappers_Compile._IOption<Dafny.ISequence<char>> _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = OptionStringDomainModel_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> AllValues()
    {
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Empty;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_raw;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringDomainModel_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _1_seen;
      _1_seen = Dafny.Set<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements();
      output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Concat(output, Dafny.Sequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.Union(_1_seen, Dafny.Set<Wrappers_Compile._IOption<Dafny.ISequence<char>>>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static Wrappers_Compile._IOption<Dafny.ISequence<char>> SomeValue()
    {
      Wrappers_Compile._IOption<Dafny.ISequence<char>> output = Wrappers_Compile.Option<Dafny.ISequence<char>>.Default();
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_allValues;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringDomainModel_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace OptionStringDomainModel_Compile
namespace CreateRawAesKeyringSmokeTest_Compile {

  public partial class __default {
    public static Dafny.ISequence<char> EvaluationRule(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x)
    {
      Dafny.ISequence<char> output = Dafny.Sequence<char>.Empty;
      if (!(ValidAesWrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey))) {
        output = Dafny.Sequence<char>.FromString("Invalid wrapping key length");
        return output;
      }
      if ((object.Equals((x).dtor_wrappingAlg, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES128__GCM__IV12__TAG16())) && (!(ValidAes128WrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey)))) {
        output = Dafny.Sequence<char>.FromString("Invalid wrapping key length");
        return output;
      } else if ((object.Equals((x).dtor_wrappingAlg, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES192__GCM__IV12__TAG16())) && (!(ValidAes192WrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey)))) {
        output = Dafny.Sequence<char>.FromString("Invalid wrapping key length");
        return output;
      } else if ((object.Equals((x).dtor_wrappingAlg, software.amazon.cryptography.materialproviders.internaldafny.types.AesWrappingAlg.create_ALG__AES256__GCM__IV12__TAG16())) && (!(ValidAes256WrappingKeyPartition_Compile.__default.IsInPartition((x).dtor_wrappingKey)))) {
        output = Dafny.Sequence<char>.FromString("Invalid wrapping key length");
        return output;
      }
      if (((x).dtor_keyNamespace).Equals(Dafny.Sequence<char>.FromString("aws-kms"))) {
        output = Dafny.Sequence<char>.FromString("Invalid key namespace for raw AES keyring");
        return output;
      }
      output = Dafny.Sequence<char>.FromString("ok");
      return output;
    }
    public static JSON_mValues_Compile._IJSON InputToJSON(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput x) {
      return CreateRawAesKeyringInput_Compile.__default.ToJSON(x);
    }
    public static JSON_mValues_Compile._IJSON OutputToJSON(Dafny.ISequence<char> x) {
      return JSON_mValues_Compile.JSON.create_String(x);
    }
    public static Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> DomainRepresentativeValues()
    {
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> output = Dafny.Sequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput>.Empty;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _out0;
      _out0 = CreateRawAesKeyringInput_Compile.__default.AllValues();
      output = _out0;
      return output;
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _0_operationsJSON;
      _0_operationsJSON = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _1_inputValues;
      Dafny.ISequence<software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput> _out0;
      _out0 = CreateRawAesKeyringSmokeTest_Compile.__default.DomainRepresentativeValues();
      _1_inputValues = _out0;
      BigInteger _hi0 = new BigInteger((_1_inputValues).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput _3_input;
        _3_input = (_1_inputValues).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        JSON_mValues_Compile._IJSON _out1;
        _out1 = CreateRawAesKeyringSmokeTest_Compile.__default.JSONToWriteForOperationInputValue(_3_input);
        _4_json = _out1;
        _0_operationsJSON = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_0_operationsJSON, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(CreateRawAesKeyringSmokeTest_Compile.__default.operationName, JSON_mValues_Compile.JSON.create_Array(_0_operationsJSON))));
      return outputJSON;
    }
    public static JSON_mValues_Compile._IJSON JSONToWriteForOperationInputValue(software.amazon.cryptography.materialproviders.internaldafny.types._ICreateRawAesKeyringInput input)
    {
      JSON_mValues_Compile._IJSON output = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<char> _0_outputValue;
      Dafny.ISequence<char> _out0;
      _out0 = CreateRawAesKeyringSmokeTest_Compile.__default.EvaluationRule(input);
      _0_outputValue = _out0;
      JSON_mValues_Compile._IJSON _1_inputJSON;
      _1_inputJSON = CreateRawAesKeyringSmokeTest_Compile.__default.InputToJSON(input);
      JSON_mValues_Compile._IJSON _2_outputJSON;
      _2_outputJSON = CreateRawAesKeyringSmokeTest_Compile.__default.OutputToJSON(_0_outputValue);
      output = JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("input"), _1_inputJSON), _System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("output"), _2_outputJSON)));
      return output;
    }
    public static Dafny.ISequence<char> operationName { get {
      return Dafny.Sequence<char>.FromString("CreateRawAesKeyringSmokeTest");
    } }
  }
} // end of namespace CreateRawAesKeyringSmokeTest_Compile
namespace CreateRawAesKeyringTestService_Compile {

  public partial class __default {
    public static JSON_mValues_Compile._IJSON GenerateOperationsJSON()
    {
      JSON_mValues_Compile._IJSON operationsJSON = JSON_mValues_Compile.JSON.Default();
      JSON_mValues_Compile._IJSON _0_CreateRawAesKeyringSmokeTestJSON;
      JSON_mValues_Compile._IJSON _out0;
      _out0 = CreateRawAesKeyringSmokeTest_Compile.__default.GenerateJSON();
      _0_CreateRawAesKeyringSmokeTestJSON = _out0;
      operationsJSON = JSON_mValues_Compile.JSON.create_Array(Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_0_CreateRawAesKeyringSmokeTestJSON));
      return operationsJSON;
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      JSON_mValues_Compile._IJSON _0_operationsJSON;
      JSON_mValues_Compile._IJSON _out0;
      _out0 = CreateRawAesKeyringTestService_Compile.__default.GenerateOperationsJSON();
      _0_operationsJSON = _out0;
      outputJSON = JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.Concat(Dafny.Sequence<char>.Concat(CreateRawAesKeyringTestService_Compile.__default.serviceNamespace, Dafny.Sequence<char>.FromString(":")), CreateRawAesKeyringTestService_Compile.__default.serviceName), _0_operationsJSON)));
      return outputJSON;
    }
    public static Dafny.ISequence<char> serviceNamespace { get {
      return Dafny.Sequence<char>.Empty;
    } }
    public static Dafny.ISequence<char> serviceName { get {
      return Dafny.Sequence<char>.FromString("CreateRawAesKeyringTestService");
    } }
  }
} // end of namespace CreateRawAesKeyringTestService_Compile
namespace Tests3_Compile {

  public partial class __default {
    [Xunit.Fact]
    public static void TestVectorsV2()
    {
      JSON_mValues_Compile._IJSON _0_keyringsInputJSON;
      JSON_mValues_Compile._IJSON _out0;
      _out0 = CreateRawAesKeyringInput_Compile.__default.GenerateJSON();
      _0_keyringsInputJSON = _out0;
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _1_out;
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _out1;
      _out1 = TestVectorsNameTBDModule_Compile.__default.WriteVectorsFile(Dafny.Sequence<char>.FromString("outkeyringsonly.json"), _0_keyringsInputJSON);
      _1_out = _out1;
      JSON_mValues_Compile._IJSON _2_outputJSON;
      JSON_mValues_Compile._IJSON _out2;
      _out2 = CreateRawAesKeyringTestService_Compile.__default.GenerateJSON();
      _2_outputJSON = _out2;
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _3_out2;
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _out3;
      _out3 = TestVectorsNameTBDModule_Compile.__default.WriteVectorsFile(Dafny.Sequence<char>.FromString("outkeyrings.json"), _2_outputJSON);
      _3_out2 = _out3;
    }
  }
} // end of namespace Tests3_Compile
namespace SimpleTypesSmithyStringTypes_Compile {

  public partial class __default {
    public static bool IsDummySubsetType(BigInteger x) {
      return (x).Sign == 1;
    }
  }

  public interface _IDafnyCallEvent<I, O> {
    bool is_DafnyCallEvent { get; }
    I dtor_input { get; }
    O dtor_output { get; }
    _IDafnyCallEvent<__I, __O> DowncastClone<__I, __O>(Func<I, __I> converter0, Func<O, __O> converter1);
  }
  public class DafnyCallEvent<I, O> : _IDafnyCallEvent<I, O> {
    public readonly I _input;
    public readonly O _output;
    public DafnyCallEvent(I input, O output) {
      this._input = input;
      this._output = output;
    }
    public _IDafnyCallEvent<__I, __O> DowncastClone<__I, __O>(Func<I, __I> converter0, Func<O, __O> converter1) {
      if (this is _IDafnyCallEvent<__I, __O> dt) { return dt; }
      return new DafnyCallEvent<__I, __O>(converter0(_input), converter1(_output));
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.DafnyCallEvent<I, O>;
      return oth != null && object.Equals(this._input, oth._input) && object.Equals(this._output, oth._output);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._input));
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._output));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.DafnyCallEvent.DafnyCallEvent";
      s += "(";
      s += Dafny.Helpers.ToString(this._input);
      s += ", ";
      s += Dafny.Helpers.ToString(this._output);
      s += ")";
      return s;
    }
    public static SimpleTypesSmithyStringTypes_Compile._IDafnyCallEvent<I, O> Default(I _default_I, O _default_O) {
      return create(_default_I, _default_O);
    }
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IDafnyCallEvent<I, O>> _TypeDescriptor(Dafny.TypeDescriptor<I> _td_I, Dafny.TypeDescriptor<O> _td_O) {
      return new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IDafnyCallEvent<I, O>>(SimpleTypesSmithyStringTypes_Compile.DafnyCallEvent<I, O>.Default(_td_I.Default(), _td_O.Default()));
    }
    public static _IDafnyCallEvent<I, O> create(I input, O output) {
      return new DafnyCallEvent<I, O>(input, output);
    }
    public static _IDafnyCallEvent<I, O> create_DafnyCallEvent(I input, O output) {
      return create(input, output);
    }
    public bool is_DafnyCallEvent { get { return true; } }
    public I dtor_input {
      get {
        return this._input;
      }
    }
    public O dtor_output {
      get {
        return this._output;
      }
    }
  }

  public interface _IGetStringInput {
    bool is_GetStringInput { get; }
    Wrappers_Compile._IOption<Dafny.ISequence<char>> dtor_value { get; }
    _IGetStringInput DowncastClone();
  }
  public class GetStringInput : _IGetStringInput {
    public readonly Wrappers_Compile._IOption<Dafny.ISequence<char>> _value;
    public GetStringInput(Wrappers_Compile._IOption<Dafny.ISequence<char>> @value) {
      this._value = @value;
    }
    public _IGetStringInput DowncastClone() {
      if (this is _IGetStringInput dt) { return dt; }
      return new GetStringInput(_value);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.GetStringInput;
      return oth != null && object.Equals(this._value, oth._value);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._value));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.GetStringInput.GetStringInput";
      s += "(";
      s += Dafny.Helpers.ToString(this._value);
      s += ")";
      return s;
    }
    private static readonly SimpleTypesSmithyStringTypes_Compile._IGetStringInput theDefault = create(Wrappers_Compile.Option<Dafny.ISequence<char>>.Default());
    public static SimpleTypesSmithyStringTypes_Compile._IGetStringInput Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>(SimpleTypesSmithyStringTypes_Compile.GetStringInput.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _TypeDescriptor() {
      return _TYPE;
    }
    public static _IGetStringInput create(Wrappers_Compile._IOption<Dafny.ISequence<char>> @value) {
      return new GetStringInput(@value);
    }
    public static _IGetStringInput create_GetStringInput(Wrappers_Compile._IOption<Dafny.ISequence<char>> @value) {
      return create(@value);
    }
    public bool is_GetStringInput { get { return true; } }
    public Wrappers_Compile._IOption<Dafny.ISequence<char>> dtor_value {
      get {
        return this._value;
      }
    }
  }

  public interface _IGetStringOutput {
    bool is_GetStringOutput { get; }
    Wrappers_Compile._IOption<Dafny.ISequence<char>> dtor_value { get; }
    _IGetStringOutput DowncastClone();
  }
  public class GetStringOutput : _IGetStringOutput {
    public readonly Wrappers_Compile._IOption<Dafny.ISequence<char>> _value;
    public GetStringOutput(Wrappers_Compile._IOption<Dafny.ISequence<char>> @value) {
      this._value = @value;
    }
    public _IGetStringOutput DowncastClone() {
      if (this is _IGetStringOutput dt) { return dt; }
      return new GetStringOutput(_value);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.GetStringOutput;
      return oth != null && object.Equals(this._value, oth._value);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._value));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.GetStringOutput.GetStringOutput";
      s += "(";
      s += Dafny.Helpers.ToString(this._value);
      s += ")";
      return s;
    }
    private static readonly SimpleTypesSmithyStringTypes_Compile._IGetStringOutput theDefault = create(Wrappers_Compile.Option<Dafny.ISequence<char>>.Default());
    public static SimpleTypesSmithyStringTypes_Compile._IGetStringOutput Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput>(SimpleTypesSmithyStringTypes_Compile.GetStringOutput.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput> _TypeDescriptor() {
      return _TYPE;
    }
    public static _IGetStringOutput create(Wrappers_Compile._IOption<Dafny.ISequence<char>> @value) {
      return new GetStringOutput(@value);
    }
    public static _IGetStringOutput create_GetStringOutput(Wrappers_Compile._IOption<Dafny.ISequence<char>> @value) {
      return create(@value);
    }
    public bool is_GetStringOutput { get { return true; } }
    public Wrappers_Compile._IOption<Dafny.ISequence<char>> dtor_value {
      get {
        return this._value;
      }
    }
  }

  public interface _IGetStringUTF8Input {
    bool is_GetStringUTF8Input { get; }
    Wrappers_Compile._IOption<Dafny.ISequence<byte>> dtor_value { get; }
    _IGetStringUTF8Input DowncastClone();
  }
  public class GetStringUTF8Input : _IGetStringUTF8Input {
    public readonly Wrappers_Compile._IOption<Dafny.ISequence<byte>> _value;
    public GetStringUTF8Input(Wrappers_Compile._IOption<Dafny.ISequence<byte>> @value) {
      this._value = @value;
    }
    public _IGetStringUTF8Input DowncastClone() {
      if (this is _IGetStringUTF8Input dt) { return dt; }
      return new GetStringUTF8Input(_value);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Input;
      return oth != null && object.Equals(this._value, oth._value);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._value));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.GetStringUTF8Input.GetStringUTF8Input";
      s += "(";
      s += Dafny.Helpers.ToString(this._value);
      s += ")";
      return s;
    }
    private static readonly SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input theDefault = create(Wrappers_Compile.Option<Dafny.ISequence<byte>>.Default());
    public static SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input>(SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Input.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input> _TypeDescriptor() {
      return _TYPE;
    }
    public static _IGetStringUTF8Input create(Wrappers_Compile._IOption<Dafny.ISequence<byte>> @value) {
      return new GetStringUTF8Input(@value);
    }
    public static _IGetStringUTF8Input create_GetStringUTF8Input(Wrappers_Compile._IOption<Dafny.ISequence<byte>> @value) {
      return create(@value);
    }
    public bool is_GetStringUTF8Input { get { return true; } }
    public Wrappers_Compile._IOption<Dafny.ISequence<byte>> dtor_value {
      get {
        return this._value;
      }
    }
  }

  public interface _IGetStringUTF8Output {
    bool is_GetStringUTF8Output { get; }
    Wrappers_Compile._IOption<Dafny.ISequence<byte>> dtor_value { get; }
    _IGetStringUTF8Output DowncastClone();
  }
  public class GetStringUTF8Output : _IGetStringUTF8Output {
    public readonly Wrappers_Compile._IOption<Dafny.ISequence<byte>> _value;
    public GetStringUTF8Output(Wrappers_Compile._IOption<Dafny.ISequence<byte>> @value) {
      this._value = @value;
    }
    public _IGetStringUTF8Output DowncastClone() {
      if (this is _IGetStringUTF8Output dt) { return dt; }
      return new GetStringUTF8Output(_value);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output;
      return oth != null && object.Equals(this._value, oth._value);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._value));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.GetStringUTF8Output.GetStringUTF8Output";
      s += "(";
      s += Dafny.Helpers.ToString(this._value);
      s += ")";
      return s;
    }
    private static readonly SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output theDefault = create(Wrappers_Compile.Option<Dafny.ISequence<byte>>.Default());
    public static SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output>(SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output> _TypeDescriptor() {
      return _TYPE;
    }
    public static _IGetStringUTF8Output create(Wrappers_Compile._IOption<Dafny.ISequence<byte>> @value) {
      return new GetStringUTF8Output(@value);
    }
    public static _IGetStringUTF8Output create_GetStringUTF8Output(Wrappers_Compile._IOption<Dafny.ISequence<byte>> @value) {
      return create(@value);
    }
    public bool is_GetStringUTF8Output { get { return true; } }
    public Wrappers_Compile._IOption<Dafny.ISequence<byte>> dtor_value {
      get {
        return this._value;
      }
    }
  }

  public interface _ISimpleStringConfig {
    bool is_SimpleStringConfig { get; }
    _ISimpleStringConfig DowncastClone();
  }
  public class SimpleStringConfig : _ISimpleStringConfig {
    public SimpleStringConfig() {
    }
    public _ISimpleStringConfig DowncastClone() {
      if (this is _ISimpleStringConfig dt) { return dt; }
      return new SimpleStringConfig();
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.SimpleStringConfig;
      return oth != null;
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.SimpleStringConfig.SimpleStringConfig";
      return s;
    }
    private static readonly SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig theDefault = create();
    public static SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig>(SimpleTypesSmithyStringTypes_Compile.SimpleStringConfig.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig> _TypeDescriptor() {
      return _TYPE;
    }
    public static _ISimpleStringConfig create() {
      return new SimpleStringConfig();
    }
    public static _ISimpleStringConfig create_SimpleStringConfig() {
      return create();
    }
    public bool is_SimpleStringConfig { get { return true; } }
    public static System.Collections.Generic.IEnumerable<_ISimpleStringConfig> AllSingletonConstructors {
      get {
        yield return SimpleStringConfig.create();
      }
    }
  }

  public partial class ISimpleTypesStringClientCallHistory {
    public ISimpleTypesStringClientCallHistory() {
    }
  }

  public interface ISimpleTypesStringClient {
    Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> GetString(SimpleTypesSmithyStringTypes_Compile._IGetStringInput input);
    Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> GetStringKnownValue(SimpleTypesSmithyStringTypes_Compile._IGetStringInput input);
    Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> GetStringUTF8(SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input input);
    Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> GetStringUTF8KnownValue(SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input input);
  }
  public class _Companion_ISimpleTypesStringClient {
  }

  public interface _IError {
    bool is_CollectionOfErrors { get; }
    bool is_Opaque { get; }
    bool is_OpaqueWithText { get; }
    Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IError> dtor_list { get; }
    Dafny.ISequence<char> dtor_message { get; }
    object dtor_obj { get; }
    Dafny.ISequence<char> dtor_objMessage { get; }
    _IError DowncastClone();
  }
  public abstract class Error : _IError {
    public Error() {
    }
    private static readonly SimpleTypesSmithyStringTypes_Compile._IError theDefault = create_CollectionOfErrors(Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IError>.Empty, Dafny.Sequence<char>.Empty);
    public static SimpleTypesSmithyStringTypes_Compile._IError Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IError> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IError>(SimpleTypesSmithyStringTypes_Compile.Error.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IError> _TypeDescriptor() {
      return _TYPE;
    }
    public static _IError create_CollectionOfErrors(Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IError> list, Dafny.ISequence<char> message) {
      return new Error_CollectionOfErrors(list, message);
    }
    public static _IError create_Opaque(object obj) {
      return new Error_Opaque(obj);
    }
    public static _IError create_OpaqueWithText(object obj, Dafny.ISequence<char> objMessage) {
      return new Error_OpaqueWithText(obj, objMessage);
    }
    public bool is_CollectionOfErrors { get { return this is Error_CollectionOfErrors; } }
    public bool is_Opaque { get { return this is Error_Opaque; } }
    public bool is_OpaqueWithText { get { return this is Error_OpaqueWithText; } }
    public Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IError> dtor_list {
      get {
        var d = this;
        return ((Error_CollectionOfErrors)d)._list;
      }
    }
    public Dafny.ISequence<char> dtor_message {
      get {
        var d = this;
        return ((Error_CollectionOfErrors)d)._message;
      }
    }
    public object dtor_obj {
      get {
        var d = this;
        if (d is Error_Opaque) { return ((Error_Opaque)d)._obj; }
        return ((Error_OpaqueWithText)d)._obj;
      }
    }
    public Dafny.ISequence<char> dtor_objMessage {
      get {
        var d = this;
        return ((Error_OpaqueWithText)d)._objMessage;
      }
    }
    public abstract _IError DowncastClone();
  }
  public class Error_CollectionOfErrors : Error {
    public readonly Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IError> _list;
    public readonly Dafny.ISequence<char> _message;
    public Error_CollectionOfErrors(Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IError> list, Dafny.ISequence<char> message) : base() {
      this._list = list;
      this._message = message;
    }
    public override _IError DowncastClone() {
      if (this is _IError dt) { return dt; }
      return new Error_CollectionOfErrors(_list, _message);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.Error_CollectionOfErrors;
      return oth != null && object.Equals(this._list, oth._list) && object.Equals(this._message, oth._message);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._list));
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._message));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.Error.CollectionOfErrors";
      s += "(";
      s += Dafny.Helpers.ToString(this._list);
      s += ", ";
      s += Dafny.Helpers.ToString(this._message);
      s += ")";
      return s;
    }
  }
  public class Error_Opaque : Error {
    public readonly object _obj;
    public Error_Opaque(object obj) : base() {
      this._obj = obj;
    }
    public override _IError DowncastClone() {
      if (this is _IError dt) { return dt; }
      return new Error_Opaque(_obj);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.Error_Opaque;
      return oth != null && this._obj == oth._obj;
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 1;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._obj));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.Error.Opaque";
      s += "(";
      s += Dafny.Helpers.ToString(this._obj);
      s += ")";
      return s;
    }
  }
  public class Error_OpaqueWithText : Error {
    public readonly object _obj;
    public readonly Dafny.ISequence<char> _objMessage;
    public Error_OpaqueWithText(object obj, Dafny.ISequence<char> objMessage) : base() {
      this._obj = obj;
      this._objMessage = objMessage;
    }
    public override _IError DowncastClone() {
      if (this is _IError dt) { return dt; }
      return new Error_OpaqueWithText(_obj, _objMessage);
    }
    public override bool Equals(object other) {
      var oth = other as SimpleTypesSmithyStringTypes_Compile.Error_OpaqueWithText;
      return oth != null && this._obj == oth._obj && object.Equals(this._objMessage, oth._objMessage);
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 2;
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._obj));
      hash = ((hash << 5) + hash) + ((ulong)Dafny.Helpers.GetHashCode(this._objMessage));
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleTypesSmithyStringTypes.Error.OpaqueWithText";
      s += "(";
      s += Dafny.Helpers.ToString(this._obj);
      s += ", ";
      s += Dafny.Helpers.ToString(this._objMessage);
      s += ")";
      return s;
    }
  }

  public partial class OpaqueError {
    private static readonly Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IError> _TYPE = new Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IError>(SimpleTypesSmithyStringTypes_Compile.Error.Default());
    public static Dafny.TypeDescriptor<SimpleTypesSmithyStringTypes_Compile._IError> _TypeDescriptor() {
      return _TYPE;
    }
    public static bool _Is(SimpleTypesSmithyStringTypes_Compile._IError __source) {
      SimpleTypesSmithyStringTypes_Compile._IError _0_e = __source;
      return ((_0_e).is_Opaque) || ((_0_e).is_OpaqueWithText);
    }
  }

  public partial class DummySubsetType {
    private static readonly BigInteger Witness = BigInteger.One;
    public static BigInteger Default() {
      return Witness;
    }
    private static readonly Dafny.TypeDescriptor<BigInteger> _TYPE = new Dafny.TypeDescriptor<BigInteger>(SimpleTypesSmithyStringTypes_Compile.DummySubsetType.Default());
    public static Dafny.TypeDescriptor<BigInteger> _TypeDescriptor() {
      return _TYPE;
    }
    public static bool _Is(BigInteger __source) {
      BigInteger _1_x = __source;
      return SimpleTypesSmithyStringTypes_Compile.__default.IsDummySubsetType(_1_x);
    }
  }
} // end of namespace SimpleTypesSmithyStringTypes_Compile
namespace GetStringInputPartitioning_Compile {

  public partial class __default {
    public static Dafny.ISequence<bool> IsInPartitionList(SimpleTypesSmithyStringTypes_Compile._IGetStringInput x) {
      return OptionStringDomainModel_Compile.__default.IsInPartitionList((x).dtor_value);
    }
    public static Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> RepresentativeValues()
    {
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> output = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.Empty;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _0_values;
      Dafny.ISequence<Wrappers_Compile._IOption<Dafny.ISequence<char>>> _out0;
      _out0 = OptionStringDomainModel_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _1_temp;
      _1_temp = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.FromElements();
      BigInteger _2_i;
      _2_i = BigInteger.Zero;
      while ((_2_i) < (new BigInteger((_0_values).Count))) {
        _1_temp = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.Concat(_1_temp, Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.FromElements(SimpleTypesSmithyStringTypes_Compile.GetStringInput.create((_0_values).Select(_2_i))));
        _2_i = (_2_i) + (BigInteger.One);
      }
      output = _1_temp;
      return output;
    }
    public static JSON_mValues_Compile._IJSON ToJSON(SimpleTypesSmithyStringTypes_Compile._IGetStringInput x) {
      return JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("value"), OptionStringDomainModel_Compile.__default.ToJSON((x).dtor_value))));
    }
    public static BigInteger CountTrues(Dafny.ISequence<bool> partitions) {
      BigInteger _0___accumulator = BigInteger.Zero;
    TAIL_CALL_START: ;
      if ((new BigInteger((partitions).Count)).Sign == 0) {
        return (BigInteger.Zero) + (_0___accumulator);
      } else if ((partitions).Select(BigInteger.Zero)) {
        _0___accumulator = (_0___accumulator) + (BigInteger.One);
        Dafny.ISequence<bool> _in0 = (partitions).Drop(BigInteger.One);
        partitions = _in0;
        goto TAIL_CALL_START;
      } else {
        Dafny.ISequence<bool> _in1 = (partitions).Drop(BigInteger.One);
        partitions = _in1;
        goto TAIL_CALL_START;
      }
    }
    public static bool IsInExactlyOnePartition(SimpleTypesSmithyStringTypes_Compile._IGetStringInput x) {
      Dafny.ISequence<bool> _0_partitionList = GetStringInputPartitioning_Compile.__default.IsInPartitionList(x);
      return (GetStringInputPartitioning_Compile.__default.CountTrues(_0_partitionList)) == (BigInteger.One);
    }
    public static bool IsValidMember(SimpleTypesSmithyStringTypes_Compile._IGetStringInput x) {
      return (true) && (GetStringInputPartitioning_Compile.__default.IsInExactlyOnePartition(x));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _0_values;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _out0;
      _out0 = GetStringInputPartitioning_Compile.__default.AllValues();
      _0_values = _out0;
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _1_out;
      _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_values).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        SimpleTypesSmithyStringTypes_Compile._IGetStringInput _3_value;
        _3_value = (_0_values).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        _4_json = GetStringInputPartitioning_Compile.__default.ToJSON(_3_value);
        _1_out = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_1_out, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Array(_1_out);
      return outputJSON;
    }
    public static Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> AllValues()
    {
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> output = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.Empty;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _0_raw;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _out0;
      _out0 = GetStringInputPartitioning_Compile.__default.RepresentativeValues();
      _0_raw = _out0;
      Dafny.ISet<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _1_seen;
      _1_seen = Dafny.Set<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.FromElements();
      output = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.FromElements();
      BigInteger _hi0 = new BigInteger((_0_raw).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        if (!(_1_seen).Contains((_0_raw).Select(_2_i))) {
          output = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.Concat(output, Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.FromElements((_0_raw).Select(_2_i)));
          _1_seen = Dafny.Set<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.Union(_1_seen, Dafny.Set<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.FromElements((_0_raw).Select(_2_i)));
        }
      }
      return output;
    }
    public static SimpleTypesSmithyStringTypes_Compile._IGetStringInput SomeValue()
    {
      SimpleTypesSmithyStringTypes_Compile._IGetStringInput output = SimpleTypesSmithyStringTypes_Compile.GetStringInput.Default();
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _0_allValues;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _out0;
      _out0 = GetStringInputPartitioning_Compile.__default.AllValues();
      _0_allValues = _out0;
      output = (_0_allValues).Select(BigInteger.Zero);
      return output;
    }
  }
} // end of namespace GetStringInputPartitioning_Compile
namespace GetStringModelImpl_Compile {

  public partial class __default {
    public static Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> DomainRepresentativeValues()
    {
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> output = Dafny.Sequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput>.Empty;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _out0;
      _out0 = GetStringInputPartitioning_Compile.__default.RepresentativeValues();
      output = _out0;
      return output;
    }
    public static SimpleTypesSmithyStringTypes_Compile._IGetStringOutput EvaluationRule(SimpleTypesSmithyStringTypes_Compile._IGetStringInput x)
    {
      SimpleTypesSmithyStringTypes_Compile._IGetStringOutput output = SimpleTypesSmithyStringTypes_Compile.GetStringOutput.Default();
      output = SimpleTypesSmithyStringTypes_Compile.GetStringOutput.create((x).dtor_value);
      return output;
    }
    public static JSON_mValues_Compile._IJSON InputToJSON(SimpleTypesSmithyStringTypes_Compile._IGetStringInput x) {
      return GetStringInputPartitioning_Compile.__default.ToJSON(x);
    }
    public static JSON_mValues_Compile._IJSON OutputToJSON(SimpleTypesSmithyStringTypes_Compile._IGetStringOutput x) {
      return JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("value"), OptionStringDomainModel_Compile.__default.ToJSON((x).dtor_value))));
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      Dafny.ISequence<JSON_mValues_Compile._IJSON> _0_operationsJSON;
      _0_operationsJSON = Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements();
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _1_inputValues;
      Dafny.ISequence<SimpleTypesSmithyStringTypes_Compile._IGetStringInput> _out0;
      _out0 = GetStringModelImpl_Compile.__default.DomainRepresentativeValues();
      _1_inputValues = _out0;
      BigInteger _hi0 = new BigInteger((_1_inputValues).Count);
      for (BigInteger _2_i = BigInteger.Zero; _2_i < _hi0; _2_i++) {
        SimpleTypesSmithyStringTypes_Compile._IGetStringInput _3_input;
        _3_input = (_1_inputValues).Select(_2_i);
        JSON_mValues_Compile._IJSON _4_json;
        JSON_mValues_Compile._IJSON _out1;
        _out1 = GetStringModelImpl_Compile.__default.JSONToWriteForOperationInputValue(_3_input);
        _4_json = _out1;
        _0_operationsJSON = Dafny.Sequence<JSON_mValues_Compile._IJSON>.Concat(_0_operationsJSON, Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_4_json));
      }
      outputJSON = JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(GetStringModelImpl_Compile.__default.operationName, JSON_mValues_Compile.JSON.create_Array(_0_operationsJSON))));
      return outputJSON;
    }
    public static JSON_mValues_Compile._IJSON JSONToWriteForOperationInputValue(SimpleTypesSmithyStringTypes_Compile._IGetStringInput input)
    {
      JSON_mValues_Compile._IJSON output = JSON_mValues_Compile.JSON.Default();
      SimpleTypesSmithyStringTypes_Compile._IGetStringOutput _0_outputValue;
      SimpleTypesSmithyStringTypes_Compile._IGetStringOutput _out0;
      _out0 = GetStringModelImpl_Compile.__default.EvaluationRule(input);
      _0_outputValue = _out0;
      JSON_mValues_Compile._IJSON _1_inputJSON;
      _1_inputJSON = GetStringModelImpl_Compile.__default.InputToJSON(input);
      JSON_mValues_Compile._IJSON _2_outputJSON;
      _2_outputJSON = GetStringModelImpl_Compile.__default.OutputToJSON(_0_outputValue);
      output = JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("input"), _1_inputJSON), _System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.FromString("output"), _2_outputJSON)));
      return output;
    }
    public static Dafny.ISequence<char> operationName { get {
      return Dafny.Sequence<char>.FromString("GetString");
    } }
  }
} // end of namespace GetStringModelImpl_Compile
namespace SimpleStringImpl_Compile {

  public partial class __default {
    public static Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> GetString(SimpleStringImpl_Compile._IConfig config, SimpleTypesSmithyStringTypes_Compile._IGetStringInput input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringOutput.Default());
      SimpleTypesSmithyStringTypes_Compile._IGetStringOutput _0_res;
      _0_res = SimpleTypesSmithyStringTypes_Compile.GetStringOutput.create((input).dtor_value);
      output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError>.create_Success(_0_res);
      return output;
      return output;
    }
    public static Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> GetStringKnownValue(SimpleStringImpl_Compile._IConfig config, SimpleTypesSmithyStringTypes_Compile._IGetStringInput input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringOutput.Default());
      if (!(((input).dtor_value).is_Some)) {
        throw new Dafny.HaltException("dafny/src/simple_types/strings/SimpleStringImpl.dfy(31,8): " + Dafny.Sequence<char>.FromString("expectation violation"));}
      if (!((((input).dtor_value).dtor_value).Equals(Dafny.Sequence<char>.FromString("TEST_SIMPLE_STRING_KNOWN_VALUE")))) {
        throw new Dafny.HaltException("dafny/src/simple_types/strings/SimpleStringImpl.dfy(32,8): " + Dafny.Sequence<char>.FromString("expectation violation"));}
      SimpleTypesSmithyStringTypes_Compile._IGetStringOutput _0_res;
      _0_res = SimpleTypesSmithyStringTypes_Compile.GetStringOutput.create((input).dtor_value);
      output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError>.create_Success(_0_res);
      return output;
      return output;
    }
    public static Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> GetStringUTF8(SimpleStringImpl_Compile._IConfig config, SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.Default());
      if (!(((input).dtor_value).is_Some)) {
        throw new Dafny.HaltException("dafny/src/simple_types/strings/SimpleStringImpl.dfy(38,8): " + Dafny.Sequence<char>.FromString("expectation violation"));}
      SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output _0_res;
      _0_res = SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.create((input).dtor_value);
      output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError>.create_Success(_0_res);
      return output;
      return output;
    }
    public static Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> GetStringUTF8KnownValue(SimpleStringImpl_Compile._IConfig config, SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.Default());
      if (!(((input).dtor_value).is_Some)) {
        throw new Dafny.HaltException("dafny/src/simple_types/strings/SimpleStringImpl.dfy(44,8): " + Dafny.Sequence<char>.FromString("expectation violation"));}
      Dafny.ISequence<byte> _0_expected;
      _0_expected = Dafny.Sequence<byte>.FromElements((byte)(72), (byte)(101), (byte)(108), (byte)(108), (byte)(111));
      if (!((((input).dtor_value).dtor_value).Equals(_0_expected))) {
        throw new Dafny.HaltException("dafny/src/simple_types/strings/SimpleStringImpl.dfy(46,8): " + Dafny.Sequence<char>.FromString("expectation violation"));}
      SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output _1_res;
      _1_res = SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.create((input).dtor_value);
      output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError>.create_Success(_1_res);
      return output;
      return output;
    }
  }

  public interface _IConfig {
    bool is_Config { get; }
    _IConfig DowncastClone();
  }
  public class Config : _IConfig {
    public Config() {
    }
    public _IConfig DowncastClone() {
      if (this is _IConfig dt) { return dt; }
      return new Config();
    }
    public override bool Equals(object other) {
      var oth = other as SimpleStringImpl_Compile.Config;
      return oth != null;
    }
    public override int GetHashCode() {
      ulong hash = 5381;
      hash = ((hash << 5) + hash) + 0;
      return (int) hash;
    }
    public override string ToString() {
      string s = "SimpleStringImpl.Config.Config";
      return s;
    }
    private static readonly SimpleStringImpl_Compile._IConfig theDefault = create();
    public static SimpleStringImpl_Compile._IConfig Default() {
      return theDefault;
    }
    private static readonly Dafny.TypeDescriptor<SimpleStringImpl_Compile._IConfig> _TYPE = new Dafny.TypeDescriptor<SimpleStringImpl_Compile._IConfig>(SimpleStringImpl_Compile.Config.Default());
    public static Dafny.TypeDescriptor<SimpleStringImpl_Compile._IConfig> _TypeDescriptor() {
      return _TYPE;
    }
    public static _IConfig create() {
      return new Config();
    }
    public static _IConfig create_Config() {
      return create();
    }
    public bool is_Config { get { return true; } }
    public static System.Collections.Generic.IEnumerable<_IConfig> AllSingletonConstructors {
      get {
        yield return Config.create();
      }
    }
  }
} // end of namespace SimpleStringImpl_Compile
namespace SimpleString_Compile {

  public partial class __default {
    public static SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig DefaultSimpleStringConfig() {
      return SimpleTypesSmithyStringTypes_Compile.SimpleStringConfig.create();
    }
    public static Wrappers_Compile._IResult<SimpleString_Compile.SimpleStringClient, SimpleTypesSmithyStringTypes_Compile._IError> SimpleString(SimpleTypesSmithyStringTypes_Compile._ISimpleStringConfig config)
    {
      Wrappers_Compile._IResult<SimpleString_Compile.SimpleStringClient, SimpleTypesSmithyStringTypes_Compile._IError> res = default(Wrappers_Compile._IResult<SimpleString_Compile.SimpleStringClient, SimpleTypesSmithyStringTypes_Compile._IError>);
      SimpleString_Compile.SimpleStringClient _0_client;
      SimpleString_Compile.SimpleStringClient _nw0 = new SimpleString_Compile.SimpleStringClient();
      _nw0.__ctor(SimpleStringImpl_Compile.Config.create());
      _0_client = _nw0;
      res = Wrappers_Compile.Result<SimpleString_Compile.SimpleStringClient, SimpleTypesSmithyStringTypes_Compile._IError>.create_Success(_0_client);
      return res;
      return res;
    }
    public static Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile.ISimpleTypesStringClient, SimpleTypesSmithyStringTypes_Compile._IError> CreateSuccessOfClient(SimpleTypesSmithyStringTypes_Compile.ISimpleTypesStringClient client) {
      return Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile.ISimpleTypesStringClient, SimpleTypesSmithyStringTypes_Compile._IError>.create_Success(client);
    }
    public static Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile.ISimpleTypesStringClient, SimpleTypesSmithyStringTypes_Compile._IError> CreateFailureOfError(SimpleTypesSmithyStringTypes_Compile._IError error) {
      return Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile.ISimpleTypesStringClient, SimpleTypesSmithyStringTypes_Compile._IError>.create_Failure(error);
    }
  }

  public partial class SimpleStringClient : SimpleTypesSmithyStringTypes_Compile.ISimpleTypesStringClient {
    public SimpleStringClient() {
      this._config = SimpleStringImpl_Compile.Config.Default();
    }
    public void __ctor(SimpleStringImpl_Compile._IConfig config)
    {
      (this)._config = config;
    }
    public Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> GetString(SimpleTypesSmithyStringTypes_Compile._IGetStringInput input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringOutput.Default());
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> _out0;
      _out0 = SimpleStringImpl_Compile.__default.GetString((this).config, input);
      output = _out0;
      return output;
    }
    public Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> GetStringKnownValue(SimpleTypesSmithyStringTypes_Compile._IGetStringInput input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringOutput.Default());
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringOutput, SimpleTypesSmithyStringTypes_Compile._IError> _out0;
      _out0 = SimpleStringImpl_Compile.__default.GetStringKnownValue((this).config, input);
      output = _out0;
      return output;
    }
    public Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> GetStringUTF8(SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.Default());
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> _out0;
      _out0 = SimpleStringImpl_Compile.__default.GetStringUTF8((this).config, input);
      output = _out0;
      return output;
    }
    public Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> GetStringUTF8KnownValue(SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Input input)
    {
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> output = Wrappers_Compile.Result<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError>.Default(SimpleTypesSmithyStringTypes_Compile.GetStringUTF8Output.Default());
      Wrappers_Compile._IResult<SimpleTypesSmithyStringTypes_Compile._IGetStringUTF8Output, SimpleTypesSmithyStringTypes_Compile._IError> _out0;
      _out0 = SimpleStringImpl_Compile.__default.GetStringUTF8KnownValue((this).config, input);
      output = _out0;
      return output;
    }
    public SimpleStringImpl_Compile._IConfig _config {get; set;}
    public SimpleStringImpl_Compile._IConfig config { get {
      return this._config;
    } }
  }
} // end of namespace SimpleString_Compile
namespace SimpleStringServiceModel_Compile {

  public partial class __default {
    public static JSON_mValues_Compile._IJSON GenerateOperationsJSON()
    {
      JSON_mValues_Compile._IJSON operationsJSON = JSON_mValues_Compile.JSON.Default();
      JSON_mValues_Compile._IJSON _0_GetStringJSON;
      JSON_mValues_Compile._IJSON _out0;
      _out0 = GetStringModelImpl_Compile.__default.GenerateJSON();
      _0_GetStringJSON = _out0;
      operationsJSON = JSON_mValues_Compile.JSON.create_Array(Dafny.Sequence<JSON_mValues_Compile._IJSON>.FromElements(_0_GetStringJSON));
      return operationsJSON;
    }
    public static JSON_mValues_Compile._IJSON GenerateJSON()
    {
      JSON_mValues_Compile._IJSON outputJSON = JSON_mValues_Compile.JSON.Default();
      JSON_mValues_Compile._IJSON _0_operationsJSON;
      JSON_mValues_Compile._IJSON _out0;
      _out0 = SimpleStringServiceModel_Compile.__default.GenerateOperationsJSON();
      _0_operationsJSON = _out0;
      outputJSON = JSON_mValues_Compile.JSON.create_Object(Dafny.Sequence<_System._ITuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>>.FromElements(_System.Tuple2<Dafny.ISequence<char>, JSON_mValues_Compile._IJSON>.create(Dafny.Sequence<char>.Concat(Dafny.Sequence<char>.Concat(SimpleStringServiceModel_Compile.__default.serviceNamespace, Dafny.Sequence<char>.FromString(":")), SimpleStringServiceModel_Compile.__default.serviceName), _0_operationsJSON)));
      return outputJSON;
    }
    public static Dafny.ISequence<char> serviceNamespace { get {
      return Dafny.Sequence<char>.FromString("simple.types.smithyString");
    } }
    public static Dafny.ISequence<char> serviceName { get {
      return Dafny.Sequence<char>.FromString("SimpleString");
    } }
  }
} // end of namespace SimpleStringServiceModel_Compile
namespace Tests2_Compile {

  public partial class __default {
    [Xunit.Fact]
    public static void TestVectorsV2()
    {
      JSON_mValues_Compile._IJSON _0_outputJSON;
      JSON_mValues_Compile._IJSON _out0;
      _out0 = SimpleStringServiceModel_Compile.__default.GenerateJSON();
      _0_outputJSON = _out0;
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _1_out;
      Wrappers_Compile._IResult<_System._ITuple0, Dafny.ISequence<char>> _out1;
      _out1 = TestVectorsNameTBDModule_Compile.__default.WriteVectorsFile(Dafny.Sequence<char>.FromString("out.json"), _0_outputJSON);
      _1_out = _out1;
    }
  }
} // end of namespace Tests2_Compile
namespace _module {

} // end of namespace _module
