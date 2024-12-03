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

# Module: TestRequiredEncryptionContext

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def TestReprEncryptionContextWithSameECHappyCase():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(39,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(40,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(48,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_encryptOutput_: Wrappers.Result
        out8_: Wrappers.Result
        out8_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_None(), Wrappers.Option_Some(d_11_multiKeyring_), Wrappers.Option_None(), Wrappers.Option_None()))
        d_13_encryptOutput_ = out8_
        if not((d_13_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(66,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_14_esdkCiphertext_: _dafny.Seq
        d_14_esdkCiphertext_ = ((d_13_encryptOutput_).value).ciphertext
        d_15_decryptOutput_: Wrappers.Result
        out9_: Wrappers.Result
        out9_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_14_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_Some(d_12_encryptionContext_)))
        d_15_decryptOutput_ = out9_
        if not((d_15_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(77,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_16_cycledPlaintext_: _dafny.Seq
        d_16_cycledPlaintext_ = ((d_15_decryptOutput_).value).plaintext
        if not((d_16_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(79,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out10_: Wrappers.Result
        out10_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_14_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_Some(d_12_encryptionContext_)))
        d_15_decryptOutput_ = out10_
        if not((d_15_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(89,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_16_cycledPlaintext_ = ((d_15_decryptOutput_).value).plaintext
        if not((d_16_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(91,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out11_: Wrappers.Result
        out11_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_14_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_Some(d_12_encryptionContext_)))
        d_15_decryptOutput_ = out11_
        if not((d_15_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(101,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_16_cycledPlaintext_ = ((d_15_decryptOutput_).value).plaintext
        if not((d_16_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(103,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_14_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_Some(d_12_encryptionContext_)))
        d_15_decryptOutput_ = out12_
        if not((d_15_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(113,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_16_cycledPlaintext_ = ((d_15_decryptOutput_).value).plaintext
        if not((d_16_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(115,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestRemoveOnEncryptAndSupplyOnDecryptHappyCase():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(124,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(125,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(133,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_reproducedEncryptionContext_: _dafny.Map
        out8_: _dafny.Map
        out8_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_A())
        d_13_reproducedEncryptionContext_ = out8_
        d_14_requiredEncryptionContextKeys_: _dafny.Seq
        out9_: _dafny.Seq
        out9_ = Fixtures.default__.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation_A())
        d_14_requiredEncryptionContextKeys_ = out9_
        d_15_valueOrError3_: Wrappers.Result = None
        out10_: Wrappers.Result
        out10_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_11_multiKeyring_))
        d_15_valueOrError3_ = out10_
        if not(not((d_15_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(148,26): " + _dafny.string_of(d_15_valueOrError3_))
        d_16_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_16_defaultCMM_ = (d_15_valueOrError3_).Extract()
        d_17_valueOrError4_: Wrappers.Result = None
        out11_: Wrappers.Result
        out11_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_17_valueOrError4_ = out11_
        if not(not((d_17_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(155,22): " + _dafny.string_of(d_17_valueOrError4_))
        d_18_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_18_reqCMM_ = (d_17_valueOrError4_).Extract()
        d_19_encryptOutput_: Wrappers.Result
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_19_encryptOutput_ = out12_
        if not((d_19_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(174,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_20_esdkCiphertext_: _dafny.Seq
        d_20_esdkCiphertext_ = ((d_19_encryptOutput_).value).ciphertext
        d_21_decryptOutput_: Wrappers.Result
        out13_: Wrappers.Result
        out13_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_21_decryptOutput_ = out13_
        if not((d_21_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(185,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_22_cycledPlaintext_: _dafny.Seq
        d_22_cycledPlaintext_ = ((d_21_decryptOutput_).value).plaintext
        if not((d_22_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(187,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out14_: Wrappers.Result
        out14_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_21_decryptOutput_ = out14_
        if not((d_21_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(197,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_22_cycledPlaintext_ = ((d_21_decryptOutput_).value).plaintext
        if not((d_22_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(199,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out15_: Wrappers.Result
        out15_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_21_decryptOutput_ = out15_
        if not((d_21_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(209,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_22_cycledPlaintext_ = ((d_21_decryptOutput_).value).plaintext
        if not((d_22_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(211,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out16_: Wrappers.Result
        out16_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_21_decryptOutput_ = out16_
        if not((d_21_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(221,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_22_cycledPlaintext_ = ((d_21_decryptOutput_).value).plaintext
        if not((d_22_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(223,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestRemoveOnEncryptRemoveAndSupplyOnDecryptHappyCase():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(233,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(234,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(242,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_reproducedEncryptionContext_: _dafny.Map
        out8_: _dafny.Map
        out8_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_A())
        d_13_reproducedEncryptionContext_ = out8_
        d_14_requiredEncryptionContextKeys_: _dafny.Seq
        out9_: _dafny.Seq
        out9_ = Fixtures.default__.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation_A())
        d_14_requiredEncryptionContextKeys_ = out9_
        d_15_valueOrError3_: Wrappers.Result = None
        out10_: Wrappers.Result
        out10_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_11_multiKeyring_))
        d_15_valueOrError3_ = out10_
        if not(not((d_15_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(257,26): " + _dafny.string_of(d_15_valueOrError3_))
        d_16_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_16_defaultCMM_ = (d_15_valueOrError3_).Extract()
        d_17_valueOrError4_: Wrappers.Result = None
        out11_: Wrappers.Result
        out11_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_17_valueOrError4_ = out11_
        if not(not((d_17_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(264,22): " + _dafny.string_of(d_17_valueOrError4_))
        d_18_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_18_reqCMM_ = (d_17_valueOrError4_).Extract()
        d_19_encryptOutput_: Wrappers.Result
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_19_encryptOutput_ = out12_
        if not((d_19_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(283,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_20_esdkCiphertext_: _dafny.Seq
        d_20_esdkCiphertext_ = ((d_19_encryptOutput_).value).ciphertext
        d_21_valueOrError5_: Wrappers.Result = None
        out13_: Wrappers.Result
        out13_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_6_rsaKeyring_))
        d_21_valueOrError5_ = out13_
        if not(not((d_21_valueOrError5_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(287,22): " + _dafny.string_of(d_21_valueOrError5_))
        d_16_defaultCMM_ = (d_21_valueOrError5_).Extract()
        d_22_valueOrError6_: Wrappers.Result = None
        out14_: Wrappers.Result
        out14_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_22_valueOrError6_ = out14_
        if not(not((d_22_valueOrError6_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(294,18): " + _dafny.string_of(d_22_valueOrError6_))
        d_18_reqCMM_ = (d_22_valueOrError6_).Extract()
        d_23_decryptOutput_: Wrappers.Result
        out15_: Wrappers.Result
        out15_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_23_decryptOutput_ = out15_
        if not((d_23_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(312,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_24_cycledPlaintext_: _dafny.Seq
        d_24_cycledPlaintext_ = ((d_23_decryptOutput_).value).plaintext
        if not((d_24_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(314,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_25_valueOrError7_: Wrappers.Result = None
        out16_: Wrappers.Result
        out16_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_7_kmsKeyring_))
        d_25_valueOrError7_ = out16_
        if not(not((d_25_valueOrError7_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(318,22): " + _dafny.string_of(d_25_valueOrError7_))
        d_16_defaultCMM_ = (d_25_valueOrError7_).Extract()
        d_26_valueOrError8_: Wrappers.Result = None
        out17_: Wrappers.Result
        out17_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_26_valueOrError8_ = out17_
        if not(not((d_26_valueOrError8_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(325,18): " + _dafny.string_of(d_26_valueOrError8_))
        d_18_reqCMM_ = (d_26_valueOrError8_).Extract()
        out18_: Wrappers.Result
        out18_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_23_decryptOutput_ = out18_
        if not((d_23_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(343,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_24_cycledPlaintext_ = ((d_23_decryptOutput_).value).plaintext
        if not((d_24_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(345,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_27_valueOrError9_: Wrappers.Result = None
        out19_: Wrappers.Result
        out19_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_8_aesKeyring_))
        d_27_valueOrError9_ = out19_
        if not(not((d_27_valueOrError9_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(349,22): " + _dafny.string_of(d_27_valueOrError9_))
        d_16_defaultCMM_ = (d_27_valueOrError9_).Extract()
        d_28_valueOrError10_: Wrappers.Result = None
        out20_: Wrappers.Result
        out20_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_28_valueOrError10_ = out20_
        if not(not((d_28_valueOrError10_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(356,18): " + _dafny.string_of(d_28_valueOrError10_))
        d_18_reqCMM_ = (d_28_valueOrError10_).Extract()
        out21_: Wrappers.Result
        out21_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_23_decryptOutput_ = out21_
        if not((d_23_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(374,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_24_cycledPlaintext_ = ((d_23_decryptOutput_).value).plaintext
        if not((d_24_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(376,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_29_valueOrError11_: Wrappers.Result = None
        out22_: Wrappers.Result
        out22_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_9_hKeyring_))
        d_29_valueOrError11_ = out22_
        if not(not((d_29_valueOrError11_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(379,22): " + _dafny.string_of(d_29_valueOrError11_))
        d_16_defaultCMM_ = (d_29_valueOrError11_).Extract()
        d_30_valueOrError12_: Wrappers.Result = None
        out23_: Wrappers.Result
        out23_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_30_valueOrError12_ = out23_
        if not(not((d_30_valueOrError12_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(386,18): " + _dafny.string_of(d_30_valueOrError12_))
        d_18_reqCMM_ = (d_30_valueOrError12_).Extract()
        out24_: Wrappers.Result
        out24_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_23_decryptOutput_ = out24_
        if not((d_23_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(404,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_24_cycledPlaintext_ = ((d_23_decryptOutput_).value).plaintext
        if not((d_24_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(406,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestRemoveOnDecryptIsBackwardsCompatibleHappyCase():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(415,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(416,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(424,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_reproducedEncryptionContext_: _dafny.Map
        out8_: _dafny.Map
        out8_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_A())
        d_13_reproducedEncryptionContext_ = out8_
        d_14_requiredEncryptionContextKeys_: _dafny.Seq
        out9_: _dafny.Seq
        out9_ = Fixtures.default__.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation_A())
        d_14_requiredEncryptionContextKeys_ = out9_
        d_15_valueOrError3_: Wrappers.Result = None
        out10_: Wrappers.Result
        out10_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_11_multiKeyring_))
        d_15_valueOrError3_ = out10_
        if not(not((d_15_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(439,26): " + _dafny.string_of(d_15_valueOrError3_))
        d_16_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_16_defaultCMM_ = (d_15_valueOrError3_).Extract()
        d_17_encryptOutput_: Wrappers.Result
        out11_: Wrappers.Result
        out11_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_17_encryptOutput_ = out11_
        if not((d_17_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(455,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_18_esdkCiphertext_: _dafny.Seq
        d_18_esdkCiphertext_ = ((d_17_encryptOutput_).value).ciphertext
        d_19_valueOrError4_: Wrappers.Result = None
        out12_: Wrappers.Result
        out12_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_6_rsaKeyring_))
        d_19_valueOrError4_ = out12_
        if not(not((d_19_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(459,22): " + _dafny.string_of(d_19_valueOrError4_))
        d_16_defaultCMM_ = (d_19_valueOrError4_).Extract()
        d_20_valueOrError5_: Wrappers.Result = None
        out13_: Wrappers.Result
        out13_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_20_valueOrError5_ = out13_
        if not(not((d_20_valueOrError5_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(466,22): " + _dafny.string_of(d_20_valueOrError5_))
        d_21_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_21_reqCMM_ = (d_20_valueOrError5_).Extract()
        d_22_decryptOutput_: Wrappers.Result
        out14_: Wrappers.Result
        out14_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_18_esdkCiphertext_, Wrappers.Option_Some(d_21_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_22_decryptOutput_ = out14_
        if not((d_22_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(484,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_23_cycledPlaintext_: _dafny.Seq
        d_23_cycledPlaintext_ = ((d_22_decryptOutput_).value).plaintext
        if not((d_23_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(486,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_24_valueOrError6_: Wrappers.Result = None
        out15_: Wrappers.Result
        out15_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_7_kmsKeyring_))
        d_24_valueOrError6_ = out15_
        if not(not((d_24_valueOrError6_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(490,22): " + _dafny.string_of(d_24_valueOrError6_))
        d_16_defaultCMM_ = (d_24_valueOrError6_).Extract()
        d_25_valueOrError7_: Wrappers.Result = None
        out16_: Wrappers.Result
        out16_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_25_valueOrError7_ = out16_
        if not(not((d_25_valueOrError7_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(497,18): " + _dafny.string_of(d_25_valueOrError7_))
        d_21_reqCMM_ = (d_25_valueOrError7_).Extract()
        out17_: Wrappers.Result
        out17_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_18_esdkCiphertext_, Wrappers.Option_Some(d_21_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_22_decryptOutput_ = out17_
        if not((d_22_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(515,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_23_cycledPlaintext_ = ((d_22_decryptOutput_).value).plaintext
        if not((d_23_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(517,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_26_valueOrError8_: Wrappers.Result = None
        out18_: Wrappers.Result
        out18_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_8_aesKeyring_))
        d_26_valueOrError8_ = out18_
        if not(not((d_26_valueOrError8_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(521,22): " + _dafny.string_of(d_26_valueOrError8_))
        d_16_defaultCMM_ = (d_26_valueOrError8_).Extract()
        d_27_valueOrError9_: Wrappers.Result = None
        out19_: Wrappers.Result
        out19_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_27_valueOrError9_ = out19_
        if not(not((d_27_valueOrError9_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(528,18): " + _dafny.string_of(d_27_valueOrError9_))
        d_21_reqCMM_ = (d_27_valueOrError9_).Extract()
        out20_: Wrappers.Result
        out20_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_18_esdkCiphertext_, Wrappers.Option_Some(d_21_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_22_decryptOutput_ = out20_
        if not((d_22_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(546,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_23_cycledPlaintext_ = ((d_22_decryptOutput_).value).plaintext
        if not((d_23_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(548,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_28_valueOrError10_: Wrappers.Result = None
        out21_: Wrappers.Result
        out21_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_9_hKeyring_))
        d_28_valueOrError10_ = out21_
        if not(not((d_28_valueOrError10_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(551,22): " + _dafny.string_of(d_28_valueOrError10_))
        d_16_defaultCMM_ = (d_28_valueOrError10_).Extract()
        d_29_valueOrError11_: Wrappers.Result = None
        out22_: Wrappers.Result
        out22_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_14_requiredEncryptionContextKeys_))
        d_29_valueOrError11_ = out22_
        if not(not((d_29_valueOrError11_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(558,18): " + _dafny.string_of(d_29_valueOrError11_))
        d_21_reqCMM_ = (d_29_valueOrError11_).Extract()
        out23_: Wrappers.Result
        out23_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_18_esdkCiphertext_, Wrappers.Option_Some(d_21_reqCMM_), Wrappers.Option_None(), Wrappers.Option_Some(d_13_reproducedEncryptionContext_)))
        d_22_decryptOutput_ = out23_
        if not((d_22_decryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(576,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_23_cycledPlaintext_ = ((d_22_decryptOutput_).value).plaintext
        if not((d_23_cycledPlaintext_) == (d_0_asdf_)):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(578,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestDifferentECOnDecryptFailure():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(590,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(591,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(599,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_reproducedAdditionalEncryptionContext_: _dafny.Map
        out8_: _dafny.Map
        out8_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_C())
        d_13_reproducedAdditionalEncryptionContext_ = out8_
        d_14_reproducedMismatchedEncryptionContext_: _dafny.Map
        out9_: _dafny.Map
        out9_ = Fixtures.default__.SmallMismatchedEncryptionContex(Fixtures.SmallEncryptionContextVariation_AB())
        d_14_reproducedMismatchedEncryptionContext_ = out9_
        d_15_encryptOutput_: Wrappers.Result
        out10_: Wrappers.Result
        out10_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_None(), Wrappers.Option_Some(d_11_multiKeyring_), Wrappers.Option_None(), Wrappers.Option_None()))
        d_15_encryptOutput_ = out10_
        if not((d_15_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(623,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_16_esdkCiphertext_: _dafny.Seq
        d_16_esdkCiphertext_ = ((d_15_encryptOutput_).value).ciphertext
        d_17_decryptOutput_: Wrappers.Result
        out11_: Wrappers.Result
        out11_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_Some(d_13_reproducedAdditionalEncryptionContext_)))
        d_17_decryptOutput_ = out11_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(634,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_Some(d_14_reproducedMismatchedEncryptionContext_)))
        d_17_decryptOutput_ = out12_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(643,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out13_: Wrappers.Result
        out13_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_Some(d_13_reproducedAdditionalEncryptionContext_)))
        d_17_decryptOutput_ = out13_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(653,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out14_: Wrappers.Result
        out14_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_Some(d_14_reproducedMismatchedEncryptionContext_)))
        d_17_decryptOutput_ = out14_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(662,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out15_: Wrappers.Result
        out15_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_Some(d_13_reproducedAdditionalEncryptionContext_)))
        d_17_decryptOutput_ = out15_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(672,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out16_: Wrappers.Result
        out16_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_Some(d_14_reproducedMismatchedEncryptionContext_)))
        d_17_decryptOutput_ = out16_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(681,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out17_: Wrappers.Result
        out17_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_Some(d_13_reproducedAdditionalEncryptionContext_)))
        d_17_decryptOutput_ = out17_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(691,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out18_: Wrappers.Result
        out18_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_16_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_Some(d_14_reproducedMismatchedEncryptionContext_)))
        d_17_decryptOutput_ = out18_
        if not((d_17_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(700,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestRemoveECAndNotSupplyOnDecryptFailure():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(715,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(716,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(724,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_requiredECKeys_: _dafny.Seq
        out8_: _dafny.Seq
        out8_ = Fixtures.default__.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation_A())
        d_13_requiredECKeys_ = out8_
        d_14_valueOrError3_: Wrappers.Result = None
        out9_: Wrappers.Result
        out9_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_11_multiKeyring_))
        d_14_valueOrError3_ = out9_
        if not(not((d_14_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(736,26): " + _dafny.string_of(d_14_valueOrError3_))
        d_15_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_15_defaultCMM_ = (d_14_valueOrError3_).Extract()
        d_16_valueOrError4_: Wrappers.Result = None
        out10_: Wrappers.Result
        out10_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_15_defaultCMM_), Wrappers.Option_None(), d_13_requiredECKeys_))
        d_16_valueOrError4_ = out10_
        if not(not((d_16_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(743,22): " + _dafny.string_of(d_16_valueOrError4_))
        d_17_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_17_reqCMM_ = (d_16_valueOrError4_).Extract()
        d_18_encryptOutput_: Wrappers.Result
        out11_: Wrappers.Result
        out11_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_Some(d_17_reqCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_18_encryptOutput_ = out11_
        if not((d_18_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(762,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_19_esdkCiphertext_: _dafny.Seq
        d_19_esdkCiphertext_ = ((d_18_encryptOutput_).value).ciphertext
        d_20_decryptOutput_: Wrappers.Result
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_19_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_None()))
        d_20_decryptOutput_ = out12_
        if not((d_20_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(773,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out13_: Wrappers.Result
        out13_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_19_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_None()))
        d_20_decryptOutput_ = out13_
        if not((d_20_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(783,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out14_: Wrappers.Result
        out14_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_19_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_None()))
        d_20_decryptOutput_ = out14_
        if not((d_20_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(793,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out15_: Wrappers.Result
        out15_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_19_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_None()))
        d_20_decryptOutput_ = out15_
        if not((d_20_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(803,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestRemoveECAndSupplyMismatchedReprECFailure():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(818,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(819,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(827,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_requiredECKeys_: _dafny.Seq
        out8_: _dafny.Seq
        out8_ = Fixtures.default__.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation_A())
        d_13_requiredECKeys_ = out8_
        d_14_mismatchedReproducedEncryptionContext_: _dafny.Map
        out9_: _dafny.Map
        out9_ = Fixtures.default__.SmallMismatchedEncryptionContex(Fixtures.SmallEncryptionContextVariation_A())
        d_14_mismatchedReproducedEncryptionContext_ = out9_
        d_15_valueOrError3_: Wrappers.Result = None
        out10_: Wrappers.Result
        out10_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_11_multiKeyring_))
        d_15_valueOrError3_ = out10_
        if not(not((d_15_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(841,26): " + _dafny.string_of(d_15_valueOrError3_))
        d_16_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_16_defaultCMM_ = (d_15_valueOrError3_).Extract()
        d_17_valueOrError4_: Wrappers.Result = None
        out11_: Wrappers.Result
        out11_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_13_requiredECKeys_))
        d_17_valueOrError4_ = out11_
        if not(not((d_17_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(848,22): " + _dafny.string_of(d_17_valueOrError4_))
        d_18_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_18_reqCMM_ = (d_17_valueOrError4_).Extract()
        d_19_encryptOutput_: Wrappers.Result
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_19_encryptOutput_ = out12_
        if not((d_19_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(867,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_20_esdkCiphertext_: _dafny.Seq
        d_20_esdkCiphertext_ = ((d_19_encryptOutput_).value).ciphertext
        d_21_decryptOutput_: Wrappers.Result
        out13_: Wrappers.Result
        out13_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_Some(d_14_mismatchedReproducedEncryptionContext_)))
        d_21_decryptOutput_ = out13_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(878,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out14_: Wrappers.Result
        out14_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_Some(d_14_mismatchedReproducedEncryptionContext_)))
        d_21_decryptOutput_ = out14_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(888,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out15_: Wrappers.Result
        out15_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_Some(d_14_mismatchedReproducedEncryptionContext_)))
        d_21_decryptOutput_ = out15_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(898,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out16_: Wrappers.Result
        out16_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_Some(d_14_mismatchedReproducedEncryptionContext_)))
        d_21_decryptOutput_ = out16_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(908,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestRemoveECAndSupplyWithMissingRequiredValueDecryptFailure():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(922,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(923,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_kmsKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out3_ = default__.GetKmsKeyring()
        d_7_kmsKeyring_ = out3_
        d_8_aesKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out4_ = default__.GetAesKeyring()
        d_8_aesKeyring_ = out4_
        d_9_hKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out5_ = default__.GetHierarchicalKeyring()
        d_9_hKeyring_ = out5_
        d_10_valueOrError2_: Wrappers.Result = None
        out6_: Wrappers.Result
        out6_ = (d_5_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_aesKeyring_), _dafny.Seq([d_6_rsaKeyring_, d_7_kmsKeyring_, d_9_hKeyring_])))
        d_10_valueOrError2_ = out6_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(931,28): " + _dafny.string_of(d_10_valueOrError2_))
        d_11_multiKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_11_multiKeyring_ = (d_10_valueOrError2_).Extract()
        d_12_encryptionContext_: _dafny.Map
        out7_: _dafny.Map
        out7_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_AB())
        d_12_encryptionContext_ = out7_
        d_13_requiredECKeys_: _dafny.Seq
        out8_: _dafny.Seq
        out8_ = Fixtures.default__.SmallEncryptionContextKeys(Fixtures.SmallEncryptionContextVariation_A())
        d_13_requiredECKeys_ = out8_
        d_14_droppedRequiredKeyEncryptionContext_: _dafny.Map
        out9_: _dafny.Map
        out9_ = Fixtures.default__.SmallEncryptionContext(Fixtures.SmallEncryptionContextVariation_B())
        d_14_droppedRequiredKeyEncryptionContext_ = out9_
        d_15_valueOrError3_: Wrappers.Result = None
        out10_: Wrappers.Result
        out10_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_11_multiKeyring_))
        d_15_valueOrError3_ = out10_
        if not(not((d_15_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(945,26): " + _dafny.string_of(d_15_valueOrError3_))
        d_16_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_16_defaultCMM_ = (d_15_valueOrError3_).Extract()
        d_17_valueOrError4_: Wrappers.Result = None
        out11_: Wrappers.Result
        out11_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_16_defaultCMM_), Wrappers.Option_None(), d_13_requiredECKeys_))
        d_17_valueOrError4_ = out11_
        if not(not((d_17_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(952,22): " + _dafny.string_of(d_17_valueOrError4_))
        d_18_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_18_reqCMM_ = (d_17_valueOrError4_).Extract()
        d_19_encryptOutput_: Wrappers.Result
        out12_: Wrappers.Result
        out12_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_12_encryptionContext_), Wrappers.Option_Some(d_18_reqCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_19_encryptOutput_ = out12_
        if not((d_19_encryptOutput_).is_Success):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(971,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_20_esdkCiphertext_: _dafny.Seq
        d_20_esdkCiphertext_ = ((d_19_encryptOutput_).value).ciphertext
        d_21_decryptOutput_: Wrappers.Result
        out13_: Wrappers.Result
        out13_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_6_rsaKeyring_), Wrappers.Option_Some(d_14_droppedRequiredKeyEncryptionContext_)))
        d_21_decryptOutput_ = out13_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(982,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out14_: Wrappers.Result
        out14_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_7_kmsKeyring_), Wrappers.Option_Some(d_14_droppedRequiredKeyEncryptionContext_)))
        d_21_decryptOutput_ = out14_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(992,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out15_: Wrappers.Result
        out15_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_8_aesKeyring_), Wrappers.Option_Some(d_14_droppedRequiredKeyEncryptionContext_)))
        d_21_decryptOutput_ = out15_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1002,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        out16_: Wrappers.Result
        out16_ = (d_3_esdk_).Decrypt(AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_20_esdkCiphertext_, Wrappers.Option_None(), Wrappers.Option_Some(d_9_hKeyring_), Wrappers.Option_Some(d_14_droppedRequiredKeyEncryptionContext_)))
        d_21_decryptOutput_ = out16_
        if not((d_21_decryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1012,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def TestReservedEncryptionContextKeyFailure():
        d_0_asdf_: _dafny.Seq
        d_0_asdf_ = _dafny.Seq([97, 115, 100, 102])
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1022,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1023,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_rsaKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_: AwsCryptographyMaterialProvidersTypes.IKeyring
        out2_ = default__.GetRsaKeyring()
        d_6_rsaKeyring_ = out2_
        d_7_encryptionContext_: _dafny.Map
        out3_: _dafny.Map
        out3_ = Fixtures.default__.GetResrvedECMap()
        d_7_encryptionContext_ = out3_
        d_8_requiredECKeys_: _dafny.Seq
        d_8_requiredECKeys_ = _dafny.Seq([Fixtures.default__.RESERVED__ENCRYPTION__CONTEXT])
        d_9_valueOrError2_: Wrappers.Result = None
        out4_: Wrappers.Result
        out4_ = (d_5_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput(d_6_rsaKeyring_))
        d_9_valueOrError2_ = out4_
        if not(not((d_9_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1031,26): " + _dafny.string_of(d_9_valueOrError2_))
        d_10_defaultCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_10_defaultCMM_ = (d_9_valueOrError2_).Extract()
        d_11_valueOrError3_: Wrappers.Result = None
        out5_: Wrappers.Result
        out5_ = (d_5_mpl_).CreateRequiredEncryptionContextCMM(AwsCryptographyMaterialProvidersTypes.CreateRequiredEncryptionContextCMMInput_CreateRequiredEncryptionContextCMMInput(Wrappers.Option_Some(d_10_defaultCMM_), Wrappers.Option_None(), d_8_requiredECKeys_))
        d_11_valueOrError3_ = out5_
        if not(not((d_11_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1041,22): " + _dafny.string_of(d_11_valueOrError3_))
        d_12_reqCMM_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_12_reqCMM_ = (d_11_valueOrError3_).Extract()
        d_13_encryptOutput_: Wrappers.Result
        out6_: Wrappers.Result
        out6_ = (d_3_esdk_).Encrypt(AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_0_asdf_, Wrappers.Option_Some(d_7_encryptionContext_), Wrappers.Option_Some(d_12_reqCMM_), Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_None()))
        d_13_encryptOutput_ = out6_
        if not((d_13_encryptOutput_).is_Failure):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1060,8): " + _dafny.string_of(_dafny.Seq("expectation violation")))

    @staticmethod
    def GetHierarchicalKeyring():
        output: AwsCryptographyMaterialProvidersTypes.IKeyring = None
        d_0_branchKeyId_: _dafny.Seq
        d_0_branchKeyId_ = default__.BRANCH__KEY__ID
        d_1_ttl_: int
        d_1_ttl_ = ((1) * (60000)) * (10)
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1070,19): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_mpl_: MaterialProviders.MaterialProvidersClient
        d_3_mpl_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = Com_Amazonaws_Kms.default__.KMSClient()
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1072,25): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_kmsClient_: ComAmazonawsKmsTypes.IKMSClient
        d_5_kmsClient_ = (d_4_valueOrError1_).Extract()
        d_6_valueOrError2_: Wrappers.Result = None
        out2_: Wrappers.Result
        out2_ = Com_Amazonaws_Dynamodb.default__.DynamoDBClient()
        d_6_valueOrError2_ = out2_
        if not(not((d_6_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1073,25): " + _dafny.string_of(d_6_valueOrError2_))
        d_7_ddbClient_: ComAmazonawsDynamodbTypes.IDynamoDBClient
        d_7_ddbClient_ = (d_6_valueOrError2_).Extract()
        d_8_kmsConfig_: AwsCryptographyKeyStoreTypes.KMSConfiguration
        d_8_kmsConfig_ = AwsCryptographyKeyStoreTypes.KMSConfiguration_kmsKeyArn(default__.hierarchyKeyArn)
        d_9_keyStoreConfig_: AwsCryptographyKeyStoreTypes.KeyStoreConfig
        d_9_keyStoreConfig_ = AwsCryptographyKeyStoreTypes.KeyStoreConfig_KeyStoreConfig(default__.branchKeyStoreName, d_8_kmsConfig_, default__.logicalKeyStoreName, Wrappers.Option_None(), Wrappers.Option_None(), Wrappers.Option_Some(d_7_ddbClient_), Wrappers.Option_Some(d_5_kmsClient_))
        d_10_valueOrError3_: Wrappers.Result = None
        out3_: Wrappers.Result
        out3_ = KeyStore.default__.KeyStore(d_9_keyStoreConfig_)
        d_10_valueOrError3_ = out3_
        if not(not((d_10_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1086,24): " + _dafny.string_of(d_10_valueOrError3_))
        d_11_keyStore_: KeyStore.KeyStoreClient
        d_11_keyStore_ = (d_10_valueOrError3_).Extract()
        d_12_valueOrError4_: Wrappers.Result = None
        out4_: Wrappers.Result
        out4_ = (d_3_mpl_).CreateAwsKmsHierarchicalKeyring(AwsCryptographyMaterialProvidersTypes.CreateAwsKmsHierarchicalKeyringInput_CreateAwsKmsHierarchicalKeyringInput(Wrappers.Option_Some(d_0_branchKeyId_), Wrappers.Option_None(), d_11_keyStore_, d_1_ttl_, Wrappers.Option_None(), Wrappers.Option_None()))
        d_12_valueOrError4_ = out4_
        if not(not((d_12_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1088,18): " + _dafny.string_of(d_12_valueOrError4_))
        output = (d_12_valueOrError4_).Extract()
        return output

    @staticmethod
    def GetRsaKeyring():
        output: AwsCryptographyMaterialProvidersTypes.IKeyring = None
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_0_valueOrError0_ = out0_
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1102,19): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_mpl_: MaterialProviders.MaterialProvidersClient
        d_1_mpl_ = (d_0_valueOrError0_).Extract()
        d_2_namespace_: _dafny.Seq
        d_3_name_: _dafny.Seq
        out1_: _dafny.Seq
        out2_: _dafny.Seq
        out1_, out2_ = Fixtures.default__.NamespaceAndName(0)
        d_2_namespace_ = out1_
        d_3_name_ = out2_
        d_4_keys_: AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput
        out3_: AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput
        out3_ = Fixtures.default__.GenerateKeyPair(2048)
        d_4_keys_ = out3_
        d_5_valueOrError1_: Wrappers.Result = None
        out4_: Wrappers.Result
        out4_ = (d_1_mpl_).CreateRawRsaKeyring(AwsCryptographyMaterialProvidersTypes.CreateRawRsaKeyringInput_CreateRawRsaKeyringInput(d_2_namespace_, d_3_name_, AwsCryptographyMaterialProvidersTypes.PaddingScheme_OAEP__SHA1__MGF1(), Wrappers.Option_Some(((d_4_keys_).publicKey).pem), Wrappers.Option_Some(((d_4_keys_).privateKey).pem)))
        d_5_valueOrError1_ = out4_
        if not(not((d_5_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1106,18): " + _dafny.string_of(d_5_valueOrError1_))
        output = (d_5_valueOrError1_).Extract()
        return output

    @staticmethod
    def GetAesKeyring():
        output: AwsCryptographyMaterialProvidersTypes.IKeyring = None
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_0_valueOrError0_ = out0_
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1120,19): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_mpl_: MaterialProviders.MaterialProvidersClient
        d_1_mpl_ = (d_0_valueOrError0_).Extract()
        d_2_namespace_: _dafny.Seq
        d_3_name_: _dafny.Seq
        out1_: _dafny.Seq
        out2_: _dafny.Seq
        out1_, out2_ = Fixtures.default__.NamespaceAndName(0)
        d_2_namespace_ = out1_
        d_3_name_ = out2_
        d_4_valueOrError1_: Wrappers.Result = None
        out3_: Wrappers.Result
        out3_ = (d_1_mpl_).CreateRawAesKeyring(AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput_CreateRawAesKeyringInput(d_2_namespace_, d_3_name_, _dafny.Seq([0 for d_5_i_ in range(32)]), AwsCryptographyMaterialProvidersTypes.AesWrappingAlg_ALG__AES256__GCM__IV12__TAG16()))
        d_4_valueOrError1_ = out3_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1123,18): " + _dafny.string_of(d_4_valueOrError1_))
        output = (d_4_valueOrError1_).Extract()
        return output

    @staticmethod
    def GetKmsKeyring():
        output: AwsCryptographyMaterialProvidersTypes.IKeyring = None
        d_0_kmsKey_: _dafny.Seq
        d_0_kmsKey_ = Fixtures.default__.keyArn
        d_1_defaultConfig_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_1_defaultConfig_ = EncryptionSdk.default__.DefaultAwsEncryptionSdkConfig()
        d_2_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptionSdk.default__.ESDK(d_1_defaultConfig_)
        d_2_valueOrError0_ = out0_
        if not(not((d_2_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1137,20): " + _dafny.string_of(d_2_valueOrError0_))
        d_3_esdk_: EncryptionSdk.ESDKClient
        d_3_esdk_ = (d_2_valueOrError0_).Extract()
        d_4_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = MaterialProviders.default__.MaterialProviders(MaterialProviders.default__.DefaultMaterialProvidersConfig())
        d_4_valueOrError1_ = out1_
        if not(not((d_4_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1138,19): " + _dafny.string_of(d_4_valueOrError1_))
        d_5_mpl_: MaterialProviders.MaterialProvidersClient
        d_5_mpl_ = (d_4_valueOrError1_).Extract()
        d_6_valueOrError2_: Wrappers.Result = None
        out2_: Wrappers.Result
        out2_ = (d_5_mpl_).CreateDefaultClientSupplier(AwsCryptographyMaterialProvidersTypes.CreateDefaultClientSupplierInput_CreateDefaultClientSupplierInput())
        d_6_valueOrError2_ = out2_
        if not(not((d_6_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1139,30): " + _dafny.string_of(d_6_valueOrError2_))
        d_7_clientSupplier_: AwsCryptographyMaterialProvidersTypes.IClientSupplier
        d_7_clientSupplier_ = (d_6_valueOrError2_).Extract()
        d_8_valueOrError3_: Wrappers.Result = None
        out3_: Wrappers.Result
        out3_ = (d_7_clientSupplier_).GetClient(AwsCryptographyMaterialProvidersTypes.GetClientInput_GetClientInput(_dafny.Seq("us-west-2")))
        d_8_valueOrError3_ = out3_
        if not(not((d_8_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1140,25): " + _dafny.string_of(d_8_valueOrError3_))
        d_9_kmsClient_: ComAmazonawsKmsTypes.IKMSClient
        d_9_kmsClient_ = (d_8_valueOrError3_).Extract()
        d_10_valueOrError4_: Wrappers.Result = None
        out4_: Wrappers.Result
        out4_ = (d_5_mpl_).CreateAwsKmsKeyring(AwsCryptographyMaterialProvidersTypes.CreateAwsKmsKeyringInput_CreateAwsKmsKeyringInput(d_0_kmsKey_, d_9_kmsClient_, Wrappers.Option_None()))
        d_10_valueOrError4_ = out4_
        if not(not((d_10_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1142,18): " + _dafny.string_of(d_10_valueOrError4_))
        output = (d_10_valueOrError4_).Extract()
        return output

    @_dafny.classproperty
    def branchKeyStoreName(instance):
        return Fixtures.default__.branchKeyStoreName
    @_dafny.classproperty
    def logicalKeyStoreName(instance):
        return default__.branchKeyStoreName
    @_dafny.classproperty
    def BRANCH__KEY__ID(instance):
        return Fixtures.default__.branchKeyId
    @_dafny.classproperty
    def hierarchyKeyArn(instance):
        return Fixtures.default__.hierarchyKeyArn
    @_dafny.classproperty
    def keyArn(instance):
        return Fixtures.default__.keyArn
