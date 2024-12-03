import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import module_ as module_
import _dafny as _dafny
import System_ as System_
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import smithy_dafny_standard_library.internaldafny.generated.BoundedInts as BoundedInts
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_UInt as StandardLibrary_UInt
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_String as StandardLibrary_String
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary as StandardLibrary
import smithy_dafny_standard_library.internaldafny.generated.UTF8 as UTF8
import aws_cryptography_internal_dynamodb.internaldafny.generated.ComAmazonawsDynamodbTypes as ComAmazonawsDynamodbTypes
import aws_cryptography_internal_kms.internaldafny.generated.ComAmazonawsKmsTypes as ComAmazonawsKmsTypes
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyKeyStoreTypes as AwsCryptographyKeyStoreTypes
import aws_cryptography_primitives.internaldafny.generated.AwsCryptographyPrimitivesTypes as AwsCryptographyPrimitivesTypes
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyMaterialProvidersTypes as AwsCryptographyMaterialProvidersTypes
import aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
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
import aws_cryptography_primitives.internaldafny.generated.AesKdfCtr as AesKdfCtr
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
import smithy_dafny_standard_library.internaldafny.generated.Sorting as Sorting
import smithy_dafny_standard_library.internaldafny.generated.HexStrings as HexStrings
import smithy_dafny_standard_library.internaldafny.generated.GetOpt as GetOpt
import smithy_dafny_standard_library.internaldafny.generated.FloatCompare as FloatCompare
import smithy_dafny_standard_library.internaldafny.generated.ConcurrentCall as ConcurrentCall
import smithy_dafny_standard_library.internaldafny.generated.Base64Lemmas as Base64Lemmas
import Fixtures as Fixtures
import TestRequiredEncryptionContext as TestRequiredEncryptionContext
import TestReproducedEncryptionContext as TestReproducedEncryptionContext
import TestEncryptDecrypt as TestEncryptDecrypt

# Module: TestCreateEsdkClient

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def TestClientCreation():
        d_0_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_0_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_1_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_0_defaultConfig_)
        d_1_valueOrError0_ = out0_
        if not(not((d_1_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(55,51): " + _dafny.string_of(d_1_valueOrError0_))
        d_2_esdk_: AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
        d_2_esdk_ = (d_1_valueOrError0_).Extract()
        def iife0_(_is_0):
            return isinstance(_is_0, EncryptionSdk.ESDKClient)
        if not(iife0_(d_2_esdk_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(56,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_3_esdkClient_: EncryptionSdk.ESDKClient
        d_3_esdkClient_ = d_2_esdk_
        if not((((d_3_esdkClient_).config).commitmentPolicy) == (((d_0_defaultConfig_).commitmentPolicy).value)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(59,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        if not((((d_3_esdkClient_).config).maxEncryptedDataKeys) == ((d_0_defaultConfig_).maxEncryptedDataKeys)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(60,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        if not((((d_3_esdkClient_).config).netV4__0__0__RetryPolicy) == (AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy_ALLOW__RETRY())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(61,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestNetRetryFlag():
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_0_valueOrError0_ = out0_
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(65,19): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_mpl_: MaterialProviders.MaterialProvidersClient
        d_1_mpl_ = (d_0_valueOrError0_).Extract()
        d_2_keyNamespace_: _dafny.Seq
        d_2_keyNamespace_ = _dafny.Seq("Some managed raw keys")
        d_3_keyName_: _dafny.Seq
        d_3_keyName_ = _dafny.Seq("My 256-bit AES wrapping key")
        d_4_expectedMessage_: _dafny.Seq
        d_4_expectedMessage_ = _dafny.Seq([84, 104, 105, 115, 32, 105, 115, 32, 97, 32, 116, 101, 115, 116, 46])
        d_5_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = (d_1_mpl_).CreateRawAesKeyring(AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput_CreateRawAesKeyringInput(d_2_keyNamespace_, d_3_keyName_, _dafny.Seq([0 for d_6_i_ in range(32)]), AwsCryptographyMaterialProvidersTypes.AesWrappingAlg_ALG__AES256__GCM__IV12__TAG16()))
        d_5_valueOrError1_ = out1_
        if not(not((d_5_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(70,29): " + _dafny.string_of(d_5_valueOrError1_))
        d_7_rawAesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_7_rawAesKeyring_ = (d_5_valueOrError1_).Extract()
        d_8_esdkConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_8_esdkConfig_ = AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig_AwsEncryptionSdkConfig(Wrappers.Option_Some(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_REQUIRE__ENCRYPT__REQUIRE__DECRYPT()), Wrappers.Option_None(), Wrappers.Option_Some(AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy_FORBID__RETRY()))
        d_9_valueOrError2_: Wrappers.Result = None
        out2_: Wrappers.Result
        out2_ = EncryptionSdk.default__.ESDK(d_8_esdkConfig_)
        d_9_valueOrError2_ = out2_
        if not(not((d_9_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(85,27): " + _dafny.string_of(d_9_valueOrError2_))
        d_10_noRetryEsdk_: EncryptionSdk.ESDKClient
        d_10_noRetryEsdk_ = (d_9_valueOrError2_).Extract()
        d_11_expectFailureDecryptOutput_: Wrappers.Result
        out3_: Wrappers.Result
        out3_ = (d_10_noRetryEsdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(default__.ESDK__NET__V400__MESSAGE, Wrappers.Option_None(), Wrappers.Option_Some(d_7_rawAesKeyring_), Wrappers.Option_None()))
        d_11_expectFailureDecryptOutput_ = out3_
        if not((d_11_expectFailureDecryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(94,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_12_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_12_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_13_valueOrError3_: Wrappers.Result = None
        out4_: Wrappers.Result
        out4_ = EncryptionSdk.default__.ESDK(d_12_defaultConfig_)
        d_13_valueOrError3_ = out4_
        if not(not((d_13_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(99,20): " + _dafny.string_of(d_13_valueOrError3_))
        d_14_esdk_: EncryptionSdk.ESDKClient
        d_14_esdk_ = (d_13_valueOrError3_).Extract()
        d_15_decryptOutput_: Wrappers.Result
        out5_: Wrappers.Result
        out5_ = (d_14_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(default__.ESDK__NET__V400__MESSAGE, Wrappers.Option_None(), Wrappers.Option_Some(d_7_rawAesKeyring_), Wrappers.Option_None()))
        d_15_decryptOutput_ = out5_
        _dafny.print(_dafny.string_of(d_15_decryptOutput_))
        if not((d_15_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(108,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        if not((((d_15_decryptOutput_).value).plaintext) == (d_4_expectedMessage_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(109,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @_dafny.classproperty
    def ESDK__NET__V400__MESSAGE(instance):
        return _dafny.Seq([2, 5, 120, 238, 5, 239, 107, 129, 136, 211, 103, 75, 18, 140, 11, 74, 26, 191, 92, 27, 202, 170, 33, 28, 9, 117, 252, 29, 29, 92, 213, 21, 231, 172, 234, 0, 95, 0, 1, 0, 21, 97, 119, 115, 45, 99, 114, 121, 112, 116, 111, 45, 112, 117, 98, 108, 105, 99, 45, 107, 101, 121, 0, 68, 65, 119, 102, 117, 103, 90, 99, 107, 57, 116, 100, 53, 104, 78, 108, 49, 78, 108, 75, 111, 47, 104, 105, 114, 53, 85, 47, 48, 81, 109, 98, 73, 111, 107, 79, 72, 81, 87, 97, 72, 83, 43, 115, 117, 119, 75, 73, 77, 82, 76, 99, 67, 80, 49, 54, 55, 56, 43, 49, 82, 75, 49, 48, 82, 101, 119, 61, 61, 0, 1, 0, 21, 83, 111, 109, 101, 32, 109, 97, 110, 97, 103, 101, 100, 32, 114, 97, 119, 32, 107, 101, 121, 115, 0, 47, 77, 121, 32, 50, 53, 54, 45, 98, 105, 116, 32, 65, 69, 83, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 107, 101, 121, 0, 0, 0, 128, 0, 0, 0, 12, 229, 254, 197, 205, 110, 124, 222, 48, 217, 121, 252, 11, 0, 48, 64, 60, 232, 232, 76, 229, 15, 118, 224, 152, 79, 93, 113, 166, 255, 172, 255, 148, 185, 150, 195, 179, 78, 52, 186, 38, 216, 48, 118, 45, 113, 204, 71, 102, 116, 148, 199, 109, 178, 19, 2, 203, 150, 201, 65, 32, 199, 180, 2, 0, 0, 16, 0, 67, 72, 208, 112, 230, 137, 188, 187, 0, 28, 183, 198, 192, 45, 248, 108, 2, 129, 34, 42, 59, 155, 70, 117, 182, 216, 239, 27, 210, 78, 62, 104, 181, 247, 141, 50, 133, 42, 72, 200, 185, 57, 20, 49, 193, 240, 171, 140, 255, 255, 255, 255, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 15, 67, 37, 106, 11, 15, 23, 78, 239, 208, 185, 4, 36, 182, 9, 63, 62, 83, 97, 42, 250, 252, 185, 165, 14, 182, 231, 83, 176, 227, 191, 92, 0, 103, 48, 101, 2, 49, 0, 193, 152, 7, 169, 197, 137, 244, 88, 9, 1, 6, 56, 96, 13, 220, 201, 56, 16, 50, 68, 70, 36, 174, 38, 14, 241, 207, 11, 139, 154, 166, 224, 191, 20, 12, 175, 56, 117, 183, 120, 119, 228, 173, 130, 71, 110, 211, 189, 2, 48, 99, 98, 250, 36, 53, 182, 2, 204, 198, 55, 150, 51, 159, 101, 231, 34, 42, 30, 57, 204, 88, 114, 138, 94, 12, 79, 52, 71, 178, 34, 61, 246, 55, 163, 145, 95, 80, 61, 85, 143, 32, 0, 98, 20, 88, 251, 204, 5])
