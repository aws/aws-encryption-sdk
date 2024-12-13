import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import aws_encryption_sdk_test_vectors.internaldafny.generated.module_ as module_
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
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AwsCryptographyMaterialProvidersTestVectorKeysTypes as AwsCryptographyMaterialProvidersTestVectorKeysTypes
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Views_Core as JSON_Utils_Views_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Views_Writers as JSON_Utils_Views_Writers
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Lexers_Core as JSON_Utils_Lexers_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Lexers_Strings as JSON_Utils_Lexers_Strings
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Cursors as JSON_Utils_Cursors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Parsers as JSON_Utils_Parsers
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
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str_CharStrConversion as JSON_Utils_Str_CharStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str_CharStrEscaping as JSON_Utils_Str_CharStrEscaping
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str as JSON_Utils_Str
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Seq as JSON_Utils_Seq
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Vectors as JSON_Utils_Vectors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Errors as JSON_Errors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Values as JSON_Values
import smithy_dafny_standard_library.internaldafny.generated.Unicode as Unicode
import smithy_dafny_standard_library.internaldafny.generated.Functions as Functions
import smithy_dafny_standard_library.internaldafny.generated.Utf8EncodingForm as Utf8EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.Utf16EncodingForm as Utf16EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.UnicodeStrings as UnicodeStrings
import smithy_dafny_standard_library.internaldafny.generated.JSON_Spec as JSON_Spec
import smithy_dafny_standard_library.internaldafny.generated.JSON_Grammar as JSON_Grammar
import smithy_dafny_standard_library.internaldafny.generated.JSON_Serializer_ByteStrConversion as JSON_Serializer_ByteStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Serializer as JSON_Serializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer_Uint16StrConversion as JSON_Deserializer_Uint16StrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer_ByteStrConversion as JSON_Deserializer_ByteStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer as JSON_Deserializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ConcreteSyntax_Spec as JSON_ConcreteSyntax_Spec
import smithy_dafny_standard_library.internaldafny.generated.JSON_ConcreteSyntax_SpecProperties as JSON_ConcreteSyntax_SpecProperties
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Serializer as JSON_ZeroCopy_Serializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Core as JSON_ZeroCopy_Deserializer_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Strings as JSON_ZeroCopy_Deserializer_Strings
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Numbers as JSON_ZeroCopy_Deserializer_Numbers
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_ObjectParams as JSON_ZeroCopy_Deserializer_ObjectParams
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Objects as JSON_ZeroCopy_Deserializer_Objects
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_ArrayParams as JSON_ZeroCopy_Deserializer_ArrayParams
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Arrays as JSON_ZeroCopy_Deserializer_Arrays
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Constants as JSON_ZeroCopy_Deserializer_Constants
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Values as JSON_ZeroCopy_Deserializer_Values
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_API as JSON_ZeroCopy_Deserializer_API
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer as JSON_ZeroCopy_Deserializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_API as JSON_ZeroCopy_API
import smithy_dafny_standard_library.internaldafny.generated.JSON_API as JSON_API
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.JSONHelpers as JSONHelpers
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.KeyDescription as KeyDescription
import smithy_dafny_standard_library.internaldafny.generated.HexStrings as HexStrings
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.KeyMaterial as KeyMaterial
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.CreateStaticKeyrings as CreateStaticKeyrings
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.CreateStaticKeyStores as CreateStaticKeyStores
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.KeyringFromKeyDescription as KeyringFromKeyDescription
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.CmmFromKeyDescription as CmmFromKeyDescription
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.WrappedMaterialProviders as WrappedMaterialProviders
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.KeysVectorOperations as KeysVectorOperations
import smithy_dafny_standard_library.internaldafny.generated.FileIO as FileIO
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.KeyVectors as KeyVectors
import aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import smithy_dafny_standard_library.internaldafny.generated.Streams as Streams
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
import aws_encryption_sdk.internaldafny.generated.EncryptDecryptHelpers as EncryptDecryptHelpers
import aws_encryption_sdk.internaldafny.generated.AwsEncryptionSdkOperations as AwsEncryptionSdkOperations
import aws_encryption_sdk.internaldafny.generated.ESDK as ESDK
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.MplManifestOptions as MplManifestOptions
import smithy_dafny_standard_library.internaldafny.generated.GetOpt as GetOpt
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllAlgorithmSuites as AllAlgorithmSuites
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.TestVectors as TestVectors
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllHierarchy as AllHierarchy
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllKms as AllKms
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllKmsMrkAware as AllKmsMrkAware
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllKmsMrkAwareDiscovery as AllKmsMrkAwareDiscovery
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllKmsRsa as AllKmsRsa
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllKmsEcdh as AllKmsEcdh
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllRawAES as AllRawAES
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllRawRSA as AllRawRSA
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllRawECDH as AllRawECDH
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllDefaultCmm as AllDefaultCmm
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllRequiredEncryptionContextCmm as AllRequiredEncryptionContextCmm
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.AllMulti as AllMulti
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.WriteJsonManifests as WriteJsonManifests
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.CompleteVectors as CompleteVectors
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.ParseJsonManifests as ParseJsonManifests
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.TestManifests as TestManifests
import aws_cryptography_materialproviders_test_vectors.internaldafny.generated.WrappedMaterialProvidersMain as WrappedMaterialProvidersMain
import aws_cryptography_primitives.internaldafny.generated.AesKdfCtr as AesKdfCtr
import smithy_dafny_standard_library.internaldafny.generated.StandardLibraryInterop as StandardLibraryInterop
import smithy_dafny_standard_library.internaldafny.generated.Sorting as Sorting
import smithy_dafny_standard_library.internaldafny.generated.FloatCompare as FloatCompare
import smithy_dafny_standard_library.internaldafny.generated.ConcurrentCall as ConcurrentCall
import smithy_dafny_standard_library.internaldafny.generated.Base64Lemmas as Base64Lemmas
import aws_encryption_sdk_test_vectors.internaldafny.generated.WrappedESDK as WrappedESDK
import aws_encryption_sdk_test_vectors.internaldafny.generated.EsdkManifestOptions as EsdkManifestOptions
import aws_encryption_sdk_test_vectors.internaldafny.generated.EsdkTestVectors as EsdkTestVectors
import aws_encryption_sdk_test_vectors.internaldafny.generated.AllEsdkV4NoReqEc as AllEsdkV4NoReqEc
import aws_encryption_sdk_test_vectors.internaldafny.generated.AllEsdkV4WithReqEc as AllEsdkV4WithReqEc

# Module: WriteEsdkJsonManifests

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def EncryptionContextKeysToJson(keys):
        if (keys).is_Some:
            def lambda0_(d_1_bytes_):
                def iife0_(_pat_let0_0):
                    def iife1_(d_2_valueOrError1_):
                        def iife2_(_pat_let1_0):
                            def iife3_(d_3_key_):
                                return Wrappers.Result_Success(JSON_Values.JSON_String(d_3_key_))
                            return iife3_(_pat_let1_0)
                        return ((d_2_valueOrError1_).PropagateFailure() if (d_2_valueOrError1_).IsFailure() else iife2_((d_2_valueOrError1_).Extract()))
                    return iife1_(_pat_let0_0)
                return iife0_(UTF8.default__.Decode(d_1_bytes_))

            d_0_valueOrError0_ = Seq.default__.MapWithResult(lambda0_, (keys).value)
            if (d_0_valueOrError0_).IsFailure():
                return (d_0_valueOrError0_).PropagateFailure()
            elif True:
                d_4_tmp_ = (d_0_valueOrError0_).Extract()
                return Wrappers.Result_Success(_dafny.Seq([(_dafny.Seq("requiredEncryptionContextKeys"), JSON_Values.JSON_Array(d_4_tmp_))]))
        elif True:
            return Wrappers.Result_Success(_dafny.Seq([]))

    @staticmethod
    def EncryptionContextToJson(key, m):
        def lambda0_(d_1_a_, d_2_b_):
            return (d_1_a_) < (d_2_b_)

        d_0_keys_ = SortedSets.default__.SetToOrderedSequence2((m).keys, lambda0_)
        def lambda1_(d_4_m_):
            def lambda2_(d_5_k_):
                def iife0_(_pat_let2_0):
                    def iife1_(d_6_valueOrError1_):
                        def iife2_(_pat_let3_0):
                            def iife3_(d_7_key_):
                                def iife4_(_pat_let4_0):
                                    def iife5_(d_8_valueOrError2_):
                                        def iife6_(_pat_let5_0):
                                            def iife7_(d_9_value_):
                                                return Wrappers.Result_Success((d_7_key_, JSON_Values.JSON_String(d_9_value_)))
                                            return iife7_(_pat_let5_0)
                                        return ((d_8_valueOrError2_).PropagateFailure() if (d_8_valueOrError2_).IsFailure() else iife6_((d_8_valueOrError2_).Extract()))
                                    return iife5_(_pat_let4_0)
                                return iife4_(UTF8.default__.Decode((d_4_m_)[d_5_k_]))
                            return iife3_(_pat_let3_0)
                        return ((d_6_valueOrError1_).PropagateFailure() if (d_6_valueOrError1_).IsFailure() else iife2_((d_6_valueOrError1_).Extract()))
                    return iife1_(_pat_let2_0)
                return iife0_(UTF8.default__.Decode(d_5_k_))

            return lambda2_

        d_3_valueOrError0_ = Seq.default__.MapWithResult(lambda1_(m), d_0_keys_)
        if (d_3_valueOrError0_).IsFailure():
            return (d_3_valueOrError0_).PropagateFailure()
        elif True:
            d_10_pairsBytes_ = (d_3_valueOrError0_).Extract()
            return Wrappers.Result_Success(_dafny.Seq([(key, JSON_Values.JSON_Object(d_10_pairsBytes_))]))

    @staticmethod
    def printJson(j):
        hresult_: tuple = ()
        _dafny.print(_dafny.string_of(j))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        hresult_ = ()
        return hresult_
        return hresult_

    @staticmethod
    def EncryptTestVectorToJson(test):
        d_0_valueOrError0_ = Wrappers.default__.Need((((test).algorithmSuiteId).is_Some) and (((test).frameLength).is_Some), _dafny.Seq("test is missing algorithmSuite ID, or frameLength"))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_id_ = AllAlgorithmSuites.default__.ToHex(((test).algorithmSuiteId).value)
            d_2_valueOrError1_ = (default__.EncryptionContextToJson(_dafny.Seq("encryption-context"), ((test).encryptionContext).value) if ((test).encryptionContext).is_Some else default__.EncryptionContextToJson(_dafny.Seq("encryption-context"), _dafny.Map({})))
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_encryptionContext_ = (d_2_valueOrError1_).Extract()
                d_4_valueOrError2_ = Wrappers.default__.Need((len(d_3_encryptionContext_)) == (1), _dafny.Seq("Error parsing encryption context"))
                if (d_4_valueOrError2_).IsFailure():
                    return (d_4_valueOrError2_).PropagateFailure()
                elif True:
                    d_5_valueOrError3_ = (default__.EncryptionContextToJson(_dafny.Seq("reproduced-encryption-context"), ((test).reproducedEncryptionContext).value) if ((test).reproducedEncryptionContext).is_Some else default__.EncryptionContextToJson(_dafny.Seq("reproduced-encryption-context"), _dafny.Map({})))
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_reproducedEncryptionContext_ = (d_5_valueOrError3_).Extract()
                        d_7_optionalValues_ = (d_3_encryptionContext_) + (d_6_reproducedEncryptionContext_)
                        source0_ = test
                        if True:
                            if source0_.is_PositiveEncryptTestVector:
                                d_8_valueOrError4_ = KeyDescription.default__.ToJson((test).encryptDescriptions, 3)
                                if (d_8_valueOrError4_).IsFailure():
                                    return (d_8_valueOrError4_).PropagateFailure()
                                elif True:
                                    d_9_encrypt_ = (d_8_valueOrError4_).Extract()
                                    d_10_valueOrError5_ = KeyDescription.default__.ToJson((test).decryptDescriptions, 3)
                                    if (d_10_valueOrError5_).IsFailure():
                                        return (d_10_valueOrError5_).PropagateFailure()
                                    elif True:
                                        d_11_decrypt_ = (d_10_valueOrError5_).Extract()
                                        d_12_scenario_ = JSON_Values.JSON_Object((_dafny.Seq([(_dafny.Seq("type"), JSON_Values.JSON_String(_dafny.Seq("positive-esdk"))), (_dafny.Seq("plaintext"), JSON_Values.JSON_String(_dafny.Seq("small"))), (_dafny.Seq("description"), JSON_Values.JSON_String((test).description)), (_dafny.Seq("algorithmSuiteId"), JSON_Values.JSON_String(d_1_id_)), (_dafny.Seq("frame-size"), JSON_Values.JSON_Number(JSON_Values.default__.Int(((test).frameLength).value))), (_dafny.Seq("encryptKeyDescription"), d_9_encrypt_), (_dafny.Seq("decryptKeyDescription"), d_11_decrypt_)])) + (d_7_optionalValues_))
                                        return Wrappers.Result_Success(JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("encryption-scenario"), d_12_scenario_)])))
                        if True:
                            return Wrappers.Result_Failure(_dafny.Seq("Only Positive Tests supported :("))

    @staticmethod
    def OptionalBytes(key, secret):
        if (secret).is_Some:
            d_0_base64_ = Base64.default__.Encode((secret).value)
            return _dafny.Seq([(key, JSON_Values.JSON_String(d_0_base64_))])
        elif True:
            return _dafny.Seq([])

    @staticmethod
    def DecryptTestVectorToJson(test):
        d_0_valueOrError0_ = Wrappers.default__.Need((((test).algorithmSuiteId).is_Some) and (((test).frameLength).is_Some), _dafny.Seq("test is missing algorithmSuite ID, or frameLength"))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_id_ = AllAlgorithmSuites.default__.ToHex(((test).algorithmSuiteId).value)
            d_2_description_ = (((test).description) + (_dafny.Seq(" "))) + (d_1_id_)
            d_3_valueOrError1_ = (default__.EncryptionContextToJson(_dafny.Seq("reproduced-encryption-context"), ((test).reproducedEncryptionContext).value) if ((test).reproducedEncryptionContext).is_Some else default__.EncryptionContextToJson(_dafny.Seq("reproduced-encryption-context"), _dafny.Map({})))
            if (d_3_valueOrError1_).IsFailure():
                return (d_3_valueOrError1_).PropagateFailure()
            elif True:
                d_4_reproducedEncryptionContext_ = (d_3_valueOrError1_).Extract()
                d_5_valueOrError2_ = Wrappers.default__.Need((len(d_4_reproducedEncryptionContext_)) == (1), _dafny.Seq("Error parsing encryption context"))
                if (d_5_valueOrError2_).IsFailure():
                    return (d_5_valueOrError2_).PropagateFailure()
                elif True:
                    d_6_optionalValues_ = d_4_reproducedEncryptionContext_
                    source0_ = test
                    if True:
                        if source0_.is_PositiveDecryptTestVector:
                            d_7_valueOrError3_ = KeyDescription.default__.ToJson((test).decryptDescriptions, 3)
                            if (d_7_valueOrError3_).IsFailure():
                                return (d_7_valueOrError3_).PropagateFailure()
                            elif True:
                                d_8_decrypt_ = (d_7_valueOrError3_).Extract()
                                d_9_scenario_ = JSON_Values.JSON_Object((_dafny.Seq([(_dafny.Seq("type"), JSON_Values.JSON_String(_dafny.Seq("positive-esdk"))), (_dafny.Seq("ciphertext"), JSON_Values.JSON_String((_dafny.Seq("file://ciphertexts/")) + ((test).id))), (_dafny.Seq("result"), JSON_Values.JSON_String((_dafny.Seq("file://")) + ((test).plaintextPath))), (_dafny.Seq("algorithmSuiteId"), JSON_Values.JSON_String(d_1_id_)), (_dafny.Seq("frame-size"), JSON_Values.JSON_Number(JSON_Values.default__.Int(((test).frameLength).value))), (_dafny.Seq("description"), JSON_Values.JSON_String((test).description)), (_dafny.Seq("decryptKeyDescription"), d_8_decrypt_)])) + (d_6_optionalValues_))
                                return Wrappers.Result_Success(JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("decryption-scenario"), d_9_scenario_)])))
                    if True:
                        return Wrappers.Result_Failure(_dafny.Seq("Only Positive Tests supported :("))

