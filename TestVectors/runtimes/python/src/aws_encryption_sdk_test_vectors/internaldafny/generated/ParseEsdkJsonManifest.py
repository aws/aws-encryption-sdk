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
import aws_encryption_sdk.internaldafny.generated.EncryptionSdk as EncryptionSdk
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
import aws_encryption_sdk_test_vectors.internaldafny.generated.WriteEsdkJsonManifests as WriteEsdkJsonManifests
import aws_encryption_sdk_test_vectors.internaldafny.generated.WriteVectors as WriteVectors

# Module: ParseEsdkJsonManifest

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def BuildDecryptTestVector(op, version, keys, obj):
        hresult_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_i_: int
        d_0_i_ = len(obj)
        d_1_vectors_: _dafny.Seq
        d_1_vectors_ = _dafny.Seq([])
        while (d_0_i_) != (0):
            d_0_i_ = (d_0_i_) - (1)
            d_2_test_: Wrappers.Result
            d_2_test_ = default__.ToDecryptTestVectors(op, version, keys, ((obj)[d_0_i_])[0], ((obj)[d_0_i_])[1])
            if ((d_2_test_).is_Failure) and (((d_2_test_).error) != (default__.negativeTestVectorFound)):
                hresult_ = Wrappers.Result_Failure(default__.buildTestVectorError)
                return hresult_
            if (d_2_test_).is_Success:
                d_1_vectors_ = (_dafny.Seq([(d_2_test_).value])) + (d_1_vectors_)
            if ((d_2_test_).is_Failure) and (((d_2_test_).error) == (default__.negativeTestVectorFound)):
                d_1_vectors_ = d_1_vectors_
        hresult_ = Wrappers.Result_Success(d_1_vectors_)
        return hresult_
        return hresult_

    @staticmethod
    def ToDecryptTestVectors(op, version, keys, name, json):
        d_0_valueOrError0_ = Wrappers.default__.Need((json).is_Object, _dafny.Seq("Vector is not an object"))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_obj_ = (json).obj
            source0_ = version
            if True:
                if (source0_) == (3):
                    d_2_valueOrError1_ = Wrappers.default__.Need((op).is_Decrypt, _dafny.Seq("Err parsing manifest expected Decrypt"))
                    if (d_2_valueOrError1_).IsFailure():
                        return (d_2_valueOrError1_).PropagateFailure()
                    elif True:
                        return default__.V3ToDecryptTestVector(op, keys, name, d_1_obj_, version)
            if True:
                if (source0_) == (2):
                    d_3_valueOrError2_ = Wrappers.default__.Need((op).is_Decrypt, _dafny.Seq("Err parsing manifest expected Decrypt"))
                    if (d_3_valueOrError2_).IsFailure():
                        return (d_3_valueOrError2_).PropagateFailure()
                    elif True:
                        return default__.V2ToDecryptTestVector(op, keys, name, d_1_obj_, version)
            if True:
                if (source0_) == (1):
                    d_4_valueOrError3_ = Wrappers.default__.Need((op).is_Decrypt, _dafny.Seq("Err parsing manifest expected Decrypt"))
                    if (d_4_valueOrError3_).IsFailure():
                        return (d_4_valueOrError3_).PropagateFailure()
                    elif True:
                        return default__.V1ToDecryptTestVector(op, keys, name, d_1_obj_, version)
            if True:
                return Wrappers.Result_Failure(_dafny.Seq("Version not supported\n"))

    @staticmethod
    def BuildEncryptTestVector(op, version, keys, obj):
        hresult_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_i_: int
        d_0_i_ = len(obj)
        d_1_vectors_: _dafny.Seq
        d_1_vectors_ = _dafny.Seq([])
        while (d_0_i_) != (0):
            d_0_i_ = (d_0_i_) - (1)
            d_2_test_: Wrappers.Result
            d_2_test_ = default__.ToEncryptTestVector(op, version, keys, ((obj)[d_0_i_])[0], ((obj)[d_0_i_])[1])
            if (d_2_test_).is_Failure:
                hresult_ = Wrappers.Result_Failure((d_2_test_).error)
                return hresult_
            d_1_vectors_ = (_dafny.Seq([(d_2_test_).value])) + (d_1_vectors_)
        hresult_ = Wrappers.Result_Success(d_1_vectors_)
        return hresult_
        return hresult_

    @staticmethod
    def ToEncryptTestVector(op, version, keys, name, json):
        d_0_valueOrError0_ = Wrappers.default__.Need((json).is_Object, _dafny.Seq("EncryptTestVector not an object"))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_obj_ = (json).obj
            source0_ = version
            if True:
                if (source0_) == (5):
                    return default__.V5ToEncryptTestVector(op, keys, name, d_1_obj_, version)
            if True:
                return Wrappers.Result_Failure(_dafny.Seq("Version not supported"))

    @staticmethod
    def V5ToEncryptTestVector(op, keys, name, obj, version):
        d_0_scenarioString_ = _dafny.Seq("encryption-scenario")
        d_1_valueOrError0_ = JSONHelpers.default__.GetObject(d_0_scenarioString_, obj)
        if (d_1_valueOrError0_).IsFailure():
            return (d_1_valueOrError0_).PropagateFailure()
        elif True:
            d_2_scenario_ = (d_1_valueOrError0_).Extract()
            d_3_typeString_ = _dafny.Seq("type")
            d_4_valueOrError1_ = JSONHelpers.default__.GetString(d_3_typeString_, d_2_scenario_)
            if (d_4_valueOrError1_).IsFailure():
                return (d_4_valueOrError1_).PropagateFailure()
            elif True:
                d_5_typ_ = (d_4_valueOrError1_).Extract()
                d_6_valueOrError2_ = JSONHelpers.default__.GetString(default__.plaintextJsonKey, d_2_scenario_)
                if (d_6_valueOrError2_).IsFailure():
                    return (d_6_valueOrError2_).PropagateFailure()
                elif True:
                    d_7_plaintextLoc_ = (d_6_valueOrError2_).Extract()
                    d_8_valueOrError3_ = ParseJsonManifests.default__.GetAlgorithmSuiteInfo(d_2_scenario_)
                    if (d_8_valueOrError3_).IsFailure():
                        return (d_8_valueOrError3_).PropagateFailure()
                    elif True:
                        d_9_algorithmSuite_ = (d_8_valueOrError3_).Extract()
                        d_10_valueOrError4_ = Wrappers.default__.Need(((d_9_algorithmSuite_).id).is_ESDK, _dafny.Seq("Unsupported algorithmSuiteId"))
                        if (d_10_valueOrError4_).IsFailure():
                            return (d_10_valueOrError4_).PropagateFailure()
                        elif True:
                            d_11_valueOrError5_ = JSONHelpers.default__.GetOptionalPositiveLong(default__.frameSizeJsonKey, d_2_scenario_)
                            if (d_11_valueOrError5_).IsFailure():
                                return (d_11_valueOrError5_).PropagateFailure()
                            elif True:
                                d_12_frameLength_ = (d_11_valueOrError5_).Extract()
                                d_13_valueOrError6_ = JSONHelpers.default__.SmallObjectToStringStringMap(default__.encryptionContextJsonKey, d_2_scenario_)
                                if (d_13_valueOrError6_).IsFailure():
                                    return (d_13_valueOrError6_).PropagateFailure()
                                elif True:
                                    d_14_encryptionContextStrings_ = (d_13_valueOrError6_).Extract()
                                    d_15_valueOrError7_ = JSONHelpers.default__.utf8EncodeMap(d_14_encryptionContextStrings_)
                                    if (d_15_valueOrError7_).IsFailure():
                                        return (d_15_valueOrError7_).PropagateFailure()
                                    elif True:
                                        d_16_encryptionContext_ = (d_15_valueOrError7_).Extract()
                                        d_17_valueOrError8_ = JSONHelpers.default__.SmallObjectToStringStringMap(default__.reproducedEncryptionContextJsonKey, d_2_scenario_)
                                        if (d_17_valueOrError8_).IsFailure():
                                            return (d_17_valueOrError8_).PropagateFailure()
                                        elif True:
                                            d_18_reproducedEncryptionContextString_ = (d_17_valueOrError8_).Extract()
                                            d_19_valueOrError9_ = JSONHelpers.default__.utf8EncodeMap(d_18_reproducedEncryptionContextString_)
                                            if (d_19_valueOrError9_).IsFailure():
                                                return (d_19_valueOrError9_).PropagateFailure()
                                            elif True:
                                                d_20_reproducedEncryptionContext_ = (d_19_valueOrError9_).Extract()
                                                d_21_valueOrError10_ = JSONHelpers.default__.GetString(_dafny.Seq("description"), d_2_scenario_)
                                                if (d_21_valueOrError10_).IsFailure():
                                                    return (d_21_valueOrError10_).PropagateFailure()
                                                elif True:
                                                    d_22_description_ = (d_21_valueOrError10_).Extract()
                                                    source0_ = d_5_typ_
                                                    if True:
                                                        if (source0_) == (_dafny.Seq("positive-esdk")):
                                                            d_23_valueOrError11_ = ParseJsonManifests.default__.GetKeyDescription(keys, default__.encryptKeyDescription, d_2_scenario_)
                                                            if (d_23_valueOrError11_).IsFailure():
                                                                return (d_23_valueOrError11_).PropagateFailure()
                                                            elif True:
                                                                d_24_encryptKeyDescription_ = (d_23_valueOrError11_).Extract()
                                                                d_25_valueOrError12_ = ParseJsonManifests.default__.GetKeyDescription(keys, default__.decryptKeyDescription, d_2_scenario_)
                                                                if (d_25_valueOrError12_).IsFailure():
                                                                    return (d_25_valueOrError12_).PropagateFailure()
                                                                elif True:
                                                                    d_26_decryptKeyDescription_ = (d_25_valueOrError12_).Extract()
                                                                    return Wrappers.Result_Success(EsdkTestVectors.EsdkEncryptTestVector_PositiveEncryptTestVector(Wrappers.Option_Some(name), version, (op).manifestPath, (op).decryptManifestOutput, d_7_plaintextLoc_, d_24_encryptKeyDescription_, d_26_decryptKeyDescription_, Wrappers.Option_Some(d_16_encryptionContext_), Wrappers.Option_Some(d_20_reproducedEncryptionContext_), AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT(), d_12_frameLength_, Wrappers.Option_Some(d_9_algorithmSuite_), d_22_description_, Wrappers.Option_Some(1)))
                                                    if True:
                                                        return Wrappers.Result_Failure((_dafny.Seq("Unsupported ESDK TestVector type: ")) + (d_5_typ_))

    @staticmethod
    def V1ToDecryptTestVector(op, keys, name, obj, version):
        d_0_valueOrError0_ = JSONHelpers.default__.GetString(_dafny.Seq("plaintext"), obj)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_plaintextLoc_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = JSONHelpers.default__.GetString(_dafny.Seq("ciphertext"), obj)
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_ciphertextLoc_ = (d_2_valueOrError1_).Extract()
                d_4_valueOrError2_ = Wrappers.default__.Need(((_dafny.Seq("file://")) < (d_3_ciphertextLoc_)) and ((_dafny.Seq("file://")) < (d_1_plaintextLoc_)), _dafny.Seq("Invalid file prefix in test vector"))
                if (d_4_valueOrError2_).IsFailure():
                    return (d_4_valueOrError2_).PropagateFailure()
                elif True:
                    d_5_valueOrError3_ = JSONHelpers.default__.GetArray(_dafny.Seq("master-keys"), obj)
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_masterKeys_ = (d_5_valueOrError3_).Extract()
                        d_7_valueOrError4_ = default__.GetKeyDescriptions(d_6_masterKeys_, keys)
                        if (d_7_valueOrError4_).IsFailure():
                            return (d_7_valueOrError4_).PropagateFailure()
                        elif True:
                            d_8_keyDescriptions_ = (d_7_valueOrError4_).Extract()
                            d_9_valueOrError5_ = default__.ToMultiKeyDescription(d_8_keyDescriptions_)
                            if (d_9_valueOrError5_).IsFailure():
                                return (d_9_valueOrError5_).PropagateFailure()
                            elif True:
                                d_10_keyDescription_ = (d_9_valueOrError5_).Extract()
                                return Wrappers.Result_Success(EsdkTestVectors.EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector(name, version, (op).manifestPath, _dafny.Seq((d_3_ciphertextLoc_)[len(default__.FILE__PREPEND)::]), _dafny.Seq((d_1_plaintextLoc_)[len(default__.FILE__PREPEND)::]), Wrappers.Option_None(), Wrappers.Option_None(), d_10_keyDescription_, AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT(), Wrappers.Option_None(), Wrappers.Option_None(), name, EsdkTestVectors.DecryptionMethod_OneShot()))

    @staticmethod
    def V2ToDecryptTestVector(op, keys, name, obj, version):
        d_0_valueOrError0_ = JSONHelpers.default__.GetObject(_dafny.Seq("result"), obj)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_resultLoc_ = (d_0_valueOrError0_).Extract()
            d_2_errorLoc_q_ = JSONHelpers.default__.GetObject(_dafny.Seq("error"), d_1_resultLoc_)
            if (d_2_errorLoc_q_).is_Success:
                return Wrappers.Result_Failure(default__.negativeTestVectorFound)
            elif True:
                d_3_valueOrError1_ = JSONHelpers.default__.GetObject(_dafny.Seq("output"), d_1_resultLoc_)
                if (d_3_valueOrError1_).IsFailure():
                    return (d_3_valueOrError1_).PropagateFailure()
                elif True:
                    d_4_outputLoc_ = (d_3_valueOrError1_).Extract()
                    d_5_valueOrError2_ = JSONHelpers.default__.GetString(_dafny.Seq("plaintext"), d_4_outputLoc_)
                    if (d_5_valueOrError2_).IsFailure():
                        return (d_5_valueOrError2_).PropagateFailure()
                    elif True:
                        d_6_plaintextLoc_ = (d_5_valueOrError2_).Extract()
                        d_7_valueOrError3_ = JSONHelpers.default__.GetString(_dafny.Seq("ciphertext"), obj)
                        if (d_7_valueOrError3_).IsFailure():
                            return (d_7_valueOrError3_).PropagateFailure()
                        elif True:
                            d_8_ciphertextLoc_ = (d_7_valueOrError3_).Extract()
                            d_9_valueOrError4_ = Wrappers.default__.Need(((_dafny.Seq("file://")) < (d_8_ciphertextLoc_)) and ((_dafny.Seq("file://")) < (d_6_plaintextLoc_)), _dafny.Seq("Invalid file prefix in test vector"))
                            if (d_9_valueOrError4_).IsFailure():
                                return (d_9_valueOrError4_).PropagateFailure()
                            elif True:
                                d_10_valueOrError5_ = JSONHelpers.default__.GetArray(_dafny.Seq("master-keys"), obj)
                                if (d_10_valueOrError5_).IsFailure():
                                    return (d_10_valueOrError5_).PropagateFailure()
                                elif True:
                                    d_11_masterKeys_ = (d_10_valueOrError5_).Extract()
                                    d_12_valueOrError6_ = default__.GetKeyDescriptions(d_11_masterKeys_, keys)
                                    if (d_12_valueOrError6_).IsFailure():
                                        return (d_12_valueOrError6_).PropagateFailure()
                                    elif True:
                                        d_13_keyDescriptions_ = (d_12_valueOrError6_).Extract()
                                        d_14_valueOrError7_ = default__.ToMultiKeyDescription(d_13_keyDescriptions_)
                                        if (d_14_valueOrError7_).IsFailure():
                                            return (d_14_valueOrError7_).PropagateFailure()
                                        elif True:
                                            d_15_keyDescription_ = (d_14_valueOrError7_).Extract()
                                            return Wrappers.Result_Success(EsdkTestVectors.EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector(name, version, (op).manifestPath, _dafny.Seq((d_8_ciphertextLoc_)[len(default__.FILE__PREPEND)::]), _dafny.Seq((d_6_plaintextLoc_)[len(default__.FILE__PREPEND)::]), Wrappers.Option_None(), Wrappers.Option_None(), d_15_keyDescription_, AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT(), Wrappers.Option_None(), Wrappers.Option_None(), name, EsdkTestVectors.DecryptionMethod_OneShot()))

    @staticmethod
    def V3ToDecryptTestVector(op, keys, name, obj, version):
        d_0_scenarioString_ = _dafny.Seq("decryption-scenario")
        d_1_valueOrError0_ = JSONHelpers.default__.GetObject(d_0_scenarioString_, obj)
        if (d_1_valueOrError0_).IsFailure():
            return (d_1_valueOrError0_).PropagateFailure()
        elif True:
            d_2_scenario_ = (d_1_valueOrError0_).Extract()
            d_3_typeString_ = _dafny.Seq("type")
            d_4_valueOrError1_ = JSONHelpers.default__.GetString(d_3_typeString_, d_2_scenario_)
            if (d_4_valueOrError1_).IsFailure():
                return (d_4_valueOrError1_).PropagateFailure()
            elif True:
                d_5_typ_ = (d_4_valueOrError1_).Extract()
                d_6_valueOrError2_ = JSONHelpers.default__.GetString(default__.ciphertextJsonKey, d_2_scenario_)
                if (d_6_valueOrError2_).IsFailure():
                    return (d_6_valueOrError2_).PropagateFailure()
                elif True:
                    d_7_ciphertextLoc_ = (d_6_valueOrError2_).Extract()
                    d_8_valueOrError3_ = ParseJsonManifests.default__.GetAlgorithmSuiteInfo(d_2_scenario_)
                    if (d_8_valueOrError3_).IsFailure():
                        return (d_8_valueOrError3_).PropagateFailure()
                    elif True:
                        d_9_algorithmSuite_ = (d_8_valueOrError3_).Extract()
                        d_10_valueOrError4_ = Wrappers.default__.Need(((d_9_algorithmSuite_).id).is_ESDK, _dafny.Seq("Unsupported algorithmSuiteId"))
                        if (d_10_valueOrError4_).IsFailure():
                            return (d_10_valueOrError4_).PropagateFailure()
                        elif True:
                            d_11_valueOrError5_ = JSONHelpers.default__.GetOptionalPositiveLong(default__.frameSizeJsonKey, d_2_scenario_)
                            if (d_11_valueOrError5_).IsFailure():
                                return (d_11_valueOrError5_).PropagateFailure()
                            elif True:
                                d_12_frameLength_ = (d_11_valueOrError5_).Extract()
                                d_13_valueOrError6_ = JSONHelpers.default__.SmallObjectToStringStringMap(default__.reproducedEncryptionContextJsonKey, d_2_scenario_)
                                if (d_13_valueOrError6_).IsFailure():
                                    return (d_13_valueOrError6_).PropagateFailure()
                                elif True:
                                    d_14_reproducedEncryptionContextStrings_ = (d_13_valueOrError6_).Extract()
                                    d_15_valueOrError7_ = JSONHelpers.default__.utf8EncodeMap(d_14_reproducedEncryptionContextStrings_)
                                    if (d_15_valueOrError7_).IsFailure():
                                        return (d_15_valueOrError7_).PropagateFailure()
                                    elif True:
                                        d_16_reproducedEncryptionContext_ = (d_15_valueOrError7_).Extract()
                                        d_17_valueOrError8_ = JSONHelpers.default__.GetString(_dafny.Seq("description"), d_2_scenario_)
                                        if (d_17_valueOrError8_).IsFailure():
                                            return (d_17_valueOrError8_).PropagateFailure()
                                        elif True:
                                            d_18_description_ = (d_17_valueOrError8_).Extract()
                                            d_19_valueOrError9_ = JSONHelpers.default__.GetString(_dafny.Seq("result"), d_2_scenario_)
                                            if (d_19_valueOrError9_).IsFailure():
                                                return (d_19_valueOrError9_).PropagateFailure()
                                            elif True:
                                                d_20_result_ = (d_19_valueOrError9_).Extract()
                                                d_21_valueOrError10_ = Wrappers.default__.Need(((_dafny.Seq("file://")) < (d_7_ciphertextLoc_)) and ((_dafny.Seq("file://")) < (d_20_result_)), _dafny.Seq("Invalid file prefix in test vector"))
                                                if (d_21_valueOrError10_).IsFailure():
                                                    return (d_21_valueOrError10_).PropagateFailure()
                                                elif True:
                                                    source0_ = d_5_typ_
                                                    if True:
                                                        if (source0_) == (_dafny.Seq("positive-esdk")):
                                                            d_22_valueOrError11_ = ParseJsonManifests.default__.GetKeyDescription(keys, default__.decryptKeyDescription, d_2_scenario_)
                                                            if (d_22_valueOrError11_).IsFailure():
                                                                return (d_22_valueOrError11_).PropagateFailure()
                                                            elif True:
                                                                d_23_decryptKeyDescription_ = (d_22_valueOrError11_).Extract()
                                                                return Wrappers.Result_Success(EsdkTestVectors.EsdkDecryptTestVector_PositiveDecryptTestVector(name, version, (op).manifestPath, _dafny.Seq((d_7_ciphertextLoc_)[len(default__.FILE__PREPEND)::]), _dafny.Seq((d_20_result_)[len(default__.FILE__PREPEND)::]), Wrappers.Option_Some(d_16_reproducedEncryptionContext_), d_23_decryptKeyDescription_, AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT(), d_12_frameLength_, Wrappers.Option_Some(d_9_algorithmSuite_), d_18_description_, EsdkTestVectors.DecryptionMethod_OneShot()))
                                                    if True:
                                                        return Wrappers.Result_Failure((_dafny.Seq("Unsupported ESDK TestVector type: ")) + (d_5_typ_))

    @staticmethod
    def GetKeyDescriptions(keyArray, keys):
        if (len(keyArray)) == (0):
            return Wrappers.Result_Success(_dafny.Seq([]))
        elif True:
            d_0_currKey_ = (keyArray)[0]
            d_1_valueOrError0_ = Wrappers.default__.Need((d_0_currKey_).is_Object, _dafny.Seq("Not an object"))
            if (d_1_valueOrError0_).IsFailure():
                return (d_1_valueOrError0_).PropagateFailure()
            elif True:
                def lambda0_(d_3_e_):
                    return (d_3_e_).ToString()

                d_2_valueOrError1_ = (JSON_API.default__.Serialize(d_0_currKey_)).MapFailure(lambda0_)
                if (d_2_valueOrError1_).IsFailure():
                    return (d_2_valueOrError1_).PropagateFailure()
                elif True:
                    d_4_encryptStr_ = (d_2_valueOrError1_).Extract()
                    d_5_valueOrError2_ = ((keys).GetKeyDescription(AwsCryptographyMaterialProvidersTestVectorKeysTypes.GetKeyDescriptionInput_GetKeyDescriptionInput(d_4_encryptStr_))).MapFailure(ParseJsonManifests.default__.ErrorToString)
                    if (d_5_valueOrError2_).IsFailure():
                        return (d_5_valueOrError2_).PropagateFailure()
                    elif True:
                        d_6_encryptDecryptKeyDescription_ = (d_5_valueOrError2_).Extract()
                        d_7_valueOrError3_ = default__.GetKeyDescriptions(_dafny.Seq((keyArray)[1::]), keys)
                        if (d_7_valueOrError3_).IsFailure():
                            return (d_7_valueOrError3_).PropagateFailure()
                        elif True:
                            d_8_tail_ = (d_7_valueOrError3_).Extract()
                            return Wrappers.Result_Success((_dafny.Seq([(d_6_encryptDecryptKeyDescription_).keyDescription])) + (d_8_tail_))

    @staticmethod
    def ToMultiKeyDescription(keyDescriptions):
        if (len(keyDescriptions)) == (1):
            return Wrappers.Result_Success((keyDescriptions)[0])
        elif True:
            d_0_valueOrError0_ = Wrappers.default__.Need((len(keyDescriptions)) > (1), _dafny.Seq("Received invalid key description length"))
            if (d_0_valueOrError0_).IsFailure():
                return (d_0_valueOrError0_).PropagateFailure()
            elif True:
                return Wrappers.Result_Success(AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription_Multi(AwsCryptographyMaterialProvidersTestVectorKeysTypes.MultiKeyring_MultiKeyring(Wrappers.Option_Some((keyDescriptions)[0]), _dafny.Seq((keyDescriptions)[1::]))))

    @staticmethod
    def GetPath(key, obj):
        d_0_valueOrError0_ = JSONHelpers.default__.GetString(key, obj)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_path_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = Wrappers.default__.Need((default__.FILE__PREPEND) < (d_1_path_), _dafny.Seq("Received Invalid location for plaintext or ciphertext."))
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                return Wrappers.Result_Success(_dafny.Seq((d_1_path_)[len(default__.FILE__PREPEND)::]))

    @staticmethod
    def Result_q(key):
        return ((key) == (_dafny.Seq("output"))) or ((key) == (_dafny.Seq("error")))

    @_dafny.classproperty
    def ciphertextJsonKey(instance):
        return _dafny.Seq("ciphertext")
    @_dafny.classproperty
    def frameSizeJsonKey(instance):
        return _dafny.Seq("frame-size")
    @_dafny.classproperty
    def reproducedEncryptionContextJsonKey(instance):
        return _dafny.Seq("reproduced-encryption-context")
    @_dafny.classproperty
    def decryptKeyDescription(instance):
        return _dafny.Seq("decryptKeyDescription")
    @_dafny.classproperty
    def FILE__PREPEND(instance):
        return _dafny.Seq("file://")
    @_dafny.classproperty
    def negativeTestVectorFound(instance):
        return _dafny.Seq("Negative test vector found; not supported yet.")
    @_dafny.classproperty
    def buildTestVectorError(instance):
        return _dafny.Seq("Error other than negative test vector found thrown")
    @_dafny.classproperty
    def plaintextJsonKey(instance):
        return _dafny.Seq("plaintext")
    @_dafny.classproperty
    def encryptionContextJsonKey(instance):
        return _dafny.Seq("encryption-context")
    @_dafny.classproperty
    def encryptKeyDescription(instance):
        return _dafny.Seq("encryptKeyDescription")
    @_dafny.classproperty
    def masterKeysJsonKey(instance):
        return _dafny.Seq("master-keys")
    @_dafny.classproperty
    def decryptionMethodJsonKey(instance):
        return _dafny.Seq("decryption-method")
