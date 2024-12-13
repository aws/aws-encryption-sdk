import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import .internaldafny.generated.module_ as module_
import _dafny as _dafny
import System_ as System_
import Wrappers as Wrappers
import BoundedInts as BoundedInts
import StandardLibrary_UInt as StandardLibrary_UInt
import StandardLibrary_Sequence as StandardLibrary_Sequence
import StandardLibrary_String as StandardLibrary_String
import StandardLibrary as StandardLibrary
import UTF8 as UTF8
import AwsCryptographyPrimitivesTypes as AwsCryptographyPrimitivesTypes
import ExternRandom as ExternRandom
import Random as Random
import AESEncryption as AESEncryption
import ExternDigest as ExternDigest
import Digest as Digest
import HMAC as HMAC
import WrappedHMAC as WrappedHMAC
import HKDF as HKDF
import WrappedHKDF as WrappedHKDF
import Signature as Signature
import KdfCtr as KdfCtr
import RSAEncryption as RSAEncryption
import ECDH as ECDH
import AwsCryptographyPrimitivesOperations as AwsCryptographyPrimitivesOperations
import AtomicPrimitives as AtomicPrimitives
import ComAmazonawsDynamodbTypes as ComAmazonawsDynamodbTypes
import ComAmazonawsKmsTypes as ComAmazonawsKmsTypes
import AwsCryptographyKeyStoreTypes as AwsCryptographyKeyStoreTypes
import AwsCryptographyMaterialProvidersTypes as AwsCryptographyMaterialProvidersTypes
import Base64 as Base64
import AlgorithmSuites as AlgorithmSuites
import Materials as Materials
import Keyring as Keyring
import Relations as Relations
import Seq_MergeSort as Seq_MergeSort
import Math as Math
import Seq as Seq
import MultiKeyring as MultiKeyring
import AwsArnParsing as AwsArnParsing
import AwsKmsMrkAreUnique as AwsKmsMrkAreUnique
import Actions as Actions
import AwsKmsMrkMatchForDecrypt as AwsKmsMrkMatchForDecrypt
import AwsKmsUtils as AwsKmsUtils
import Constants as Constants
import UUID as UUID
import MaterialWrapping as MaterialWrapping
import SortedSets as SortedSets
import CanonicalEncryptionContext as CanonicalEncryptionContext
import IntermediateKeyWrapping as IntermediateKeyWrapping
import EdkWrapping as EdkWrapping
import ErrorMessages as ErrorMessages
import AwsKmsKeyring as AwsKmsKeyring
import StrictMultiKeyring as StrictMultiKeyring
import AwsKmsDiscoveryKeyring as AwsKmsDiscoveryKeyring
import Com_Amazonaws_Kms as Com_Amazonaws_Kms
import Com_Amazonaws_Dynamodb as Com_Amazonaws_Dynamodb
import DiscoveryMultiKeyring as DiscoveryMultiKeyring
import AwsKmsMrkDiscoveryKeyring as AwsKmsMrkDiscoveryKeyring
import MrkAwareDiscoveryMultiKeyring as MrkAwareDiscoveryMultiKeyring
import AwsKmsMrkKeyring as AwsKmsMrkKeyring
import MrkAwareStrictMultiKeyring as MrkAwareStrictMultiKeyring
import DafnyLibraries as DafnyLibraries
import Time as Time
import LocalCMC as LocalCMC
import SynchronizedLocalCMC as SynchronizedLocalCMC
import StormTracker as StormTracker
import StormTrackingCMC as StormTrackingCMC
import CacheConstants as CacheConstants
import AwsKmsHierarchicalKeyring as AwsKmsHierarchicalKeyring
import AwsKmsRsaKeyring as AwsKmsRsaKeyring
import EcdhEdkWrapping as EcdhEdkWrapping
import RawECDHKeyring as RawECDHKeyring
import AwsKmsEcdhKeyring as AwsKmsEcdhKeyring
import RawAESKeyring as RawAESKeyring
import RawRSAKeyring as RawRSAKeyring
import CMM as CMM
import Defaults as Defaults
import Commitment as Commitment
import DefaultCMM as DefaultCMM
import DefaultClientSupplier as DefaultClientSupplier
import Utils as Utils
import RequiredEncryptionContextCMM as RequiredEncryptionContextCMM
import AwsCryptographyMaterialProvidersOperations as AwsCryptographyMaterialProvidersOperations
import MaterialProviders as MaterialProviders
import KeyStoreErrorMessages as KeyStoreErrorMessages
import KmsArn as KmsArn
import Structure as Structure
import KMSKeystoreOperations as KMSKeystoreOperations
import DDBKeystoreOperations as DDBKeystoreOperations
import CreateKeys as CreateKeys
import CreateKeyStoreTable as CreateKeyStoreTable
import GetKeys as GetKeys
import AwsCryptographyKeyStoreOperations as AwsCryptographyKeyStoreOperations
import KeyStore as KeyStore
import AwsCryptographyMaterialProvidersTestVectorKeysTypes as AwsCryptographyMaterialProvidersTestVectorKeysTypes
import JSON_Utils_Views_Core as JSON_Utils_Views_Core
import JSON_Utils_Views_Writers as JSON_Utils_Views_Writers
import JSON_Utils_Lexers_Core as JSON_Utils_Lexers_Core
import JSON_Utils_Lexers_Strings as JSON_Utils_Lexers_Strings
import JSON_Utils_Cursors as JSON_Utils_Cursors
import JSON_Utils_Parsers as JSON_Utils_Parsers
import GeneralInternals as GeneralInternals
import MulInternalsNonlinear as MulInternalsNonlinear
import MulInternals as MulInternals
import Mul as Mul
import ModInternalsNonlinear as ModInternalsNonlinear
import DivInternalsNonlinear as DivInternalsNonlinear
import ModInternals as ModInternals
import DivInternals as DivInternals
import DivMod as DivMod
import Power as Power
import Logarithm as Logarithm
import JSON_Utils_Str_CharStrConversion as JSON_Utils_Str_CharStrConversion
import JSON_Utils_Str_CharStrEscaping as JSON_Utils_Str_CharStrEscaping
import JSON_Utils_Str as JSON_Utils_Str
import JSON_Utils_Seq as JSON_Utils_Seq
import JSON_Utils_Vectors as JSON_Utils_Vectors
import JSON_Errors as JSON_Errors
import JSON_Values as JSON_Values
import Unicode as Unicode
import Functions as Functions
import Utf8EncodingForm as Utf8EncodingForm
import Utf16EncodingForm as Utf16EncodingForm
import UnicodeStrings as UnicodeStrings
import JSON_Spec as JSON_Spec
import JSON_Grammar as JSON_Grammar
import JSON_Serializer_ByteStrConversion as JSON_Serializer_ByteStrConversion
import JSON_Serializer as JSON_Serializer
import JSON_Deserializer_Uint16StrConversion as JSON_Deserializer_Uint16StrConversion
import JSON_Deserializer_ByteStrConversion as JSON_Deserializer_ByteStrConversion
import JSON_Deserializer as JSON_Deserializer
import JSON_ConcreteSyntax_Spec as JSON_ConcreteSyntax_Spec
import JSON_ConcreteSyntax_SpecProperties as JSON_ConcreteSyntax_SpecProperties
import JSON_ZeroCopy_Serializer as JSON_ZeroCopy_Serializer
import JSON_ZeroCopy_Deserializer_Core as JSON_ZeroCopy_Deserializer_Core
import JSON_ZeroCopy_Deserializer_Strings as JSON_ZeroCopy_Deserializer_Strings
import JSON_ZeroCopy_Deserializer_Numbers as JSON_ZeroCopy_Deserializer_Numbers
import JSON_ZeroCopy_Deserializer_ObjectParams as JSON_ZeroCopy_Deserializer_ObjectParams
import JSON_ZeroCopy_Deserializer_Objects as JSON_ZeroCopy_Deserializer_Objects
import JSON_ZeroCopy_Deserializer_ArrayParams as JSON_ZeroCopy_Deserializer_ArrayParams
import JSON_ZeroCopy_Deserializer_Arrays as JSON_ZeroCopy_Deserializer_Arrays
import JSON_ZeroCopy_Deserializer_Constants as JSON_ZeroCopy_Deserializer_Constants
import JSON_ZeroCopy_Deserializer_Values as JSON_ZeroCopy_Deserializer_Values
import JSON_ZeroCopy_Deserializer_API as JSON_ZeroCopy_Deserializer_API
import JSON_ZeroCopy_Deserializer as JSON_ZeroCopy_Deserializer
import JSON_ZeroCopy_API as JSON_ZeroCopy_API
import JSON_API as JSON_API
import JSONHelpers as JSONHelpers
import KeyDescription as KeyDescription
import HexStrings as HexStrings
import KeyMaterial as KeyMaterial
import CreateStaticKeyrings as CreateStaticKeyrings
import CreateStaticKeyStores as CreateStaticKeyStores
import KeyringFromKeyDescription as KeyringFromKeyDescription
import CmmFromKeyDescription as CmmFromKeyDescription
import WrappedMaterialProviders as WrappedMaterialProviders
import KeysVectorOperations as KeysVectorOperations
import FileIO as FileIO
import KeyVectors as KeyVectors
import AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import Streams as Streams
import SerializableTypes as SerializableTypes
import SerializeFunctions as SerializeFunctions
import EncryptionContext as EncryptionContext
import HeaderTypes as HeaderTypes
import SharedHeaderFunctions as SharedHeaderFunctions
import EncryptedDataKeys as EncryptedDataKeys
import V1HeaderBody as V1HeaderBody
import V2HeaderBody as V2HeaderBody
import HeaderAuth as HeaderAuth
import Header as Header
import Frames as Frames
import MessageBody as MessageBody
import KeyDerivation as KeyDerivation
import EncryptDecryptHelpers as EncryptDecryptHelpers
import AwsEncryptionSdkOperations as AwsEncryptionSdkOperations
import EncryptionSdk as EncryptionSdk
import MplManifestOptions as MplManifestOptions
import GetOpt as GetOpt
import AllAlgorithmSuites as AllAlgorithmSuites
import TestVectors as TestVectors
import AllHierarchy as AllHierarchy
import AllKms as AllKms
import AllKmsMrkAware as AllKmsMrkAware
import AllKmsMrkAwareDiscovery as AllKmsMrkAwareDiscovery
import AllKmsRsa as AllKmsRsa
import AllKmsEcdh as AllKmsEcdh
import AllRawAES as AllRawAES
import AllRawRSA as AllRawRSA
import AllRawECDH as AllRawECDH
import AllDefaultCmm as AllDefaultCmm
import AllRequiredEncryptionContextCmm as AllRequiredEncryptionContextCmm
import AllMulti as AllMulti
import WriteJsonManifests as WriteJsonManifests
import CompleteVectors as CompleteVectors
import ParseJsonManifests as ParseJsonManifests
import TestManifests as TestManifests
import WrappedMaterialProvidersMain as WrappedMaterialProvidersMain
import AesKdfCtr as AesKdfCtr
import StandardLibraryInterop as StandardLibraryInterop
import Sorting as Sorting
import FloatCompare as FloatCompare
import ConcurrentCall as ConcurrentCall
import Base64Lemmas as Base64Lemmas
import .internaldafny.generated.WrappedESDK as WrappedESDK
import .internaldafny.generated.EsdkManifestOptions as EsdkManifestOptions
import .internaldafny.generated.EsdkTestVectors as EsdkTestVectors
import .internaldafny.generated.AllEsdkV4NoReqEc as AllEsdkV4NoReqEc
import .internaldafny.generated.AllEsdkV4WithReqEc as AllEsdkV4WithReqEc
import .internaldafny.generated.WriteEsdkJsonManifests as WriteEsdkJsonManifests
import .internaldafny.generated.WriteVectors as WriteVectors

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
