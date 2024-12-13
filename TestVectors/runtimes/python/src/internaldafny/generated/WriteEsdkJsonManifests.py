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

