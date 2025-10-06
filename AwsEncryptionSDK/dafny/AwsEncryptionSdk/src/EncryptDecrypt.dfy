// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../Model/AwsCryptographyEncryptionSdkTypes.dfy"
include "MessageBody.dfy"
include "Serialize/SerializableTypes.dfy"
include "Serialize/HeaderAuth.dfy"
include "Serialize/SerializeFunctions.dfy"
include "KeyDerivation.dfy"

module EncryptDecryptHelpers {
  import opened Wrappers
  import opened StandardLibrary
  import opened UInt = StandardLibrary.UInt
  import Types = AwsCryptographyEncryptionSdkTypes
  import MPL = AwsCryptographyMaterialProvidersTypes
  import MaterialProviders
  import Primitives = AtomicPrimitives
  import MessageBody
  import SerializableTypes
  import opened SerializeFunctions
  import HeaderAuth
  import HeaderTypes
  import KeyDerivation
  import Header
  import EncryptionContext
  import Frames
  import Seq

  type FrameLength = frameLength : int64 | 0 < frameLength <= 0xFFFF_FFFF witness *

  //= compliance/client-apis/encrypt.txt#2.4.6
  //= type=implication
  //# This
  //# value MUST default to 4096 bytes.
  const DEFAULT_FRAME_LENGTH : FrameLength := 4096

  // UTF-8 encoded "aws-crypto-"
  const RESERVED_ENCRYPTION_CONTEXT: UTF8.ValidUTF8Bytes :=
    var s := [ 0x61, 0x77, 0x73, 0x2D, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6F, 0x2D ];
    assert UTF8.ValidUTF8Range(s, 0, 11);
    s


  // Specification of Encrypt with signature
  function method SerializeMessageWithSignature(
    framedMessage: MessageBody.FramedMessage,
    signature: seq<uint8>,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(res: Result<seq<uint8>, Types.Error>)
    requires |signature| < UINT16_LIMIT
    ensures res.Success?
    ==>
      && var message := SerializeMessageWithoutSignature(framedMessage, suite);
      && message.Success?
      && res.value == message.value + WriteShortLengthSeq(signature)
  {
    var serializedSignature := WriteShortLengthSeq(signature);
    var serializedMessage :- SerializeMessageWithoutSignature(framedMessage, suite);
    Success(serializedMessage + serializedSignature)
  }

  // Specification of Encrypt without signature
  function method SerializeMessageWithoutSignature(
    framedMessage: MessageBody.FramedMessage,
    suite: MPL.AlgorithmSuiteInfo
  )
    :(message: Result<seq<uint8>, Types.Error>)
  {
    // The header
    var headerAuth :- HeaderAuth.WriteHeaderAuthTag(framedMessage.finalFrame.header.headerAuth, suite);
    //= compliance/data-format/message-footer.txt#2.5.2
    //# This signature MUST be calculated over both the message header
    //# (message-header.md) and the message body (message-body.md), in the
    //# order of serialization.
    Success(
      //= compliance/client-apis/encrypt.txt#2.6.2
      //# The encrypted message output by this operation MUST have a message
      //# header equal to the message header calculated in this step.
      framedMessage.finalFrame.header.rawHeader
      // The header authentication
      + headerAuth
      // The message body i.e. "all the frames"
      + MessageBody.WriteFramedMessageBody(framedMessage)
    )
  }

  method VerifySignature(
    buffer: SerializeFunctions.ReadableBuffer,
    msg: seq<uint8>,
    decMat: MPL.DecryptionMaterials,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<SerializeFunctions.ReadableBuffer, Types.Error>)
    // DecryptionMaterialsWithPlaintextDataKey ensures that the materials and the suite match.
    // requires Client.Materials.DecryptionMaterialsWithPlaintextDataKey(decMat)

    requires decMat.verificationKey.Some? ==> decMat.algorithmSuite.signature.ECDSA?

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    // TODO: Add Proof
    // ensures match res
    //   case Failure(_) => true
    //   case Success(_) =>
    //     && 2 <= old(rd.reader.pos) + 2 <= rd.reader.pos
    //     && SignatureBySequence(signature, rd.reader.data[old(rd.reader.pos)..rd.reader.pos])

    //= compliance/client-apis/decrypt.txt#2.7
    //= type=implication
    //# Otherwise this operation MUST NOT perform this
    //# step.
    ensures decMat.verificationKey.None? ==> res.Success? && res.value == buffer

    ensures
      && decMat.verificationKey.Some?
      && SerializeFunctions.ReadShortLengthSeq(buffer).Failure?
    ==>
      && res.Failure?

    ensures
      && decMat.verificationKey.Some?
      && SerializeFunctions.ReadShortLengthSeq(buffer).Success?
    ==>
      && |old(crypto.History.ECDSAVerify)| + 1 == |crypto.History.ECDSAVerify|
      && var ECDSAVerifyInput := Seq.Last(crypto.History.ECDSAVerify).input;
      && ECDSAVerifyInput.signatureAlgorithm == decMat.algorithmSuite.signature.ECDSA.curve
      && ECDSAVerifyInput.verificationKey == decMat.verificationKey.value
      && ECDSAVerifyInput.message == msg
      && ECDSAVerifyInput.signature == SerializeFunctions.ReadShortLengthSeq(buffer).value.data

    ensures
      && |old(crypto.History.ECDSAVerify)| + 1 == |crypto.History.ECDSAVerify|
      // The verification call succeeded
      // and the value it returned was false
      // (indicating invalid signature)
      && Seq.Last(crypto.History.ECDSAVerify).output.Success?
      && !Seq.Last(crypto.History.ECDSAVerify).output.value
    ==>
      && res.Failure?

    ensures
      && |old(crypto.History.ECDSAVerify)| + 1 == |crypto.History.ECDSAVerify|
      // The verification call failed
      && Seq.Last(crypto.History.ECDSAVerify).output.Failure?
    ==>
      && res.Failure?

    ensures
      && |old(crypto.History.ECDSAVerify)| + 1 == |crypto.History.ECDSAVerify|
      // The verification call succeeded and the value it returned was true
      // (indicating valid signature)
      && Seq.Last(crypto.History.ECDSAVerify).output.Success?
      && Seq.Last(crypto.History.ECDSAVerify).output.value
    ==>
      res.Success?

  {
    // If there is no verification key, that lets us conclude that the suite does not have a signature.
    //= compliance/client-apis/decrypt.txt#2.7
    //# Otherwise this operation MUST NOT perform this
    //# step.
    if decMat.verificationKey.None? {
      return Success(buffer);
    }
    //= compliance/client-apis/decrypt.txt#2.7.5
    //# If the algorithm suite has a signature algorithm, this operation MUST
    //# verify the message footer using the specified signature algorithm.

    
    //= compliance/client-apis/decrypt.txt#2.7
    //# ./framework/algorithm-
    //# suites.md#signature-algorithm), this operation MUST perform
    //# this step.
    var signature :- SerializeFunctions
    
      //= compliance/client-apis/decrypt.txt#2.7.5
      //# After deserializing the body, this operation MUST deserialize the
      //# next encrypted message bytes as the message footer (../data-format/
      //# message-footer.md).
      .ReadShortLengthSeq(buffer)
      .MapFailure(MapSerializeFailure(": ReadShortLengthSeq"));

    var ecdsaParams := decMat.algorithmSuite.signature.ECDSA.curve;

    //= compliance/client-apis/decrypt.txt#2.7.5
    //# Once the message footer is deserialized, this operation MUST use the
    //# signature algorithm (../framework/algorithm-suites.md#signature-
    //# algorithm) from the algorithm suite (../framework/algorithm-
    //# suites.md) in the decryption materials to verify the encrypted
    //# message, with the following inputs:
    var maybeSignatureVerifiedResult := crypto.ECDSAVerify(Primitives.Types.ECDSAVerifyInput(
        signatureAlgorithm := ecdsaParams,
        //#*  The verification key is the verification key (../framework/
        //#   structures.md#verification-key) in the decryption materials.
        verificationKey := decMat.verificationKey.value,
        //#*  The input to verify is the concatenation of the serialization of
        //#   the message header (../data-format/message-header.md) and message
        //#   body (../data-format/message-body.md).
        message := msg,
        signature := signature.data
    ));

    var signatureVerifiedResult :- maybeSignatureVerifiedResult
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    if (!signatureVerifiedResult) {
      return Failure(Types.AwsEncryptionSdkException( message := "Invalid signature" ));
    }

    return Success(signature.tail);
  }

  function method MapSerializeFailure(s: string): SerializeFunctions.ReadProblems -> Types.Error {
    (e: SerializeFunctions.ReadProblems) =>
    match e
      case Error(e) => Types.AwsEncryptionSdkException(message := e)
      case MoreNeeded(_) => Types.AwsEncryptionSdkException(message := "Incomplete message" + s)
  }

  function method ValidateEncryptionContext(input: Option<MPL.EncryptionContext>)
    : (output: Outcome<Types.Error>)
    //= compliance/client-apis/encrypt.txt#2.4.2
    //= type=implication
    //# The prefix "aws-crypto-" is reserved for internal use by the AWS
    //# Encryption SDK; see the the Default CMM spec (default-cmm.md) for one
    //# such use.
    //# If the input encryption context contains any entries with
    //# a key beginning with this prefix, the encryption operation MUST fail.
    ensures
      && input.Some?
      && (exists key: UTF8.ValidUTF8Bytes | key in input.value.Keys :: RESERVED_ENCRYPTION_CONTEXT <= key)
    ==>
      output.Fail?
  {
    if 
      && input.Some?
      && exists key: UTF8.ValidUTF8Bytes | key in input.value.Keys :: RESERVED_ENCRYPTION_CONTEXT <= key
    then
      Fail(Types.AwsEncryptionSdkException(
        message := "Encryption context keys cannot contain reserved prefix 'aws-crypto-'"))
    else
      Pass
  }

    /*
    * Helper method for taking optional input keyrings/CMMs and returning a CMM,
    * either directly the one that was provided or a new default CMM from the
    * provided keyring.
    */
  method CreateCmmFromInput(
    inputCmm: Option<MPL.ICryptographicMaterialsManager>,
    inputKeyring: Option<MPL.IKeyring>
  )
    returns (res: Result<MPL.ICryptographicMaterialsManager, Types.Error>)

    requires inputKeyring.Some?
    ==>
      && inputKeyring.value.ValidState()
    requires inputCmm.Some?
    ==>
      && inputCmm.value.ValidState()
    ensures res.Success?
    ==>
      && res.value.ValidState()

    modifies (if inputKeyring.Some? then inputKeyring.value.Modifies else {})

    //= compliance/client-apis/encrypt.txt#2.6.1
    //= type=implication
    //# The
    //# CMM used MUST be the input CMM, if supplied.

    //= compliance/client-apis/decrypt.txt#2.7.2
    //= type=implication
    //# The CMM used MUST be the input CMM, if supplied.
    ensures
      && res.Success?
      && inputCmm.Some?
    ==>
      res.value == inputCmm.value

    ensures
      && res.Success?
      && inputKeyring.Some?
    ==>
      fresh(res.value.Modifies - inputKeyring.value.Modifies)

    ensures
      && inputCmm.Some?
      && inputKeyring.Some?
    ==>
      res.Failure?

    ensures
      && inputCmm.None?
      && inputKeyring.None?
    ==>
      res.Failure?
  {
    :- Need(inputCmm.None? || inputKeyring.None?,
      Types.AwsEncryptionSdkException(
        message := "Cannot provide both a keyring and a CMM"));
    :- Need(inputCmm.Some? || inputKeyring.Some?,
      Types.AwsEncryptionSdkException(
        message := "Must provide either a keyring or a CMM"));

    var cmm : MPL.ICryptographicMaterialsManager;
    if inputCmm.Some? {
      return Success(inputCmm.value);
    } else {
      var maybeMaterialsProviders := MaterialProviders.MaterialProviders();
      var materialProviders :- maybeMaterialsProviders
        .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));
      // Each of these three citations refer to creating a default CMM from
      // the input keyring.

      //= compliance/client-apis/encrypt.txt#2.6.1
      //# If instead the caller
      //# supplied a keyring (../framework/keyring-interface.md), this behavior
      //# MUST use a default CMM (../framework/default-cmm.md) constructed
      //# using the caller-supplied keyring as input.

      //= compliance/client-apis/decrypt.txt#2.5.3
      //# If the Keyring is provided as the input, the client MUST construct a
      //# default CMM (../framework/default-cmm.md) that uses this keyring, to
      //# obtain the decryption materials (../framework/
      //# structures.md#decryption-materials) that is required for decryption.

      //= compliance/client-apis/decrypt.txt#2.7.2
      //# If a CMM is not
      //# supplied as the input, the decrypt operation MUST construct a default
      //# CMM (../framework/default-cmm.md) from the input keyring
      //# (../framework/keyring-interface.md).
      var maybeCmm := materialProviders
          .CreateDefaultCryptographicMaterialsManager(MPL.CreateDefaultCryptographicMaterialsManagerInput(
              keyring := inputKeyring.value
          )
      );
      return maybeCmm
        .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));
    }
  }

  function method ValidateMaxEncryptedDataKeys(
    maxEncryptedDataKeys: Option<Types.CountingNumbers>,
    edks: seq<MPL.EncryptedDataKey> // SerializableTypes.ESDKEncryptedDataKeys
  )
    : (output: Outcome<Types.Error>)

    ensures maxEncryptedDataKeys.None? ==> output.Pass?

    ensures
        && maxEncryptedDataKeys.Some?
        && |edks| > maxEncryptedDataKeys.value as int
    ==>
        output.Fail?
  {
    if
        && maxEncryptedDataKeys.Some?
        && |edks| > maxEncryptedDataKeys.value as int
    then
      Fail(Types.AwsEncryptionSdkException( message := "Encrypted data keys exceed maxEncryptedDataKeys"))
    else
      Pass
  }

  /*
  * Generate a message id of appropriate length for the given algorithm suite.
  */
  method GenerateMessageId(
    suite: MPL.AlgorithmSuiteInfo,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<HeaderTypes.MessageId, Types.Error>)

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    ensures
        && res.Success?
        && suite.messageVersion == 1
    ==>
        |res.value| == HeaderTypes.MESSAGE_ID_LEN_V1

    ensures
        && res.Success?
        && suite.messageVersion == 2
    ==>
        |res.value| == HeaderTypes.MESSAGE_ID_LEN_V2
  {
    var maybeId;
    if suite.messageVersion == 1 {
      maybeId := crypto.GenerateRandomBytes(
        Primitives.Types.GenerateRandomBytesInput( length := HeaderTypes.MESSAGE_ID_LEN_V1 as int32));
    } else {
      maybeId := crypto.GenerateRandomBytes(
        Primitives.Types.GenerateRandomBytesInput( length := HeaderTypes.MESSAGE_ID_LEN_V2 as int32));
    }
    var id :- maybeId
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));

    return Success(id);
  }

    // We restrict this method to the encrypt path so that we can assume the body is framed.
  method {:vcs_split_on_every_assert} BuildHeaderForEncrypt(
    messageId: HeaderTypes.MessageId,
    suite: HeaderTypes.ESDKAlgorithmSuite,
    encryptionContext: MPL.EncryptionContext,
    requiredEncryptionContextKeys: MPL.EncryptionContextKeys,
    encryptedDataKeys: SerializableTypes.ESDKEncryptedDataKeys,
    frameLength: uint32,
    derivedDataKeys: KeyDerivation.ExpandedKeyMaterial,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<Header.HeaderInfo, Types.Error>)

    requires !suite.commitment.IDENTITY?

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    requires SerializableTypes.IsESDKEncryptionContext(encryptionContext)
    
    requires forall k: UTF8.ValidUTF8Bytes | k in requiredEncryptionContextKeys :: k in encryptionContext

    requires suite.commitment.HKDF? ==>
        && derivedDataKeys.commitmentKey.Some?
        && |derivedDataKeys.commitmentKey.value| == suite.commitment.HKDF.outputKeyLength as int

    requires frameLength > 0

    // Make sure the output correctly uses the values that were given as input
    ensures res.Success? ==>
        && res.value.suite == suite
        && res.value.body.frameLength == frameLength
        && res.value.encryptionContext == encryptionContext

    ensures res.Success? ==> Header.IsHeader(res.value)

    ensures res.Success? ==> res.value.body.contentType.Framed?
  {

      //= aws-encryption-sdk-specification/client-apis/encrypt.md#construct-the-header
      //# - [AAD](../data-format/message-header.md#aad): MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
      //#  in the [encryption materials](../framework/structures.md#encryption-materials),
      //#  and this serialization MUST NOT contain any key value pairs listed in
      //#  the [encryption material's](../framework/structures.md#encryption-materials)
      //#  [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
      var reqKeySet : set<UTF8.ValidUTF8Bytes> := set k <- requiredEncryptionContextKeys;
      var storedEncryptionContext: MPL.EncryptionContext :=
        map f | f in (encryptionContext - reqKeySet) :: f := encryptionContext[f];
      EncryptionContext.SubsetOfESDKEncryptionContextIsESDKEncryptionContext(encryptionContext, storedEncryptionContext);

      var canonicalStoredEncryptionContext := EncryptionContext.GetCanonicalEncryptionContext(storedEncryptionContext);

      var body := BuildHeaderBody(
          messageId,
          suite,
          canonicalStoredEncryptionContext,
          encryptedDataKeys,
          frameLength as uint32,
          derivedDataKeys.commitmentKey
      );

      var requiredEncryptionContextMap: MPL.EncryptionContext := 
        map r | r in reqKeySet :: r := encryptionContext[r];
      EncryptionContext.SubsetOfESDKEncryptionContextIsESDKEncryptionContext(encryptionContext, requiredEncryptionContextMap);
      
      var canonicalReqEncryptionContext := 
        EncryptionContext.GetCanonicalEncryptionContext(requiredEncryptionContextMap);
      var serializedReqEncryptionContext := 
        EncryptionContext.WriteEmptyEcOrWriteAAD(canonicalReqEncryptionContext);
       
      //= compliance/client-apis/encrypt.txt#2.6.2
      //# Before encrypting input plaintext, this operation MUST serialize the
      //# message header body (../data-format/message-header.md).
      var rawHeader := Header.WriteHeaderBody(body);

      //= compliance/client-apis/encrypt.txt#2.6.2
      //# After serializing the message header body, this operation MUST
      //# calculate an authentication tag (../data-format/message-
      //# header.md#authentication-tag) over the message header body.
      var headerAuth :- BuildHeaderAuthTag(suite, derivedDataKeys.dataKey, rawHeader, serializedReqEncryptionContext, crypto);

      var header := Header.HeaderInfo(
          body := body,
          rawHeader := rawHeader,
          encryptionContext := encryptionContext,
          suite := suite,
          headerAuth := headerAuth
      );

      return Success(header);
  }

  method BuildHeaderBody(
    messageId: HeaderTypes.MessageId,
    suite: HeaderTypes.ESDKAlgorithmSuite,
    encryptionContext: EncryptionContext.ESDKCanonicalEncryptionContext,
    encryptedDataKeys: SerializableTypes.ESDKEncryptedDataKeys,
    frameLength: uint32,
    suiteData: Option<seq<uint8>>
  ) returns (res: HeaderTypes.HeaderBody)

  requires !suite.commitment.IDENTITY?

  //= compliance/data-format/message-header.txt#2.5.2
  //= type=implication
  //# The length of the suite data field MUST be equal to
  //# the Algorithm Suite Data Length (../framework/algorithm-
  //# suites.md#algorithm-suite-data-length) value of the algorithm suite
  //# (../framework/algorithm-suites.md) specified by the Algorithm Suite
  //# ID (Section 2.5.1.5) field.
  requires suite.commitment.HKDF? ==>
      && suiteData.Some?
      && |suiteData.value| == suite.commitment.HKDF.outputKeyLength as int

  // This ensures that our header is internally consistent with respect to
  // commitment (e.g. creating the right header version for the given suite)
  ensures Header.HeaderVersionSupportsCommitment?(suite, res)

  // Correct construction of V2 headers
  ensures
    && suite.commitment.HKDF?
  ==>
    && res == HeaderTypes.HeaderBody.V2HeaderBody(
      algorithmSuite := suite,
      messageId := messageId,
      encryptionContext := encryptionContext,
      encryptedDataKeys := encryptedDataKeys,
      contentType := HeaderTypes.ContentType.Framed,
      frameLength := frameLength,
      suiteData := suiteData.value
    )

  // Correct construction of V1 headers
  ensures
    && suite.commitment.None?
  ==>
    && res == HeaderTypes.HeaderBody.V1HeaderBody(
      messageType := HeaderTypes.MessageType.TYPE_CUSTOMER_AED,
      algorithmSuite := suite,
      messageId := messageId,
      encryptionContext := encryptionContext,
      encryptedDataKeys := encryptedDataKeys,
      contentType := HeaderTypes.ContentType.Framed,
      headerIvLength := SerializableTypes.GetIvLength(suite) as nat,
      frameLength := frameLength
    )
  {
    reveal Header.HeaderVersionSupportsCommitment?();

    //= compliance/client-apis/encrypt.txt#2.8.1
    //# Implementations of the AWS Encryption SDK MUST NOT encrypt using the
    //# Non-Framed content type.
    var contentType := HeaderTypes.ContentType.Framed;

    match suite.commitment {
      case None(_) => return HeaderTypes.HeaderBody.V1HeaderBody(
        messageType := HeaderTypes.MessageType.TYPE_CUSTOMER_AED,
        algorithmSuite := suite,
        messageId := messageId,
        encryptionContext := encryptionContext,
        encryptedDataKeys := encryptedDataKeys,
        contentType := contentType,
        headerIvLength := SerializableTypes.GetIvLength(suite) as nat,
        frameLength := frameLength
      );
      case HKDF(_) => return HeaderTypes.HeaderBody.V2HeaderBody(
        algorithmSuite := suite,
        messageId := messageId,
        encryptionContext := encryptionContext,
        encryptedDataKeys := encryptedDataKeys,
        contentType := contentType,
        frameLength := frameLength,
        suiteData := suiteData.value
      );
    }
  }

  method BuildHeaderAuthTag(
    suite: MPL.AlgorithmSuiteInfo,
    dataKey: seq<uint8>,
    rawHeader: seq<uint8>,
    serializedReqEncryptionContext: seq<uint8>,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<HeaderTypes.HeaderAuth, Types.Error>)

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    ensures res.Success? ==> Header.HeaderAuth?(suite, res.value)
  {
    //= compliance/client-apis/encrypt.txt#2.6.2
    //# The
    //# value of this MUST be the output of the authenticated encryption
    //# algorithm (../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the algorithm suite (../framework/algorithm-suites.md),
    //# with the following inputs:
    var keyLength := SerializableTypes.GetEncryptKeyLength(suite) as nat;
    :- Need(|dataKey| == keyLength,
      Types.AwsEncryptionSdkException( message := "Incorrect data key length"));

    var ivLength := SerializableTypes.GetIvLength(suite);
    //#*  The IV has a value of 0.
    var iv: seq<uint8> := seq(ivLength, _ => 0);

    var maybeEncryptionOutput := crypto.AESEncrypt(
      Primitives.Types.AESEncryptInput(
        encAlg := suite.encrypt.AES_GCM,
        iv := iv,
        //#*  The cipherkey is the derived data key
        key := dataKey,
        //#*  The plaintext is an empty byte array
        msg := [],
        //#* The AAD MUST be the concatenation of the serialized 
        //#  [message header body](../data-format/message-header.md#header-body)
        //#  and the serialization of encryption context to only authenticate.
        aad := rawHeader + serializedReqEncryptionContext
      )
    );
    var encryptionOutput :- maybeEncryptionOutput
      .MapFailure(e => Types.AwsCryptographyPrimitives(e));
    var headerAuth := HeaderTypes.HeaderAuth.AESMac(
        headerIv := iv,
        headerAuthTag := encryptionOutput.authTag
    );
    return Success(headerAuth);
  }

  method GetEncryptionMaterials(
    cmm: MPL.ICryptographicMaterialsManager,
    algorithmSuiteId: Option<MPL.AlgorithmSuiteId>,
    encryptionContext: MPL.EncryptionContext,
    maxPlaintextLength: int64,
    commitmentPolicy: MPL.ESDKCommitmentPolicy,
    mpl: MaterialProviders.MaterialProvidersClient
  ) returns (res: Result<MPL.EncryptionMaterials, Types.Error>)

    requires cmm.ValidState() && mpl.ValidState()
    modifies cmm.Modifies, mpl.Modifies
    ensures cmm.ValidState() && mpl.ValidState()

    ensures res.Success? ==>
      && mpl.EncryptionMaterialsHasPlaintextDataKey(res.value).Success?
      && !res.value.algorithmSuite.commitment.IDENTITY?;

    ensures res.Success? ==>
      && SerializableTypes.IsESDKEncryptionContext(res.value.encryptionContext)

    ensures res.Success? ==>
      && HasUint16Len(res.value.encryptedDataKeys)

    ensures res.Success? ==>
      && forall edk | edk in res.value.encryptedDataKeys
        :: SerializableTypes.IsESDKEncryptedDataKey(edk)
  {
    var encMatRequest := MPL.GetEncryptionMaterialsInput(
      encryptionContext := encryptionContext,
      commitmentPolicy := MPL.CommitmentPolicy.ESDK(commitmentPolicy),
      algorithmSuiteId := algorithmSuiteId,
      maxPlaintextLength := Option.Some(maxPlaintextLength),
      requiredEncryptionContextKeys := None
    );

    //= compliance/client-apis/encrypt.txt#2.6.1
    //# This operation MUST obtain this set of encryption
    //# materials (../framework/structures.md#encryption-materials) by
    //# calling Get Encryption Materials (../framework/cmm-interface.md#get-
    //# encryption-materials) on a CMM (../framework/cmm-interface.md).
    var getEncMatResult := cmm.GetEncryptionMaterials(encMatRequest);
    var output :- getEncMatResult
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));

    var materials := output.encryptionMaterials;

    // Validations to ensure we got back materials that we can use

    //= compliance/client-apis/encrypt.txt#2.6.1
    //# If this
    //# algorithm suite (../framework/algorithm-suites.md) is not supported
    //# by the commitment policy (client.md#commitment-policy) configured in
    //# the client (client.md) encrypt MUST yield an error.
    var _ :- mpl
      .ValidateCommitmentPolicyOnEncrypt(MPL.ValidateCommitmentPolicyOnEncryptInput(
        algorithm := materials.algorithmSuite.id,
        commitmentPolicy := MPL.CommitmentPolicy.ESDK(commitmentPolicy)
      ))
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));
    var _ :- mpl.EncryptionMaterialsHasPlaintextDataKey(materials)
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));
    :- Need(
      SerializableTypes.IsESDKEncryptionContext(materials.encryptionContext),
      Types.AwsEncryptionSdkException(
        message := "CMM failed to return serializable encryption materials.")
    );
    :- Need(HasUint16Len(materials.encryptedDataKeys),
      Types.AwsEncryptionSdkException(
        message := "CMM returned EDKs that exceed the allowed maximum."));
    :- Need(forall edk | edk in materials.encryptedDataKeys
      :: SerializableTypes.IsESDKEncryptedDataKey(edk),
      Types.AwsEncryptionSdkException(
        message := "CMM returned non-serializable encrypted data key."));

    return Success(materials);
  }

  method GetDecryptionMaterials(
    cmm: MPL.ICryptographicMaterialsManager,
    algorithmSuiteId: MPL.AlgorithmSuiteId,
    headerBody: HeaderTypes.HeaderBody,
    reproducedEncryptionContext: Option<MPL.EncryptionContext>,
    commitmentPolicy: MPL.ESDKCommitmentPolicy,
    mpl: MaterialProviders.MaterialProvidersClient
  ) returns (res: Result<MPL.DecryptionMaterials, Types.Error>)

    requires cmm.ValidState() && mpl.ValidState()
    modifies cmm.Modifies, mpl.Modifies
    ensures cmm.ValidState() && mpl.ValidState()

    ensures res.Success? ==>
      && mpl.DecryptionMaterialsWithPlaintextDataKey(res.value).Success?
      && SerializableTypes.IsESDKEncryptionContext(res.value.encryptionContext)

    ensures old(cmm.History.DecryptMaterials) < cmm.History.DecryptMaterials

    ensures res.Success? ==>
      && Seq.Last(cmm.History.DecryptMaterials).output.Success?
      && Seq.Last(cmm.History.DecryptMaterials).output.value.decryptionMaterials == res.value
  {
    var encryptionContext := EncryptionContext.GetEncryptionContext(headerBody.encryptionContext);
    //= compliance/client-apis/decrypt.txt#2.7.2
    //# ./framework/cmm-
    //# interface.md#decrypt-materials) operation MUST be constructed as
    //# follows:
    var decMatRequest := MPL.DecryptMaterialsInput(
      //#*  Algorithm Suite ID: This is the parsed algorithm suite ID
      //#   (../data-format/message-header.md#algorithm-suite-id) from the
      //#   message header.
      algorithmSuiteId := algorithmSuiteId,
      commitmentPolicy := MPL.CommitmentPolicy.ESDK(commitmentPolicy),
      //#*  Encrypted Data Keys: This is the parsed encrypted data keys
      //#   (../data-format/message-header#encrypted-data-keys) from the
      //#   message header.
      encryptedDataKeys := headerBody.encryptedDataKeys,
      //#*  Encryption Context: This is the parsed encryption context
      //#   (../data-format/message-header.md#aad) from the message header.
      encryptionContext := encryptionContext,
      //#* Reproduced Encryption Context: This is the
      //# [input](#input) encryption context.
      reproducedEncryptionContext := reproducedEncryptionContext
    );
    var decMatResult := cmm.DecryptMaterials(decMatRequest);
    var output :- decMatResult
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));
    var materials := output.decryptionMaterials;

    //= compliance/client-apis/decrypt.txt#2.7.2
    //# If the
    //# algorithm suite is not supported by the commitment policy
    //# (client.md#commitment-policy) configured in the client (client.md)
    //# decrypt MUST yield an error.
    var _ :- mpl
      .ValidateCommitmentPolicyOnDecrypt(MPL.ValidateCommitmentPolicyOnDecryptInput(
        algorithm := materials.algorithmSuite.id,
        commitmentPolicy := MPL.CommitmentPolicy.ESDK(commitmentPolicy)
      ))
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));

    var _ :- mpl.DecryptionMaterialsWithPlaintextDataKey(materials)
      .MapFailure(e => Types.AwsCryptographyMaterialProviders(e));      

    :- Need(SerializableTypes.IsESDKEncryptionContext(materials.encryptionContext),
      Types.AwsEncryptionSdkException(
        message := "CMM failed to return serializable encryption materials."));

    return Success(materials);
  }


  /*
    * Ensures that the suite data contained in the header of a message matches
    * the expected suite data
    */
  method ValidateSuiteData(
    suite: MPL.AlgorithmSuiteInfo,
    header: HeaderTypes.HeaderBody,
    expectedSuiteData: seq<uint8>
  ) returns (res: Result<(), Types.Error>)

  // Validating suite data is only relevant for suites with commitment
  requires suite.commitment.HKDF?

  // We can't dereference the 'suiteData' portion of the body, which only exists
  // in V2 headers, unless we know we have a V2HeaderBody
  requires header.V2HeaderBody?

  //= compliance/client-apis/decrypt.txt#2.7.2
  //= type=implication
  //# The
  //# derived commit key MUST equal the commit key stored in the message
  //# header.
  ensures res.Success? ==> header.suiteData == expectedSuiteData

  // Failure cases
  ensures header.suiteData != expectedSuiteData ==> res.Failure?
  ensures |header.suiteData| != suite.commitment.HKDF.outputKeyLength as int ==> res.Failure?
  {
    :- Need(
      |header.suiteData| == suite.commitment.HKDF.outputKeyLength as int,
      Types.AwsEncryptionSdkException(
        message := "Commitment key is invalid")
    );

    :- Need(
      expectedSuiteData == header.suiteData,
      Types.AwsEncryptionSdkException(
        message := "Commitment key does not match")
    );

    return Success(());
  }


  method {:vcs_split_on_every_assert} ReadAndDecryptFramedMessageBody(
    buffer: SerializeFunctions.ReadableBuffer,
    header: Frames.FramedHeader,
    key: seq<uint8>,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<(seq<uint8>, SerializeFunctions.ReadableBuffer), Types.Error>)

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    requires buffer.start <= |buffer.bytes|
    requires |key| == SerializableTypes.GetEncryptKeyLength(header.suite) as nat
    ensures res.Success? ==>
        var (plaintext, tail) := res.value;
        && buffer.start <= tail.start <= |buffer.bytes|
        && SerializeFunctions.CorrectlyReadRange(buffer, tail, buffer.bytes[buffer.start..tail.start])
  {
      assert CorrectlyReadRange(buffer, buffer, []) by { reveal CorrectlyReadRange(); }
      CorrectlyReadByteRange(buffer, buffer, []);

      var messageBody :- MessageBody.ReadFramedMessageBody(buffer, header, [], buffer)
          .MapFailure(MapSerializeFailure(": ReadFramedMessageBody"));

      CorrectlyReadByteRange(buffer, messageBody.tail, MessageBody.WriteFramedMessageBody(messageBody.data));

      assert header == messageBody.data.finalFrame.header;
      assert |key| == SerializableTypes.GetEncryptKeyLength(messageBody.data.finalFrame.header.suite) as nat;

      var plaintext :- MessageBody.DecryptFramedMessageBody(messageBody.data, key, crypto);
      var messageBodyTail := messageBody.tail;

      return Success((plaintext, messageBodyTail));
  }

  method {:vcs_split_on_every_assert} ReadAndDecryptNonFramedMessageBody(
    buffer: SerializeFunctions.ReadableBuffer,
    header: Frames.NonFramedHeader,
    key: seq<uint8>,
    crypto: Primitives.AtomicPrimitivesClient
  )
    returns (res: Result<(seq<uint8>, SerializeFunctions.ReadableBuffer), Types.Error>)

    requires crypto.ValidState()
    modifies crypto.Modifies
    ensures crypto.ValidState()

    requires buffer.start <= |buffer.bytes|
    requires |key| == SerializableTypes.GetEncryptKeyLength(header.suite) as nat
    ensures res.Success? ==>
        var (plaintext, tail) := res.value;
        && buffer.start <= tail.start <= |buffer.bytes|
        && SerializeFunctions.CorrectlyReadRange(buffer, tail, buffer.bytes[buffer.start..tail.start])
  {
      var messageBody :- MessageBody.ReadNonFramedMessageBody(buffer, header)
          .MapFailure(MapSerializeFailure(": ReadNonFramedMessageBody"));

      CorrectlyReadByteRange(buffer, messageBody.tail, MessageBody.WriteNonFramedMessageBody(messageBody.data));

      var frame: Frames.Frame := messageBody.data;
      assert frame.NonFramed?;

      assert header == frame.header;
      assert |key| == SerializableTypes.GetEncryptKeyLength(frame.header.suite) as nat;

      var plaintext :- MessageBody.DecryptFrame(frame, key, crypto);
      var messageBodyTail := messageBody.tail;

      return Success((plaintext, messageBodyTail));
  }

}
