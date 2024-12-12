import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import aws_encryption_sdk.internaldafny.generated.module_ as module_
import _dafny as _dafny
import System_ as System_
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import smithy_dafny_standard_library.internaldafny.generated.BoundedInts as BoundedInts
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_UInt as StandardLibrary_UInt
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_Sequence as StandardLibrary_Sequence
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_String as StandardLibrary_String
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary as StandardLibrary
import smithy_dafny_standard_library.internaldafny.generated.UTF8 as UTF8
import aws_cryptography_primitives.internaldafny.generated.AwsCryptographyPrimitivesTypes as AwsCryptographyPrimitivesTypes
import aws_cryptography_primitives.internaldafny.generated.ExternRandom as ExternRandom
import aws_cryptography_primitives.internaldafny.generated.Random as Random
import aws_cryptography_primitives.internaldafny.generated.AESEncryption as AESEncryption
import aws_cryptography_primitives.internaldafny.generated.ExternDigest as ExternDigest
import aws_cryptography_primitives.internaldafny.generated.Digest as Digest
import aws_cryptography_primitives.internaldafny.generated.HMAC as HMAC
import aws_cryptography_primitives.internaldafny.generated.WrappedHMAC as WrappedHMAC
import aws_cryptography_primitives.internaldafny.generated.HKDF as HKDF
import aws_cryptography_primitives.internaldafny.generated.WrappedHKDF as WrappedHKDF
import aws_cryptography_primitives.internaldafny.generated.Signature as Signature
import aws_cryptography_primitives.internaldafny.generated.KdfCtr as KdfCtr
import aws_cryptography_primitives.internaldafny.generated.RSAEncryption as RSAEncryption
import aws_cryptography_primitives.internaldafny.generated.ECDH as ECDH
import aws_cryptography_primitives.internaldafny.generated.AwsCryptographyPrimitivesOperations as AwsCryptographyPrimitivesOperations
import aws_cryptography_primitives.internaldafny.generated.AtomicPrimitives as AtomicPrimitives
import aws_cryptography_internal_dynamodb.internaldafny.generated.ComAmazonawsDynamodbTypes as ComAmazonawsDynamodbTypes
import aws_cryptography_internal_kms.internaldafny.generated.ComAmazonawsKmsTypes as ComAmazonawsKmsTypes
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyKeyStoreTypes as AwsCryptographyKeyStoreTypes
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyMaterialProvidersTypes as AwsCryptographyMaterialProvidersTypes
import smithy_dafny_standard_library.internaldafny.generated.Base64 as Base64
import aws_cryptographic_material_providers.internaldafny.generated.AlgorithmSuites as AlgorithmSuites
import aws_cryptographic_material_providers.internaldafny.generated.Materials as Materials
import aws_cryptographic_material_providers.internaldafny.generated.Keyring as Keyring
import smithy_dafny_standard_library.internaldafny.generated.Relations as Relations
import smithy_dafny_standard_library.internaldafny.generated.Seq_MergeSort as Seq_MergeSort
import smithy_dafny_standard_library.internaldafny.generated.Math as Math
import smithy_dafny_standard_library.internaldafny.generated.Seq as Seq
import aws_cryptographic_material_providers.internaldafny.generated.MultiKeyring as MultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsArnParsing as AwsArnParsing
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkAreUnique as AwsKmsMrkAreUnique
import smithy_dafny_standard_library.internaldafny.generated.Actions as Actions
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkMatchForDecrypt as AwsKmsMrkMatchForDecrypt
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsUtils as AwsKmsUtils
import aws_cryptographic_material_providers.internaldafny.generated.Constants as Constants
import smithy_dafny_standard_library.internaldafny.generated.UUID as UUID
import aws_cryptographic_material_providers.internaldafny.generated.MaterialWrapping as MaterialWrapping
import smithy_dafny_standard_library.internaldafny.generated.SortedSets as SortedSets
import aws_cryptographic_material_providers.internaldafny.generated.CanonicalEncryptionContext as CanonicalEncryptionContext
import aws_cryptographic_material_providers.internaldafny.generated.IntermediateKeyWrapping as IntermediateKeyWrapping
import aws_cryptographic_material_providers.internaldafny.generated.EdkWrapping as EdkWrapping
import aws_cryptographic_material_providers.internaldafny.generated.ErrorMessages as ErrorMessages
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsKeyring as AwsKmsKeyring
import aws_cryptographic_material_providers.internaldafny.generated.StrictMultiKeyring as StrictMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsDiscoveryKeyring as AwsKmsDiscoveryKeyring
import aws_cryptography_internal_kms.internaldafny.generated.Com_Amazonaws_Kms as Com_Amazonaws_Kms
import aws_cryptography_internal_dynamodb.internaldafny.generated.Com_Amazonaws_Dynamodb as Com_Amazonaws_Dynamodb
import aws_cryptographic_material_providers.internaldafny.generated.DiscoveryMultiKeyring as DiscoveryMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkDiscoveryKeyring as AwsKmsMrkDiscoveryKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MrkAwareDiscoveryMultiKeyring as MrkAwareDiscoveryMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkKeyring as AwsKmsMrkKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MrkAwareStrictMultiKeyring as MrkAwareStrictMultiKeyring
import smithy_dafny_standard_library.internaldafny.generated.DafnyLibraries as DafnyLibraries
import smithy_dafny_standard_library.internaldafny.generated.Time as Time
import aws_cryptographic_material_providers.internaldafny.generated.LocalCMC as LocalCMC
import aws_cryptographic_material_providers.internaldafny.generated.SynchronizedLocalCMC as SynchronizedLocalCMC
import aws_cryptographic_material_providers.internaldafny.generated.StormTracker as StormTracker
import aws_cryptographic_material_providers.internaldafny.generated.StormTrackingCMC as StormTrackingCMC
import aws_cryptographic_material_providers.internaldafny.generated.CacheConstants as CacheConstants
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsHierarchicalKeyring as AwsKmsHierarchicalKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsRsaKeyring as AwsKmsRsaKeyring
import aws_cryptographic_material_providers.internaldafny.generated.EcdhEdkWrapping as EcdhEdkWrapping
import aws_cryptographic_material_providers.internaldafny.generated.RawECDHKeyring as RawECDHKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsEcdhKeyring as AwsKmsEcdhKeyring
import aws_cryptographic_material_providers.internaldafny.generated.RawAESKeyring as RawAESKeyring
import aws_cryptographic_material_providers.internaldafny.generated.RawRSAKeyring as RawRSAKeyring
import aws_cryptographic_material_providers.internaldafny.generated.CMM as CMM
import aws_cryptographic_material_providers.internaldafny.generated.Defaults as Defaults
import aws_cryptographic_material_providers.internaldafny.generated.Commitment as Commitment
import aws_cryptographic_material_providers.internaldafny.generated.DefaultCMM as DefaultCMM
import aws_cryptographic_material_providers.internaldafny.generated.DefaultClientSupplier as DefaultClientSupplier
import aws_cryptographic_material_providers.internaldafny.generated.Utils as Utils
import aws_cryptographic_material_providers.internaldafny.generated.RequiredEncryptionContextCMM as RequiredEncryptionContextCMM
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyMaterialProvidersOperations as AwsCryptographyMaterialProvidersOperations
import aws_cryptographic_material_providers.internaldafny.generated.MaterialProviders as MaterialProviders
import aws_cryptographic_material_providers.internaldafny.generated.KeyStoreErrorMessages as KeyStoreErrorMessages
import aws_cryptographic_material_providers.internaldafny.generated.KmsArn as KmsArn
import aws_cryptographic_material_providers.internaldafny.generated.Structure as Structure
import aws_cryptographic_material_providers.internaldafny.generated.KMSKeystoreOperations as KMSKeystoreOperations
import aws_cryptographic_material_providers.internaldafny.generated.DDBKeystoreOperations as DDBKeystoreOperations
import aws_cryptographic_material_providers.internaldafny.generated.CreateKeys as CreateKeys
import aws_cryptographic_material_providers.internaldafny.generated.CreateKeyStoreTable as CreateKeyStoreTable
import aws_cryptographic_material_providers.internaldafny.generated.GetKeys as GetKeys
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyKeyStoreOperations as AwsCryptographyKeyStoreOperations
import aws_cryptographic_material_providers.internaldafny.generated.KeyStore as KeyStore
import aws_cryptography_primitives.internaldafny.generated.AesKdfCtr as AesKdfCtr
import smithy_dafny_standard_library.internaldafny.generated.Unicode as Unicode
import smithy_dafny_standard_library.internaldafny.generated.Functions as Functions
import smithy_dafny_standard_library.internaldafny.generated.Utf8EncodingForm as Utf8EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.Utf16EncodingForm as Utf16EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.UnicodeStrings as UnicodeStrings
import smithy_dafny_standard_library.internaldafny.generated.FileIO as FileIO
import smithy_dafny_standard_library.internaldafny.generated.GeneralInternals as GeneralInternals
import smithy_dafny_standard_library.internaldafny.generated.MulInternalsNonlinear as MulInternalsNonlinear
import smithy_dafny_standard_library.internaldafny.generated.MulInternals as MulInternals
import smithy_dafny_standard_library.internaldafny.generated.Mul as Mul
import smithy_dafny_standard_library.internaldafny.generated.ModInternalsNonlinear as ModInternalsNonlinear
import smithy_dafny_standard_library.internaldafny.generated.DivInternalsNonlinear as DivInternalsNonlinear
import smithy_dafny_standard_library.internaldafny.generated.ModInternals as ModInternals
import smithy_dafny_standard_library.internaldafny.generated.DivInternals as DivInternals
import smithy_dafny_standard_library.internaldafny.generated.DivMod as DivMod
import smithy_dafny_standard_library.internaldafny.generated.Power as Power
import smithy_dafny_standard_library.internaldafny.generated.Logarithm as Logarithm
import smithy_dafny_standard_library.internaldafny.generated.StandardLibraryInterop as StandardLibraryInterop
import smithy_dafny_standard_library.internaldafny.generated.Streams as Streams
import smithy_dafny_standard_library.internaldafny.generated.Sorting as Sorting
import smithy_dafny_standard_library.internaldafny.generated.HexStrings as HexStrings
import smithy_dafny_standard_library.internaldafny.generated.GetOpt as GetOpt
import smithy_dafny_standard_library.internaldafny.generated.FloatCompare as FloatCompare
import smithy_dafny_standard_library.internaldafny.generated.ConcurrentCall as ConcurrentCall
import smithy_dafny_standard_library.internaldafny.generated.Base64Lemmas as Base64Lemmas
import aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import aws_encryption_sdk.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptionContext as EncryptionContext
import aws_encryption_sdk.internaldafny.generated.HeaderTypes as HeaderTypes
import aws_encryption_sdk.internaldafny.generated.SharedHeaderFunctions as SharedHeaderFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptedDataKeys as EncryptedDataKeys
import aws_encryption_sdk.internaldafny.generated.V1HeaderBody as V1HeaderBody
import aws_encryption_sdk.internaldafny.generated.V2HeaderBody as V2HeaderBody
import aws_encryption_sdk.internaldafny.generated.HeaderAuth as HeaderAuth
import aws_encryption_sdk.internaldafny.generated.Header as Header
import aws_encryption_sdk.internaldafny.generated.Frames as Frames

# Module: MessageBody

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def BodyAADContentTypeString(bc):
        source0_ = bc
        if True:
            if source0_.is_AADRegularFrame:
                return default__.BODY__AAD__CONTENT__REGULAR__FRAME
        if True:
            if source0_.is_AADFinalFrame:
                return default__.BODY__AAD__CONTENT__FINAL__FRAME
        if True:
            return default__.BODY__AAD__CONTENT__SINGLE__BLOCK

    @staticmethod
    def IVSeq(suite, sequenceNumber):
        return (_dafny.Seq([0 for d_0___v0_ in range((SerializableTypes.default__.GetIvLength(suite)) - (4))])) + (StandardLibrary_UInt.default__.UInt32ToSeq(sequenceNumber))

    @staticmethod
    def EncryptMessageBody(plaintext, header, key, crypto):
        result: Wrappers.Result = None
        d_0_n_: int
        d_1_sequenceNumber_: int
        rhs0_ = 0
        rhs1_ = default__.START__SEQUENCE__NUMBER
        d_0_n_ = rhs0_
        d_1_sequenceNumber_ = rhs1_
        d_2_regularFrames_: _dafny.Seq
        d_2_regularFrames_ = _dafny.Seq([])
        while ((d_0_n_) + (((header).body).frameLength)) < (len(plaintext)):
            d_3_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_3_valueOrError0_ = Wrappers.default__.Need((d_1_sequenceNumber_) < (default__.ENDFRAME__SEQUENCE__NUMBER), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("too many frames")))
            if (d_3_valueOrError0_).IsFailure():
                result = (d_3_valueOrError0_).PropagateFailure()
                return result
            d_4_plaintextFrame_: _dafny.Seq
            d_4_plaintextFrame_ = _dafny.Seq((plaintext)[d_0_n_:(d_0_n_) + (((header).body).frameLength):])
            d_5_valueOrError1_: Wrappers.Result = None
            out0_: Wrappers.Result
            out0_ = default__.EncryptRegularFrame(key, header, d_4_plaintextFrame_, d_1_sequenceNumber_, crypto)
            d_5_valueOrError1_ = out0_
            if (d_5_valueOrError1_).IsFailure():
                result = (d_5_valueOrError1_).PropagateFailure()
                return result
            d_6_regularFrame_: Frames.Frame
            d_6_regularFrame_ = (d_5_valueOrError1_).Extract()
            d_2_regularFrames_ = (d_2_regularFrames_) + (_dafny.Seq([d_6_regularFrame_]))
            d_0_n_ = (d_0_n_) + (((header).body).frameLength)
            d_1_sequenceNumber_ = (d_1_sequenceNumber_) + (1)
        d_7_valueOrError2_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = default__.EncryptFinalFrame(key, header, _dafny.Seq((plaintext)[d_0_n_::]), d_1_sequenceNumber_, crypto)
        d_7_valueOrError2_ = out1_
        if (d_7_valueOrError2_).IsFailure():
            result = (d_7_valueOrError2_).PropagateFailure()
            return result
        d_8_finalFrame_: Frames.Frame
        d_8_finalFrame_ = (d_7_valueOrError2_).Extract()
        result = Wrappers.Result_Success(FramedMessageBody_FramedMessageBody(d_2_regularFrames_, d_8_finalFrame_))
        return result

    @staticmethod
    def EncryptRegularFrame(key, header, plaintext, sequenceNumber, crypto):
        res: Wrappers.Result = None
        d_0_iv_: _dafny.Seq
        d_0_iv_ = default__.IVSeq((header).suite, sequenceNumber)
        d_1_aad_: _dafny.Seq
        d_1_aad_ = default__.BodyAAD(((header).body).messageId, BodyAADContent_AADRegularFrame(), sequenceNumber, len(plaintext))
        d_2_aesEncryptInput_: AwsCryptographyPrimitivesTypes.AESEncryptInput
        d_2_aesEncryptInput_ = AwsCryptographyPrimitivesTypes.AESEncryptInput_AESEncryptInput((((header).suite).encrypt).AES__GCM, d_0_iv_, key, plaintext, d_1_aad_)
        d_3_maybeEncryptionOutput_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (crypto).AESEncrypt(d_2_aesEncryptInput_)
        d_3_maybeEncryptionOutput_ = out0_
        d_4_valueOrError0_: Wrappers.Result = Wrappers.Result.default(AwsCryptographyPrimitivesTypes.AESEncryptOutput.default())()
        def lambda0_(d_5_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_5_e_)

        d_4_valueOrError0_ = (d_3_maybeEncryptionOutput_).MapFailure(lambda0_)
        if (d_4_valueOrError0_).IsFailure():
            res = (d_4_valueOrError0_).PropagateFailure()
            return res
        d_6_encryptionOutput_: AwsCryptographyPrimitivesTypes.AESEncryptOutput
        d_6_encryptionOutput_ = (d_4_valueOrError0_).Extract()
        d_7_frame_: Frames.Frame
        d_7_frame_ = Frames.Frame_RegularFrame(header, sequenceNumber, d_0_iv_, (d_6_encryptionOutput_).cipherText, (d_6_encryptionOutput_).authTag)
        res = Wrappers.Result_Success(d_7_frame_)
        return res
        return res

    @staticmethod
    def EncryptFinalFrame(key, header, plaintext, sequenceNumber, crypto):
        res: Wrappers.Result = None
        d_0_iv_: _dafny.Seq
        d_0_iv_ = default__.IVSeq((header).suite, sequenceNumber)
        d_1_aad_: _dafny.Seq
        d_1_aad_ = default__.BodyAAD(((header).body).messageId, BodyAADContent_AADFinalFrame(), sequenceNumber, len(plaintext))
        d_2_aesEncryptInput_: AwsCryptographyPrimitivesTypes.AESEncryptInput
        d_2_aesEncryptInput_ = AwsCryptographyPrimitivesTypes.AESEncryptInput_AESEncryptInput((((header).suite).encrypt).AES__GCM, d_0_iv_, key, plaintext, d_1_aad_)
        d_3_maybeEncryptionOutput_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (crypto).AESEncrypt(d_2_aesEncryptInput_)
        d_3_maybeEncryptionOutput_ = out0_
        d_4_valueOrError0_: Wrappers.Result = Wrappers.Result.default(AwsCryptographyPrimitivesTypes.AESEncryptOutput.default())()
        def lambda0_(d_5_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_5_e_)

        d_4_valueOrError0_ = (d_3_maybeEncryptionOutput_).MapFailure(lambda0_)
        if (d_4_valueOrError0_).IsFailure():
            res = (d_4_valueOrError0_).PropagateFailure()
            return res
        d_6_encryptionOutput_: AwsCryptographyPrimitivesTypes.AESEncryptOutput
        d_6_encryptionOutput_ = (d_4_valueOrError0_).Extract()
        d_7_finalFrame_: Frames.Frame
        d_7_finalFrame_ = Frames.Frame_FinalFrame(header, sequenceNumber, d_0_iv_, (d_6_encryptionOutput_).cipherText, (d_6_encryptionOutput_).authTag)
        res = Wrappers.Result_Success(d_7_finalFrame_)
        return res
        return res

    @staticmethod
    def DecryptFramedMessageBody(body, key, crypto):
        res: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_plaintext_: _dafny.Seq
        d_0_plaintext_ = _dafny.Seq([])
        hi0_ = len((body).regularFrames)
        for d_1_i_ in range(0, hi0_):
            d_2_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            out0_: Wrappers.Result
            out0_ = default__.DecryptFrame(((body).regularFrames)[d_1_i_], key, crypto)
            d_2_valueOrError0_ = out0_
            if (d_2_valueOrError0_).IsFailure():
                res = (d_2_valueOrError0_).PropagateFailure()
                return res
            d_3_plaintextSegment_: _dafny.Seq
            d_3_plaintextSegment_ = (d_2_valueOrError0_).Extract()
            d_0_plaintext_ = (d_0_plaintext_) + (d_3_plaintextSegment_)
        d_4_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out1_: Wrappers.Result
        out1_ = default__.DecryptFrame((body).finalFrame, key, crypto)
        d_4_valueOrError1_ = out1_
        if (d_4_valueOrError1_).IsFailure():
            res = (d_4_valueOrError1_).PropagateFailure()
            return res
        d_5_finalPlaintextSegment_: _dafny.Seq
        d_5_finalPlaintextSegment_ = (d_4_valueOrError1_).Extract()
        d_0_plaintext_ = (d_0_plaintext_) + (d_5_finalPlaintextSegment_)
        res = Wrappers.Result_Success(d_0_plaintext_)
        return res

    @staticmethod
    def DecryptFrame(frame, key, crypto):
        res: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_aad_: _dafny.Seq
        d_0_aad_ = default__.BodyAADByFrameType(frame)
        d_1_maybePlaintextSegment_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (crypto).AESDecrypt(AwsCryptographyPrimitivesTypes.AESDecryptInput_AESDecryptInput(((((frame).header).suite).encrypt).AES__GCM, key, (frame).encContent, (frame).authTag, (frame).iv, d_0_aad_))
        d_1_maybePlaintextSegment_ = out0_
        d_2_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda0_(d_3_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_3_e_)

        d_2_valueOrError0_ = (d_1_maybePlaintextSegment_).MapFailure(lambda0_)
        if (d_2_valueOrError0_).IsFailure():
            res = (d_2_valueOrError0_).PropagateFailure()
            return res
        d_4_plaintextSegment_: _dafny.Seq
        d_4_plaintextSegment_ = (d_2_valueOrError0_).Extract()
        res = Wrappers.Result_Success(d_4_plaintextSegment_)
        return res
        return res

    @staticmethod
    def BodyAADByFrameType(frame):
        def lambda0_():
            source0_ = frame
            if True:
                if source0_.is_RegularFrame:
                    d_0_header_ = source0_.header
                    d_1_seqNum_ = source0_.seqNum
                    return (d_1_seqNum_, BodyAADContent_AADRegularFrame(), ((d_0_header_).body).frameLength)
            if True:
                if source0_.is_FinalFrame:
                    d_2_seqNum_ = source0_.seqNum
                    d_3_encContent_ = source0_.encContent
                    return (d_2_seqNum_, BodyAADContent_AADFinalFrame(), len(d_3_encContent_))
            if True:
                d_4_encContent_ = source0_.encContent
                return (default__.NONFRAMED__SEQUENCE__NUMBER, BodyAADContent_AADSingleBlock(), len(d_4_encContent_))

        let_tmp_rhs0_ = lambda0_()
        d_5_sequenceNumber_ = let_tmp_rhs0_[0]
        d_6_bc_ = let_tmp_rhs0_[1]
        d_7_length_ = let_tmp_rhs0_[2]
        return default__.BodyAAD((((frame).header).body).messageId, d_6_bc_, d_5_sequenceNumber_, d_7_length_)

    @staticmethod
    def BodyAAD(messageID, bc, sequenceNumber, length):
        d_0_contentAAD_ = UTF8.default__.Encode(default__.BodyAADContentTypeString(bc))
        return (((messageID) + ((d_0_contentAAD_).value)) + (StandardLibrary_UInt.default__.UInt32ToSeq(sequenceNumber))) + (StandardLibrary_UInt.default__.UInt64ToSeq(length))

    @staticmethod
    def WriteFramedMessageBody(body):
        return (default__.WriteMessageRegularFrames((body).regularFrames)) + (Frames.default__.WriteFinalFrame((body).finalFrame))

    @staticmethod
    def WriteMessageRegularFrames(frames):
        d_0___accumulator_ = _dafny.Seq([])
        while True:
            with _dafny.label():
                if (len(frames)) == (0):
                    return (_dafny.Seq([])) + (d_0___accumulator_)
                elif True:
                    d_0___accumulator_ = (Frames.default__.WriteRegularFrame(Seq.default__.Last(frames))) + (d_0___accumulator_)
                    in0_ = Seq.default__.DropLast(frames)
                    frames = in0_
                    raise _dafny.TailCall()
                break

    @staticmethod
    def ReadFramedMessageBody(buffer, header, regularFrames, continuation):
        while True:
            with _dafny.label():
                d_0_valueOrError0_ = SerializeFunctions.default__.ReadUInt32(continuation)
                if (d_0_valueOrError0_).IsFailure():
                    return (d_0_valueOrError0_).PropagateFailure()
                elif True:
                    d_1_sequenceNumber_ = (d_0_valueOrError0_).Extract()
                    if ((d_1_sequenceNumber_).data) != (default__.ENDFRAME__SEQUENCE__NUMBER):
                        d_2_valueOrError1_ = Frames.default__.ReadRegularFrame(continuation, header)
                        if (d_2_valueOrError1_).IsFailure():
                            return (d_2_valueOrError1_).PropagateFailure()
                        elif True:
                            d_3_regularFrame_ = (d_2_valueOrError1_).Extract()
                            d_4_valueOrError2_ = Wrappers.default__.Need((((d_3_regularFrame_).data).seqNum) == ((len(regularFrames)) + (1)), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Sequence number out of order.")))
                            if (d_4_valueOrError2_).IsFailure():
                                return (d_4_valueOrError2_).PropagateFailure()
                            elif True:
                                d_5_nextRegularFrames_ = (regularFrames) + (_dafny.Seq([(d_3_regularFrame_).data]))
                                in0_ = buffer
                                in1_ = header
                                in2_ = d_5_nextRegularFrames_
                                in3_ = (d_3_regularFrame_).tail
                                buffer = in0_
                                header = in1_
                                regularFrames = in2_
                                continuation = in3_
                                raise _dafny.TailCall()
                    elif True:
                        d_6_valueOrError3_ = Frames.default__.ReadFinalFrame(continuation, header)
                        if (d_6_valueOrError3_).IsFailure():
                            return (d_6_valueOrError3_).PropagateFailure()
                        elif True:
                            d_7_finalFrame_ = (d_6_valueOrError3_).Extract()
                            d_8_valueOrError4_ = Wrappers.default__.Need((((d_7_finalFrame_).data).seqNum) == ((len(regularFrames)) + (1)), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Sequence number out of order.")))
                            if (d_8_valueOrError4_).IsFailure():
                                return (d_8_valueOrError4_).PropagateFailure()
                            elif True:
                                d_9_body_ = FramedMessageBody_FramedMessageBody(regularFrames, (d_7_finalFrame_).data)
                                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_9_body_, (d_7_finalFrame_).tail))
                break

    @staticmethod
    def ReadNonFramedMessageBody(buffer, header):
        d_0_valueOrError0_ = Frames.default__.ReadNonFrame(buffer, header)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_block_ = (d_0_valueOrError0_).Extract()
            return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead((d_1_block_).data, (d_1_block_).tail))

    @_dafny.classproperty
    def BODY__AAD__CONTENT__REGULAR__FRAME(instance):
        return _dafny.Seq("AWSKMSEncryptionClient Frame")
    @_dafny.classproperty
    def BODY__AAD__CONTENT__FINAL__FRAME(instance):
        return _dafny.Seq("AWSKMSEncryptionClient Final Frame")
    @_dafny.classproperty
    def BODY__AAD__CONTENT__SINGLE__BLOCK(instance):
        return _dafny.Seq("AWSKMSEncryptionClient Single Block")
    @_dafny.classproperty
    def ENDFRAME__SEQUENCE__NUMBER(instance):
        return Frames.default__.ENDFRAME__SEQUENCE__NUMBER
    @_dafny.classproperty
    def START__SEQUENCE__NUMBER(instance):
        return Frames.default__.START__SEQUENCE__NUMBER
    @_dafny.classproperty
    def NONFRAMED__SEQUENCE__NUMBER(instance):
        return Frames.default__.NONFRAMED__SEQUENCE__NUMBER

class BodyAADContent:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [BodyAADContent_AADRegularFrame(), BodyAADContent_AADFinalFrame(), BodyAADContent_AADSingleBlock()]
    @classmethod
    def default(cls, ):
        return lambda: BodyAADContent_AADRegularFrame()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_AADRegularFrame(self) -> bool:
        return isinstance(self, BodyAADContent_AADRegularFrame)
    @property
    def is_AADFinalFrame(self) -> bool:
        return isinstance(self, BodyAADContent_AADFinalFrame)
    @property
    def is_AADSingleBlock(self) -> bool:
        return isinstance(self, BodyAADContent_AADSingleBlock)

class BodyAADContent_AADRegularFrame(BodyAADContent, NamedTuple('AADRegularFrame', [])):
    def __dafnystr__(self) -> str:
        return f'MessageBody.BodyAADContent.AADRegularFrame'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, BodyAADContent_AADRegularFrame)
    def __hash__(self) -> int:
        return super().__hash__()

class BodyAADContent_AADFinalFrame(BodyAADContent, NamedTuple('AADFinalFrame', [])):
    def __dafnystr__(self) -> str:
        return f'MessageBody.BodyAADContent.AADFinalFrame'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, BodyAADContent_AADFinalFrame)
    def __hash__(self) -> int:
        return super().__hash__()

class BodyAADContent_AADSingleBlock(BodyAADContent, NamedTuple('AADSingleBlock', [])):
    def __dafnystr__(self) -> str:
        return f'MessageBody.BodyAADContent.AADSingleBlock'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, BodyAADContent_AADSingleBlock)
    def __hash__(self) -> int:
        return super().__hash__()


class MessageRegularFrames:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return _dafny.Seq({})

class FramedMessageBody:
    @classmethod
    def default(cls, ):
        return lambda: FramedMessageBody_FramedMessageBody(_dafny.Seq({}), Frames.Frame.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_FramedMessageBody(self) -> bool:
        return isinstance(self, FramedMessageBody_FramedMessageBody)

class FramedMessageBody_FramedMessageBody(FramedMessageBody, NamedTuple('FramedMessageBody', [('regularFrames', Any), ('finalFrame', Any)])):
    def __dafnystr__(self) -> str:
        return f'MessageBody.FramedMessageBody.FramedMessageBody({_dafny.string_of(self.regularFrames)}, {_dafny.string_of(self.finalFrame)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, FramedMessageBody_FramedMessageBody) and self.regularFrames == __o.regularFrames and self.finalFrame == __o.finalFrame
    def __hash__(self) -> int:
        return super().__hash__()


class FramedMessage:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return FramedMessageBody.default()()

class MessageFrame:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Frames.Frame.default()()

class Frame:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Frames.Frame.default()()
