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
import aws_encryption_sdk_dafny.internaldafny.generated.V2HeaderBody as V2HeaderBody
import aws_encryption_sdk_dafny.internaldafny.generated.HeaderAuth as HeaderAuth
import aws_encryption_sdk_dafny.internaldafny.generated.Header as Header
import aws_encryption_sdk_dafny.internaldafny.generated.Frames as Frames
import aws_encryption_sdk_dafny.internaldafny.generated.MessageBody as MessageBody

# Module: KeyDerivation

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def DeriveKey(messageId, plaintextDataKey, suite, crypto, onNetV4Retry):
        res: Wrappers.Result = Wrappers.Result.default(ExpandedKeyMaterial.default())()
        source0_ = (suite).kdf
        with _dafny.label("match0"):
            if True:
                if source0_.is_IDENTITY:
                    d_0_i_ = source0_.IDENTITY
                    if True:
                        res = Wrappers.Result_Success(ExpandedKeyMaterial_ExpandedKeyMaterial(plaintextDataKey, Wrappers.Option_None()))
                        return res
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_HKDF:
                    d_1_hkdf_ = source0_.HKDF
                    if True:
                        d_2_hkdfInput_: AwsCryptographyPrimitivesTypes.HkdfInput
                        d_2_hkdfInput_ = AwsCryptographyPrimitivesTypes.HkdfInput_HkdfInput((d_1_hkdf_).hmac, Wrappers.Option_None(), plaintextDataKey, ((suite).binaryId) + (messageId), (d_1_hkdf_).outputKeyLength)
                        if onNetV4Retry:
                            d_2_hkdfInput_ = AwsCryptographyPrimitivesTypes.HkdfInput_HkdfInput((d_1_hkdf_).hmac, Wrappers.Option_None(), plaintextDataKey, (suite).binaryId, (d_1_hkdf_).outputKeyLength)
                        d_3_maybeDerivedKey_: Wrappers.Result
                        out0_: Wrappers.Result
                        out0_ = (crypto).Hkdf(d_2_hkdfInput_)
                        d_3_maybeDerivedKey_ = out0_
                        d_4_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
                        def lambda0_(d_5_e_):
                            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_5_e_)

                        d_4_valueOrError0_ = (d_3_maybeDerivedKey_).MapFailure(lambda0_)
                        if (d_4_valueOrError0_).IsFailure():
                            res = (d_4_valueOrError0_).PropagateFailure()
                            return res
                        d_6_derivedKey_: _dafny.Seq
                        d_6_derivedKey_ = (d_4_valueOrError0_).Extract()
                        res = Wrappers.Result_Success(ExpandedKeyMaterial_ExpandedKeyMaterial(d_6_derivedKey_, Wrappers.Option_None()))
                        return res
                    raise _dafny.Break("match0")
            if True:
                d_7_None_ = source0_
                if True:
                    res = Wrappers.Result_Failure(AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("None is not a valid Key Derivation Function")))
                    return res
            pass
        return res

    @staticmethod
    def ExpandKeyMaterial(messageId, plaintextKey, suite, crypto):
        res: Wrappers.Result = Wrappers.Result.default(ExpandedKeyMaterial.default())()
        d_0_digest_: AwsCryptographyPrimitivesTypes.DigestAlgorithm
        d_0_digest_ = (((suite).commitment).HKDF).hmac
        d_1_info_: _dafny.Seq
        d_1_info_ = ((suite).binaryId) + (default__.KEY__LABEL)
        d_2_hkdfExtractInput_: AwsCryptographyPrimitivesTypes.HkdfExtractInput
        d_2_hkdfExtractInput_ = AwsCryptographyPrimitivesTypes.HkdfExtractInput_HkdfExtractInput(d_0_digest_, Wrappers.Option_Some(messageId), plaintextKey)
        d_3_maybePseudoRandomKey_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = (crypto).HkdfExtract(d_2_hkdfExtractInput_)
        d_3_maybePseudoRandomKey_ = out0_
        d_4_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda0_(d_5_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_5_e_)

        d_4_valueOrError0_ = (d_3_maybePseudoRandomKey_).MapFailure(lambda0_)
        if (d_4_valueOrError0_).IsFailure():
            res = (d_4_valueOrError0_).PropagateFailure()
            return res
        d_6_pseudoRandomKey_: _dafny.Seq
        d_6_pseudoRandomKey_ = (d_4_valueOrError0_).Extract()
        d_7_encryptKeyInput_: AwsCryptographyPrimitivesTypes.HkdfExpandInput
        d_7_encryptKeyInput_ = AwsCryptographyPrimitivesTypes.HkdfExpandInput_HkdfExpandInput(d_0_digest_, d_6_pseudoRandomKey_, d_1_info_, (((suite).kdf).HKDF).outputKeyLength)
        d_8_commitKeyInput_: AwsCryptographyPrimitivesTypes.HkdfExpandInput
        d_9_dt__update__tmp_h0_ = d_7_encryptKeyInput_
        d_10_dt__update_hexpectedLength_h0_ = (((suite).commitment).HKDF).outputKeyLength
        d_11_dt__update_hinfo_h0_ = default__.COMMIT__LABEL
        d_8_commitKeyInput_ = AwsCryptographyPrimitivesTypes.HkdfExpandInput_HkdfExpandInput((d_9_dt__update__tmp_h0_).digestAlgorithm, (d_9_dt__update__tmp_h0_).prk, d_11_dt__update_hinfo_h0_, d_10_dt__update_hexpectedLength_h0_)
        d_12_maybeEncryptKey_: Wrappers.Result
        out1_: Wrappers.Result
        out1_ = (crypto).HkdfExpand(d_7_encryptKeyInput_)
        d_12_maybeEncryptKey_ = out1_
        d_13_maybeCommitKey_: Wrappers.Result
        out2_: Wrappers.Result
        out2_ = (crypto).HkdfExpand(d_8_commitKeyInput_)
        d_13_maybeCommitKey_ = out2_
        d_14_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda1_(d_15_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_15_e_)

        d_14_valueOrError1_ = (d_12_maybeEncryptKey_).MapFailure(lambda1_)
        if (d_14_valueOrError1_).IsFailure():
            res = (d_14_valueOrError1_).PropagateFailure()
            return res
        d_16_encryptKey_: _dafny.Seq
        d_16_encryptKey_ = (d_14_valueOrError1_).Extract()
        d_17_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda2_(d_18_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_18_e_)

        d_17_valueOrError2_ = (d_13_maybeCommitKey_).MapFailure(lambda2_)
        if (d_17_valueOrError2_).IsFailure():
            res = (d_17_valueOrError2_).PropagateFailure()
            return res
        d_19_commitKey_: _dafny.Seq
        d_19_commitKey_ = (d_17_valueOrError2_).Extract()
        res = Wrappers.Result_Success(ExpandedKeyMaterial_ExpandedKeyMaterial(d_16_encryptKey_, Wrappers.Option_Some(d_19_commitKey_)))
        return res
        return res

    @staticmethod
    def DeriveKeys(messageId, plaintextKey, suite, crypto, netV4__0__0__RetryPolicy, onNetV4Retry):
        res: Wrappers.Result = Wrappers.Result.default(ExpandedKeyMaterial.default())()
        d_0_keys_: ExpandedKeyMaterial = ExpandedKeyMaterial.default()()
        if ((suite).messageVersion) == (2):
            d_1_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_1_valueOrError0_ = Wrappers.default__.Need((((suite).commitment).is_HKDF) and (((suite).kdf) == ((suite).commitment)), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Suites with message version 2 must have commitment")))
            if (d_1_valueOrError0_).IsFailure():
                res = (d_1_valueOrError0_).PropagateFailure()
                return res
            d_2_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_2_valueOrError1_ = Wrappers.default__.Need(((SerializableTypes.default__.GetEncryptKeyLength(suite)) == ((((suite).kdf).HKDF).outputKeyLength)) and ((len(plaintextKey)) == ((((suite).kdf).HKDF).inputKeyLength)), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Invalid Materials")))
            if (d_2_valueOrError1_).IsFailure():
                res = (d_2_valueOrError1_).PropagateFailure()
                return res
            d_3_valueOrError2_: Wrappers.Result = Wrappers.Result.default(ExpandedKeyMaterial.default())()
            out0_: Wrappers.Result
            out0_ = default__.ExpandKeyMaterial(messageId, plaintextKey, suite, crypto)
            d_3_valueOrError2_ = out0_
            if (d_3_valueOrError2_).IsFailure():
                res = (d_3_valueOrError2_).PropagateFailure()
                return res
            d_0_keys_ = (d_3_valueOrError2_).Extract()
        elif ((suite).messageVersion) == (1):
            d_4_valueOrError3_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_4_valueOrError3_ = Wrappers.default__.Need(((suite).commitment).is_None, AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Suites with message version 1 must not have commitment")))
            if (d_4_valueOrError3_).IsFailure():
                res = (d_4_valueOrError3_).PropagateFailure()
                return res
            d_5_valueOrError4_: Wrappers.Outcome = Wrappers.Outcome.default()()
            def lambda0_():
                source0_ = (suite).kdf
                if True:
                    if source0_.is_IDENTITY:
                        d_6_i_ = source0_.IDENTITY
                        return (len(plaintextKey)) == (SerializableTypes.default__.GetEncryptKeyLength(suite))
                if True:
                    if source0_.is_HKDF:
                        d_7_hkdf_ = source0_.HKDF
                        return ((len(plaintextKey)) == ((((suite).kdf).HKDF).inputKeyLength)) and (((((suite).kdf).HKDF).outputKeyLength) == (SerializableTypes.default__.GetEncryptKeyLength(suite)))
                if True:
                    d_8_None_ = source0_
                    return False

            d_5_valueOrError4_ = Wrappers.default__.Need(lambda0_(), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Suites with message version 1 must not have commitment")))
            if (d_5_valueOrError4_).IsFailure():
                res = (d_5_valueOrError4_).PropagateFailure()
                return res
            if ((netV4__0__0__RetryPolicy) == (AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy_ALLOW__RETRY())) and (onNetV4Retry):
                d_9_valueOrError5_: Wrappers.Result = Wrappers.Result.default(ExpandedKeyMaterial.default())()
                out1_: Wrappers.Result
                out1_ = default__.DeriveKey(messageId, plaintextKey, suite, crypto, True)
                d_9_valueOrError5_ = out1_
                if (d_9_valueOrError5_).IsFailure():
                    res = (d_9_valueOrError5_).PropagateFailure()
                    return res
                d_0_keys_ = (d_9_valueOrError5_).Extract()
            elif True:
                d_10_valueOrError6_: Wrappers.Result = Wrappers.Result.default(ExpandedKeyMaterial.default())()
                out2_: Wrappers.Result
                out2_ = default__.DeriveKey(messageId, plaintextKey, suite, crypto, False)
                d_10_valueOrError6_ = out2_
                if (d_10_valueOrError6_).IsFailure():
                    res = (d_10_valueOrError6_).PropagateFailure()
                    return res
                d_0_keys_ = (d_10_valueOrError6_).Extract()
        elif True:
            res = Wrappers.Result_Failure(AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Unknown message version")))
            return res
        res = Wrappers.Result_Success(d_0_keys_)
        return res
        return res

    @_dafny.classproperty
    def KEY__LABEL(instance):
        d_0_s_ = _dafny.Seq([68, 69, 82, 73, 86, 69, 75, 69, 89])
        return d_0_s_
    @_dafny.classproperty
    def COMMIT__LABEL(instance):
        d_0_s_ = _dafny.Seq([67, 79, 77, 77, 73, 84, 75, 69, 89])
        return d_0_s_

class ExpandedKeyMaterial:
    @classmethod
    def default(cls, ):
        return lambda: ExpandedKeyMaterial_ExpandedKeyMaterial(_dafny.Seq({}), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_ExpandedKeyMaterial(self) -> bool:
        return isinstance(self, ExpandedKeyMaterial_ExpandedKeyMaterial)

class ExpandedKeyMaterial_ExpandedKeyMaterial(ExpandedKeyMaterial, NamedTuple('ExpandedKeyMaterial', [('dataKey', Any), ('commitmentKey', Any)])):
    def __dafnystr__(self) -> str:
        return f'KeyDerivation.ExpandedKeyMaterial.ExpandedKeyMaterial({_dafny.string_of(self.dataKey)}, {_dafny.string_of(self.commitmentKey)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ExpandedKeyMaterial_ExpandedKeyMaterial) and self.dataKey == __o.dataKey and self.commitmentKey == __o.commitmentKey
    def __hash__(self) -> int:
        return super().__hash__()

