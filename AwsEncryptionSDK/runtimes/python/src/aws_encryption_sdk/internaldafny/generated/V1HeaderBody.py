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

# Module: V1HeaderBody

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def WriteV1HeaderBody(body):
        return (((((((((SharedHeaderFunctions.default__.WriteMessageFormatVersion(HeaderTypes.MessageFormatVersion_V1())) + (default__.WriteV1MessageType((body).messageType))) + (SharedHeaderFunctions.default__.WriteESDKSuiteId((body).algorithmSuite))) + (SharedHeaderFunctions.default__.WriteMessageId((body).messageId))) + (EncryptionContext.default__.WriteAADSection((body).encryptionContext))) + (EncryptedDataKeys.default__.WriteEncryptedDataKeysSection((body).encryptedDataKeys))) + (SharedHeaderFunctions.default__.WriteContentType((body).contentType))) + (default__.WriteV1ReservedBytes(default__.RESERVED__BYTES))) + (default__.WriteV1HeaderIvLength(SerializableTypes.default__.GetIvLength((body).algorithmSuite)))) + (SerializeFunctions.default__.WriteUint32((body).frameLength))

    @staticmethod
    def ReadV1HeaderBody(buffer, maxEdks, mpl):
        d_0_valueOrError0_ = SharedHeaderFunctions.default__.ReadMessageFormatVersion(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_version_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = Wrappers.default__.Need(((d_1_version_).data).is_V1, SerializeFunctions.ReadProblems_Error(_dafny.Seq("Message version must be version 1.")))
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_valueOrError2_ = default__.ReadV1MessageType((d_1_version_).tail)
                if (d_3_valueOrError2_).IsFailure():
                    return (d_3_valueOrError2_).PropagateFailure()
                elif True:
                    d_4_messageType_ = (d_3_valueOrError2_).Extract()
                    d_5_valueOrError3_ = SharedHeaderFunctions.default__.ReadESDKSuiteId((d_4_messageType_).tail, mpl)
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_suite_ = (d_5_valueOrError3_).Extract()
                        d_7_valueOrError4_ = Wrappers.default__.Need((((d_6_suite_).data).commitment).is_None, SerializeFunctions.ReadProblems_Error(_dafny.Seq("Algorithm suite must not support commitment.")))
                        if (d_7_valueOrError4_).IsFailure():
                            return (d_7_valueOrError4_).PropagateFailure()
                        elif True:
                            d_8_valueOrError5_ = SharedHeaderFunctions.default__.ReadMessageIdV1((d_6_suite_).tail)
                            if (d_8_valueOrError5_).IsFailure():
                                return (d_8_valueOrError5_).PropagateFailure()
                            elif True:
                                d_9_messageId_ = (d_8_valueOrError5_).Extract()
                                d_10_valueOrError6_ = EncryptionContext.default__.ReadAADSection((d_9_messageId_).tail)
                                if (d_10_valueOrError6_).IsFailure():
                                    return (d_10_valueOrError6_).PropagateFailure()
                                elif True:
                                    d_11_encryptionContext_ = (d_10_valueOrError6_).Extract()
                                    d_12_valueOrError7_ = EncryptedDataKeys.default__.ReadEncryptedDataKeysSection((d_11_encryptionContext_).tail, maxEdks)
                                    if (d_12_valueOrError7_).IsFailure():
                                        return (d_12_valueOrError7_).PropagateFailure()
                                    elif True:
                                        d_13_encryptedDataKeys_ = (d_12_valueOrError7_).Extract()
                                        d_14_valueOrError8_ = SharedHeaderFunctions.default__.ReadContentType((d_13_encryptedDataKeys_).tail)
                                        if (d_14_valueOrError8_).IsFailure():
                                            return (d_14_valueOrError8_).PropagateFailure()
                                        elif True:
                                            d_15_contentType_ = (d_14_valueOrError8_).Extract()
                                            d_16_valueOrError9_ = default__.ReadV1ReservedBytes((d_15_contentType_).tail)
                                            if (d_16_valueOrError9_).IsFailure():
                                                return (d_16_valueOrError9_).PropagateFailure()
                                            elif True:
                                                d_17_reservedBytes_ = (d_16_valueOrError9_).Extract()
                                                d_18_valueOrError10_ = default__.ReadV1HeaderIvLength((d_17_reservedBytes_).tail, (d_6_suite_).data)
                                                if (d_18_valueOrError10_).IsFailure():
                                                    return (d_18_valueOrError10_).PropagateFailure()
                                                elif True:
                                                    d_19_headerIvLength_ = (d_18_valueOrError10_).Extract()
                                                    d_20_valueOrError11_ = SerializeFunctions.default__.ReadUInt32((d_19_headerIvLength_).tail)
                                                    if (d_20_valueOrError11_).IsFailure():
                                                        return (d_20_valueOrError11_).PropagateFailure()
                                                    elif True:
                                                        d_21_frameLength_ = (d_20_valueOrError11_).Extract()
                                                        d_22_body_ = HeaderTypes.HeaderBody_V1HeaderBody((d_4_messageType_).data, (d_6_suite_).data, (d_9_messageId_).data, (d_11_encryptionContext_).data, (d_13_encryptedDataKeys_).data, (d_15_contentType_).data, (d_19_headerIvLength_).data, (d_21_frameLength_).data)
                                                        return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_22_body_, (d_21_frameLength_).tail))

    @staticmethod
    def WriteV1MessageType(messageType):
        return _dafny.Seq([(messageType).Serialize()])

    @staticmethod
    def ReadV1MessageType(buffer):
        d_0_valueOrError0_ = SerializeFunctions.default__.Read(buffer, 1)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_raw_ = let_tmp_rhs0_.data
            d_2_tail_ = let_tmp_rhs0_.tail
            def lambda0_(d_4_e_):
                return SerializeFunctions.ReadProblems_Error(d_4_e_)

            d_3_valueOrError1_ = (HeaderTypes.MessageType.Get((d_1_raw_)[0])).MapFailure(lambda0_)
            if (d_3_valueOrError1_).IsFailure():
                return (d_3_valueOrError1_).PropagateFailure()
            elif True:
                d_5_messageType_ = (d_3_valueOrError1_).Extract()
                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_5_messageType_, d_2_tail_))

    @staticmethod
    def WriteV1ReservedBytes(reservedBytes):
        return reservedBytes

    @staticmethod
    def ReadV1ReservedBytes(buffer):
        d_0_valueOrError0_ = SerializeFunctions.default__.Read(buffer, len(default__.RESERVED__BYTES))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_raw_ = let_tmp_rhs0_.data
            d_2_tail_ = let_tmp_rhs0_.tail
            d_3_valueOrError1_ = Wrappers.default__.Need((d_1_raw_) == (default__.RESERVED__BYTES), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Incorrect reserved bytes.")))
            if (d_3_valueOrError1_).IsFailure():
                return (d_3_valueOrError1_).PropagateFailure()
            elif True:
                d_4_reservedBytes_ = d_1_raw_
                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_4_reservedBytes_, d_2_tail_))

    @staticmethod
    def WriteV1HeaderIvLength(ivLength):
        return _dafny.Seq([ivLength])

    @staticmethod
    def ReadV1HeaderIvLength(buffer, suite):
        d_0_valueOrError0_ = SerializeFunctions.default__.Read(buffer, 1)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_raw_ = let_tmp_rhs0_.data
            d_2_tail_ = let_tmp_rhs0_.tail
            d_3_valueOrError1_ = Wrappers.default__.Need(((d_1_raw_)[0]) == (SerializableTypes.default__.GetIvLength(suite)), SerializeFunctions.ReadProblems_Error(_dafny.Seq("HeaderIv Length does not match Algorithm Suite.")))
            if (d_3_valueOrError1_).IsFailure():
                return (d_3_valueOrError1_).PropagateFailure()
            elif True:
                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead((d_1_raw_)[0], d_2_tail_))

    @_dafny.classproperty
    def RESERVED__BYTES(instance):
        return _dafny.Seq([0, 0, 0, 0])

class V1HeaderBody:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return HeaderTypes.HeaderBody.default()()
    def _Is(source__):
        d_0_h_: HeaderTypes.HeaderBody = source__
        return (d_0_h_).is_V1HeaderBody

class ReservedBytes:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return default__.RESERVED__BYTES
    def _Is(source__):
        d_1_s_: _dafny.Seq = source__
        return (d_1_s_) == (default__.RESERVED__BYTES)
