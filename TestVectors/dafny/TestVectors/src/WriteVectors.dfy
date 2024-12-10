// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"
include "VectorsComposition/AllEsdkV4NoReqEc.dfy"
include "VectorsComposition/AllEsdkV4WithReqEc.dfy"
include "WriteEsdkJsonManifests.dfy"

module {:options "-functionSyntax:4"} WriteVectors {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import ESDK
  import MaterialProviders
  import opened CompleteVectors
  import opened Wrappers
  import opened StandardLibrary.UInt
  import HexStrings
  import opened JSON.Values
  import JSONHelpers
  import EsdkManifestOptions
  import EsdkTestVectors
  import AllEsdkV4NoReqEc
  import AllEsdkV4WithReqEc
  import WriteEsdkJsonManifests

  import UUID
  import UTF8
  import JSON.API
  import SortedSets
  import FileIO
  import opened Relations
  import opened Seq.MergeSort

  // This is a HACK!
  // This is *ONLY* because this is wrapping the MPL
  import AlgorithmSuites

  function GetCommitmentPolicyString(algorithmSuite: mplTypes.AlgorithmSuiteInfo)
    : (commitmentPolicy: string)
  {
    match algorithmSuite.id
    case ESDK(_) =>
      if algorithmSuite.commitment.None? then
        "FORBID_ENCRYPT_ALLOW_DECRYPT"
      else
        "REQUIRE_ENCRYPT_REQUIRE_DECRYPT"
    case DBE(_) => "NOT SUPPORTED FOR UNSTRUCTURED ENCRYPTION"
  }

  function GetCommitmentPolicyType(commitmentPolicy: string)
    : (commitmentPolicyType: mplTypes.CommitmentPolicy)
  {
    if commitmentPolicy == "FORBID_ENCRYPT_ALLOW_DECRYPT" then
      mplTypes.CommitmentPolicy.ESDK(mplTypes.ESDKCommitmentPolicy.FORBID_ENCRYPT_ALLOW_DECRYPT)
    else
      mplTypes.CommitmentPolicy.ESDK(mplTypes.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT)
  }


  method {:vcs_split_on_every_assert} WriteTestVectors(op: EsdkManifestOptions.ManifestOptions)
    returns (output: Result<(), string>)
    requires op.EncryptManifest?
  {
    var version := op.version;
    var allTests :- getVersionTests(version);

    var tests := SortedSets.ComputeSetToSequence(allTests);

    DescriptionLessThanIsTotal();
    var sortedTests := MergeSortBy(tests, DescriptionLessThan);

    var testsJSON: seq<(string, JSON)> := [];

    for i := 0 to |sortedTests|
    {
      :- Need(
        && sortedTests[i].algorithmSuiteId.Some?,
        "No algorithm suite defined in test"
      );

      var id := AllAlgorithmSuites.ToHex(sortedTests[i].algorithmSuiteId.value);
      var uuid :- expect UUID.GenerateUUID();
      var test :- WriteEsdkJsonManifests.EncryptTestVectorToJson(sortedTests[i]);
      testsJSON := testsJSON + [(uuid, test)];
    }

    var manifestJson := Object([
                                 ("type", String("awses-encrypt")),
                                 ("version", Number(Int(5)))]);

    var plaintexts := Object([("small", Number(Int(10240)))]);

    var esdkEncryptManifests := Object(
      [
        ("manifest", manifestJson),
        ("keys", String("file://keys.json")),
        ("plaintexts", plaintexts),
        ("tests", Object(testsJSON))
      ]
    );

    var esdkEncryptManifestBytes :- expect API.Serialize(esdkEncryptManifests);
    var esdkEncryptManifestBv := JSONHelpers.BytesBv(esdkEncryptManifestBytes);

    var _ :- expect FileIO.WriteBytesToFile(
      op.encryptManifestOutput + "encrypt-manifest.json",
      esdkEncryptManifestBv
    );

    output := Success(());
  }

  method {:vcs_split_on_every_assert} WriteDecryptManifest(
    op: EsdkManifestOptions.ManifestOptions,
    keys: KeyVectors.KeyVectorsClient,
    tests: seq<EsdkTestVectors.EsdkDecryptTestVector>
  )
    returns (output: Result<(), string>)
    requires op.Encrypt?
    requires keys.ValidState()
    ensures keys.ValidState()
  {
    var testsJSON: seq<(string, JSON)> := [];

    for i := 0 to |tests|
    {
      var name := tests[i].id;
      var test :- WriteEsdkJsonManifests.DecryptTestVectorToJson(tests[i]);
      testsJSON := testsJSON + [(name, test)];
    }

    var manifestJson := Object([
                                 ("type", String("awses-decrypt")),
                                 ("version", Number(Int(5)))]);
    var clientJson := Object([
                               ("name", String("aws-encryption-sdk-dafny")),
                               ("version", String("4.1.0"))]);

    var esdkDecryptManifest := Object(
      [
        ("manifest", manifestJson),
        // TODO create an extern that gets that runtimes namespace and latest version
        ("client", clientJson),
        ("keys", String("file://keys.json")),
        ("tests", Object(testsJSON))
      ]
    );

    var esdkDecryptManifestBytes :- expect API.Serialize(esdkDecryptManifest);
    var esdkDecryptManifestBv := JSONHelpers.BytesBv(esdkDecryptManifestBytes);

    var _ :- expect FileIO.WriteBytesToFile(
      op.decryptManifestOutput + "decrypt-manifest.json",
      esdkDecryptManifestBv
    );

    output := Success(());
  }

  function getVersionTests(version: nat): (ret: Result<set<EsdkTestVectors.EsdkEncryptTestVector>, string>)
  {
    match version
    case 5 => Success(AllEsdkV4NoReqEc.Tests + AllEsdkV4WithReqEc.Tests)
    case _ => Failure("Only version 4 of generate manifest is supported\n")
  }

  predicate DescriptionLessThan(x: EsdkTestVectors.EsdkEncryptTestVector, y: EsdkTestVectors.EsdkEncryptTestVector) {
    Below(x.description, y.description)
  }

  // These lemmas are needed to help speed up and reduce verification resources of the DescriptionLessThan predicate that
  // is used in the MergeSortBy() above.
  lemma DescriptionLessThanIsTotal()
    ensures TotalOrdering(DescriptionLessThan)
  {
    BelowIsTotal();
    assert TotalOrdering(Below);
    assert Reflexive(DescriptionLessThan) by {
      forall x {
        DescriptionLessThanIsReflexive(x);
      }
    }
    assert AntiSymmetric(DescriptionLessThan) by {
      forall x, y | DescriptionLessThan(x, y) && DescriptionLessThan(y, x) {
        DescriptionLessThanIsAntiSymmetric(x, y);
      }
    }
    assert Relations.Transitive(DescriptionLessThan) by {
      forall x, y, z | DescriptionLessThan(x, y) && DescriptionLessThan(y, z) {
        DescriptionLessThanIsTransitive(x, y, z);
      }
    }
    assert StronglyConnected(DescriptionLessThan) by {
      forall x, y {
        DescriptionLessThanIsStronglyConnected(x, y);
      }
    }
  }

  lemma DescriptionLessThanIsReflexive(x: EsdkTestVectors.EsdkEncryptTestVector)
    ensures DescriptionLessThan(x, x)
  {
    BelowIsReflexive(x.description);
  }

  // not actually required for sorting. Standard library being updated.
  lemma {:axiom} DescriptionLessThanIsAntiSymmetric(x: EsdkTestVectors.EsdkEncryptTestVector, y: EsdkTestVectors.EsdkEncryptTestVector)
    requires DescriptionLessThan(x, y) && DescriptionLessThan(y, x)
    ensures x == y

  lemma DescriptionLessThanIsTransitive(x: EsdkTestVectors.EsdkEncryptTestVector, y: EsdkTestVectors.EsdkEncryptTestVector, z: EsdkTestVectors.EsdkEncryptTestVector)
    requires DescriptionLessThan(x, y) && DescriptionLessThan(y, z)
    ensures DescriptionLessThan(x, z)
  {
    BelowIsTransitive(x.description, y.description, z.description);
  }

  lemma DescriptionLessThanIsStronglyConnected(x: EsdkTestVectors.EsdkEncryptTestVector, y: EsdkTestVectors.EsdkEncryptTestVector)
    ensures DescriptionLessThan(x, y) || DescriptionLessThan(y, x)
  {
    BelowIsStronglyConnected(x.description, y.description);
  }

  predicate Below(x: string, y: string) {
    |x| != 0 ==>
      && |y| != 0
      && x[0] <= y[0]
      && (x[0] == y[0] ==> Below(x[1..], y[1..]))
  }

  lemma BelowIsTotal()
    ensures TotalOrdering(Below)
  {
    assert Reflexive(Below) by {
      forall x {
        BelowIsReflexive(x);
      }
    }
    assert AntiSymmetric(Below) by {
      forall x, y | Below(x, y) && Below(y, x) {
        BelowIsAntiSymmetric(x, y);
      }
    }
    assert Relations.Transitive(Below) by {
      forall x, y, z | Below(x, y) && Below(y, z) {
        BelowIsTransitive(x, y, z);
      }
    }
    assert StronglyConnected(Below) by {
      forall x, y {
        BelowIsStronglyConnected(x, y);
      }
    }
  }

  lemma BelowIsReflexive(x: seq<char>)
    ensures Below(x, x)
  {
  }

  lemma BelowIsAntiSymmetric(x: seq<char>, y: seq<char>)
    requires Below(x, y) && Below(y, x)
    ensures x == y
  {
  }

  lemma BelowIsTransitive(x: seq<char>, y: seq<char>, z: seq<char>)
    requires Below(x, y) && Below(y, z)
    ensures Below(x, z)
  {
  }

  lemma BelowIsStronglyConnected(x: seq<char>, y: seq<char>)
    ensures Below(x, y) || Below(y, x)
  {
  }
}