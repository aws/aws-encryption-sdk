// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "Serialize/HeaderTypes.dfy"
include "Serialize/SerializableTypes.dfy"

module KeyDerivation {
  import opened Wrappers
  import opened UInt = StandardLibrary.UInt
  import UTF8
  import Types = AwsCryptographyEncryptionSdkTypes
  import MPL = AwsCryptographyMaterialProvidersTypes
  import AwsCryptographyPrimitivesTypes
  import Primitives = AtomicPrimitives
  import HeaderTypes
  import SerializableTypes

  // Convenience container to hold both a data key and an optional commitment key
  // to support algorithm suites that provide commitment and those that do not
  datatype ExpandedKeyMaterial = ExpandedKeyMaterial(
    nameonly dataKey: seq<uint8>,
    nameonly commitmentKey: Option<seq<uint8>>
  )

  /*
   * Derives a single data key from an input plaintext data key, using "v1"-style
   * key derivation (that is, no key commitment).
   */
  method DeriveKey(
    messageId: HeaderTypes.MessageId,
    plaintextDataKey: seq<uint8>,
    suite: MPL.AlgorithmSuiteInfo,
    crypto: Primitives.AtomicPrimitivesClient,
    // TODO Post-#619: Refactor, breaking Net v4.0.0 logic out into independent method
    onNetV4Retry: bool
  )
    returns (res: Result<ExpandedKeyMaterial, Types.Error>)

    // This should only be used for v1 algorithms
    requires suite.messageVersion == 1
    requires suite.commitment.None?

    requires suite.kdf.HKDF?
    ==>
      && |plaintextDataKey| == suite.kdf.HKDF.inputKeyLength as nat
      && suite.kdf.HKDF.outputKeyLength == SerializableTypes.GetEncryptKeyLength(suite)
    requires suite.kdf.IDENTITY? ==> |plaintextDataKey| == SerializableTypes.GetEncryptKeyLength(suite) as nat

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    ensures res.Success? ==> |res.value.dataKey| == SerializableTypes.GetEncryptKeyLength(suite) as nat
    // ensures res.Success? ==> IsDerivedKey(res.value.dataKey)
    ensures res.Success? ==> res.value.commitmentKey.None?
    ensures res.Success? ==> suite.kdf.IDENTITY? || suite.kdf.HKDF?
    ensures suite.kdf.None? ==> res.Failure?
  {
    //= compliance/client-apis/encrypt.txt#2.6.1
    //# The algorithm used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) defined
    //# above.

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# The algorithm suite used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) associated
    //# with the returned decryption materials.
    match suite.kdf {
      case IDENTITY(i) => {
        return Success(ExpandedKeyMaterial(dataKey:=plaintextDataKey, commitmentKey:=None()));
      }
      case HKDF(hkdf) => {
        var hkdfInput := AwsCryptographyPrimitivesTypes.HkdfInput(
          digestAlgorithm := hkdf.hmac,
          salt := None,
          ikm := plaintextDataKey,
          info := suite.binaryId + messageId,
          expectedLength := hkdf.outputKeyLength
        );
        // TODO Post-#619: Formally Verify this section
        // TODO Post-#619: Duvet this section
        if onNetV4Retry {
          hkdfInput := AwsCryptographyPrimitivesTypes.HkdfInput(
            digestAlgorithm := hkdf.hmac,
            salt := None,
            ikm := plaintextDataKey,
            info := suite.binaryId,
            expectedLength := hkdf.outputKeyLength
            );
        }
        var maybeDerivedKey := crypto.Hkdf(hkdfInput);
        var derivedKey :- maybeDerivedKey.MapFailure(e => Types.AwsCryptographyPrimitives(e));

        return Success(ExpandedKeyMaterial(dataKey:=derivedKey, commitmentKey:=None()));
      }
      case None => {
        return Failure(Types.AwsEncryptionSdkException(
          message := "None is not a valid Key Derivation Function"));
      }
    }
  }

  // UTF8 encode of "COMMITKEY"
  const COMMIT_LABEL: UTF8.ValidUTF8Bytes :=
    var s := [0x43, 0x4f, 0x4D, 0x4D, 0x49, 0x54, 0x4b, 0x45, 0x59];
    assert UTF8.ValidUTF8Range(s, 0, 9);
    s
  // UTF8 encode of "DERIVEKEY"
  const KEY_LABEL: UTF8.ValidUTF8Bytes :=
    var s := [0x44, 0x45, 0x52, 0x49, 0x56, 0x45, 0x4B, 0x45, 0x59];
    assert UTF8.ValidUTF8Range(s, 0, 9);
    s

  predicate {:opaque} IsDerivedKey(derivedDataKey: seq<uint8>)
  {
    true
  }

  /*
   * Derives keys from an input plaintext data key, using "v2"-style
   * key derivation (that is, including key commitment).
   */
  method ExpandKeyMaterial(
    messageId: HeaderTypes.MessageId,
    plaintextKey: seq<uint8>,
    suite: MPL.AlgorithmSuiteInfo,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<ExpandedKeyMaterial, Types.Error>)

    // This should only be used for v2 algorithms
    requires suite.messageVersion == 2
    requires suite.commitment.HKDF?

    // For v2 algorithms, KDF can only be HKDF
    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# The algorithm suite used to derive a data key from the
    //# plaintext data key MUST be the key derivation algorithm
    //# (../framework/algorithm-suites.md#key-derivation-algorithm) included
    //# in the algorithm suite (../framework/algorithm-suites.md) associated
    //# with the returned decryption materials.
    requires suite.kdf.HKDF?
    requires SerializableTypes.GetEncryptKeyLength(suite) == suite.kdf.HKDF.outputKeyLength

    requires |messageId| != 0
    requires |plaintextKey| == suite.kdf.HKDF.inputKeyLength as nat
    // TODO: seems like the below pre-condition should follow from the above
    // pre-condition (|key| == keyLength), but apparently we don't prove/require
    // anywhere that keys be small.
    requires |plaintextKey| < INT32_MAX_LIMIT

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# If the algorithm suite (../framework/
    //# algorithm-suites.md#algorithm-suites-encryption-key-derivation-
    //# settings) supports key commitment (../framework/algorithm-
    //# suites.md#key-commitment) then the commit key (../framework/
    //# algorithm-suites.md#commit-key) MUST be derived from the plaintext
    //# data key using the commit key derivation (../framework/algorithm-
    //# suites.md#algorithm-suites-commit-key-derivation-settings).
    ensures res.Success? ==>
      && res.value.commitmentKey.Some?
      && |res.value.commitmentKey.value| == suite.commitment.HKDF.outputKeyLength as nat

    ensures res.Success? ==> |res.value.dataKey|  == SerializableTypes.GetEncryptKeyLength(suite) as nat

  {
    var digest := suite.commitment.HKDF.hmac;
    var info := suite.binaryId + KEY_LABEL;

    var hkdfExtractInput := AwsCryptographyPrimitivesTypes.HkdfExtractInput(
      digestAlgorithm := digest,
      salt := Some(messageId),
      ikm := plaintextKey
    );

    var maybePseudoRandomKey := crypto.HkdfExtract(hkdfExtractInput);

    var pseudoRandomKey :- maybePseudoRandomKey
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    var encryptKeyInput := AwsCryptographyPrimitivesTypes.HkdfExpandInput(
      digestAlgorithm := digest,
      prk := pseudoRandomKey,
      info := info,
      expectedLength := suite.kdf.HKDF.outputKeyLength
    );
    var commitKeyInput := encryptKeyInput.(
      info := COMMIT_LABEL,
      expectedLength := suite.commitment.HKDF.outputKeyLength
    );

    var maybeEncryptKey := crypto.HkdfExpand(encryptKeyInput);
    var maybeCommitKey := crypto.HkdfExpand(commitKeyInput);

    var encryptKey :- maybeEncryptKey
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));
    var commitKey :- maybeCommitKey
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    return Success(ExpandedKeyMaterial(
      dataKey:=encryptKey,
      commitmentKey:=Some(commitKey)
    ));
  }

  /*
   * Derives key material for encryption/decryption. Delegates out to specific methods
   * based on the input algorithm suite.
   */
  method DeriveKeys(
    messageId: HeaderTypes.MessageId,
    plaintextKey: seq<uint8>,
    suite: MPL.AlgorithmSuiteInfo,
    crypto: Primitives.AtomicPrimitivesClient,
    // TODO Post-#619: Refactor, breaking Net v4.0.0 logic out into independent method
    netV4_0_0_RetryPolicy: Types.NetV4_0_0_RetryPolicy,
    onNetV4Retry: bool
  )
    returns (res: Result<ExpandedKeyMaterial, Types.Error>)

    requires |messageId| != 0
    requires |plaintextKey| == SerializableTypes.GetEncryptKeyLength(suite) as nat
    requires |plaintextKey| < INT32_MAX_LIMIT

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    ensures res.Success? ==>
      |res.value.dataKey| == SerializableTypes.GetEncryptKeyLength(suite) as nat

    ensures
      && res.Success?
      && suite.commitment.None?
    ==>
      res.value.commitmentKey.None?

    ensures
      && res.Success?
      && suite.commitment.HKDF?
    ==>
      && res.value.commitmentKey.Some?
      && |res.value.commitmentKey.value| == suite.commitment.HKDF.outputKeyLength as nat
  {
    var keys : ExpandedKeyMaterial;
    if (suite.messageVersion == 2) {
      :- Need(suite.commitment.HKDF? && suite.kdf == suite.commitment, Types.AwsEncryptionSdkException(
          message := "Suites with message version 2 must have commitment"));

      :- Need(
        && SerializableTypes.GetEncryptKeyLength(suite) == suite.kdf.HKDF.outputKeyLength
        && |plaintextKey| == suite.kdf.HKDF.inputKeyLength as nat, Types.AwsEncryptionSdkException(
          message := "Invalid Materials"));

      keys :- ExpandKeyMaterial(messageId, plaintextKey, suite, crypto);
    } else if (suite.messageVersion == 1) {
      :- Need(suite.commitment.None?, Types.AwsEncryptionSdkException(
          message := "Suites with message version 1 must not have commitment"));

      :- Need(match suite.kdf {
          case IDENTITY(i) => |plaintextKey| == SerializableTypes.GetEncryptKeyLength(suite) as nat
          case HKDF(hkdf) =>
            && |plaintextKey| == suite.kdf.HKDF.inputKeyLength as nat
            && suite.kdf.HKDF.outputKeyLength == SerializableTypes.GetEncryptKeyLength(suite)
          case None => false
        }, Types.AwsEncryptionSdkException(
        message := "Suites with message version 1 must not have commitment"));

        if netV4_0_0_RetryPolicy == Types.NetV4_0_0_RetryPolicy.ALLOW_RETRY && onNetV4Retry {
          keys :- DeriveKey(messageId, plaintextKey, suite, crypto, true);
        } else {
          keys :- DeriveKey(messageId, plaintextKey, suite, crypto, false);
        }
        
    } else {
      return Failure(Types.AwsEncryptionSdkException(
          message := "Unknown message version"));
    }

    return Success(keys);
  }
}
