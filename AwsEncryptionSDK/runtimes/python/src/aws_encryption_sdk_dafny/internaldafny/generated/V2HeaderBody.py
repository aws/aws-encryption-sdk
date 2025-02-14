import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import aws_encryption_sdk_dafny.internaldafny.generated.module_ as module_
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
import smithy_dafny_standard_library.internaldafny.generated.OsLang as OsLang
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
import aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptionContext as EncryptionContext
import aws_encryption_sdk_dafny.internaldafny.generated.HeaderTypes as HeaderTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SharedHeaderFunctions as SharedHeaderFunctions
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptedDataKeys as EncryptedDataKeys
import aws_encryption_sdk_dafny.internaldafny.generated.V1HeaderBody as V1HeaderBody

# Module: V2HeaderBody

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def WriteV2HeaderBody(body):
        return (((((((SharedHeaderFunctions.default__.WriteMessageFormatVersion(HeaderTypes.MessageFormatVersion_V2())) + (SharedHeaderFunctions.default__.WriteESDKSuiteId((body).algorithmSuite))) + (SharedHeaderFunctions.default__.WriteMessageId((body).messageId))) + (EncryptionContext.default__.WriteAADSection((body).encryptionContext))) + (EncryptedDataKeys.default__.WriteEncryptedDataKeysSection((body).encryptedDataKeys))) + (SharedHeaderFunctions.default__.WriteContentType((body).contentType))) + (SerializeFunctions.default__.WriteUint32((body).frameLength))) + (SerializeFunctions.default__.Write((body).suiteData))

    @staticmethod
    def ReadV2HeaderBody(buffer, maxEdks, mpl):
        d_0_valueOrError0_ = SharedHeaderFunctions.default__.ReadMessageFormatVersion(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_version_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = Wrappers.default__.Need(((d_1_version_).data).is_V2, SerializeFunctions.ReadProblems_Error(_dafny.Seq("Message version must be version 2.")))
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_valueOrError2_ = SharedHeaderFunctions.default__.ReadESDKSuiteId((d_1_version_).tail, mpl)
                if (d_3_valueOrError2_).IsFailure():
                    return (d_3_valueOrError2_).PropagateFailure()
                elif True:
                    d_4_suite_ = (d_3_valueOrError2_).Extract()
                    d_5_valueOrError3_ = Wrappers.default__.Need((((d_4_suite_).data).commitment).is_HKDF, SerializeFunctions.ReadProblems_Error(_dafny.Seq("Algorithm suite must support commitment.")))
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_valueOrError4_ = SharedHeaderFunctions.default__.ReadMessageIdV2((d_4_suite_).tail)
                        if (d_6_valueOrError4_).IsFailure():
                            return (d_6_valueOrError4_).PropagateFailure()
                        elif True:
                            d_7_messageId_ = (d_6_valueOrError4_).Extract()
                            d_8_valueOrError5_ = EncryptionContext.default__.ReadAADSection((d_7_messageId_).tail)
                            if (d_8_valueOrError5_).IsFailure():
                                return (d_8_valueOrError5_).PropagateFailure()
                            elif True:
                                d_9_encryptionContext_ = (d_8_valueOrError5_).Extract()
                                d_10_valueOrError6_ = EncryptedDataKeys.default__.ReadEncryptedDataKeysSection((d_9_encryptionContext_).tail, maxEdks)
                                if (d_10_valueOrError6_).IsFailure():
                                    return (d_10_valueOrError6_).PropagateFailure()
                                elif True:
                                    d_11_encryptedDataKeys_ = (d_10_valueOrError6_).Extract()
                                    d_12_valueOrError7_ = SharedHeaderFunctions.default__.ReadContentType((d_11_encryptedDataKeys_).tail)
                                    if (d_12_valueOrError7_).IsFailure():
                                        return (d_12_valueOrError7_).PropagateFailure()
                                    elif True:
                                        d_13_contentType_ = (d_12_valueOrError7_).Extract()
                                        d_14_valueOrError8_ = SerializeFunctions.default__.ReadUInt32((d_13_contentType_).tail)
                                        if (d_14_valueOrError8_).IsFailure():
                                            return (d_14_valueOrError8_).PropagateFailure()
                                        elif True:
                                            d_15_frameLength_ = (d_14_valueOrError8_).Extract()
                                            d_16_valueOrError9_ = SerializeFunctions.default__.Read((d_15_frameLength_).tail, ((((d_4_suite_).data).commitment).HKDF).outputKeyLength)
                                            if (d_16_valueOrError9_).IsFailure():
                                                return (d_16_valueOrError9_).PropagateFailure()
                                            elif True:
                                                d_17_suiteData_ = (d_16_valueOrError9_).Extract()
                                                d_18_body_ = HeaderTypes.HeaderBody_V2HeaderBody((d_4_suite_).data, (d_7_messageId_).data, (d_9_encryptionContext_).data, (d_11_encryptedDataKeys_).data, (d_13_contentType_).data, (d_15_frameLength_).data, (d_17_suiteData_).data)
                                                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_18_body_, (d_17_suiteData_).tail))

    @_dafny.classproperty
    def headerBytesToAADStart(instance):
        return ((1) + (2)) + (32)

class V2HeaderBody:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return HeaderTypes.HeaderBody.default()()
    def _Is(source__):
        d_0_h_: HeaderTypes.HeaderBody = source__
        return (d_0_h_).is_V2HeaderBody
