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
import aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import smithy_dafny_standard_library.internaldafny.generated.Streams as Streams
import aws_encryption_sdk_dafny.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptionContext as EncryptionContext
import aws_encryption_sdk_dafny.internaldafny.generated.HeaderTypes as HeaderTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SharedHeaderFunctions as SharedHeaderFunctions
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptedDataKeys as EncryptedDataKeys
import aws_encryption_sdk_dafny.internaldafny.generated.V1HeaderBody as V1HeaderBody
import aws_encryption_sdk_dafny.internaldafny.generated.V2HeaderBody as V2HeaderBody
import aws_encryption_sdk_dafny.internaldafny.generated.HeaderAuth as HeaderAuth
import aws_encryption_sdk_dafny.internaldafny.generated.Header as Header
import aws_encryption_sdk_dafny.internaldafny.generated.Frames as Frames
import aws_encryption_sdk_dafny.internaldafny.generated.MessageBody as MessageBody
import aws_encryption_sdk_dafny.internaldafny.generated.KeyDerivation as KeyDerivation
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptDecryptHelpers as EncryptDecryptHelpers
import aws_encryption_sdk_dafny.internaldafny.generated.AwsEncryptionSdkOperations as AwsEncryptionSdkOperations
import aws_encryption_sdk_dafny.internaldafny.generated.ESDK as ESDK
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
import aws_encryption_sdk_test_vectors.internaldafny.generated.ParseEsdkJsonManifest as ParseEsdkJsonManifest

# Module: EsdkTestManifests

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def StartDecryptVectors(op):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = default__.GetManifest((op).manifestPath, (op).manifestFileName)
        d_0_valueOrError0_ = out0_
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(40,27): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_decryptManifest_: ManifestData
        d_1_decryptManifest_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_2_valueOrError1_ = Wrappers.default__.Need((d_1_decryptManifest_).is_DecryptManifest, _dafny.Seq("Not a decrypt manifest"))
        if (d_2_valueOrError1_).IsFailure():
            output = (d_2_valueOrError1_).PropagateFailure()
            return output
        d_3_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_3_valueOrError2_ = ParseEsdkJsonManifest.default__.BuildDecryptTestVector(op, (d_1_decryptManifest_).clientName, (d_1_decryptManifest_).clientVersion, (d_1_decryptManifest_).version, (d_1_decryptManifest_).keys, (d_1_decryptManifest_).jsonTests)
        if (d_3_valueOrError2_).IsFailure():
            output = (d_3_valueOrError2_).PropagateFailure()
            return output
        d_4_decryptVectors_: _dafny.Seq
        d_4_decryptVectors_ = (d_3_valueOrError2_).Extract()
        out1_: Wrappers.Result
        out1_ = default__.TestDecrypts((d_1_decryptManifest_).keys, d_4_decryptVectors_)
        output = out1_
        return output

    @staticmethod
    def TestDecryptVector_q(v):
        return (True) and (((v).decryptionMethod).is_OneShot)

    @staticmethod
    def TestDecrypts(keys, vectors):
        manifest: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        _dafny.print(_dafny.string_of(_dafny.Seq("\n=================== Starting ")))
        _dafny.print(_dafny.string_of(len(vectors)))
        _dafny.print(_dafny.string_of(_dafny.Seq(" Decrypt Tests =================== \n\n")))
        d_0_hasFailure_: bool
        d_0_hasFailure_ = False
        d_1_skipped_: int
        d_1_skipped_ = 0
        hi0_ = len(vectors)
        for d_2_i_ in range(0, hi0_):
            d_3_vector_: EsdkTestVectors.EsdkDecryptTestVector
            d_3_vector_ = (vectors)[d_2_i_]
            if default__.TestDecryptVector_q(d_3_vector_):
                d_4_pass_: bool
                out0_: bool
                out0_ = EsdkTestVectors.default__.TestDecrypt(keys, d_3_vector_)
                d_4_pass_ = out0_
                if not(d_4_pass_):
                    d_0_hasFailure_ = True
            elif True:
                d_1_skipped_ = (d_1_skipped_) + (1)
                _dafny.print(_dafny.string_of(_dafny.Seq("\nSKIP===> ")))
                _dafny.print(_dafny.string_of((d_3_vector_).id))
                _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n=================== Completed ")))
        _dafny.print(_dafny.string_of(len(vectors)))
        _dafny.print(_dafny.string_of(_dafny.Seq(" Decrypt Tests =================== \n\n")))
        if (0) < (d_1_skipped_):
            _dafny.print(_dafny.string_of(_dafny.Seq("Skipped: ")))
            _dafny.print(_dafny.string_of(d_1_skipped_))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        if not(d_0_hasFailure_):
            manifest = Wrappers.Result_Success(_dafny.Seq([]))
        elif True:
            manifest = Wrappers.Result_Failure(_dafny.Seq("Test Vectors failed, see errors above.\n"))
        return manifest

    @staticmethod
    def StartEncryptVectors(op):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = default__.GetManifest((op).manifestPath, (op).manifest)
        d_0_valueOrError0_ = out0_
        if (d_0_valueOrError0_).IsFailure():
            output = (d_0_valueOrError0_).PropagateFailure()
            return output
        d_1_encryptManifest_: ManifestData
        d_1_encryptManifest_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_2_valueOrError1_ = Wrappers.default__.Need((d_1_encryptManifest_).is_EncryptManifest, _dafny.Seq("Not a encrypt manifest"))
        if (d_2_valueOrError1_).IsFailure():
            output = (d_2_valueOrError1_).PropagateFailure()
            return output
        d_3_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_3_valueOrError2_ = ParseEsdkJsonManifest.default__.BuildEncryptTestVector(op, (d_1_encryptManifest_).version, (d_1_encryptManifest_).keys, (d_1_encryptManifest_).jsonTests)
        if (d_3_valueOrError2_).IsFailure():
            output = (d_3_valueOrError2_).PropagateFailure()
            return output
        d_4_encryptVectors_: _dafny.Seq
        d_4_encryptVectors_ = (d_3_valueOrError2_).Extract()
        d_5_valueOrError3_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = AtomicPrimitives.default__.AtomicPrimitives(AtomicPrimitives.default__.DefaultCryptoConfig())
        d_5_valueOrError3_ = out1_
        if not(not((d_5_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(114,13): " + _dafny.string_of(d_5_valueOrError3_))
        d_6_p_: AtomicPrimitives.AtomicPrimitivesClient
        d_6_p_ = (d_5_valueOrError3_).Extract()
        d_7_plaintext_: _dafny.Map
        d_7_plaintext_ = _dafny.Map({})
        hi0_ = len((d_1_encryptManifest_).plaintext)
        for d_8_i_ in range(0, hi0_):
            let_tmp_rhs0_ = ((d_1_encryptManifest_).plaintext)[d_8_i_]
            d_9_name_ = let_tmp_rhs0_[0]
            d_10_length_ = let_tmp_rhs0_[1]
            d_11_valueOrError4_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            out2_: Wrappers.Result
            out2_ = (d_6_p_).GenerateRandomBytes(AwsCryptographyPrimitivesTypes.GenerateRandomBytesInput_GenerateRandomBytesInput(d_10_length_))
            d_11_valueOrError4_ = out2_
            if not(not((d_11_valueOrError4_).IsFailure())):
                raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(119,18): " + _dafny.string_of(d_11_valueOrError4_))
            d_12_data_: _dafny.Seq
            d_12_data_ = (d_11_valueOrError4_).Extract()
            _dafny.print(_dafny.string_of((((op).decryptManifestOutput) + (EsdkTestVectors.default__.plaintextPathRoot)) + (d_9_name_)))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n\n")))
            d_13_valueOrError5_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
            out3_: Wrappers.Result
            out3_ = EsdkTestVectors.default__.WriteVectorsFile((((op).decryptManifestOutput) + (EsdkTestVectors.default__.plaintextPathRoot)) + (d_9_name_), d_12_data_)
            d_13_valueOrError5_ = out3_
            if (d_13_valueOrError5_).IsFailure():
                output = (d_13_valueOrError5_).PropagateFailure()
                return output
            d_14___v0_: tuple
            d_14___v0_ = (d_13_valueOrError5_).Extract()
            d_7_plaintext_ = (d_7_plaintext_) | (_dafny.Map({d_9_name_: d_12_data_}))
        d_15_encryptTests_q_: Wrappers.Result
        out4_: Wrappers.Result
        out4_ = default__.ToEncryptTests((d_1_encryptManifest_).keys, d_4_encryptVectors_)
        d_15_encryptTests_q_ = out4_
        d_16_valueOrError6_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda0_(d_17_e_):
            def iife0_(_pat_let6_0):
                def iife1_(d_18___v1_):
                    return _dafny.Seq("Cmm failure")
                return iife1_(_pat_let6_0)
            return iife0_(EsdkTestVectors.default__.MplVectorPrintErr(d_17_e_))

        d_16_valueOrError6_ = (d_15_encryptTests_q_).MapFailure(lambda0_)
        if (d_16_valueOrError6_).IsFailure():
            output = (d_16_valueOrError6_).PropagateFailure()
            return output
        d_19_encryptTests_: _dafny.Seq
        d_19_encryptTests_ = (d_16_valueOrError6_).Extract()
        d_20_valueOrError7_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out5_: Wrappers.Result
        out5_ = default__.TestEncrypts(d_7_plaintext_, (d_1_encryptManifest_).keys, d_19_encryptTests_)
        d_20_valueOrError7_ = out5_
        if (d_20_valueOrError7_).IsFailure():
            output = (d_20_valueOrError7_).PropagateFailure()
            return output
        d_21_decryptVectors_: _dafny.Seq
        d_21_decryptVectors_ = (d_20_valueOrError7_).Extract()
        d_22_valueOrError8_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        out6_: Wrappers.Result
        out6_ = WriteVectors.default__.WriteDecryptManifest(op, (d_1_encryptManifest_).keys, d_21_decryptVectors_)
        d_22_valueOrError8_ = out6_
        if (d_22_valueOrError8_).IsFailure():
            output = (d_22_valueOrError8_).PropagateFailure()
            return output
        d_23___v2_: tuple
        d_23___v2_ = (d_22_valueOrError8_).Extract()
        output = Wrappers.Result_Success(())
        return output

    @staticmethod
    def TestEncryptVector_q(vector):
        return (True) and (not (((vector).frameLength).is_Some) or (AwsCryptographyEncryptionSdkTypes.default__.IsValid__FrameLength(((vector).frameLength).value)))

    @staticmethod
    def ToEncryptTests(keys, vectors):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_encryptTests_: _dafny.Seq
        d_0_encryptTests_ = _dafny.Seq([])
        hi0_ = len(vectors)
        for d_1_i_ in range(0, hi0_):
            d_2_valueOrError0_: Wrappers.Result = None
            out0_: Wrappers.Result
            out0_ = EsdkTestVectors.default__.EncryptVectorToEncryptTest(keys, (vectors)[d_1_i_])
            d_2_valueOrError0_ = out0_
            if (d_2_valueOrError0_).IsFailure():
                output = (d_2_valueOrError0_).PropagateFailure()
                return output
            d_3_test_: EsdkTestVectors.EncryptTest
            d_3_test_ = (d_2_valueOrError0_).Extract()
            d_0_encryptTests_ = (d_0_encryptTests_) + (_dafny.Seq([d_3_test_]))
        output = Wrappers.Result_Success(d_0_encryptTests_)
        return output
        return output

    @staticmethod
    def TestEncrypts(plaintexts, keys, tests):
        manifest: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        _dafny.print(_dafny.string_of(_dafny.Seq("\n=================== Starting ")))
        _dafny.print(_dafny.string_of(len(tests)))
        _dafny.print(_dafny.string_of(_dafny.Seq(" Encrypt Tests =================== \n\n")))
        d_0_hasFailure_: bool
        d_0_hasFailure_ = False
        d_1_decryptVectors_: _dafny.Seq
        d_1_decryptVectors_ = _dafny.Seq([])
        d_2_skipped_: _dafny.Seq
        d_2_skipped_ = _dafny.Seq([])
        hi0_ = len(tests)
        for d_3_i_ in range(0, hi0_):
            d_4_test_: EsdkTestVectors.EncryptTest
            d_4_test_ = (tests)[d_3_i_]
            d_5_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_5_valueOrError0_ = Wrappers.default__.Need((((d_4_test_).vector).id).is_Some, _dafny.Seq("Vector is missing uuid"))
            if (d_5_valueOrError0_).IsFailure():
                manifest = (d_5_valueOrError0_).PropagateFailure()
                return manifest
            if default__.TestEncryptVector_q((d_4_test_).vector):
                d_6_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
                d_6_valueOrError1_ = Wrappers.default__.Need(((((d_4_test_).vector).algorithmSuiteId).is_Some) and ((((((d_4_test_).vector).algorithmSuiteId).value).id).is_ESDK), _dafny.Seq("Vector is using an algorithm suite other than ESDK"))
                if (d_6_valueOrError1_).IsFailure():
                    manifest = (d_6_valueOrError1_).PropagateFailure()
                    return manifest
                d_7_valueOrError2_: Wrappers.Result = Wrappers.Result.default(EsdkTestVectors.EncryptTestOutput.default())()
                out0_: Wrappers.Result
                out0_ = EsdkTestVectors.default__.TestEncrypt(plaintexts, keys, d_4_test_)
                d_7_valueOrError2_ = out0_
                if (d_7_valueOrError2_).IsFailure():
                    manifest = (d_7_valueOrError2_).PropagateFailure()
                    return manifest
                d_8_pass_: EsdkTestVectors.EncryptTestOutput
                d_8_pass_ = (d_7_valueOrError2_).Extract()
                if not((d_8_pass_).output):
                    d_0_hasFailure_ = True
                elif ((d_8_pass_).vector).is_Some:
                    d_1_decryptVectors_ = (d_1_decryptVectors_) + (_dafny.Seq([((d_8_pass_).vector).value]))
            elif True:
                d_2_skipped_ = (d_2_skipped_) + (_dafny.Seq([((((d_4_test_).vector).id).value) + (_dafny.Seq("\n"))]))
                _dafny.print(_dafny.string_of(_dafny.Seq("\nSKIP===> ")))
                _dafny.print(_dafny.string_of((((d_4_test_).vector).id).value))
                _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n=================== Completed ")))
        _dafny.print(_dafny.string_of(len(tests)))
        _dafny.print(_dafny.string_of(_dafny.Seq(" Encrypt Tests =================== \n\n")))
        if not(not(d_0_hasFailure_)):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(215,4): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        manifest = Wrappers.Result_Success(d_1_decryptVectors_)
        return manifest

    @staticmethod
    def GetManifest(manifestPath, manifestFileName):
        manifestData: Wrappers.Result = None
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out0_: Wrappers.Result
        out0_ = FileIO.default__.ReadBytesFromFile((manifestPath) + (manifestFileName))
        d_0_valueOrError0_ = out0_
        if (d_0_valueOrError0_).IsFailure():
            manifestData = (d_0_valueOrError0_).PropagateFailure()
            return manifestData
        d_1_decryptManifestBv_: _dafny.Seq
        d_1_decryptManifestBv_ = (d_0_valueOrError0_).Extract()
        d_2_decryptManifestBytes_: _dafny.Seq
        d_2_decryptManifestBytes_ = JSONHelpers.default__.BvToBytes(d_1_decryptManifestBv_)
        d_3_valueOrError1_: Wrappers.Result = Wrappers.Result.default(JSON_Values.JSON.default())()
        def lambda0_(d_4_e_):
            return (d_4_e_).ToString()

        d_3_valueOrError1_ = (JSON_API.default__.Deserialize(d_2_decryptManifestBytes_)).MapFailure(lambda0_)
        if (d_3_valueOrError1_).IsFailure():
            manifestData = (d_3_valueOrError1_).PropagateFailure()
            return manifestData
        d_5_manifestJson_: JSON_Values.JSON
        d_5_manifestJson_ = (d_3_valueOrError1_).Extract()
        d_6_valueOrError2_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_6_valueOrError2_ = Wrappers.default__.Need((d_5_manifestJson_).is_Object, _dafny.Seq("Not a JSON object"))
        if (d_6_valueOrError2_).IsFailure():
            manifestData = (d_6_valueOrError2_).PropagateFailure()
            return manifestData
        d_7_valueOrError3_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_7_valueOrError3_ = JSONHelpers.default__.GetObject(_dafny.Seq("manifest"), (d_5_manifestJson_).obj)
        if (d_7_valueOrError3_).IsFailure():
            manifestData = (d_7_valueOrError3_).PropagateFailure()
            return manifestData
        d_8_manifest_: _dafny.Seq
        d_8_manifest_ = (d_7_valueOrError3_).Extract()
        d_9_valueOrError4_: Wrappers.Result = Wrappers.Result.default(System_.nat.default)()
        d_9_valueOrError4_ = JSONHelpers.default__.GetNat(_dafny.Seq("version"), d_8_manifest_)
        if (d_9_valueOrError4_).IsFailure():
            manifestData = (d_9_valueOrError4_).PropagateFailure()
            return manifestData
        d_10_version_: int
        d_10_version_ = (d_9_valueOrError4_).Extract()
        d_11_valueOrError5_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_11_valueOrError5_ = JSONHelpers.default__.GetString(_dafny.Seq("type"), d_8_manifest_)
        if (d_11_valueOrError5_).IsFailure():
            manifestData = (d_11_valueOrError5_).PropagateFailure()
            return manifestData
        d_12_typ_: _dafny.Seq
        d_12_typ_ = (d_11_valueOrError5_).Extract()
        d_13_valueOrError6_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_13_valueOrError6_ = JSONHelpers.default__.GetObject(_dafny.Seq("client"), (d_5_manifestJson_).obj)
        if (d_13_valueOrError6_).IsFailure():
            manifestData = (d_13_valueOrError6_).PropagateFailure()
            return manifestData
        d_14_client_: _dafny.Seq
        d_14_client_ = (d_13_valueOrError6_).Extract()
        d_15_valueOrError7_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_15_valueOrError7_ = JSONHelpers.default__.GetString(_dafny.Seq("name"), d_14_client_)
        if (d_15_valueOrError7_).IsFailure():
            manifestData = (d_15_valueOrError7_).PropagateFailure()
            return manifestData
        d_16_clientName_: _dafny.Seq
        d_16_clientName_ = (d_15_valueOrError7_).Extract()
        d_17_valueOrError8_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_17_valueOrError8_ = JSONHelpers.default__.GetString(_dafny.Seq("version"), d_14_client_)
        if (d_17_valueOrError8_).IsFailure():
            manifestData = (d_17_valueOrError8_).PropagateFailure()
            return manifestData
        d_18_clientVersion_: _dafny.Seq
        d_18_clientVersion_ = (d_17_valueOrError8_).Extract()
        d_19_valueOrError9_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_19_valueOrError9_ = JSONHelpers.default__.GetString(_dafny.Seq("keys"), (d_5_manifestJson_).obj)
        if (d_19_valueOrError9_).IsFailure():
            manifestData = (d_19_valueOrError9_).PropagateFailure()
            return manifestData
        d_20_keyManifestUri_: _dafny.Seq
        d_20_keyManifestUri_ = (d_19_valueOrError9_).Extract()
        d_21_valueOrError10_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_21_valueOrError10_ = Wrappers.default__.Need((_dafny.Seq("file://")) < (d_20_keyManifestUri_), _dafny.Seq("Unexpected URI prefix"))
        if (d_21_valueOrError10_).IsFailure():
            manifestData = (d_21_valueOrError10_).PropagateFailure()
            return manifestData
        d_22_keyManifestPath_: _dafny.Seq
        d_22_keyManifestPath_ = (manifestPath) + (_dafny.Seq((d_20_keyManifestUri_)[7::]))
        d_23_valueOrError11_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = KeyVectors.default__.KeyVectors(AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyVectorsConfig_KeyVectorsConfig(d_22_keyManifestPath_))
        d_23_valueOrError11_ = out1_
        if not(not((d_23_valueOrError11_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(268,16): " + _dafny.string_of(d_23_valueOrError11_))
        d_24_keys_: KeyVectors.KeyVectorsClient
        d_24_keys_ = (d_23_valueOrError11_).Extract()
        d_25_valueOrError12_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_25_valueOrError12_ = JSONHelpers.default__.GetObject(_dafny.Seq("tests"), (d_5_manifestJson_).obj)
        if (d_25_valueOrError12_).IsFailure():
            manifestData = (d_25_valueOrError12_).PropagateFailure()
            return manifestData
        d_26_jsonTests_: _dafny.Seq
        d_26_jsonTests_ = (d_25_valueOrError12_).Extract()
        source0_ = d_12_typ_
        with _dafny.label("match0"):
            if True:
                if (source0_) == (_dafny.Seq("awses-decrypt")):
                    d_27_valueOrError13_: Wrappers.Outcome = Wrappers.Outcome.default()()
                    d_27_valueOrError13_ = Wrappers.default__.Need(EsdkTestVectors.default__.SupportedDecryptVersion_q(d_10_version_), _dafny.Seq("Unsupported manifest version"))
                    if (d_27_valueOrError13_).IsFailure():
                        manifestData = (d_27_valueOrError13_).PropagateFailure()
                        return manifestData
                    manifestData = Wrappers.Result_Success(ManifestData_DecryptManifest(d_10_version_, d_24_keys_, d_16_clientName_, d_18_clientVersion_, d_26_jsonTests_))
                    raise _dafny.Break("match0")
            if True:
                if (source0_) == (_dafny.Seq("awses-encrypt")):
                    d_28_valueOrError14_: Wrappers.Outcome = Wrappers.Outcome.default()()
                    d_28_valueOrError14_ = Wrappers.default__.Need(EsdkTestVectors.default__.SupportedEncryptVersion_q(d_10_version_), _dafny.Seq("Unsupported manifest version"))
                    if (d_28_valueOrError14_).IsFailure():
                        manifestData = (d_28_valueOrError14_).PropagateFailure()
                        return manifestData
                    d_29_valueOrError15_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
                    d_29_valueOrError15_ = JSONHelpers.default__.GetObject(_dafny.Seq("plaintexts"), (d_5_manifestJson_).obj)
                    if (d_29_valueOrError15_).IsFailure():
                        manifestData = (d_29_valueOrError15_).PropagateFailure()
                        return manifestData
                    d_30_plaintextsJson_: _dafny.Seq
                    d_30_plaintextsJson_ = (d_29_valueOrError15_).Extract()
                    d_31_valueOrError17_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
                    def lambda1_(d_32_obj_):
                        def iife0_(_pat_let7_0):
                            def iife1_(d_33_valueOrError16_):
                                return ((d_33_valueOrError16_).PropagateFailure() if (d_33_valueOrError16_).IsFailure() else Wrappers.Result_Success(((d_32_obj_)[0], (((d_32_obj_)[1]).num).n)))
                            return iife1_(_pat_let7_0)
                        return iife0_(Wrappers.default__.Need((((d_32_obj_)[1]).is_Number) and (((0) < ((((d_32_obj_)[1]).num).n)) and (((((d_32_obj_)[1]).num).n) <= (BoundedInts.default__.INT32__MAX))), _dafny.Seq("Size is not a natural number.")))

                    d_31_valueOrError17_ = Seq.default__.MapWithResult(lambda1_, d_30_plaintextsJson_)
                    if (d_31_valueOrError17_).IsFailure():
                        manifestData = (d_31_valueOrError17_).PropagateFailure()
                        return manifestData
                    d_34_plaintextsLength_: _dafny.Seq
                    d_34_plaintextsLength_ = (d_31_valueOrError17_).Extract()
                    manifestData = Wrappers.Result_Success(ManifestData_EncryptManifest(d_10_version_, d_24_keys_, d_34_plaintextsLength_, d_26_jsonTests_))
                    raise _dafny.Break("match0")
            if True:
                manifestData = Wrappers.Result_Failure((_dafny.Seq("Unsupported manifest type:")) + (d_12_typ_))
            pass
        return manifestData


class ManifestData:
    @classmethod
    def default(cls, ):
        return lambda: ManifestData_DecryptManifest(int(0), None, _dafny.Seq(""), _dafny.Seq(""), _dafny.Seq({}))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_DecryptManifest(self) -> bool:
        return isinstance(self, ManifestData_DecryptManifest)
    @property
    def is_EncryptManifest(self) -> bool:
        return isinstance(self, ManifestData_EncryptManifest)

class ManifestData_DecryptManifest(ManifestData, NamedTuple('DecryptManifest', [('version', Any), ('keys', Any), ('clientName', Any), ('clientVersion', Any), ('jsonTests', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestManifests.ManifestData.DecryptManifest({_dafny.string_of(self.version)}, {_dafny.string_of(self.keys)}, {_dafny.string_of(self.clientName)}, {_dafny.string_of(self.clientVersion)}, {_dafny.string_of(self.jsonTests)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ManifestData_DecryptManifest) and self.version == __o.version and self.keys == __o.keys and self.clientName == __o.clientName and self.clientVersion == __o.clientVersion and self.jsonTests == __o.jsonTests
    def __hash__(self) -> int:
        return super().__hash__()

class ManifestData_EncryptManifest(ManifestData, NamedTuple('EncryptManifest', [('version', Any), ('keys', Any), ('plaintext', Any), ('jsonTests', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestManifests.ManifestData.EncryptManifest({_dafny.string_of(self.version)}, {_dafny.string_of(self.keys)}, {_dafny.string_of(self.plaintext)}, {_dafny.string_of(self.jsonTests)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ManifestData_EncryptManifest) and self.version == __o.version and self.keys == __o.keys and self.plaintext == __o.plaintext and self.jsonTests == __o.jsonTests
    def __hash__(self) -> int:
        return super().__hash__()

