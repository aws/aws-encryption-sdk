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
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_Sequence as StandardLibrary_Sequence
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

# Module: TestEncryptDecrypt

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def TestEncryptDecrypt():
        d_0_kmsKey_: _dafny.Seq
        d_0_kmsKey_ = Fixtures.default__.keyArn
        d_1_asdf_: _dafny.Seq
        d_1_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_2_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_2_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_3_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_2_defaultConfig_)
        d_3_valueOrError0_ = out0_
        if not(not((d_3_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(24,20): " + _dafny.string_of(d_3_valueOrError0_))
        d_4_esdk_: EncryptionSdk.ESDKClient
        d_4_esdk_ = (d_3_valueOrError0_).Extract()
        d_5_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_5_valueOrError1_ = out1_
        if not(not((d_5_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(25,19): " + _dafny.string_of(d_5_valueOrError1_))
        d_6_mpl_: MaterialProviders.MaterialProvidersClient
        d_6_mpl_ = (d_5_valueOrError1_).Extract()
        d_7_valueOrError2_: Wrappers.Result = None
        out2_: Wrappers.Result
        out2_ = (d_6_mpl_).CreateDefaultClientSupplier(AwsCryptographyMaterialProvidersTypes.CreateDefaultClientSupplierInput_CreateDefaultClientSupplierInput())
        d_7_valueOrError2_ = out2_
        if not(not((d_7_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(26,30): " + _dafny.string_of(d_7_valueOrError2_))
        d_8_clientSupplier_: AwsCryptographyMaterialProvidersTypes.IClientSupplier
        d_8_clientSupplier_ = (d_7_valueOrError2_).Extract()
        d_9_valueOrError3_: Wrappers.Result = None
        out3_: Wrappers.Result
        out3_ = (d_8_clientSupplier_).GetClient(AwsCryptographyMaterialProvidersTypes.GetClientInput_GetClientInput(_dafny.Seq("us-west-2")))
        d_9_valueOrError3_ = out3_
        if not(not((d_9_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(27,25): " + _dafny.string_of(d_9_valueOrError3_))
        d_10_kmsClient_: ComAmazonawsKmsTypes.IKMSClient
        d_10_kmsClient_ = (d_9_valueOrError3_).Extract()
        d_11_valueOrError4_: Wrappers.Result = None
        out4_: Wrappers.Result
        out4_ = (d_6_mpl_).CreateAwsKmsKeyring(AwsCryptographyMaterialProvidersTypes.CreateAwsKmsKeyringInput_CreateAwsKmsKeyringInput(d_0_kmsKey_, d_10_kmsClient_, Wrappers.Option_None()))
        d_11_valueOrError4_ = out4_
        if not(not((d_11_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(29,26): " + _dafny.string_of(d_11_valueOrError4_))
        d_12_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_12_kmsKeyring_ = (d_11_valueOrError4_).Extract()
        d_13_encryptOutput_: Wrappers.Result
        out5_: Wrappers.Result
        out5_ = (d_4_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_1_asdf_, Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_Some(d_12_kmsKeyring_), Wrappers.Option_None(), Wrappers.Option_None()))
        d_13_encryptOutput_ = out5_
        if not((d_13_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(46,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_14_esdkCiphertext_: _dafny.Seq
        d_14_esdkCiphertext_ = ((d_13_encryptOutput_).value).ciphertext
        d_15_decryptOutput_: Wrappers.Result
        out6_: Wrappers.Result
        out6_ = (d_4_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_14_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_12_kmsKeyring_), Wrappers.Option_None()))
        d_15_decryptOutput_ = out6_
        if not((d_15_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(56,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_16_cycledPlaintext_: _dafny.Seq
        d_16_cycledPlaintext_ = ((d_15_decryptOutput_).value).plaintext
        if not((d_16_cycledPlaintext_) == (d_1_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestEncryptDecrypt.dfy(59,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

