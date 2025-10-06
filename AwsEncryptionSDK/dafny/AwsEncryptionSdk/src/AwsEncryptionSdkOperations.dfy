// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "EncryptDecrypt.dfy"

include "KeyDerivation.dfy"

include "Serialize/SerializableTypes.dfy"
include "Serialize/Header.dfy"
include "Serialize/HeaderTypes.dfy"
include "Serialize/V1HeaderBody.dfy"
include "Serialize/HeaderAuth.dfy"
include "Serialize/Frames.dfy"
include "Serialize/SerializeFunctions.dfy"
include "Serialize/EncryptionContext.dfy"

module AwsEncryptionSdkOperations refines AbstractAwsCryptographyEncryptionSdkOperations {

  import Primitives = AtomicPrimitives
  import MPL = AwsCryptographyMaterialProvidersTypes
  import MaterialProviders
  import EncryptDecryptHelpers

  import KeyDerivation

  import SerializableTypes
  import SerializeFunctions

  import MessageBody
  import Header
  import HeaderTypes
  import HeaderAuth
  import V1HeaderBody
  import Frames
  import EncryptionContext

  import opened Seq

  datatype Config = Config(
    nameonly crypto: Primitives.AtomicPrimitivesClient,
    nameonly mpl: MaterialProviders.MaterialProvidersClient,
    nameonly commitmentPolicy: AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy,
    nameonly maxEncryptedDataKeys: Option<CountingNumbers>,
    nameonly netV4_0_0_RetryPolicy: NetV4_0_0_RetryPolicy
  )

  type InternalConfig = Config

  type FrameLength = frameLength : int64 | 0 < frameLength <= 0xFFFF_FFFF witness *

  predicate ValidInternalConfig?(config: InternalConfig)
  {
    && config.crypto.ValidState()
    && config.mpl.ValidState()
  }

  function ModifiesInternalConfig(config: InternalConfig) : set<object>
  {
    config.crypto.Modifies + config.mpl.Modifies
  }

  predicate EncryptEnsuresPublicly(input: EncryptInput, output: Result<EncryptOutput, Error>)
  {true}

  method {:vcs_split_on_every_assert} Encrypt ( config: InternalConfig,  input: EncryptInput )
    returns (output: Result<EncryptOutput, Error>)

    //= compliance/client-apis/encrypt.txt#2.6.1
    //= type=implication
    //# If an input algorithm suite (Section 2.4.5) is provided that is not
    //# supported by the commitment policy (client.md#commitment-policy)
    //# configured in the client (client.md) encrypt MUST yield an error.
    //
    //= compliance/client-apis/client.txt#2.4.2.1
    //= type=implication
    //# *  encrypt (encrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of False
    //
    //= compliance/client-apis/client.txt#2.4.2.2
    //= type=implication
    //# *  encrypt (encrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of True
    //
    //= compliance/client-apis/client.txt#2.4.2.3
    //= type=implication
    //# *  encrypt (encrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of True
    ensures
    (
      && input.algorithmSuiteId.Some?
      && config.mpl.ValidateCommitmentPolicyOnEncrypt(MPL.ValidateCommitmentPolicyOnEncryptInput(
        algorithm := MPL.AlgorithmSuiteId.ESDK(input.algorithmSuiteId.value),
        commitmentPolicy := MPL.CommitmentPolicy.ESDK(config.commitmentPolicy)
      )).Failure?
    )
    ==>
      output.Failure?

    //= compliance/client-apis/encrypt.txt#2.4.6
    //= type=implication
    //# This value
    //# MUST be greater than 0 and MUST NOT exceed the value 2^32 - 1.
    ensures
      && input.frameLength.Some?
      && (input.frameLength.value <= 0 || input.frameLength.value > 0xFFFF_FFFF)
    ==>
      output.Failure?

    // //= compliance/client-apis/encrypt.txt#2.6.1
    // //= type=implication
    // //# If the number of
    // //# encrypted data keys (../framework/structures.md#encrypted-data-keys)
    // //# on the encryption materials (../framework/structures.md#encryption-
    // //# materials) is greater than the maximum number of encrypted data keys
    // //# (client.md#maximum-number-of-encrypted-data-keys) configured in the
    // //# client (client.md) encrypt MUST yield an error.
    // ensures
    //     && config.maxEncryptedDataKeys.Some?
    //     && ghostMaterials.Some?
    //     && |ghostMaterials.value.encryptedDataKeys| > this.maxEncryptedDataKeys.value as int
    // ==>
    //     output.Failure?
  {
    var frameLength : Types.FrameLength :- if input.frameLength.Some? then
      :- Need(
      0 < input.frameLength.value <= 0xFFFF_FFFF,
      Types.AwsEncryptionSdkException(
      message := "FrameLength must be greater than 0 and less than 2^32")
      );
      Success(input.frameLength.value)
    else
      Success(EncryptDecryptHelpers.DEFAULT_FRAME_LENGTH);

    :- EncryptDecryptHelpers.ValidateEncryptionContext(input.encryptionContext);
    var encryptionContext := if input.encryptionContext.Some? then
      input.encryptionContext.value
    else
      map[];

    var cmm :- EncryptDecryptHelpers.CreateCmmFromInput(input.materialsManager, input.keyring);

    //= compliance/client-apis/encrypt.txt#2.4.5
    //# The algorithm suite (../framework/algorithm-suite.md) that SHOULD be
    //# used for encryption.
    var algorithmSuiteId := if input.algorithmSuiteId.Some? then
      Some(MPL.AlgorithmSuiteId.ESDK(input.algorithmSuiteId.value))
    else
      None;

    if algorithmSuiteId.Some? {
      var _ :- config.mpl
      .ValidateCommitmentPolicyOnEncrypt(MPL.ValidateCommitmentPolicyOnEncryptInput(
        algorithm := algorithmSuiteId.value,
        commitmentPolicy := MPL.CommitmentPolicy.ESDK(config.commitmentPolicy)
      ))
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));
    }

    // int64 fits 9 exabytes so we're never going to actually hit this. But if we don't
    // include this the verifier is not convinced that we can cast the size to int64
    :- Need(|input.plaintext| < INT64_MAX_LIMIT,
      Types.AwsEncryptionSdkException(
      message := "Plaintext exceeds maximum allowed size"));

    var materials :- EncryptDecryptHelpers.GetEncryptionMaterials(
      cmm,
      algorithmSuiteId,
      encryptionContext,
      //= compliance/client-apis/encrypt.txt#2.6.1
      //# *  Max Plaintext Length: If the input plaintext (Section 2.4.1) has
      //# known length, this length MUST be used.
      |input.plaintext| as int64,
      config.commitmentPolicy,
      config.mpl
    );

    :- Need(materials.algorithmSuite.id.ESDK?,
      Types.AwsEncryptionSdkException(
      message := "Encryption materials contain incompatible algorithm suite for the AWS Encryption SDK."));

    :- EncryptDecryptHelpers.ValidateMaxEncryptedDataKeys(config.maxEncryptedDataKeys, materials.encryptedDataKeys);

    var encryptedDataKeys: SerializableTypes.ESDKEncryptedDataKeys := materials.encryptedDataKeys;

    //= compliance/client-apis/encrypt.txt#2.6.1
    //# The algorithm suite (../framework/algorithm-suites.md) used in all
    //# aspects of this operation MUST be the algorithm suite in the
    //# encryption materials (../framework/structures.md#encryption-
    //# materials) returned from the Get Encryption Materials (../framework/
    //# cmm-interface.md#get-encryption-materials) call.
    var messageId: HeaderTypes.MessageId :- EncryptDecryptHelpers.GenerateMessageId(materials.algorithmSuite, config.crypto);

    // TODO Post-#619: Remove Net v4.0.0 references
    // TODO Post-#619: Formally Verify Message ID is in info of HKDF
    var maybeDerivedDataKeys := KeyDerivation.DeriveKeys(
      messageId, materials.plaintextDataKey.value, materials.algorithmSuite, config.crypto, config.netV4_0_0_RetryPolicy, false
    );
      
    var derivedDataKeys :- maybeDerivedDataKeys
      .MapFailure(e => Types.AwsEncryptionSdkException( message := "Failed to derive data keys"));

    var maybeHeader := EncryptDecryptHelpers.BuildHeaderForEncrypt(
      messageId,
      materials.algorithmSuite,
      materials.encryptionContext,
      materials.requiredEncryptionContextKeys,
      encryptedDataKeys,
      frameLength as uint32,
      derivedDataKeys,
      config.crypto
    );

    :- Need(maybeHeader.Success?, Types.AwsEncryptionSdkException( message := "Failed to build header body"));
    var header := maybeHeader.value;

    // Encrypt the given plaintext into the framed message
    var framedMessage :- MessageBody.EncryptMessageBody(
      input.plaintext,
      header,
      derivedDataKeys.dataKey,
      config.crypto
    );

    var maybeSignedMessage := SignAndSerializeMessage(config, header, framedMessage, materials);

    output := maybeSignedMessage;
  }

  // Responsible for signing and serializing the message
  // This method branches into two depending on if the algorithm suite used to encrypt
  // this message has ECDSA signatures enabled.
  // This method can fail if the message fails to serialize, materials do not contain a signing key,
  // ECDSA signature fails,  or if signature byte length exceeds the UINT16 limit
  method {:vcs_split_on_every_assert} SignAndSerializeMessage(
    config: InternalConfig,
    header: Header.HeaderInfo,
    framedMessage: MessageBody.FramedMessage, 
    materials: MPL.EncryptionMaterials
  )
    returns (output: Result<EncryptOutput, Error>)
    requires header.suite.id.ESDK?
    requires config.crypto.ValidState()
    modifies config.crypto.Modifies
    ensures config.crypto.ValidState()
    ensures framedMessage.finalFrame.header.suite.signature.ECDSA? && output.Success?
      ==>
        && |config.crypto.History.ECDSASign| == |old(config.crypto.History.ECDSASign)| + 1
        && EncryptDecryptHelpers.SerializeMessageWithoutSignature(framedMessage, materials.algorithmSuite).Success?
        && materials.signingKey.Some?
        && var message := EncryptDecryptHelpers.SerializeMessageWithoutSignature(framedMessage, materials.algorithmSuite).value;
        && var ecdsaParams := framedMessage.finalFrame.header.suite.signature.ECDSA.curve;
        && var ecdsaSignInput := Seq.Last(config.crypto.History.ECDSASign).input;
        && var ecdsaSignOutput := Seq.Last(config.crypto.History.ECDSASign).output;
        && ecdsaSignInput.signatureAlgorithm == ecdsaParams
        && ecdsaSignInput.signingKey == materials.signingKey.value
        && ecdsaSignInput.message == message

        && ecdsaSignOutput.Success?
        && var signatureBytes := ecdsaSignOutput.value;
        && |signatureBytes| < UINT16_LIMIT
        && var signature := UInt16ToSeq(|signatureBytes| as uint16) + signatureBytes;
        && EncryptOutput(ciphertext := message + signature, encryptionContext := header.encryptionContext, algorithmSuiteId := header.suite.id.ESDK) == output.value
    ensures framedMessage.finalFrame.header.suite.signature.None? && output.Success? 
      ==>
        && EncryptDecryptHelpers.SerializeMessageWithoutSignature(framedMessage, materials.algorithmSuite).Success?
        && var message := EncryptDecryptHelpers.SerializeMessageWithoutSignature(framedMessage, materials.algorithmSuite).value;
        && EncryptOutput(ciphertext := message, encryptionContext := header.encryptionContext, algorithmSuiteId := header.suite.id.ESDK) == output.value
  {
    if framedMessage.finalFrame.header.suite.signature.ECDSA? {
      //= compliance/client-apis/encrypt.txt#2.7.2
      //# If the algorithm suite (../framework/algorithm-suites.md) contains a
      //# signature algorithm (../framework/algorithm-suites.md#signature-
      //# algorithm), this operation MUST calculate a signature over the
      //# message, and the output encrypted message (Section 2.5.1) MUST
      //# contain a message footer (../data-format/message-footer.md).

      //= compliance/client-apis/encrypt.txt#2.6
      //# If the encryption materials gathered (Section 2.6.1) has a
      //# algorithm suite including a signature algorithm (../framework/
      //# algorithm-suites.md#signature-algorithm), the encrypt operation
      //# MUST perform this step.

      //= compliance/data-format/message-footer.txt#2.3
      //# When an algorithm suite (../framework/algorithm-suites.md) includes a
      //# signature algorithm (../framework/algorithm-suites.md#signature-
      //# algorithm), the message (message.md) MUST contain a footer.

      var msg :- EncryptDecryptHelpers.SerializeMessageWithoutSignature(framedMessage, materials.algorithmSuite);
      var ecdsaParams := framedMessage.finalFrame.header.suite.signature.ECDSA.curve;
      // TODO: This should just work, but Proof is difficult
      :- Need(materials.signingKey.Some?,
      Types.AwsEncryptionSdkException( message := "Missing signing key."));

      //= compliance/client-apis/encrypt.txt#2.7.2
      //# To calculate a signature, this operation MUST use the signature
      //# algorithm (../framework/algorithm-suites.md#signature-algorithm)
      //# specified by the algorithm suite (../framework/algorithm-suites.md),
      //# with the following input:
      //#*  the signature key is the signing key (../framework/
      //#   structures.md#signing-key) in the encryption materials
      //#   (../framework/structures.md#encryption-materials)
      //#*  the input to sign is the concatenation of the serialization of the
      //#   message header (../data-format/message-header.md) and message body
      //#   (../data-format/message-body.md)
      var maybeBytes := config.crypto.ECDSASign(
      Primitives.Types.ECDSASignInput(
        signatureAlgorithm := ecdsaParams,
        signingKey := materials.signingKey.value,
        message := msg
      )
      );
      var bytes :- maybeBytes
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

      :- Need(|bytes| < UINT16_LIMIT,
      Types.AwsEncryptionSdkException(
        message := "Length of signature bytes is larger than the uint16 limit."));

      // TODO
      // :- Need(|bytes| == ecdsaParams.SignatureLength() as int,
      //   Types.AwsEncryptionSdkException( message := "Malformed response from Sign()."));

      //= compliance/client-apis/encrypt.txt#2.7.2
      //# This operation MUST then serialize a message footer with the
      //# following specifics:

      //= compliance/client-apis/encrypt.txt#2.7.2
      //# *  Signature Length (../data-format/message-footer.md#signature-
      //# length): MUST be the length of the output of the calculation
      //# above.

      //= compliance/client-apis/encrypt.txt#2.7.2
      //# *  Signature (../data-format/message-footer.md#signature): MUST be
      //# the output of the calculation above.
      var signature := UInt16ToSeq(|bytes| as uint16) + bytes;

      //= compliance/client-apis/encrypt.txt#2.7.2
      //# The encrypted message output by this operation MUST have a message
      //# footer equal to the message footer calculated in this step.
      msg := msg + signature;
      // TODO: Come back and prove this
      // assert msg == SerializeMessageWithSignature(framedMessage, signature); // Header, frames and signature can be serialized into the stream

      //= compliance/client-apis/encrypt.txt#2.5
      //# This behavior MUST output the following if the behavior is
      //# successful:
      //# *  Encrypted Message (Section 2.5.1)
      //# *  Encryption Context (Section 2.4.2)
      //# *  Algorithm Suite (Section 2.4.5)
      return Success(
        Types.EncryptOutput(
          ciphertext := msg,
          encryptionContext := header.encryptionContext,
          algorithmSuiteId := header.suite.id.ESDK
        )
      );
    } else {
      //= compliance/client-apis/encrypt.txt#2.6
      //# Otherwise the encrypt operation MUST
      //# NOT perform this step.
      var msg :- EncryptDecryptHelpers.SerializeMessageWithoutSignature(framedMessage, materials.algorithmSuite);
      return Success(
        Types.EncryptOutput(
          ciphertext := msg,
          encryptionContext := header.encryptionContext,
          algorithmSuiteId := header.suite.id.ESDK
        )
      );
    }
  }

  predicate DecryptEnsuresPublicly(input: DecryptInput, output: Result<DecryptOutput, Error>)
  {true}

  method {:vcs_split_on_every_assert} Decrypt ( config: InternalConfig,  input: DecryptInput )
    returns (output: Result<DecryptOutput, Error>)
    //= compliance/client-apis/decrypt.txt#2.7
    //= type=TODO
    //# This operation MUST perform all the above steps unless otherwise
    //# specified, and it MUST perform them in the above order.

    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# If the parsed algorithm suite ID (../data-format/message-
    //# header.md#algorithm-suite-id) is not supported by the commitment
    //# policy (client.md#commitment-policy) configured in the client
    //# (client.md) decrypt MUST yield an error.

    //= compliance/client-apis/client.txt#2.4.2.1
    //= type=implication
    //# *  decrypt (decrypt.md) MUST support all algorithm suites
    //
    //= compliance/client-apis/client.txt#2.4.2.2
    //= type=implication
    //# *  decrypt (decrypt.md) MUST support all algorithm suites
    //
    //= compliance/client-apis/client.txt#2.4.2.3
    //= type=implication
    //# *  decrypt (decrypt.md) MUST only support algorithm suites that have
    //# a Key Commitment (../framework/algorithm-suites.md#algorithm-
    //# suites-encryption-key-derivation-settings) value of True


    //= compliance/client-apis/decrypt.txt#2.5
    //= type=implication
    //# The client MUST require exactly one of the following types of inputs:
    //#*  Cryptographic Materials Manager (CMM) (../framework/cmm-
    //#   interface.md)
    //#*  Keyring (../framework/keyring-interface.md)
    ensures
    (
      || (input.materialsManager.Some? && input.keyring.Some?)
      || (input.materialsManager.None? && input.keyring.None?)
    )
    ==>
      output.Failure?

  {
    var cmm :- EncryptDecryptHelpers.CreateCmmFromInput(input.materialsManager, input.keyring);

    //= compliance/client-apis/decrypt.txt#2.7.1
    //# Given encrypted message bytes, this operation MUST process those
    //# bytes sequentially, deserializing those bytes according to the
    //# message format (../data-format/message.md).

    var buffer := SerializeFunctions.ReadableBuffer(input.ciphertext, 0);

    output := InternalDecrypt(config, cmm, buffer, input.encryptionContext);
  }

  method {:vcs_split_on_every_assert} InternalDecrypt
   (
    config: InternalConfig,
    cmm: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager,
    buffer: SerializeFunctions.ReadableBuffer,
    inputEncryptionContext: Option<AwsCryptographyMaterialProvidersTypes.EncryptionContext>
  )
    returns (output: Result<DecryptOutput, Error>)

    requires cmm.ValidState()
    modifies cmm.Modifies
    ensures cmm.ValidState()

    requires ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    ensures ValidInternalConfig?(config)

    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# If the
    //# algorithm suite is not supported by the commitment policy
    //# (client.md#commitment-policy) configured in the client (client.md)
    //# decrypt MUST yield an error.
    // TODO :: Consider removing from spec as this is redundant
    ensures
    (
      && var headerBody := Header.ReadHeaderBody(buffer, config.maxEncryptedDataKeys, config.mpl);
      && headerBody.Success?
      && config.mpl.ValidateCommitmentPolicyOnDecrypt(MPL.ValidateCommitmentPolicyOnDecryptInput(
        algorithm := headerBody.value.data.algorithmSuite.id,
        commitmentPolicy := MPL.CommitmentPolicy.ESDK(config.commitmentPolicy)
      )).Failure?
    )
    ==>
      output.Failure?

    //= compliance/client-apis/decrypt.txt#2.6
    //= type=implication
    //# The client MUST return as output to this operation:
    ensures output.Success?
    ==>
      && var headerBody := Header.ReadHeaderBody(buffer, config.maxEncryptedDataKeys, config.mpl);
      && headerBody.Success?
      // *  Algorithm Suite (Section 2.6.3)
      && headerBody.value.data.algorithmSuite.id.ESDK?
      && output.value.algorithmSuiteId == headerBody.value.data.algorithmSuite.id.ESDK
      && old(cmm.History.DecryptMaterials) < cmm.History.DecryptMaterials
      && Seq.Last(cmm.History.DecryptMaterials).output.Success?
      && var decMat := Seq.Last(cmm.History.DecryptMaterials).output.value.decryptionMaterials;
      // *  Encryption Context (Section 2.6.2)
      && var headerEncryptionContext := EncryptionContext.GetEncryptionContext(headerBody.value.data.encryptionContext);
      && output.value.encryptionContext ==
        headerEncryptionContext + buildEncryptionContextToOnlyAuthenticate(decMat)
  {
    // Track if a failure has triggered a V4 Retry
    var v4Retry := false;
    
    //= compliance/client-apis/decrypt.txt#2.5.1.1
    //= type=TODO
    //# To make diagnosing this mistake easier, implementations SHOULD detect
    //# the first two bytes of the Base64 encoding of any supported message
    //# versions (../data-format/message-header.md#version-1) and types
    //# (../data-format/message-header.md#type) and fail with a more specific
    //# error message.

    var headerBody :- Header
      .ReadHeaderBody(buffer, config.maxEncryptedDataKeys, config.mpl)
      .MapFailure(EncryptDecryptHelpers.MapSerializeFailure(": ReadHeaderBody"));

    AnyCorrectlyReadByteRange(buffer, headerBody.tail);
    
    var rawHeader := buffer.bytes[buffer.start..headerBody.tail.start];

    var algorithmSuite := headerBody.data.algorithmSuite;

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# If the
    //# algorithm suite is not supported by the commitment policy
    //# (client.md#commitment-policy) configured in the client (client.md)
    //# decrypt MUST yield an error.
    var _ :- config.mpl
      .ValidateCommitmentPolicyOnDecrypt(MPL.ValidateCommitmentPolicyOnDecryptInput(
      algorithm := algorithmSuite.id,
      commitmentPolicy := MPL.CommitmentPolicy.ESDK(config.commitmentPolicy)
      ))
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));

    //= compliance/client-apis/decrypt.txt#2.5.2
    //# This CMM MUST obtain the decryption materials (../framework/
    //# structures.md#decryption-materials) required for decryption.

    //= compliance/client-apis/decrypt.txt#2.5.3
    //# This default CMM MUST obtain the decryption materials required for
    //# decryption.
    // TODO :: Consider removing "Default CMM MUST obtain" from spec.
    // It is redundent and hard to prove.

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# This operation MUST obtain this set of decryption materials
    //# (../framework/structures.md#decryption-materials), by calling Decrypt
    //# Materials (../framework/cmm-interface.md#decrypt-materials) on a CMM
    //# (../framework/cmm-interface.md).
    var decMat :- EncryptDecryptHelpers.GetDecryptionMaterials(
      cmm,
      algorithmSuite.id,
      headerBody.data,
      inputEncryptionContext,
      config.commitmentPolicy,
      config.mpl
    );
    
    // It SHOULD be the case that cmm.History !in config.crypto.Modifies;
    // However proving this is very expensive.
    // This is a HACK to address this history element
    // See the end of the method, where I update the history.
    ghost var DecryptMaterialsHistory := cmm.History.DecryptMaterials;
    assert old(cmm.History.DecryptMaterials) < cmm.History.DecryptMaterials;

    var suite := decMat.algorithmSuite;

    :- Need(suite == algorithmSuite,
      Types.AwsEncryptionSdkException(
      message := "Stored header algorithm suite does not match decryption algorithm suite."));

    //= compliance/client-apis/decrypt.txt#2.4.2
    //# This operation MUST NOT release any unauthenticated plaintext or
    //# unauthenticated associated data.
    var headerAuth :- HeaderAuth
      .ReadHeaderAuthTag(headerBody.tail, suite)
      .MapFailure(EncryptDecryptHelpers.MapSerializeFailure(": ReadHeaderAuthTag"));

    if suite.messageVersion == 1 {
      reveal HeaderAuth.ReadHeaderAuthTag();
      assert headerAuth == HeaderAuth.ReadHeaderAuthTagV1(headerBody.tail, suite).value;
      SerializeFunctions.CorrectlyReadByteRange(headerBody.tail, headerAuth.tail, HeaderAuth.WriteHeaderAuthTagV1(headerAuth.data));
    } else if suite.messageVersion == 2 {
      reveal HeaderAuth.ReadHeaderAuthTag();
      assert headerAuth == HeaderAuth.ReadHeaderAuthTagV2(headerBody.tail, suite).value;
      SerializeFunctions.CorrectlyReadByteRange(headerBody.tail, headerAuth.tail, HeaderAuth.WriteHeaderAuthTagV2(headerAuth.data));
    } else {
      assert false;
    }
    
    ConcatenateCorrectlyReadByteRanges(buffer, headerBody.tail, headerAuth.tail);
    
    var maybeDerivedDataKeys := KeyDerivation.DeriveKeys(
      headerBody.data.messageId, decMat.plaintextDataKey.value, suite, config.crypto, config.netV4_0_0_RetryPolicy, false
    );
    :- Need(maybeDerivedDataKeys.Success?,
      Types.AwsEncryptionSdkException(
      message := "Failed to derive data keys"));
    var derivedDataKeys := maybeDerivedDataKeys.value;

    :- Need(Header.HeaderVersionSupportsCommitment?(suite, headerBody.data),
      Types.AwsEncryptionSdkException(
      message := "Invalid commitment values found in header body"));
    if suite.commitment.HKDF? {
      reveal Header.HeaderVersionSupportsCommitment?();
      var _ :- EncryptDecryptHelpers.ValidateSuiteData(suite, headerBody.data, derivedDataKeys.commitmentKey.value);
    }

    var headerEncryptionContext := EncryptionContext.GetEncryptionContext(headerBody.data.encryptionContext);

    EncryptionContext.LemmaESDKCanonicalEncryptionContextIsESDKEncryptionContext(
      headerBody.data.encryptionContext,
      headerEncryptionContext
    );

    //#*  The encryption context to only authenticate MUST be the [encryption context]
    //#   (../framework/structures.md#encryption-context)
    //#   in the [decryption materials](../framework/structures.md#decryption-materials)
    //#   filtered to only contain key value pairs listed in
    //#   the [decryption material's](../framework/structures.md#decryption-materials)
    //#   [required encryption context keys]
    //#   (../framework/structures.md#required-encryption-context-keys-1)
    //#   serialized according to the [encryption context serialization specification]
    //#   (../framework/structures.md#serialization).
    var encryptionContextToOnlyAuthenticate := buildEncryptionContextToOnlyAuthenticate(decMat);

    EncryptionContext.SubsetOfESDKEncryptionContextIsESDKEncryptionContext(
        decMat.encryptionContext,
        encryptionContextToOnlyAuthenticate
      );

    var canonicalReqEncryptionContext := 
      EncryptionContext.GetCanonicalEncryptionContext(encryptionContextToOnlyAuthenticate);
    var serializedReqEncryptionContext :=
      EncryptionContext.WriteEmptyEcOrWriteAAD(canonicalReqEncryptionContext);

    var maybeHeaderAuth :=
      //= compliance/client-apis/decrypt.txt#2.7.3
      //# Once a valid message header is deserialized and decryption materials
      //# are available, this operation MUST validate the message header body
      //# (../data-format/message-header.md#header-body) by using the
      //# authenticated encryption algorithm (../framework/algorithm-
      //# suites.md#encryption-algorithm) to decrypt with the following inputs:
      config.crypto.AESDecrypt(Primitives.Types.AESDecryptInput(
        encAlg := suite.encrypt.AES_GCM,
        //#*  the cipherkey is the derived data key
        key := derivedDataKeys.dataKey,
        //#*  the ciphertext is an empty byte array
        cipherTxt := [],
        //#*  the tag is the value serialized in the message header's
        //#   authentication tag field (../data-format/message-
        //#   header.md#authentication-tag)
        authTag := headerAuth.data.headerAuthTag,
        //#*  the IV is the value serialized in the message header's IV field
        //#   (../data-format/message-header#iv).
        iv := headerAuth.data.headerIv,
        //#*  MUST be the concatenation of the serialized [message header body]
        //#   (../data-format/message-header.md#header-body)
        //#   and the serialization of encryption context to only authenticate.
        aad := rawHeader + serializedReqEncryptionContext
      ));


    // TODO Post-#619: Add to the ESDK Specification the following:
    // ESDK-NET v4.0.0 Header Auth Catch
    // This will catch the Header Auth failure,
    // The Retry MUST
    // calculate the HKDF without the Message ID in the info and
    // use EncryptionContext.WriteAAD to serialize the
    // the Canonical Required Encryption Context.

    // TODO Post-#619: Formally Verify this section
    // TODO Post-#619: Duvet this section
    // TODO Post-#619: Refactor this to eliminate duplicate code
    if maybeHeaderAuth.Failure?
      && config.netV4_0_0_RetryPolicy == NetV4_0_0_RetryPolicy.ALLOW_RETRY
      && v4Retry == false
    {
      v4Retry := true;
      // Derive Keys following ESDK-NET @ v4.0.0 Behavior
      maybeDerivedDataKeys := KeyDerivation.DeriveKeys(
        headerBody.data.messageId, decMat.plaintextDataKey.value, suite, config.crypto, config.netV4_0_0_RetryPolicy, true
      );
      :- Need(maybeDerivedDataKeys.Success?,
        Types.AwsEncryptionSdkException(
        message := "Failed to derive data keys")
      );
      derivedDataKeys := maybeDerivedDataKeys.value;
      // Serialize Required Encryption Context following ESDK-NET @ v4.0.0 Behavior
      serializedReqEncryptionContext := EncryptionContext.WriteAAD(canonicalReqEncryptionContext);
      maybeHeaderAuth :=
        //= compliance/client-apis/decrypt.txt#2.7.3
        //# Once a valid message header is deserialized and decryption materials
        //# are available, this operation MUST validate the message header body
        //# (../data-format/message-header.md#header-body) by using the
        //# authenticated encryption algorithm (../framework/algorithm-
        //# suites.md#encryption-algorithm) to decrypt with the following inputs:
        config.crypto.AESDecrypt(Primitives.Types.AESDecryptInput(
          encAlg := suite.encrypt.AES_GCM,
          //#*  the cipherkey is the derived data key
          key := derivedDataKeys.dataKey,
          //#*  the ciphertext is an empty byte array
          cipherTxt := [],
          //#*  the tag is the value serialized in the message header's
          //#   authentication tag field (../data-format/message-
          //#   header.md#authentication-tag)
          authTag := headerAuth.data.headerAuthTag,
          //#*  the IV is the value serialized in the message header's IV field
          //#   (../data-format/message-header#iv).
          iv := headerAuth.data.headerIv,
          //#*  MUST be the concatenation of the serialized [message header body]
          //#   (../data-format/message-header.md#header-body)
          //#   and the serialization of encryption context to only authenticate.
          aad := rawHeader + serializedReqEncryptionContext
          ));
    }
    //= compliance/client-apis/decrypt.txt#2.7.3
    //# If this tag verification fails, this operation MUST immediately halt
    //# and fail.
    var _ :- maybeHeaderAuth
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    var header := Header.HeaderInfo(
      body := headerBody.data,
      rawHeader := rawHeader,
      encryptionContext := headerEncryptionContext,
      suite := suite,
      headerAuth := headerAuth.data
    );

    ghost var headerSubset: Header.Header := HeaderSubset(header);

    var key := derivedDataKeys.dataKey;
    var plaintext: seq<uint8>;
    var messageBodyTail: SerializeFunctions.ReadableBuffer;
    
    //= compliance/client-apis/decrypt.txt#2.7.4
    //# Once the message header is successfully parsed, the next sequential
    //# bytes MUST be deserialized according to the message body spec
    //# (../data-format/message-body.md).

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# The content type (../data-format/message-header.md#content-type)
    //# field parsed from the message header above determines whether these
    //# bytes MUST be deserialized as framed data (../data-format/message-
    //# body.md#framed-data) or un-framed data (../data-format/message-
    //# body.md#un-framed-data).
    match header.body.contentType {
      case NonFramed =>
        assert Frames.NonFramedHeader?(headerSubset);
        //= compliance/client-apis/decrypt.txt#2.7.4
        //# If this decryption fails, this operation MUST immediately halt and
        //# fail.
        var decryptRes :- EncryptDecryptHelpers.ReadAndDecryptNonFramedMessageBody(headerAuth.tail, header, key, config.crypto);
        plaintext := decryptRes.0;
        messageBodyTail := decryptRes.1;

      case Framed =>
        assert Frames.FramedHeader?(headerSubset);
        //= compliance/client-apis/decrypt.txt#2.7.4
        //# If this decryption fails, this operation MUST immediately halt and
        //# fail.
        var decryptRes :- EncryptDecryptHelpers.ReadAndDecryptFramedMessageBody(headerAuth.tail, header, key, config.crypto);
        plaintext := decryptRes.0;
        messageBodyTail := decryptRes.1;
    }
    ConcatenateCorrectlyReadByteRanges(buffer, headerAuth.tail, messageBodyTail);

    //= compliance/client-apis/decrypt.txt#2.7.5
    //# If this verification is not successful, this operation MUST
    //# immediately halt and fail.
    var signature :- EncryptDecryptHelpers.VerifySignature(
      messageBodyTail,
      messageBodyTail.bytes[buffer.start..messageBodyTail.start],
      decMat,
      config.crypto
    );

    :- Need(signature.start == |signature.bytes|,
      Types.AwsEncryptionSdkException(
      message := "Data after message footer."));

    //= compliance/client-apis/decrypt.txt#2.7.1
    //# Until the header is verified (Section 2.7.3), this operation MUST NOT
    //# release any parsed information from the header.
    // Note that the header is verified above

    //= compliance/client-apis/decrypt.txt#2.7.4
    //# This operation MUST NOT release any unauthenticated plaintext.

    //= compliance/client-apis/decrypt.txt#2.7
    //# If the input encrypted message is not being streamed (streaming.md)
    //# to this operation, all output MUST NOT be released until after these
    //# steps complete successfully.
    output := Success(
      Types.DecryptOutput(
        plaintext := plaintext,
        encryptionContext := header.encryptionContext + encryptionContextToOnlyAuthenticate,
        algorithmSuiteId := header.suite.id.ESDK
      )
    );

    // It SHOULD be the case that cmm.History !in config.crypto.Modifies;
    // See var decMat :- EncryptDecryptHelpers.GetDecryptionMaterials(
    cmm.History.DecryptMaterials := DecryptMaterialsHistory;

    assert header.encryptionContext == headerEncryptionContext;
    assert output.value.encryptionContext == header.encryptionContext + encryptionContextToOnlyAuthenticate;

  }

  // The encryption context to only authenticate MUST be
  // the encryption context in the decryption materials filtered
  // to only contain key value pairs listed
  // in the decryption material's required encryption context keys.
  // TODO Post-#619: Duvet this section
  function method buildEncryptionContextToOnlyAuthenticate(
    decMat: MPL.DecryptionMaterials
  ): MPL.EncryptionContext
  {
    map
      k <- decMat.encryptionContext
      |
        && k in decMat.requiredEncryptionContextKeys
      :: k := decMat.encryptionContext[k]
  }

  lemma AnyCorrectlyReadByteRange(
    buffer: SerializeFunctions.ReadableBuffer,
    verifiedTail: SerializeFunctions.ReadableBuffer
  )
    requires exists readRange: seq<uint8> :: SerializeFunctions.CorrectlyReadRange(buffer, verifiedTail, readRange)
    ensures
      && buffer.start <= verifiedTail.start <= |buffer.bytes|
      && SerializeFunctions.CorrectlyReadRange(buffer, verifiedTail, buffer.bytes[buffer.start..verifiedTail.start])
      && buffer.bytes == verifiedTail.bytes
  {
    reveal SerializeFunctions.CorrectlyReadRange();
  }

  lemma ConcatenateCorrectlyReadByteRanges(
    buffer: SerializeFunctions.ReadableBuffer,
    verifiedMid: SerializeFunctions.ReadableBuffer,
    verifiedTail: SerializeFunctions.ReadableBuffer
  )
    requires buffer.start <= verifiedMid.start <= verifiedTail.start <= |buffer.bytes|
    requires SerializeFunctions.CorrectlyReadRange(buffer, verifiedMid, buffer.bytes[buffer.start..verifiedMid.start])
    requires SerializeFunctions.CorrectlyReadRange(verifiedMid, verifiedTail, buffer.bytes[verifiedMid.start..verifiedTail.start])
    ensures SerializeFunctions.CorrectlyReadRange(buffer, verifiedTail, buffer.bytes[buffer.start..verifiedTail.start])
    ensures buffer.bytes == verifiedMid.bytes == verifiedTail.bytes
  {
    reveal SerializeFunctions.CorrectlyReadRange();
  }

  // There seems to be some complexity with Dafny doing this assignment
  // So a lemma just works around this.
  lemma HeaderSubset(i: Header.HeaderInfo)
    returns (h: Header.Header)
    requires Header.IsHeader(i)
    ensures i == h
  {
    h := i;
  }

}
