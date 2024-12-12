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

# Module: Fixtures

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def SmallEncryptionContext(v):
        encryptionContext: _dafny.Map = _dafny.Map({})
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_0_valueOrError0_ = UTF8.default__.Encode(_dafny.Seq("keyA"))
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(36,16): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_keyA_: _dafny.Seq
        d_1_keyA_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_2_valueOrError1_ = UTF8.default__.Encode(_dafny.Seq("valA"))
        if not(not((d_2_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(37,16): " + _dafny.string_of(d_2_valueOrError1_))
        d_3_valA_: _dafny.Seq
        d_3_valA_ = (d_2_valueOrError1_).Extract()
        d_4_valueOrError2_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_4_valueOrError2_ = UTF8.default__.Encode(_dafny.Seq("keyB"))
        if not(not((d_4_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(38,16): " + _dafny.string_of(d_4_valueOrError2_))
        d_5_keyB_: _dafny.Seq
        d_5_keyB_ = (d_4_valueOrError2_).Extract()
        d_6_valueOrError3_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_6_valueOrError3_ = UTF8.default__.Encode(_dafny.Seq("valB"))
        if not(not((d_6_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(39,16): " + _dafny.string_of(d_6_valueOrError3_))
        d_7_valB_: _dafny.Seq
        d_7_valB_ = (d_6_valueOrError3_).Extract()
        d_8_valueOrError4_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_8_valueOrError4_ = UTF8.default__.Encode(_dafny.Seq("keyC"))
        if not(not((d_8_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(40,16): " + _dafny.string_of(d_8_valueOrError4_))
        d_9_keyC_: _dafny.Seq
        d_9_keyC_ = (d_8_valueOrError4_).Extract()
        d_10_valueOrError5_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_10_valueOrError5_ = UTF8.default__.Encode(_dafny.Seq("valC"))
        if not(not((d_10_valueOrError5_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(41,16): " + _dafny.string_of(d_10_valueOrError5_))
        d_11_valC_: _dafny.Seq
        d_11_valC_ = (d_10_valueOrError5_).Extract()
        d_12_valueOrError6_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_12_valueOrError6_ = UTF8.default__.Encode(_dafny.Seq("keyD"))
        if not(not((d_12_valueOrError6_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(42,16): " + _dafny.string_of(d_12_valueOrError6_))
        d_13_keyD_: _dafny.Seq
        d_13_keyD_ = (d_12_valueOrError6_).Extract()
        d_14_valueOrError7_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_14_valueOrError7_ = UTF8.default__.Encode(_dafny.Seq("valD"))
        if not(not((d_14_valueOrError7_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(43,16): " + _dafny.string_of(d_14_valueOrError7_))
        d_15_valD_: _dafny.Seq
        d_15_valD_ = (d_14_valueOrError7_).Extract()
        source0_ = v
        with _dafny.label("match0"):
            if True:
                if source0_.is_Empty:
                    encryptionContext = _dafny.Map({})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_A:
                    encryptionContext = _dafny.Map({d_1_keyA_: d_3_valA_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_B:
                    encryptionContext = _dafny.Map({d_5_keyB_: d_7_valB_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_AB:
                    encryptionContext = _dafny.Map({d_1_keyA_: d_3_valA_, d_5_keyB_: d_7_valB_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_BA:
                    encryptionContext = _dafny.Map({d_5_keyB_: d_7_valB_, d_1_keyA_: d_3_valA_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_C:
                    encryptionContext = _dafny.Map({d_9_keyC_: d_11_valC_})
                    raise _dafny.Break("match0")
            if True:
                d_16_CE_ = source0_
                encryptionContext = _dafny.Map({d_9_keyC_: d_11_valC_, d_13_keyD_: d_15_valD_})
            pass
        return encryptionContext

    @staticmethod
    def GetResrvedECMap():
        encryptionContext: _dafny.Map = _dafny.Map({})
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_0_valueOrError0_ = UTF8.default__.Encode(_dafny.Seq("aws-crypto-public-key"))
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(67,23): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_reservedKey_: _dafny.Seq
        d_1_reservedKey_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_2_valueOrError1_ = UTF8.default__.Encode(_dafny.Seq("not a real public key"))
        if not(not((d_2_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(68,15): " + _dafny.string_of(d_2_valueOrError1_))
        d_3_val_: _dafny.Seq
        d_3_val_ = (d_2_valueOrError1_).Extract()
        encryptionContext = _dafny.Map({d_1_reservedKey_: d_3_val_})
        return encryptionContext

    @staticmethod
    def SmallEncryptionContextKeys(v):
        encryptionContextKeys: _dafny.Seq = _dafny.Seq({})
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_0_valueOrError0_ = UTF8.default__.Encode(_dafny.Seq("keyA"))
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(76,16): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_keyA_: _dafny.Seq
        d_1_keyA_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_2_valueOrError1_ = UTF8.default__.Encode(_dafny.Seq("keyB"))
        if not(not((d_2_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(77,16): " + _dafny.string_of(d_2_valueOrError1_))
        d_3_keyB_: _dafny.Seq
        d_3_keyB_ = (d_2_valueOrError1_).Extract()
        d_4_valueOrError2_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_4_valueOrError2_ = UTF8.default__.Encode(_dafny.Seq("keyC"))
        if not(not((d_4_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(78,16): " + _dafny.string_of(d_4_valueOrError2_))
        d_5_keyC_: _dafny.Seq
        d_5_keyC_ = (d_4_valueOrError2_).Extract()
        d_6_valueOrError3_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_6_valueOrError3_ = UTF8.default__.Encode(_dafny.Seq("keyD"))
        if not(not((d_6_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(79,16): " + _dafny.string_of(d_6_valueOrError3_))
        d_7_keyD_: _dafny.Seq
        d_7_keyD_ = (d_6_valueOrError3_).Extract()
        source0_ = v
        with _dafny.label("match0"):
            if True:
                if source0_.is_Empty:
                    encryptionContextKeys = _dafny.Seq([])
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_A:
                    encryptionContextKeys = _dafny.Seq([d_1_keyA_])
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_B:
                    encryptionContextKeys = _dafny.Seq([d_3_keyB_])
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_AB:
                    encryptionContextKeys = _dafny.Seq([d_1_keyA_, d_3_keyB_])
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_BA:
                    encryptionContextKeys = _dafny.Seq([d_3_keyB_, d_1_keyA_])
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_C:
                    encryptionContextKeys = _dafny.Seq([d_5_keyC_])
                    raise _dafny.Break("match0")
            if True:
                d_8_CE_ = source0_
                encryptionContextKeys = _dafny.Seq([d_5_keyC_, d_7_keyD_])
            pass
        return encryptionContextKeys

    @staticmethod
    def SmallMismatchedEncryptionContex(v):
        encryptionContext: _dafny.Map = _dafny.Map({})
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_0_valueOrError0_ = UTF8.default__.Encode(_dafny.Seq("keyA"))
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(102,16): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_keyA_: _dafny.Seq
        d_1_keyA_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_2_valueOrError1_ = UTF8.default__.Encode(_dafny.Seq("valA"))
        if not(not((d_2_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(103,16): " + _dafny.string_of(d_2_valueOrError1_))
        d_3_valA_: _dafny.Seq
        d_3_valA_ = (d_2_valueOrError1_).Extract()
        d_4_valueOrError2_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_4_valueOrError2_ = UTF8.default__.Encode(_dafny.Seq("keyB"))
        if not(not((d_4_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(104,16): " + _dafny.string_of(d_4_valueOrError2_))
        d_5_keyB_: _dafny.Seq
        d_5_keyB_ = (d_4_valueOrError2_).Extract()
        d_6_valueOrError3_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_6_valueOrError3_ = UTF8.default__.Encode(_dafny.Seq("valB"))
        if not(not((d_6_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(105,16): " + _dafny.string_of(d_6_valueOrError3_))
        d_7_valB_: _dafny.Seq
        d_7_valB_ = (d_6_valueOrError3_).Extract()
        d_8_valueOrError4_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_8_valueOrError4_ = UTF8.default__.Encode(_dafny.Seq("keyC"))
        if not(not((d_8_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(106,16): " + _dafny.string_of(d_8_valueOrError4_))
        d_9_keyC_: _dafny.Seq
        d_9_keyC_ = (d_8_valueOrError4_).Extract()
        d_10_valueOrError5_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_10_valueOrError5_ = UTF8.default__.Encode(_dafny.Seq("valC"))
        if not(not((d_10_valueOrError5_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(107,16): " + _dafny.string_of(d_10_valueOrError5_))
        d_11_valC_: _dafny.Seq
        d_11_valC_ = (d_10_valueOrError5_).Extract()
        d_12_valueOrError6_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_12_valueOrError6_ = UTF8.default__.Encode(_dafny.Seq("keyD"))
        if not(not((d_12_valueOrError6_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(108,16): " + _dafny.string_of(d_12_valueOrError6_))
        d_13_keyD_: _dafny.Seq
        d_13_keyD_ = (d_12_valueOrError6_).Extract()
        d_14_valueOrError7_: Wrappers.Result = Wrappers.Result.default(UTF8.ValidUTF8Bytes.default)()
        d_14_valueOrError7_ = UTF8.default__.Encode(_dafny.Seq("valD"))
        if not(not((d_14_valueOrError7_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(109,16): " + _dafny.string_of(d_14_valueOrError7_))
        d_15_valD_: _dafny.Seq
        d_15_valD_ = (d_14_valueOrError7_).Extract()
        source0_ = v
        with _dafny.label("match0"):
            if True:
                if source0_.is_Empty:
                    encryptionContext = _dafny.Map({})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_A:
                    encryptionContext = _dafny.Map({d_1_keyA_: d_7_valB_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_B:
                    encryptionContext = _dafny.Map({d_5_keyB_: d_3_valA_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_AB:
                    encryptionContext = _dafny.Map({d_1_keyA_: d_11_valC_, d_5_keyB_: d_15_valD_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_BA:
                    encryptionContext = _dafny.Map({d_5_keyB_: d_3_valA_, d_1_keyA_: d_7_valB_})
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_C:
                    encryptionContext = _dafny.Map({d_9_keyC_: d_3_valA_})
                    raise _dafny.Break("match0")
            if True:
                d_16_CE_ = source0_
                encryptionContext = _dafny.Map({d_9_keyC_: d_3_valA_, d_13_keyD_: d_7_valB_})
            pass
        return encryptionContext

    @staticmethod
    def NamespaceAndName(n):
        namespace: _dafny.Seq = _dafny.Seq("")
        name: _dafny.Seq = _dafny.Seq("")
        d_0_s_: _dafny.Seq
        d_0_s_ = (_dafny.Seq("child")) + (_dafny.Seq([_dafny.plus_char(chr(n), '0')]))
        namespace = (d_0_s_) + (_dafny.Seq(" Namespace"))
        name = (d_0_s_) + (_dafny.Seq(" Name"))
        return namespace, name

    @staticmethod
    def GenerateKeyPair(keyModulusLength):
        keys: AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput = None
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = AtomicPrimitives.default__.AtomicPrimitives(AtomicPrimitives.default__.DefaultCryptoConfig())
        d_0_valueOrError0_ = out0_
        if not(not((d_0_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(143,72): " + _dafny.string_of(d_0_valueOrError0_))
        d_1_cryptoX_: AwsCryptographyPrimitivesTypes.IAwsCryptographicPrimitivesClient
        d_1_cryptoX_ = (d_0_valueOrError0_).Extract()
        d_2_crypto_: AtomicPrimitives.AtomicPrimitivesClient
        d_2_crypto_ = d_1_cryptoX_
        d_3_valueOrError1_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = (d_2_crypto_).GenerateRSAKeyPair(AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairInput_GenerateRSAKeyPairInput(keyModulusLength))
        d_3_valueOrError1_ = out1_
        if not(not((d_3_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/AwsEncryptionSdk/test/Fixtures.dfy(147,14): " + _dafny.string_of(d_3_valueOrError1_))
        keys = (d_3_valueOrError1_).Extract()
        return keys

    @_dafny.classproperty
    def branchKeyStoreName(instance):
        return _dafny.Seq("KeyStoreDdbTable")
    @_dafny.classproperty
    def logicalKeyStoreName(instance):
        return default__.branchKeyStoreName
    @_dafny.classproperty
    def keyArn(instance):
        return _dafny.Seq("arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f")
    @_dafny.classproperty
    def hierarchyKeyArn(instance):
        return _dafny.Seq("arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126")
    @_dafny.classproperty
    def mkrKeyArn(instance):
        return _dafny.Seq("arn:aws:kms:us-west-2:370957321024:key/mrk-63d386cb70614ea59b32ad65c9315297")
    @_dafny.classproperty
    def branchKeyId(instance):
        return _dafny.Seq("75789115-1deb-4fe3-a2ec-be9e885d1945")
    @_dafny.classproperty
    def RESERVED__ENCRYPTION__CONTEXT(instance):
        d_0_s_ = _dafny.Seq([97, 119, 115, 45, 99, 114, 121, 112, 116, 111, 45])
        return d_0_s_

class SmallEncryptionContextVariation:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [SmallEncryptionContextVariation_Empty(), SmallEncryptionContextVariation_A(), SmallEncryptionContextVariation_B(), SmallEncryptionContextVariation_AB(), SmallEncryptionContextVariation_BA(), SmallEncryptionContextVariation_C(), SmallEncryptionContextVariation_CD()]
    @classmethod
    def default(cls, ):
        return lambda: SmallEncryptionContextVariation_Empty()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_Empty(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_Empty)
    @property
    def is_A(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_A)
    @property
    def is_B(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_B)
    @property
    def is_AB(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_AB)
    @property
    def is_BA(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_BA)
    @property
    def is_C(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_C)
    @property
    def is_CD(self) -> bool:
        return isinstance(self, SmallEncryptionContextVariation_CD)

class SmallEncryptionContextVariation_Empty(SmallEncryptionContextVariation, NamedTuple('Empty', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.Empty'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_Empty)
    def __hash__(self) -> int:
        return super().__hash__()

class SmallEncryptionContextVariation_A(SmallEncryptionContextVariation, NamedTuple('A', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.A'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_A)
    def __hash__(self) -> int:
        return super().__hash__()

class SmallEncryptionContextVariation_B(SmallEncryptionContextVariation, NamedTuple('B', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.B'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_B)
    def __hash__(self) -> int:
        return super().__hash__()

class SmallEncryptionContextVariation_AB(SmallEncryptionContextVariation, NamedTuple('AB', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.AB'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_AB)
    def __hash__(self) -> int:
        return super().__hash__()

class SmallEncryptionContextVariation_BA(SmallEncryptionContextVariation, NamedTuple('BA', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.BA'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_BA)
    def __hash__(self) -> int:
        return super().__hash__()

class SmallEncryptionContextVariation_C(SmallEncryptionContextVariation, NamedTuple('C', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.C'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_C)
    def __hash__(self) -> int:
        return super().__hash__()

class SmallEncryptionContextVariation_CD(SmallEncryptionContextVariation, NamedTuple('CD', [])):
    def __dafnystr__(self) -> str:
        return f'Fixtures.SmallEncryptionContextVariation.CD'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SmallEncryptionContextVariation_CD)
    def __hash__(self) -> int:
        return super().__hash__()

