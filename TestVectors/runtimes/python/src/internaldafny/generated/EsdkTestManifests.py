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
import .internaldafny.generated.ParseEsdkJsonManifest as ParseEsdkJsonManifest

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
        d_3_valueOrError2_ = ParseEsdkJsonManifest.default__.BuildDecryptTestVector(op, (d_1_decryptManifest_).version, (d_1_decryptManifest_).keys, (d_1_decryptManifest_).jsonTests)
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
        if not(not(d_0_hasFailure_)):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(92,4): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        manifest = Wrappers.Result_Success(_dafny.Seq([]))
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
        d_13_valueOrError6_ = JSONHelpers.default__.GetString(_dafny.Seq("keys"), (d_5_manifestJson_).obj)
        if (d_13_valueOrError6_).IsFailure():
            manifestData = (d_13_valueOrError6_).PropagateFailure()
            return manifestData
        d_14_keyManifestUri_: _dafny.Seq
        d_14_keyManifestUri_ = (d_13_valueOrError6_).Extract()
        d_15_valueOrError7_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_15_valueOrError7_ = Wrappers.default__.Need((_dafny.Seq("file://")) < (d_14_keyManifestUri_), _dafny.Seq("Unexpected URI prefix"))
        if (d_15_valueOrError7_).IsFailure():
            manifestData = (d_15_valueOrError7_).PropagateFailure()
            return manifestData
        d_16_keyManifestPath_: _dafny.Seq
        d_16_keyManifestPath_ = (manifestPath) + (_dafny.Seq((d_14_keyManifestUri_)[7::]))
        d_17_valueOrError8_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = KeyVectors.default__.KeyVectors(AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyVectorsConfig_KeyVectorsConfig(d_16_keyManifestPath_))
        d_17_valueOrError8_ = out1_
        if not(not((d_17_valueOrError8_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestManifests.dfy(263,16): " + _dafny.string_of(d_17_valueOrError8_))
        d_18_keys_: KeyVectors.KeyVectorsClient
        d_18_keys_ = (d_17_valueOrError8_).Extract()
        d_19_valueOrError9_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_19_valueOrError9_ = JSONHelpers.default__.GetObject(_dafny.Seq("tests"), (d_5_manifestJson_).obj)
        if (d_19_valueOrError9_).IsFailure():
            manifestData = (d_19_valueOrError9_).PropagateFailure()
            return manifestData
        d_20_jsonTests_: _dafny.Seq
        d_20_jsonTests_ = (d_19_valueOrError9_).Extract()
        source0_ = d_12_typ_
        with _dafny.label("match0"):
            if True:
                if (source0_) == (_dafny.Seq("awses-decrypt")):
                    d_21_valueOrError10_: Wrappers.Outcome = Wrappers.Outcome.default()()
                    d_21_valueOrError10_ = Wrappers.default__.Need(EsdkTestVectors.default__.SupportedDecryptVersion_q(d_10_version_), _dafny.Seq("Unsupported manifest version"))
                    if (d_21_valueOrError10_).IsFailure():
                        manifestData = (d_21_valueOrError10_).PropagateFailure()
                        return manifestData
                    d_22_valueOrError11_: Wrappers.Result = Wrappers.Result.default(JSON_Values.JSON.default())()
                    d_22_valueOrError11_ = JSONHelpers.default__.Get(_dafny.Seq("client"), (d_5_manifestJson_).obj)
                    if (d_22_valueOrError11_).IsFailure():
                        manifestData = (d_22_valueOrError11_).PropagateFailure()
                        return manifestData
                    d_23_client_: JSON_Values.JSON
                    d_23_client_ = (d_22_valueOrError11_).Extract()
                    manifestData = Wrappers.Result_Success(ManifestData_DecryptManifest(d_10_version_, d_18_keys_, d_23_client_, d_20_jsonTests_))
                    raise _dafny.Break("match0")
            if True:
                if (source0_) == (_dafny.Seq("awses-encrypt")):
                    d_24_valueOrError12_: Wrappers.Outcome = Wrappers.Outcome.default()()
                    d_24_valueOrError12_ = Wrappers.default__.Need(EsdkTestVectors.default__.SupportedEncryptVersion_q(d_10_version_), _dafny.Seq("Unsupported manifest version"))
                    if (d_24_valueOrError12_).IsFailure():
                        manifestData = (d_24_valueOrError12_).PropagateFailure()
                        return manifestData
                    d_25_valueOrError13_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
                    d_25_valueOrError13_ = JSONHelpers.default__.GetObject(_dafny.Seq("plaintexts"), (d_5_manifestJson_).obj)
                    if (d_25_valueOrError13_).IsFailure():
                        manifestData = (d_25_valueOrError13_).PropagateFailure()
                        return manifestData
                    d_26_plaintextsJson_: _dafny.Seq
                    d_26_plaintextsJson_ = (d_25_valueOrError13_).Extract()
                    d_27_valueOrError15_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
                    def lambda1_(d_28_obj_):
                        def iife0_(_pat_let7_0):
                            def iife1_(d_29_valueOrError14_):
                                return ((d_29_valueOrError14_).PropagateFailure() if (d_29_valueOrError14_).IsFailure() else Wrappers.Result_Success(((d_28_obj_)[0], (((d_28_obj_)[1]).num).n)))
                            return iife1_(_pat_let7_0)
                        return iife0_(Wrappers.default__.Need((((d_28_obj_)[1]).is_Number) and (((0) < ((((d_28_obj_)[1]).num).n)) and (((((d_28_obj_)[1]).num).n) <= (BoundedInts.default__.INT32__MAX))), _dafny.Seq("Size is not a natural number.")))

                    d_27_valueOrError15_ = Seq.default__.MapWithResult(lambda1_, d_26_plaintextsJson_)
                    if (d_27_valueOrError15_).IsFailure():
                        manifestData = (d_27_valueOrError15_).PropagateFailure()
                        return manifestData
                    d_30_plaintextsLength_: _dafny.Seq
                    d_30_plaintextsLength_ = (d_27_valueOrError15_).Extract()
                    manifestData = Wrappers.Result_Success(ManifestData_EncryptManifest(d_10_version_, d_18_keys_, d_30_plaintextsLength_, d_20_jsonTests_))
                    raise _dafny.Break("match0")
            if True:
                manifestData = Wrappers.Result_Failure((_dafny.Seq("Unsupported manifest type:")) + (d_12_typ_))
            pass
        return manifestData


class ManifestData:
    @classmethod
    def default(cls, ):
        return lambda: ManifestData_DecryptManifest(int(0), None, JSON_Values.JSON.default()(), _dafny.Seq({}))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_DecryptManifest(self) -> bool:
        return isinstance(self, ManifestData_DecryptManifest)
    @property
    def is_EncryptManifest(self) -> bool:
        return isinstance(self, ManifestData_EncryptManifest)

class ManifestData_DecryptManifest(ManifestData, NamedTuple('DecryptManifest', [('version', Any), ('keys', Any), ('client', Any), ('jsonTests', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestManifests.ManifestData.DecryptManifest({_dafny.string_of(self.version)}, {_dafny.string_of(self.keys)}, {_dafny.string_of(self.client)}, {_dafny.string_of(self.jsonTests)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ManifestData_DecryptManifest) and self.version == __o.version and self.keys == __o.keys and self.client == __o.client and self.jsonTests == __o.jsonTests
    def __hash__(self) -> int:
        return super().__hash__()

class ManifestData_EncryptManifest(ManifestData, NamedTuple('EncryptManifest', [('version', Any), ('keys', Any), ('plaintext', Any), ('jsonTests', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestManifests.ManifestData.EncryptManifest({_dafny.string_of(self.version)}, {_dafny.string_of(self.keys)}, {_dafny.string_of(self.plaintext)}, {_dafny.string_of(self.jsonTests)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ManifestData_EncryptManifest) and self.version == __o.version and self.keys == __o.keys and self.plaintext == __o.plaintext and self.jsonTests == __o.jsonTests
    def __hash__(self) -> int:
        return super().__hash__()

