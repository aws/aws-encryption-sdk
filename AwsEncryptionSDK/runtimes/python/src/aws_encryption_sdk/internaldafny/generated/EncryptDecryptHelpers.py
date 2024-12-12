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
import aws_encryption_sdk.internaldafny.generated.MessageBody as MessageBody
import aws_encryption_sdk.internaldafny.generated.KeyDerivation as KeyDerivation

# Module: EncryptDecryptHelpers

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def SerializeMessageWithSignature(framedMessage, signature, suite):
        d_0_serializedSignature_ = SerializeFunctions.default__.WriteShortLengthSeq(signature)
        d_1_valueOrError0_ = default__.SerializeMessageWithoutSignature(framedMessage, suite)
        if (d_1_valueOrError0_).IsFailure():
            return (d_1_valueOrError0_).PropagateFailure()
        elif True:
            d_2_serializedMessage_ = (d_1_valueOrError0_).Extract()
            return Wrappers.Result_Success((d_2_serializedMessage_) + (d_0_serializedSignature_))

    @staticmethod
    def SerializeMessageWithoutSignature(framedMessage, suite):
        d_0_valueOrError0_ = HeaderAuth.default__.WriteHeaderAuthTag((((framedMessage).finalFrame).header).headerAuth, suite)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_headerAuth_ = (d_0_valueOrError0_).Extract()
            return Wrappers.Result_Success((((((framedMessage).finalFrame).header).rawHeader) + (d_1_headerAuth_)) + (MessageBody.default__.WriteFramedMessageBody(framedMessage)))

    @staticmethod
    def VerifySignature(buffer, msg, decMat, crypto):
        res: Wrappers.Result = Wrappers.Result.default(SerializeFunctions.ReadableBuffer.default())()
        if ((decMat).verificationKey).is_None:
            res = Wrappers.Result_Success(buffer)
            return res
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(SerializeFunctions.SuccessfulRead.default(StandardLibrary_UInt.seq16.default))()
        d_0_valueOrError0_ = (SerializeFunctions.default__.ReadShortLengthSeq(buffer)).MapFailure(default__.MapSerializeFailure(_dafny.Seq(": ReadShortLengthSeq")))
        if (d_0_valueOrError0_).IsFailure():
            res = (d_0_valueOrError0_).PropagateFailure()
            return res
        d_1_signature_: SerializeFunctions.SuccessfulRead
        d_1_signature_ = (d_0_valueOrError0_).Extract()
        d_2_ecdsaParams_: AwsCryptographyPrimitivesTypes.ECDSASignatureAlgorithm
        d_2_ecdsaParams_ = ((((decMat).algorithmSuite).signature).ECDSA).curve
        d_3_maybeSignatureVerifiedResult_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (crypto).ECDSAVerify(AwsCryptographyPrimitivesTypes.ECDSAVerifyInput_ECDSAVerifyInput(d_2_ecdsaParams_, ((decMat).verificationKey).value, msg, (d_1_signature_).data))
        d_3_maybeSignatureVerifiedResult_ = out0_
        d_4_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.bool)()
        def lambda0_(d_5_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_5_e_)

        d_4_valueOrError1_ = (d_3_maybeSignatureVerifiedResult_).MapFailure(lambda0_)
        if (d_4_valueOrError1_).IsFailure():
            res = (d_4_valueOrError1_).PropagateFailure()
            return res
        d_6_signatureVerifiedResult_: bool
        d_6_signatureVerifiedResult_ = (d_4_valueOrError1_).Extract()
        if not(d_6_signatureVerifiedResult_):
            res = Wrappers.Result_Failure(AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Invalid signature")))
            return res
        res = Wrappers.Result_Success((d_1_signature_).tail)
        return res
        return res

    @staticmethod
    def MapSerializeFailure(s):
        def lambda0_(d_0_s_):
            def lambda1_(d_1_e_):
                def lambda2_():
                    source0_ = d_1_e_
                    if True:
                        if source0_.is_Error:
                            d_2_e_ = source0_.message
                            return AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(d_2_e_)
                    if True:
                        return AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException((_dafny.Seq("Incomplete message")) + (d_0_s_))

                return lambda2_()

            return lambda1_

        return lambda0_(s)

    @staticmethod
    def ValidateEncryptionContext(input):
        def lambda0_(exists_var_0_):
            d_0_key_: _dafny.Seq = exists_var_0_
            if UTF8.ValidUTF8Bytes._Is(d_0_key_):
                return ((d_0_key_) in (((input).value).keys)) and ((default__.RESERVED__ENCRYPTION__CONTEXT) <= (d_0_key_))
            elif True:
                return False

        if ((input).is_Some) and (_dafny.quantifier((((input).value).keys).Elements, False, lambda0_)):
            return Wrappers.Outcome_Fail(AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Encryption context keys cannot contain reserved prefix 'aws-crypto-'")))
        elif True:
            return Wrappers.Outcome_Pass()

    @staticmethod
    def CreateCmmFromInput(inputCmm, inputKeyring):
        res: Wrappers.Result = None
        d_0_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_0_valueOrError0_ = Wrappers.default__.Need(((inputCmm).is_None) or ((inputKeyring).is_None), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Cannot provide both a keyring and a CMM")))
        if (d_0_valueOrError0_).IsFailure():
            res = (d_0_valueOrError0_).PropagateFailure()
            return res
        d_1_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_1_valueOrError1_ = Wrappers.default__.Need(((inputCmm).is_Some) or ((inputKeyring).is_Some), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Must provide either a keyring or a CMM")))
        if (d_1_valueOrError1_).IsFailure():
            res = (d_1_valueOrError1_).PropagateFailure()
            return res
        d_2_cmm_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager = None
        if (inputCmm).is_Some:
            res = Wrappers.Result_Success((inputCmm).value)
            return res
        elif True:
            d_3_maybeMaterialsProviders_: Wrappers.Result
            out0_: Wrappers.Result
            out0_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
            d_3_maybeMaterialsProviders_ = out0_
            d_4_valueOrError2_: Wrappers.Result = None
            def lambda0_(d_5_e_):
                return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_5_e_)

            d_4_valueOrError2_ = (d_3_maybeMaterialsProviders_).MapFailure(lambda0_)
            if (d_4_valueOrError2_).IsFailure():
                res = (d_4_valueOrError2_).PropagateFailure()
                return res
            d_6_materialProviders_: MaterialProviders.MaterialProvidersClient
            d_6_materialProviders_ = (d_4_valueOrError2_).Extract()
            d_7_maybeCmm_: Wrappers.Result
            out1_: Wrappers.Result
            out1_ = (d_6_materialProviders_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput((inputKeyring).value))
            d_7_maybeCmm_ = out1_
            def lambda1_(d_8_e_):
                return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_8_e_)

            res = (d_7_maybeCmm_).MapFailure(lambda1_)
            return res
        return res

    @staticmethod
    def ValidateMaxEncryptedDataKeys(maxEncryptedDataKeys, edks):
        if ((maxEncryptedDataKeys).is_Some) and ((len(edks)) > ((maxEncryptedDataKeys).value)):
            return Wrappers.Outcome_Fail(AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Encrypted data keys exceed maxEncryptedDataKeys")))
        elif True:
            return Wrappers.Outcome_Pass()

    @staticmethod
    def GenerateMessageId(suite, crypto):
        res: Wrappers.Result = None
        d_0_maybeId_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        if ((suite).messageVersion) == (1):
            out0_: Wrappers.Result
            out0_ = (crypto).GenerateRandomBytes(AwsCryptographyPrimitivesTypes.GenerateRandomBytesInput_GenerateRandomBytesInput(HeaderTypes.default__.MESSAGE__ID__LEN__V1))
            d_0_maybeId_ = out0_
        elif True:
            out1_: Wrappers.Result
            out1_ = (crypto).GenerateRandomBytes(AwsCryptographyPrimitivesTypes.GenerateRandomBytesInput_GenerateRandomBytesInput(HeaderTypes.default__.MESSAGE__ID__LEN__V2))
            d_0_maybeId_ = out1_
        d_1_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda0_(d_2_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_2_e_)

        d_1_valueOrError0_ = (d_0_maybeId_).MapFailure(lambda0_)
        if (d_1_valueOrError0_).IsFailure():
            res = (d_1_valueOrError0_).PropagateFailure()
            return res
        d_3_id_: _dafny.Seq
        d_3_id_ = (d_1_valueOrError0_).Extract()
        res = Wrappers.Result_Success(d_3_id_)
        return res
        return res

    @staticmethod
    def BuildHeaderForEncrypt(messageId, suite, encryptionContext, requiredEncryptionContextKeys, encryptedDataKeys, frameLength, derivedDataKeys, crypto):
        res: Wrappers.Result = None
        d_0_reqKeySet_: _dafny.Set
        def iife0_():
            coll0_ = _dafny.Set()
            compr_0_: _dafny.Seq
            for compr_0_ in (requiredEncryptionContextKeys).Elements:
                d_1_k_: _dafny.Seq = compr_0_
                if UTF8.ValidUTF8Bytes._Is(d_1_k_):
                    if (d_1_k_) in (requiredEncryptionContextKeys):
                        coll0_ = coll0_.union(_dafny.Set([d_1_k_]))
            return _dafny.Set(coll0_)
        d_0_reqKeySet_ = iife0_()
        
        d_2_storedEncryptionContext_: _dafny.Map
        def iife1_():
            coll1_ = _dafny.Map()
            compr_1_: _dafny.Seq
            for compr_1_ in ((encryptionContext) - (d_0_reqKeySet_)).keys.Elements:
                d_3_f_: _dafny.Seq = compr_1_
                if UTF8.ValidUTF8Bytes._Is(d_3_f_):
                    if (d_3_f_) in ((encryptionContext) - (d_0_reqKeySet_)):
                        coll1_[d_3_f_] = (encryptionContext)[d_3_f_]
            return _dafny.Map(coll1_)
        d_2_storedEncryptionContext_ = iife1_()
        
        d_4_canonicalStoredEncryptionContext_: _dafny.Seq
        d_4_canonicalStoredEncryptionContext_ = EncryptionContext.default__.GetCanonicalEncryptionContext(d_2_storedEncryptionContext_)
        d_5_body_: HeaderTypes.HeaderBody
        out0_: HeaderTypes.HeaderBody
        out0_ = default__.BuildHeaderBody(messageId, suite, d_4_canonicalStoredEncryptionContext_, encryptedDataKeys, frameLength, (derivedDataKeys).commitmentKey)
        d_5_body_ = out0_
        d_6_requiredEncryptionContextMap_: _dafny.Map
        def iife2_():
            coll2_ = _dafny.Map()
            compr_2_: _dafny.Seq
            for compr_2_ in (d_0_reqKeySet_).Elements:
                d_7_r_: _dafny.Seq = compr_2_
                if UTF8.ValidUTF8Bytes._Is(d_7_r_):
                    if (d_7_r_) in (d_0_reqKeySet_):
                        coll2_[d_7_r_] = (encryptionContext)[d_7_r_]
            return _dafny.Map(coll2_)
        d_6_requiredEncryptionContextMap_ = iife2_()
        
        d_8_canonicalReqEncryptionContext_: _dafny.Seq
        d_8_canonicalReqEncryptionContext_ = EncryptionContext.default__.GetCanonicalEncryptionContext(d_6_requiredEncryptionContextMap_)
        d_9_serializedReqEncryptionContext_: _dafny.Seq
        d_9_serializedReqEncryptionContext_ = EncryptionContext.default__.WriteEmptyEcOrWriteAAD(d_8_canonicalReqEncryptionContext_)
        d_10_rawHeader_: _dafny.Seq
        d_10_rawHeader_ = Header.default__.WriteHeaderBody(d_5_body_)
        d_11_valueOrError0_: Wrappers.Result = Wrappers.Result.default(HeaderTypes.HeaderAuth.default())()
        out1_: Wrappers.Result
        out1_ = default__.BuildHeaderAuthTag(suite, (derivedDataKeys).dataKey, d_10_rawHeader_, d_9_serializedReqEncryptionContext_, crypto)
        d_11_valueOrError0_ = out1_
        if (d_11_valueOrError0_).IsFailure():
            res = (d_11_valueOrError0_).PropagateFailure()
            return res
        d_12_headerAuth_: HeaderTypes.HeaderAuth
        d_12_headerAuth_ = (d_11_valueOrError0_).Extract()
        d_13_header_: Header.HeaderInfo
        d_13_header_ = Header.HeaderInfo_HeaderInfo(d_5_body_, d_10_rawHeader_, encryptionContext, suite, d_12_headerAuth_)
        res = Wrappers.Result_Success(d_13_header_)
        return res
        return res

    @staticmethod
    def BuildHeaderBody(messageId, suite, encryptionContext, encryptedDataKeys, frameLength, suiteData):
        res: HeaderTypes.HeaderBody = None
        d_0_contentType_: HeaderTypes.ContentType
        d_0_contentType_ = HeaderTypes.ContentType_Framed()
        source0_ = (suite).commitment
        with _dafny.label("match0"):
            if True:
                if source0_.is_None:
                    res = HeaderTypes.HeaderBody_V1HeaderBody(HeaderTypes.MessageType_TYPE__CUSTOMER__AED(), suite, messageId, encryptionContext, encryptedDataKeys, d_0_contentType_, SerializableTypes.default__.GetIvLength(suite), frameLength)
                    return res
                    raise _dafny.Break("match0")
            if True:
                res = HeaderTypes.HeaderBody_V2HeaderBody(suite, messageId, encryptionContext, encryptedDataKeys, d_0_contentType_, frameLength, (suiteData).value)
                return res
            pass
        return res

    @staticmethod
    def BuildHeaderAuthTag(suite, dataKey, rawHeader, serializedReqEncryptionContext, crypto):
        res: Wrappers.Result = Wrappers.Result.default(HeaderTypes.HeaderAuth.default())()
        d_0_keyLength_: int
        d_0_keyLength_ = SerializableTypes.default__.GetEncryptKeyLength(suite)
        d_1_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_1_valueOrError0_ = Wrappers.default__.Need((len(dataKey)) == (d_0_keyLength_), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Incorrect data key length")))
        if (d_1_valueOrError0_).IsFailure():
            res = (d_1_valueOrError0_).PropagateFailure()
            return res
        d_2_ivLength_: int
        d_2_ivLength_ = SerializableTypes.default__.GetIvLength(suite)
        d_3_iv_: _dafny.Seq
        d_3_iv_ = _dafny.Seq([0 for d_4___v3_ in range(d_2_ivLength_)])
        d_5_maybeEncryptionOutput_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (crypto).AESEncrypt(AwsCryptographyPrimitivesTypes.AESEncryptInput_AESEncryptInput(((suite).encrypt).AES__GCM, d_3_iv_, dataKey, _dafny.Seq([]), (rawHeader) + (serializedReqEncryptionContext)))
        d_5_maybeEncryptionOutput_ = out0_
        d_6_valueOrError1_: Wrappers.Result = Wrappers.Result.default(AwsCryptographyPrimitivesTypes.AESEncryptOutput.default())()
        def lambda0_(d_7_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_7_e_)

        d_6_valueOrError1_ = (d_5_maybeEncryptionOutput_).MapFailure(lambda0_)
        if (d_6_valueOrError1_).IsFailure():
            res = (d_6_valueOrError1_).PropagateFailure()
            return res
        d_8_encryptionOutput_: AwsCryptographyPrimitivesTypes.AESEncryptOutput
        d_8_encryptionOutput_ = (d_6_valueOrError1_).Extract()
        d_9_headerAuth_: HeaderTypes.HeaderAuth
        d_9_headerAuth_ = HeaderTypes.HeaderAuth_AESMac(d_3_iv_, (d_8_encryptionOutput_).authTag)
        res = Wrappers.Result_Success(d_9_headerAuth_)
        return res
        return res

    @staticmethod
    def GetEncryptionMaterials(cmm, algorithmSuiteId, encryptionContext, maxPlaintextLength, commitmentPolicy, mpl):
        res: Wrappers.Result = None
        d_0_encMatRequest_: AwsCryptographyMaterialProvidersTypes.GetEncryptionMaterialsInput
        d_0_encMatRequest_ = AwsCryptographyMaterialProvidersTypes.GetEncryptionMaterialsInput_GetEncryptionMaterialsInput(encryptionContext, AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(commitmentPolicy), algorithmSuiteId, Wrappers.Option_Some(maxPlaintextLength), Wrappers.Option_None())
        d_1_getEncMatResult_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (cmm).GetEncryptionMaterials(d_0_encMatRequest_)
        d_1_getEncMatResult_ = out0_
        d_2_valueOrError0_: Wrappers.Result = None
        def lambda0_(d_3_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_3_e_)

        d_2_valueOrError0_ = (d_1_getEncMatResult_).MapFailure(lambda0_)
        if (d_2_valueOrError0_).IsFailure():
            res = (d_2_valueOrError0_).PropagateFailure()
            return res
        d_4_output_: AwsCryptographyMaterialProvidersTypes.GetEncryptionMaterialsOutput
        d_4_output_ = (d_2_valueOrError0_).Extract()
        d_5_materials_: AwsCryptographyMaterialProvidersTypes.EncryptionMaterials
        d_5_materials_ = (d_4_output_).encryptionMaterials
        d_6_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        def lambda1_(d_7_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_7_e_)

        d_6_valueOrError1_ = ((mpl).ValidateCommitmentPolicyOnEncrypt(AwsCryptographyMaterialProvidersTypes.ValidateCommitmentPolicyOnEncryptInput_ValidateCommitmentPolicyOnEncryptInput(((d_5_materials_).algorithmSuite).id, AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(commitmentPolicy)))).MapFailure(lambda1_)
        if (d_6_valueOrError1_).IsFailure():
            res = (d_6_valueOrError1_).PropagateFailure()
            return res
        d_8___v4_: tuple
        d_8___v4_ = (d_6_valueOrError1_).Extract()
        d_9_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        def lambda2_(d_10_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_10_e_)

        d_9_valueOrError2_ = ((mpl).EncryptionMaterialsHasPlaintextDataKey(d_5_materials_)).MapFailure(lambda2_)
        if (d_9_valueOrError2_).IsFailure():
            res = (d_9_valueOrError2_).PropagateFailure()
            return res
        d_11___v5_: tuple
        d_11___v5_ = (d_9_valueOrError2_).Extract()
        d_12_valueOrError3_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_12_valueOrError3_ = Wrappers.default__.Need(SerializableTypes.default__.IsESDKEncryptionContext((d_5_materials_).encryptionContext), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("CMM failed to return serializable encryption materials.")))
        if (d_12_valueOrError3_).IsFailure():
            res = (d_12_valueOrError3_).PropagateFailure()
            return res
        d_13_valueOrError4_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_13_valueOrError4_ = Wrappers.default__.Need(StandardLibrary_UInt.default__.HasUint16Len((d_5_materials_).encryptedDataKeys), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("CMM returned EDKs that exceed the allowed maximum.")))
        if (d_13_valueOrError4_).IsFailure():
            res = (d_13_valueOrError4_).PropagateFailure()
            return res
        d_14_valueOrError5_: Wrappers.Outcome = Wrappers.Outcome.default()()
        def lambda3_(forall_var_0_):
            d_15_edk_: AwsCryptographyMaterialProvidersTypes.EncryptedDataKey = forall_var_0_
            return not ((d_15_edk_) in ((d_5_materials_).encryptedDataKeys)) or (SerializableTypes.default__.IsESDKEncryptedDataKey(d_15_edk_))

        d_14_valueOrError5_ = Wrappers.default__.Need(_dafny.quantifier(((d_5_materials_).encryptedDataKeys).UniqueElements, True, lambda3_), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("CMM returned non-serializable encrypted data key.")))
        if (d_14_valueOrError5_).IsFailure():
            res = (d_14_valueOrError5_).PropagateFailure()
            return res
        res = Wrappers.Result_Success(d_5_materials_)
        return res
        return res

    @staticmethod
    def GetDecryptionMaterials(cmm, algorithmSuiteId, headerBody, reproducedEncryptionContext, commitmentPolicy, mpl):
        res: Wrappers.Result = None
        d_0_encryptionContext_: _dafny.Map
        d_0_encryptionContext_ = EncryptionContext.default__.GetEncryptionContext((headerBody).encryptionContext)
        d_1_decMatRequest_: AwsCryptographyMaterialProvidersTypes.DecryptMaterialsInput
        d_1_decMatRequest_ = AwsCryptographyMaterialProvidersTypes.DecryptMaterialsInput_DecryptMaterialsInput(algorithmSuiteId, AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(commitmentPolicy), (headerBody).encryptedDataKeys, d_0_encryptionContext_, reproducedEncryptionContext)
        d_2_decMatResult_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (cmm).DecryptMaterials(d_1_decMatRequest_)
        d_2_decMatResult_ = out0_
        d_3_valueOrError0_: Wrappers.Result = None
        def lambda0_(d_4_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_4_e_)

        d_3_valueOrError0_ = (d_2_decMatResult_).MapFailure(lambda0_)
        if (d_3_valueOrError0_).IsFailure():
            res = (d_3_valueOrError0_).PropagateFailure()
            return res
        d_5_output_: AwsCryptographyMaterialProvidersTypes.DecryptMaterialsOutput
        d_5_output_ = (d_3_valueOrError0_).Extract()
        d_6_materials_: AwsCryptographyMaterialProvidersTypes.DecryptionMaterials
        d_6_materials_ = (d_5_output_).decryptionMaterials
        d_7_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        def lambda1_(d_8_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_8_e_)

        d_7_valueOrError1_ = ((mpl).ValidateCommitmentPolicyOnDecrypt(AwsCryptographyMaterialProvidersTypes.ValidateCommitmentPolicyOnDecryptInput_ValidateCommitmentPolicyOnDecryptInput(((d_6_materials_).algorithmSuite).id, AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(commitmentPolicy)))).MapFailure(lambda1_)
        if (d_7_valueOrError1_).IsFailure():
            res = (d_7_valueOrError1_).PropagateFailure()
            return res
        d_9___v6_: tuple
        d_9___v6_ = (d_7_valueOrError1_).Extract()
        d_10_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        def lambda2_(d_11_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_11_e_)

        d_10_valueOrError2_ = ((mpl).DecryptionMaterialsWithPlaintextDataKey(d_6_materials_)).MapFailure(lambda2_)
        if (d_10_valueOrError2_).IsFailure():
            res = (d_10_valueOrError2_).PropagateFailure()
            return res
        d_12___v7_: tuple
        d_12___v7_ = (d_10_valueOrError2_).Extract()
        d_13_valueOrError3_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_13_valueOrError3_ = Wrappers.default__.Need(SerializableTypes.default__.IsESDKEncryptionContext((d_6_materials_).encryptionContext), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("CMM failed to return serializable encryption materials.")))
        if (d_13_valueOrError3_).IsFailure():
            res = (d_13_valueOrError3_).PropagateFailure()
            return res
        res = Wrappers.Result_Success(d_6_materials_)
        return res
        return res

    @staticmethod
    def ValidateSuiteData(suite, header, expectedSuiteData):
        res: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_0_valueOrError0_ = Wrappers.default__.Need((len((header).suiteData)) == ((((suite).commitment).HKDF).outputKeyLength), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Commitment key is invalid")))
        if (d_0_valueOrError0_).IsFailure():
            res = (d_0_valueOrError0_).PropagateFailure()
            return res
        d_1_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_1_valueOrError1_ = Wrappers.default__.Need((expectedSuiteData) == ((header).suiteData), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Commitment key does not match")))
        if (d_1_valueOrError1_).IsFailure():
            res = (d_1_valueOrError1_).PropagateFailure()
            return res
        res = Wrappers.Result_Success(())
        return res
        return res

    @staticmethod
    def ReadAndDecryptFramedMessageBody(buffer, header, key, crypto):
        res: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple(_dafny.Seq, SerializeFunctions.ReadableBuffer.default()))()
        d_0_valueOrError0_: Wrappers.Result = None
        d_0_valueOrError0_ = (MessageBody.default__.ReadFramedMessageBody(buffer, header, _dafny.Seq([]), buffer)).MapFailure(default__.MapSerializeFailure(_dafny.Seq(": ReadFramedMessageBody")))
        if (d_0_valueOrError0_).IsFailure():
            res = (d_0_valueOrError0_).PropagateFailure()
            return res
        d_1_messageBody_: SerializeFunctions.SuccessfulRead
        d_1_messageBody_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out0_: Wrappers.Result
        out0_ = MessageBody.default__.DecryptFramedMessageBody((d_1_messageBody_).data, key, crypto)
        d_2_valueOrError1_ = out0_
        if (d_2_valueOrError1_).IsFailure():
            res = (d_2_valueOrError1_).PropagateFailure()
            return res
        d_3_plaintext_: _dafny.Seq
        d_3_plaintext_ = (d_2_valueOrError1_).Extract()
        d_4_messageBodyTail_: SerializeFunctions.ReadableBuffer
        d_4_messageBodyTail_ = (d_1_messageBody_).tail
        res = Wrappers.Result_Success((d_3_plaintext_, d_4_messageBodyTail_))
        return res
        return res

    @staticmethod
    def ReadAndDecryptNonFramedMessageBody(buffer, header, key, crypto):
        res: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple(_dafny.Seq, SerializeFunctions.ReadableBuffer.default()))()
        d_0_valueOrError0_: Wrappers.Result = None
        d_0_valueOrError0_ = (MessageBody.default__.ReadNonFramedMessageBody(buffer, header)).MapFailure(default__.MapSerializeFailure(_dafny.Seq(": ReadNonFramedMessageBody")))
        if (d_0_valueOrError0_).IsFailure():
            res = (d_0_valueOrError0_).PropagateFailure()
            return res
        d_1_messageBody_: SerializeFunctions.SuccessfulRead
        d_1_messageBody_ = (d_0_valueOrError0_).Extract()
        d_2_frame_: Frames.Frame
        d_2_frame_ = (d_1_messageBody_).data
        d_3_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out0_: Wrappers.Result
        out0_ = MessageBody.default__.DecryptFrame(d_2_frame_, key, crypto)
        d_3_valueOrError1_ = out0_
        if (d_3_valueOrError1_).IsFailure():
            res = (d_3_valueOrError1_).PropagateFailure()
            return res
        d_4_plaintext_: _dafny.Seq
        d_4_plaintext_ = (d_3_valueOrError1_).Extract()
        d_5_messageBodyTail_: SerializeFunctions.ReadableBuffer
        d_5_messageBodyTail_ = (d_1_messageBody_).tail
        res = Wrappers.Result_Success((d_4_plaintext_, d_5_messageBodyTail_))
        return res
        return res

    @_dafny.classproperty
    def DEFAULT__FRAME__LENGTH(instance):
        return 4096
    @_dafny.classproperty
    def RESERVED__ENCRYPTION__CONTEXT(instance):
        d_0_s_ = _dafny.Seq([97, 119, 115, 45, 99, 114, 121, 112, 116, 111, 45])
        return d_0_s_

class FrameLength:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return int(0)
    def _Is(source__):
        d_1_frameLength_: int = source__
        if True:
            return ((0) < (d_1_frameLength_)) and ((d_1_frameLength_) <= (4294967295))
        return False
