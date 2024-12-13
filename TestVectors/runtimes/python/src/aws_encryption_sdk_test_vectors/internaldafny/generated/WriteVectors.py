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

# Module: WriteVectors

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def GetCommitmentPolicyString(algorithmSuite):
        source0_ = (algorithmSuite).id
        if True:
            if source0_.is_ESDK:
                if ((algorithmSuite).commitment).is_None:
                    return _dafny.Seq("FORBID_ENCRYPT_ALLOW_DECRYPT")
                elif True:
                    return _dafny.Seq("REQUIRE_ENCRYPT_REQUIRE_DECRYPT")
        if True:
            return _dafny.Seq("NOT SUPPORTED FOR UNSTRUCTURED ENCRYPTION")

    @staticmethod
    def GetCommitmentPolicyType(commitmentPolicy):
        if (commitmentPolicy) == (_dafny.Seq("FORBID_ENCRYPT_ALLOW_DECRYPT")):
            return AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT())
        elif True:
            return AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_REQUIRE__ENCRYPT__REQUIRE__DECRYPT())

    @staticmethod
    def WriteTestVectors(op):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_version_: int
        d_0_version_ = (op).version
        d_1_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Set)()
        d_1_valueOrError0_ = default__.getVersionTests(d_0_version_)
        if (d_1_valueOrError0_).IsFailure():
            output = (d_1_valueOrError0_).PropagateFailure()
            return output
        d_2_allTests_: _dafny.Set
        d_2_allTests_ = (d_1_valueOrError0_).Extract()
        d_3_tests_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = SortedSets.default__.SetToSequence(d_2_allTests_)
        d_3_tests_ = out0_
        d_4_sortedTests_: _dafny.Seq
        d_4_sortedTests_ = Seq_MergeSort.default__.MergeSortBy(d_3_tests_, default__.DescriptionLessThan)
        d_5_testsJSON_: _dafny.Seq
        d_5_testsJSON_ = _dafny.Seq([])
        hi0_ = len(d_4_sortedTests_)
        for d_6_i_ in range(0, hi0_):
            d_7_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_7_valueOrError1_ = Wrappers.default__.Need((True) and ((((d_4_sortedTests_)[d_6_i_]).algorithmSuiteId).is_Some), _dafny.Seq("No algorithm suite defined in test"))
            if (d_7_valueOrError1_).IsFailure():
                output = (d_7_valueOrError1_).PropagateFailure()
                return output
            d_8_id_: _dafny.Seq
            d_8_id_ = AllAlgorithmSuites.default__.ToHex((((d_4_sortedTests_)[d_6_i_]).algorithmSuiteId).value)
            d_9_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            out1_: Wrappers.Result
            out1_ = UUID.default__.GenerateUUID()
            d_9_valueOrError2_ = out1_
            if not(not((d_9_valueOrError2_).IsFailure())):
                raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(82,18): " + _dafny.string_of(d_9_valueOrError2_))
            d_10_uuid_: _dafny.Seq
            d_10_uuid_ = (d_9_valueOrError2_).Extract()
            d_11_valueOrError3_: Wrappers.Result = Wrappers.Result.default(JSON_Values.JSON.default())()
            d_11_valueOrError3_ = WriteEsdkJsonManifests.default__.EncryptTestVectorToJson((d_4_sortedTests_)[d_6_i_])
            if (d_11_valueOrError3_).IsFailure():
                output = (d_11_valueOrError3_).PropagateFailure()
                return output
            d_12_test_: JSON_Values.JSON
            d_12_test_ = (d_11_valueOrError3_).Extract()
            d_5_testsJSON_ = (d_5_testsJSON_) + (_dafny.Seq([(d_10_uuid_, d_12_test_)]))
        d_13_manifestJson_: JSON_Values.JSON
        d_13_manifestJson_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("type"), JSON_Values.JSON_String(_dafny.Seq("awses-encrypt"))), (_dafny.Seq("version"), JSON_Values.JSON_Number(JSON_Values.default__.Int(5)))]))
        d_14_plaintexts_: JSON_Values.JSON
        d_14_plaintexts_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("small"), JSON_Values.JSON_Number(JSON_Values.default__.Int(10240)))]))
        d_15_esdkEncryptManifests_: JSON_Values.JSON
        d_15_esdkEncryptManifests_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("manifest"), d_13_manifestJson_), (_dafny.Seq("keys"), JSON_Values.JSON_String(_dafny.Seq("file://keys.json"))), (_dafny.Seq("plaintexts"), d_14_plaintexts_), (_dafny.Seq("tests"), JSON_Values.JSON_Object(d_5_testsJSON_))]))
        d_16_valueOrError4_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_16_valueOrError4_ = JSON_API.default__.Serialize(d_15_esdkEncryptManifests_)
        if not(not((d_16_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(102,36): " + _dafny.string_of(d_16_valueOrError4_))
        d_17_esdkEncryptManifestBytes_: _dafny.Seq
        d_17_esdkEncryptManifestBytes_ = (d_16_valueOrError4_).Extract()
        d_18_esdkEncryptManifestBv_: _dafny.Seq
        d_18_esdkEncryptManifestBv_ = JSONHelpers.default__.BytesBv(d_17_esdkEncryptManifestBytes_)
        d_19_valueOrError5_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        out2_: Wrappers.Result
        out2_ = FileIO.default__.WriteBytesToFile(((op).encryptManifestOutput) + (_dafny.Seq("encrypt-manifest.json")), d_18_esdkEncryptManifestBv_)
        d_19_valueOrError5_ = out2_
        if not(not((d_19_valueOrError5_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(105,13): " + _dafny.string_of(d_19_valueOrError5_))
        d_20___v2_: tuple
        d_20___v2_ = (d_19_valueOrError5_).Extract()
        output = Wrappers.Result_Success(())
        return output

    @staticmethod
    def WriteDecryptManifest(op, keys, tests):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_testsJSON_: _dafny.Seq
        d_0_testsJSON_ = _dafny.Seq([])
        hi0_ = len(tests)
        for d_1_i_ in range(0, hi0_):
            d_2_name_: _dafny.Seq
            d_2_name_ = ((tests)[d_1_i_]).id
            d_3_valueOrError0_: Wrappers.Result = Wrappers.Result.default(JSON_Values.JSON.default())()
            d_3_valueOrError0_ = WriteEsdkJsonManifests.default__.DecryptTestVectorToJson((tests)[d_1_i_])
            if (d_3_valueOrError0_).IsFailure():
                output = (d_3_valueOrError0_).PropagateFailure()
                return output
            d_4_test_: JSON_Values.JSON
            d_4_test_ = (d_3_valueOrError0_).Extract()
            d_0_testsJSON_ = (d_0_testsJSON_) + (_dafny.Seq([(d_2_name_, d_4_test_)]))
        d_5_manifestJson_: JSON_Values.JSON
        d_5_manifestJson_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("type"), JSON_Values.JSON_String(_dafny.Seq("awses-decrypt"))), (_dafny.Seq("version"), JSON_Values.JSON_Number(JSON_Values.default__.Int(3)))]))
        d_6_esdkDecryptManifest_: JSON_Values.JSON
        d_6_esdkDecryptManifest_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("manifest"), d_5_manifestJson_), (_dafny.Seq("client"), JSON_Values.JSON_String(_dafny.Seq("aws-encryption-sdk-dafny"))), (_dafny.Seq("keys"), JSON_Values.JSON_String(_dafny.Seq("file://keys.json"))), (_dafny.Seq("tests"), JSON_Values.JSON_Object(d_0_testsJSON_))]))
        d_7_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_7_valueOrError1_ = JSON_API.default__.Serialize(d_6_esdkDecryptManifest_)
        if not(not((d_7_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(146,36): " + _dafny.string_of(d_7_valueOrError1_))
        d_8_esdkDecryptManifestBytes_: _dafny.Seq
        d_8_esdkDecryptManifestBytes_ = (d_7_valueOrError1_).Extract()
        d_9_esdkDecryptManifestBv_: _dafny.Seq
        d_9_esdkDecryptManifestBv_ = JSONHelpers.default__.BytesBv(d_8_esdkDecryptManifestBytes_)
        d_10_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        out0_: Wrappers.Result
        out0_ = FileIO.default__.WriteBytesToFile(((op).decryptManifestOutput) + (_dafny.Seq("decrypt-manifest.json")), d_9_esdkDecryptManifestBv_)
        d_10_valueOrError2_ = out0_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(149,13): " + _dafny.string_of(d_10_valueOrError2_))
        d_11___v3_: tuple
        d_11___v3_ = (d_10_valueOrError2_).Extract()
        output = Wrappers.Result_Success(())
        return output

    @staticmethod
    def getVersionTests(version):
        source0_ = version
        if True:
            if (source0_) == (5):
                return Wrappers.Result_Success((AllEsdkV4NoReqEc.default__.Tests) | (AllEsdkV4WithReqEc.default__.Tests))
        if True:
            return Wrappers.Result_Failure(_dafny.Seq("Only version 4 of generate manifest is supported\n"))

    @staticmethod
    def DescriptionLessThan(x, y):
        return default__.Below((x).description, (y).description)

    @staticmethod
    def Below(x, y):
        return not ((len(x)) != (0)) or ((((len(y)) != (0)) and (((x)[0]) <= ((y)[0]))) and (not (((x)[0]) == ((y)[0])) or (default__.Below(_dafny.Seq((x)[1::]), _dafny.Seq((y)[1::])))))

