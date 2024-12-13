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

# Module: EsdkTestVectors

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def SupportedGenerateManifestVersion_q(v):
        return (False) or ((v) == (4))

    @staticmethod
    def SupportedEncryptVersion_q(v):
        return (((v) == (1)) or ((v) == (4))) or ((v) == (5))

    @staticmethod
    def SupportedDecryptVersion_q(v):
        return (((v) == (1)) or ((v) == (2))) or ((v) == (3))

    @staticmethod
    def TestDecrypt(keys, vector):
        output: bool = False
        if ((vector).algorithmSuiteId).is_Some:
            d_0_id_: _dafny.Seq
            d_0_id_ = AllAlgorithmSuites.default__.ToHex(((vector).algorithmSuiteId).value)
            _dafny.print(_dafny.string_of(_dafny.Seq("\nTEST-DECRYPT===> ")))
            _dafny.print(_dafny.string_of((vector).id))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
            _dafny.print(_dafny.string_of(d_0_id_))
            _dafny.print(_dafny.string_of(_dafny.Seq(" ")))
            _dafny.print(_dafny.string_of((vector).description))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        elif True:
            _dafny.print(_dafny.string_of(_dafny.Seq("\nTEST-DECRYPT===> ")))
            _dafny.print(_dafny.string_of((vector).id))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
            _dafny.print(_dafny.string_of((vector).description))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        d_1_test_q_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = default__.DecryptVectorToDecryptTest(keys, vector)
        d_1_test_q_ = out0_
        if (d_1_test_q_).is_Failure:
            _dafny.print(_dafny.string_of((d_1_test_q_).error))
            _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
            _dafny.print(_dafny.string_of(_dafny.Seq("\nFAILED! <-----------\n")))
            output = False
            return output
        d_2_test_: DecryptTest
        d_2_test_ = (d_1_test_q_).value
        d_3_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out1_: Wrappers.Result
        out1_ = default__.ReadVectorsFile((((d_2_test_).vector).manifestPath) + (((d_2_test_).vector).ciphertextPath))
        d_3_valueOrError0_ = out1_
        if not(not((d_3_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(202,22): " + _dafny.string_of(d_3_valueOrError0_))
        d_4_ciphertext_: _dafny.Seq
        d_4_ciphertext_ = (d_3_valueOrError0_).Extract()
        d_5_plaintext_: _dafny.Seq = _dafny.Seq({})
        if (((d_2_test_).vector).is_PositiveDecryptTestVector) or (((d_2_test_).vector).is_PositiveV1OrV2DecryptTestVector):
            d_6_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            out2_: Wrappers.Result
            out2_ = default__.ReadVectorsFile((((d_2_test_).vector).manifestPath) + (((d_2_test_).vector).plaintextPath))
            d_6_valueOrError1_ = out2_
            if not(not((d_6_valueOrError1_).IsFailure())):
                raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(205,19): " + _dafny.string_of(d_6_valueOrError1_))
            d_5_plaintext_ = (d_6_valueOrError1_).Extract()
        d_7_input_: AwsCryptographyEncryptionSdkTypes.DecryptInput
        d_7_input_ = AwsCryptographyEncryptionSdkTypes.DecryptInput_DecryptInput(d_4_ciphertext_, Wrappers.Option_Some((d_2_test_).cmm), Wrappers.Option_None(), ((d_2_test_).vector).reproducedEncryptionContext)
        d_8_result_: Wrappers.Result
        out3_: Wrappers.Result
        out3_ = ((d_2_test_).client).Decrypt(d_7_input_)
        d_8_result_ = out3_
        source0_ = (d_2_test_).vector
        with _dafny.label("match0"):
            if True:
                if source0_.is_PositiveDecryptTestVector:
                    output = ((d_8_result_).is_Success) and ((((d_8_result_).value).plaintext) == (d_5_plaintext_))
                    raise _dafny.Break("match0")
            if True:
                if source0_.is_NegativeDecryptTestVector:
                    output = (True) and ((d_8_result_).is_Failure)
                    raise _dafny.Break("match0")
            if True:
                output = ((d_8_result_).is_Success) and ((((d_8_result_).value).plaintext) == (d_5_plaintext_))
            pass
        if not(output):
            if ((((d_2_test_).vector).is_PositiveDecryptTestVector) or (((d_2_test_).vector).is_PositiveV1OrV2DecryptTestVector)) and ((d_8_result_).is_Failure):
                _dafny.print(_dafny.string_of((d_8_result_).error))
                _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
                if (((d_8_result_).error).is_AwsCryptographyMaterialProviders) and ((((d_8_result_).error).AwsCryptographyMaterialProviders).is_CollectionOfErrors):
                    _dafny.print(_dafny.string_of(_dafny.Seq("list:")))
                    _dafny.print(_dafny.string_of((((d_8_result_).error).AwsCryptographyMaterialProviders).list))
                    _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
            _dafny.print(_dafny.string_of(_dafny.Seq("\nFAILED! <-----------\n")))
        return output

    @staticmethod
    def DecryptVectorToDecryptTest(keys, vector):
        output: Wrappers.Result = None
        d_0_valueOrError0_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_0_valueOrError0_ = Wrappers.default__.Need(not((vector).is_NegativeDecryptTestVector), AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_KeyVectorException(_dafny.Seq("Negative Test Vectors not supported at this time")))
        if (d_0_valueOrError0_).IsFailure():
            output = (d_0_valueOrError0_).PropagateFailure()
            return output
        d_1_valueOrError1_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = (keys).CreateWrappedTestVectorCmm(AwsCryptographyMaterialProvidersTestVectorKeysTypes.TestVectorCmmInput_TestVectorCmmInput((vector).decryptDescriptions, AwsCryptographyMaterialProvidersTestVectorKeysTypes.CmmOperation_DECRYPT()))
        d_1_valueOrError1_ = out0_
        if (d_1_valueOrError1_).IsFailure():
            output = (d_1_valueOrError1_).PropagateFailure()
            return output
        d_2_cmm_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_2_cmm_ = (d_1_valueOrError1_).Extract()
        d_3_commitmentPolicy_: AwsCryptographyMaterialProvidersTypes.CommitmentPolicy
        if ((vector).algorithmSuiteId).is_Some:
            d_3_commitmentPolicy_ = AllAlgorithmSuites.default__.GetCompatibleCommitmentPolicy(((vector).algorithmSuiteId).value)
        elif True:
            d_3_commitmentPolicy_ = AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT())
        d_4_valueOrError2_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_4_valueOrError2_ = Wrappers.default__.Need((d_3_commitmentPolicy_).is_ESDK, AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_KeyVectorException(_dafny.Seq("Compatible commitment policy is not for ESDK")))
        if (d_4_valueOrError2_).IsFailure():
            output = (d_4_valueOrError2_).PropagateFailure()
            return output
        d_5_config_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_5_config_ = WrappedESDK.default__.WrappedAwsEncryptionSdkConfigWithSuppliedCommitment((d_3_commitmentPolicy_).ESDK)
        d_6_valueOrError3_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = WrappedESDK.default__.WrappedESDK(d_5_config_)
        d_6_valueOrError3_ = out1_
        if not(not((d_6_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(281,18): " + _dafny.string_of(d_6_valueOrError3_))
        d_7_client_: AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
        d_7_client_ = (d_6_valueOrError3_).Extract()
        d_8_test_: DecryptTest
        d_8_test_ = DecryptTest_DecryptTest(d_2_cmm_, d_7_client_, vector)
        output = Wrappers.Result_Success(d_8_test_)
        return output

    @staticmethod
    def TestEncrypt(plaintexts, keys, test):
        output: Wrappers.Result = Wrappers.Result.default(EncryptTestOutput.default())()
        d_0_id_: _dafny.Seq
        d_0_id_ = AllAlgorithmSuites.default__.ToHex((((test).vector).algorithmSuiteId).value)
        _dafny.print(_dafny.string_of(_dafny.Seq("\nTEST-ENCRYPT===> ")))
        _dafny.print(_dafny.string_of((((test).vector).id).value))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        _dafny.print(_dafny.string_of(d_0_id_))
        _dafny.print(_dafny.string_of(_dafny.Seq(" ")))
        _dafny.print(_dafny.string_of(((test).vector).description))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        d_1_vector_: EsdkEncryptTestVector
        d_1_vector_ = (test).vector
        if not((((test).vector).plaintextPath) in (plaintexts)):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(324,4): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_2_plaintext_: _dafny.Seq
        d_2_plaintext_ = (plaintexts)[((test).vector).plaintextPath]
        d_3_frameLength_: Wrappers.Option
        d_3_frameLength_ = (d_1_vector_).frameLength
        d_4_input_: AwsCryptographyEncryptionSdkTypes.EncryptInput
        d_4_input_ = AwsCryptographyEncryptionSdkTypes.EncryptInput_EncryptInput(d_2_plaintext_, ((test).vector).encryptionContext, Wrappers.Option_Some((test).cmm), Wrappers.Option_None(), Wrappers.Option_Some((((((test).vector).algorithmSuiteId).value).id).ESDK), d_3_frameLength_)
        d_5_result_: Wrappers.Result
        out0_: Wrappers.Result
        out0_ = ((test).client).Encrypt(d_4_input_)
        d_5_result_ = out0_
        if ((d_5_result_).is_Success) and ((((test).vector).is_PositiveEncryptTestVector) or (((test).vector).is_PositiveEncryptNegativeDecryptTestVector)):
            d_6_valueOrError0_: Wrappers.Result = Wrappers.Result.default(EsdkDecryptTestVector.default())()
            out1_: Wrappers.Result
            out1_ = default__.EncryptTestToDecryptVector(test, (d_5_result_).value)
            d_6_valueOrError0_ = out1_
            if (d_6_valueOrError0_).IsFailure():
                output = (d_6_valueOrError0_).PropagateFailure()
                return output
            d_7_decryptVector_: EsdkDecryptTestVector
            d_7_decryptVector_ = (d_6_valueOrError0_).Extract()
            output = Wrappers.Result_Success(EncryptTestOutput_EncryptTestOutput(True, Wrappers.Option_Some(d_7_decryptVector_)))
        elif ((d_5_result_).is_Failure) and (((test).vector).is_NegativeEncryptTestVector):
            output = Wrappers.Result_Success(EncryptTestOutput_EncryptTestOutput(True, Wrappers.Option_None()))
        elif True:
            output = Wrappers.Result_Success(EncryptTestOutput_EncryptTestOutput(False, Wrappers.Option_None()))
            if (not(((test).vector).is_NegativeEncryptTestVector)) and ((d_5_result_).is_Failure):
                _dafny.print(_dafny.string_of((d_5_result_).error))
            _dafny.print(_dafny.string_of(_dafny.Seq("\nFAILED! <-----------\n")))
        return output

    @staticmethod
    def EncryptVectorToEncryptTest(keys, vector):
        output: Wrappers.Result = None
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = (keys).CreateWrappedTestVectorCmm(AwsCryptographyMaterialProvidersTestVectorKeysTypes.TestVectorCmmInput_TestVectorCmmInput(((vector).encryptDescriptions if (vector).is_PositiveEncryptTestVector else ((vector).encryptDescriptions if (vector).is_PositiveEncryptNegativeDecryptTestVector else (vector).encryptDescriptions)), AwsCryptographyMaterialProvidersTestVectorKeysTypes.CmmOperation_ENCRYPT()))
        d_0_valueOrError0_ = out0_
        if (d_0_valueOrError0_).IsFailure():
            output = (d_0_valueOrError0_).PropagateFailure()
            return output
        d_1_cmm_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_1_cmm_ = (d_0_valueOrError0_).Extract()
        d_2_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_2_valueOrError1_ = Wrappers.default__.Need(((vector).algorithmSuiteId).is_Some, AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_KeyVectorException(_dafny.Seq("Missing AlgorithmSuiteId in test vector")))
        if (d_2_valueOrError1_).IsFailure():
            output = (d_2_valueOrError1_).PropagateFailure()
            return output
        d_3_commitmentPolicy_: AwsCryptographyMaterialProvidersTypes.CommitmentPolicy
        d_3_commitmentPolicy_ = AllAlgorithmSuites.default__.GetCompatibleCommitmentPolicy(((vector).algorithmSuiteId).value)
        d_4_valueOrError2_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_4_valueOrError2_ = Wrappers.default__.Need((d_3_commitmentPolicy_).is_ESDK, AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_KeyVectorException(_dafny.Seq("Compatible commitment policy is not for ESDK")))
        if (d_4_valueOrError2_).IsFailure():
            output = (d_4_valueOrError2_).PropagateFailure()
            return output
        d_5_config_: AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
        d_5_config_ = WrappedESDK.default__.WrappedAwsEncryptionSdkConfigWithSuppliedCommitment((d_3_commitmentPolicy_).ESDK)
        d_6_valueOrError3_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = WrappedESDK.default__.WrappedESDK(d_5_config_)
        d_6_valueOrError3_ = out1_
        if not(not((d_6_valueOrError3_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(397,18): " + _dafny.string_of(d_6_valueOrError3_))
        d_7_client_: AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
        d_7_client_ = (d_6_valueOrError3_).Extract()
        d_8_test_: EncryptTest
        d_8_test_ = EncryptTest_EncryptTest(d_1_cmm_, d_7_client_, vector)
        output = Wrappers.Result_Success(d_8_test_)
        return output

    @staticmethod
    def EncryptTestToDecryptVector(test, result):
        output: Wrappers.Result = Wrappers.Result.default(EsdkDecryptTestVector.default())()
        source0_ = (test).vector
        with _dafny.label("match0"):
            if True:
                if source0_.is_PositiveEncryptTestVector:
                    output = Wrappers.Result_Success(EsdkDecryptTestVector_PositiveDecryptTestVector((((test).vector).id).value, 3, ((test).vector).decryptManifestPath, default__.ciphertextPathPathRoot, (default__.plaintextPathRoot) + (((test).vector).plaintextPath), ((test).vector).reproducedEncryptionContext, ((test).vector).decryptDescriptions, ((test).vector).commitmentPolicy, ((test).vector).frameLength, ((test).vector).algorithmSuiteId, ((test).vector).description, DecryptionMethod_OneShot()))
                    raise _dafny.Break("match0")
            if True:
                output = Wrappers.Result_Failure(_dafny.Seq("Only postive tests supported"))
            pass
        d_0_decryptManifestCiphertext_: _dafny.Seq
        d_0_decryptManifestCiphertext_ = ((((test).vector).decryptManifestPath) + (default__.ciphertextPathPathRoot)) + ((((test).vector).id).value)
        d_1_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        out0_: Wrappers.Result
        out0_ = default__.WriteVectorsFile(d_0_decryptManifestCiphertext_, (result).ciphertext)
        d_1_valueOrError0_ = out0_
        if not(not((d_1_valueOrError0_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(439,13): " + _dafny.string_of(d_1_valueOrError0_))
        d_2___v52_: tuple
        d_2___v52_ = (d_1_valueOrError0_).Extract()
        return output

    @staticmethod
    def MplPrintErr(e):
        hresult_: tuple = ()
        _dafny.print(_dafny.string_of(e))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        hresult_ = ()
        return hresult_
        return hresult_

    @staticmethod
    def MplVectorPrintErr(e):
        hresult_: tuple = ()
        _dafny.print(_dafny.string_of(e))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        _dafny.print(_dafny.string_of(_dafny.Seq("\n")))
        hresult_ = ()
        return hresult_
        return hresult_

    @staticmethod
    def KeyDescriptionToCmm(keys, keyDescriptions):
        output: Wrappers.Result = None
        d_0_keyringList_: _dafny.Seq
        d_0_keyringList_ = _dafny.Seq([])
        hi0_ = len(keyDescriptions)
        for d_1_i_ in range(0, hi0_):
            d_2_keyDescription_: AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
            d_2_keyDescription_ = (keyDescriptions)[d_1_i_]
            d_3_valueOrError0_: Wrappers.Result = None
            out0_: Wrappers.Result
            out0_ = (keys).CreateWrappedTestVectorKeyring(AwsCryptographyMaterialProvidersTestVectorKeysTypes.TestVectorKeyringInput_TestVectorKeyringInput(d_2_keyDescription_))
            d_3_valueOrError0_ = out0_
            if (d_3_valueOrError0_).IsFailure():
                output = (d_3_valueOrError0_).PropagateFailure()
                return output
            d_4_keyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
            d_4_keyring_ = (d_3_valueOrError0_).Extract()
            d_0_keyringList_ = (d_0_keyringList_) + (_dafny.Seq([d_4_keyring_]))
        d_5_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_5_valueOrError1_ = Wrappers.default__.Need((len(d_0_keyringList_)) == (1), AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_KeyVectorException(_dafny.Seq("Failed to create any keyrings")))
        if (d_5_valueOrError1_).IsFailure():
            output = (d_5_valueOrError1_).PropagateFailure()
            return output
        d_6_valueOrError2_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = WrappedMaterialProviders.default__.WrappedMaterialProviders(WrappedMaterialProviders.default__.WrappedDefaultMaterialProvidersConfig())
        d_6_valueOrError2_ = out1_
        if not(not((d_6_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/EsdkTestVectors.dfy(482,15): " + _dafny.string_of(d_6_valueOrError2_))
        d_7_mpl_: AwsCryptographyMaterialProvidersTypes.IAwsCryptographicMaterialProvidersClient
        d_7_mpl_ = (d_6_valueOrError2_).Extract()
        d_8_generatorKeyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_8_generatorKeyring_ = (d_0_keyringList_)[0]
        d_9_maybeMultiKeyring_: Wrappers.Result
        out2_: Wrappers.Result
        out2_ = (d_7_mpl_).CreateMultiKeyring(AwsCryptographyMaterialProvidersTypes.CreateMultiKeyringInput_CreateMultiKeyringInput(Wrappers.Option_Some(d_8_generatorKeyring_), _dafny.Seq((d_0_keyringList_)[1::])))
        d_9_maybeMultiKeyring_ = out2_
        d_10_valueOrError3_: Wrappers.Result = None
        def lambda0_(d_11_e_):
            return AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_AwsCryptographyMaterialProviders(d_11_e_)

        d_10_valueOrError3_ = (d_9_maybeMultiKeyring_).MapFailure(lambda0_)
        if (d_10_valueOrError3_).IsFailure():
            output = (d_10_valueOrError3_).PropagateFailure()
            return output
        d_12_keyring_: AwsCryptographyMaterialProvidersTypes.IKeyring
        d_12_keyring_ = (d_10_valueOrError3_).Extract()
        d_13_maybeCmm_: Wrappers.Result
        out3_: Wrappers.Result
        out3_ = (d_7_mpl_).CreateDefaultCryptographicMaterialsManager(AwsCryptographyMaterialProvidersTypes.CreateDefaultCryptographicMaterialsManagerInput_CreateDefaultCryptographicMaterialsManagerInput((d_9_maybeMultiKeyring_).value))
        d_13_maybeCmm_ = out3_
        def lambda1_(d_14_e_):
            return AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error_AwsCryptographyMaterialProviders(d_14_e_)

        output = (d_13_maybeCmm_).MapFailure(lambda1_)
        return output

    @staticmethod
    def ReadVectorsFile(location):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        out0_: Wrappers.Result
        out0_ = FileIO.default__.ReadBytesFromFile(location)
        d_0_valueOrError0_ = out0_
        if (d_0_valueOrError0_).IsFailure():
            output = (d_0_valueOrError0_).PropagateFailure()
            return output
        d_1_fileBv_: _dafny.Seq
        d_1_fileBv_ = (d_0_valueOrError0_).Extract()
        output = Wrappers.Result_Success(JSONHelpers.default__.BvToBytes(d_1_fileBv_))
        return output

    @staticmethod
    def WriteVectorsFile(location, bytes):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_bv_: _dafny.Seq
        d_0_bv_ = JSONHelpers.default__.BytesBv(bytes)
        out0_: Wrappers.Result
        out0_ = FileIO.default__.WriteBytesToFile(location, d_0_bv_)
        output = out0_
        return output

    @_dafny.classproperty
    def ciphertextPathPathRoot(instance):
        return _dafny.Seq("ciphertexts/")
    @_dafny.classproperty
    def plaintextPathRoot(instance):
        return _dafny.Seq("plaintexts/")

class EncryptTest:
    @classmethod
    def default(cls, ):
        return lambda: EncryptTest_EncryptTest(None, None, EsdkEncryptTestVector.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_EncryptTest(self) -> bool:
        return isinstance(self, EncryptTest_EncryptTest)

class EncryptTest_EncryptTest(EncryptTest, NamedTuple('EncryptTest', [('cmm', Any), ('client', Any), ('vector', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EncryptTest.EncryptTest({_dafny.string_of(self.cmm)}, {_dafny.string_of(self.client)}, {_dafny.string_of(self.vector)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EncryptTest_EncryptTest) and self.cmm == __o.cmm and self.client == __o.client and self.vector == __o.vector
    def __hash__(self) -> int:
        return super().__hash__()


class DecryptTest:
    @classmethod
    def default(cls, ):
        return lambda: DecryptTest_DecryptTest(None, None, EsdkDecryptTestVector.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_DecryptTest(self) -> bool:
        return isinstance(self, DecryptTest_DecryptTest)

class DecryptTest_DecryptTest(DecryptTest, NamedTuple('DecryptTest', [('cmm', Any), ('client', Any), ('vector', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.DecryptTest.DecryptTest({_dafny.string_of(self.cmm)}, {_dafny.string_of(self.client)}, {_dafny.string_of(self.vector)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DecryptTest_DecryptTest) and self.cmm == __o.cmm and self.client == __o.client and self.vector == __o.vector
    def __hash__(self) -> int:
        return super().__hash__()


class SupportedGenerateManifestVersion:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return 4
    def _Is(source__):
        d_0_v_: int = source__
        if System_.nat._Is(d_0_v_):
            return default__.SupportedGenerateManifestVersion_q(d_0_v_)
        return False

class SupportedEncryptVersion:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return 1
    def _Is(source__):
        d_1_v_: int = source__
        if System_.nat._Is(d_1_v_):
            return default__.SupportedEncryptVersion_q(d_1_v_)
        return False

class EsdkEncryptTestVector:
    @classmethod
    def default(cls, ):
        return lambda: EsdkEncryptTestVector_PositiveEncryptTestVector(Wrappers.Option.default()(), SupportedEncryptVersion.default(), _dafny.Seq(""), _dafny.Seq(""), _dafny.Seq(""), AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription.default()(), AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription.default()(), Wrappers.Option.default()(), Wrappers.Option.default()(), AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy.default()(), Wrappers.Option.default()(), Wrappers.Option.default()(), _dafny.Seq(""), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_PositiveEncryptTestVector(self) -> bool:
        return isinstance(self, EsdkEncryptTestVector_PositiveEncryptTestVector)
    @property
    def is_PositiveEncryptNegativeDecryptTestVector(self) -> bool:
        return isinstance(self, EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector)
    @property
    def is_NegativeEncryptTestVector(self) -> bool:
        return isinstance(self, EsdkEncryptTestVector_NegativeEncryptTestVector)

class EsdkEncryptTestVector_PositiveEncryptTestVector(EsdkEncryptTestVector, NamedTuple('PositiveEncryptTestVector', [('id', Any), ('version', Any), ('manifestPath', Any), ('decryptManifestPath', Any), ('plaintextPath', Any), ('encryptDescriptions', Any), ('decryptDescriptions', Any), ('encryptionContext', Any), ('reproducedEncryptionContext', Any), ('commitmentPolicy', Any), ('frameLength', Any), ('algorithmSuiteId', Any), ('description', Any), ('maxEncryptedDataKeys', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EsdkEncryptTestVector.PositiveEncryptTestVector({_dafny.string_of(self.id)}, {_dafny.string_of(self.version)}, {_dafny.string_of(self.manifestPath)}, {_dafny.string_of(self.decryptManifestPath)}, {_dafny.string_of(self.plaintextPath)}, {_dafny.string_of(self.encryptDescriptions)}, {_dafny.string_of(self.decryptDescriptions)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.reproducedEncryptionContext)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.description)}, {_dafny.string_of(self.maxEncryptedDataKeys)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EsdkEncryptTestVector_PositiveEncryptTestVector) and self.id == __o.id and self.version == __o.version and self.manifestPath == __o.manifestPath and self.decryptManifestPath == __o.decryptManifestPath and self.plaintextPath == __o.plaintextPath and self.encryptDescriptions == __o.encryptDescriptions and self.decryptDescriptions == __o.decryptDescriptions and self.encryptionContext == __o.encryptionContext and self.reproducedEncryptionContext == __o.reproducedEncryptionContext and self.commitmentPolicy == __o.commitmentPolicy and self.frameLength == __o.frameLength and self.algorithmSuiteId == __o.algorithmSuiteId and self.description == __o.description and self.maxEncryptedDataKeys == __o.maxEncryptedDataKeys
    def __hash__(self) -> int:
        return super().__hash__()

class EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector(EsdkEncryptTestVector, NamedTuple('PositiveEncryptNegativeDecryptTestVector', [('id', Any), ('version', Any), ('manifestPath', Any), ('decryptManifestPath', Any), ('plaintextPath', Any), ('encryptDescriptions', Any), ('decryptDescriptions', Any), ('encryptionContext', Any), ('reproducedEncryptionContext', Any), ('commitmentPolicy', Any), ('frameLength', Any), ('algorithmSuiteId', Any), ('decryptErrorDescription', Any), ('description', Any), ('maxEncryptedDataKeys', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EsdkEncryptTestVector.PositiveEncryptNegativeDecryptTestVector({_dafny.string_of(self.id)}, {_dafny.string_of(self.version)}, {_dafny.string_of(self.manifestPath)}, {_dafny.string_of(self.decryptManifestPath)}, {_dafny.string_of(self.plaintextPath)}, {_dafny.string_of(self.encryptDescriptions)}, {_dafny.string_of(self.decryptDescriptions)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.reproducedEncryptionContext)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.decryptErrorDescription)}, {_dafny.string_of(self.description)}, {_dafny.string_of(self.maxEncryptedDataKeys)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector) and self.id == __o.id and self.version == __o.version and self.manifestPath == __o.manifestPath and self.decryptManifestPath == __o.decryptManifestPath and self.plaintextPath == __o.plaintextPath and self.encryptDescriptions == __o.encryptDescriptions and self.decryptDescriptions == __o.decryptDescriptions and self.encryptionContext == __o.encryptionContext and self.reproducedEncryptionContext == __o.reproducedEncryptionContext and self.commitmentPolicy == __o.commitmentPolicy and self.frameLength == __o.frameLength and self.algorithmSuiteId == __o.algorithmSuiteId and self.decryptErrorDescription == __o.decryptErrorDescription and self.description == __o.description and self.maxEncryptedDataKeys == __o.maxEncryptedDataKeys
    def __hash__(self) -> int:
        return super().__hash__()

class EsdkEncryptTestVector_NegativeEncryptTestVector(EsdkEncryptTestVector, NamedTuple('NegativeEncryptTestVector', [('id', Any), ('version', Any), ('manifestPath', Any), ('plaintextPath', Any), ('encryptDescriptions', Any), ('encryptionContext', Any), ('reproducedEncryptionContext', Any), ('commitmentPolicy', Any), ('frameLength', Any), ('algorithmSuiteId', Any), ('errorDescription', Any), ('description', Any), ('maxEncryptedDataKeys', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EsdkEncryptTestVector.NegativeEncryptTestVector({_dafny.string_of(self.id)}, {_dafny.string_of(self.version)}, {_dafny.string_of(self.manifestPath)}, {_dafny.string_of(self.plaintextPath)}, {_dafny.string_of(self.encryptDescriptions)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.reproducedEncryptionContext)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.errorDescription)}, {_dafny.string_of(self.description)}, {_dafny.string_of(self.maxEncryptedDataKeys)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EsdkEncryptTestVector_NegativeEncryptTestVector) and self.id == __o.id and self.version == __o.version and self.manifestPath == __o.manifestPath and self.plaintextPath == __o.plaintextPath and self.encryptDescriptions == __o.encryptDescriptions and self.encryptionContext == __o.encryptionContext and self.reproducedEncryptionContext == __o.reproducedEncryptionContext and self.commitmentPolicy == __o.commitmentPolicy and self.frameLength == __o.frameLength and self.algorithmSuiteId == __o.algorithmSuiteId and self.errorDescription == __o.errorDescription and self.description == __o.description and self.maxEncryptedDataKeys == __o.maxEncryptedDataKeys
    def __hash__(self) -> int:
        return super().__hash__()


class SupportedDecryptVersion:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return 1
    def _Is(source__):
        d_2_v_: int = source__
        if System_.nat._Is(d_2_v_):
            return default__.SupportedDecryptVersion_q(d_2_v_)
        return False

class EsdkDecryptTestVector:
    @classmethod
    def default(cls, ):
        return lambda: EsdkDecryptTestVector_PositiveDecryptTestVector(_dafny.Seq(""), SupportedDecryptVersion.default(), _dafny.Seq(""), _dafny.Seq(""), _dafny.Seq(""), Wrappers.Option.default()(), AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription.default()(), AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy.default()(), Wrappers.Option.default()(), Wrappers.Option.default()(), _dafny.Seq(""), DecryptionMethod.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_PositiveDecryptTestVector(self) -> bool:
        return isinstance(self, EsdkDecryptTestVector_PositiveDecryptTestVector)
    @property
    def is_NegativeDecryptTestVector(self) -> bool:
        return isinstance(self, EsdkDecryptTestVector_NegativeDecryptTestVector)
    @property
    def is_PositiveV1OrV2DecryptTestVector(self) -> bool:
        return isinstance(self, EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector)

class EsdkDecryptTestVector_PositiveDecryptTestVector(EsdkDecryptTestVector, NamedTuple('PositiveDecryptTestVector', [('id', Any), ('version', Any), ('manifestPath', Any), ('ciphertextPath', Any), ('plaintextPath', Any), ('reproducedEncryptionContext', Any), ('decryptDescriptions', Any), ('commitmentPolicy', Any), ('frameLength', Any), ('algorithmSuiteId', Any), ('description', Any), ('decryptionMethod', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EsdkDecryptTestVector.PositiveDecryptTestVector({_dafny.string_of(self.id)}, {_dafny.string_of(self.version)}, {_dafny.string_of(self.manifestPath)}, {_dafny.string_of(self.ciphertextPath)}, {_dafny.string_of(self.plaintextPath)}, {_dafny.string_of(self.reproducedEncryptionContext)}, {_dafny.string_of(self.decryptDescriptions)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.description)}, {_dafny.string_of(self.decryptionMethod)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EsdkDecryptTestVector_PositiveDecryptTestVector) and self.id == __o.id and self.version == __o.version and self.manifestPath == __o.manifestPath and self.ciphertextPath == __o.ciphertextPath and self.plaintextPath == __o.plaintextPath and self.reproducedEncryptionContext == __o.reproducedEncryptionContext and self.decryptDescriptions == __o.decryptDescriptions and self.commitmentPolicy == __o.commitmentPolicy and self.frameLength == __o.frameLength and self.algorithmSuiteId == __o.algorithmSuiteId and self.description == __o.description and self.decryptionMethod == __o.decryptionMethod
    def __hash__(self) -> int:
        return super().__hash__()

class EsdkDecryptTestVector_NegativeDecryptTestVector(EsdkDecryptTestVector, NamedTuple('NegativeDecryptTestVector', [('id', Any), ('version', Any), ('manifestPath', Any), ('ciphertextPath', Any), ('errorDescription', Any), ('reproducedEncryptionContext', Any), ('decryptDescriptions', Any), ('commitmentPolicy', Any), ('frameLength', Any), ('algorithmSuiteId', Any), ('description', Any), ('decryptionMethod', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EsdkDecryptTestVector.NegativeDecryptTestVector({_dafny.string_of(self.id)}, {_dafny.string_of(self.version)}, {_dafny.string_of(self.manifestPath)}, {_dafny.string_of(self.ciphertextPath)}, {_dafny.string_of(self.errorDescription)}, {_dafny.string_of(self.reproducedEncryptionContext)}, {_dafny.string_of(self.decryptDescriptions)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.description)}, {_dafny.string_of(self.decryptionMethod)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EsdkDecryptTestVector_NegativeDecryptTestVector) and self.id == __o.id and self.version == __o.version and self.manifestPath == __o.manifestPath and self.ciphertextPath == __o.ciphertextPath and self.errorDescription == __o.errorDescription and self.reproducedEncryptionContext == __o.reproducedEncryptionContext and self.decryptDescriptions == __o.decryptDescriptions and self.commitmentPolicy == __o.commitmentPolicy and self.frameLength == __o.frameLength and self.algorithmSuiteId == __o.algorithmSuiteId and self.description == __o.description and self.decryptionMethod == __o.decryptionMethod
    def __hash__(self) -> int:
        return super().__hash__()

class EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector(EsdkDecryptTestVector, NamedTuple('PositiveV1OrV2DecryptTestVector', [('id', Any), ('version', Any), ('manifestPath', Any), ('ciphertextPath', Any), ('plaintextPath', Any), ('reproducedEncryptionContext', Any), ('requiredEncryptionContextKeys', Any), ('decryptDescriptions', Any), ('commitmentPolicy', Any), ('frameLength', Any), ('algorithmSuiteId', Any), ('description', Any), ('decryptionMethod', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EsdkDecryptTestVector.PositiveV1OrV2DecryptTestVector({_dafny.string_of(self.id)}, {_dafny.string_of(self.version)}, {_dafny.string_of(self.manifestPath)}, {_dafny.string_of(self.ciphertextPath)}, {_dafny.string_of(self.plaintextPath)}, {_dafny.string_of(self.reproducedEncryptionContext)}, {_dafny.string_of(self.requiredEncryptionContextKeys)}, {_dafny.string_of(self.decryptDescriptions)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.description)}, {_dafny.string_of(self.decryptionMethod)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector) and self.id == __o.id and self.version == __o.version and self.manifestPath == __o.manifestPath and self.ciphertextPath == __o.ciphertextPath and self.plaintextPath == __o.plaintextPath and self.reproducedEncryptionContext == __o.reproducedEncryptionContext and self.requiredEncryptionContextKeys == __o.requiredEncryptionContextKeys and self.decryptDescriptions == __o.decryptDescriptions and self.commitmentPolicy == __o.commitmentPolicy and self.frameLength == __o.frameLength and self.algorithmSuiteId == __o.algorithmSuiteId and self.description == __o.description and self.decryptionMethod == __o.decryptionMethod
    def __hash__(self) -> int:
        return super().__hash__()


class DecryptionMethod:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [DecryptionMethod_StreamingUnsignedOnly(), DecryptionMethod_OneShot()]
    @classmethod
    def default(cls, ):
        return lambda: DecryptionMethod_StreamingUnsignedOnly()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_StreamingUnsignedOnly(self) -> bool:
        return isinstance(self, DecryptionMethod_StreamingUnsignedOnly)
    @property
    def is_OneShot(self) -> bool:
        return isinstance(self, DecryptionMethod_OneShot)

class DecryptionMethod_StreamingUnsignedOnly(DecryptionMethod, NamedTuple('StreamingUnsignedOnly', [])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.DecryptionMethod.StreamingUnsignedOnly'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DecryptionMethod_StreamingUnsignedOnly)
    def __hash__(self) -> int:
        return super().__hash__()

class DecryptionMethod_OneShot(DecryptionMethod, NamedTuple('OneShot', [])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.DecryptionMethod.OneShot'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DecryptionMethod_OneShot)
    def __hash__(self) -> int:
        return super().__hash__()


class EncryptTestOutput:
    @classmethod
    def default(cls, ):
        return lambda: EncryptTestOutput_EncryptTestOutput(False, Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_EncryptTestOutput(self) -> bool:
        return isinstance(self, EncryptTestOutput_EncryptTestOutput)

class EncryptTestOutput_EncryptTestOutput(EncryptTestOutput, NamedTuple('EncryptTestOutput', [('output', Any), ('vector', Any)])):
    def __dafnystr__(self) -> str:
        return f'EsdkTestVectors.EncryptTestOutput.EncryptTestOutput({_dafny.string_of(self.output)}, {_dafny.string_of(self.vector)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EncryptTestOutput_EncryptTestOutput) and self.output == __o.output and self.vector == __o.vector
    def __hash__(self) -> int:
        return super().__hash__()

