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
import .internaldafny.generated.EsdkTestManifests as EsdkTestManifests

# Module: WrappedESDKMain

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def Main2(args):
        d_0_vectorOptions_: GetOpt.Options
        d_0_vectorOptions_ = GetOpt.Options_Options(_dafny.Seq("test-vectors"), _dafny.Seq("?"), _dafny.Seq([GetOpt.Param_Command(GetOpt.Options_Options(_dafny.Seq("decrypt"), _dafny.Seq("decrypt command for test-vectors"), _dafny.Seq([GetOpt.Param_Opt(_dafny.Seq("manifest-path"), _dafny.Seq("relative path to the location of the manifest"), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_Required(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No()), GetOpt.Param_Opt(_dafny.Seq("manifest-name"), _dafny.Seq("name of file that contains the decrypt vectors file"), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_Required(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No()), GetOpt.Param_Opt(_dafny.Seq("test-name"), _dafny.Seq("id of the test to run"), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_UnusedOk(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No())]))), GetOpt.Param_Command(GetOpt.Options_Options(_dafny.Seq("encrypt"), _dafny.Seq("encrypt command for test-vectors"), _dafny.Seq([GetOpt.Param_Opt(_dafny.Seq("manifest-path"), _dafny.Seq("relative path to the location of the manifest"), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_Required(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No()), GetOpt.Param_Opt(_dafny.Seq("decrypt-manifest-path"), _dafny.Seq("relative path to the location where the decrypted manifest will be written to."), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_Required(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No()), GetOpt.Param_Opt(_dafny.Seq("test-name"), _dafny.Seq("id of the test to run"), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_UnusedOk(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No())]))), GetOpt.Param_Command(GetOpt.Options_Options(_dafny.Seq("encrypt-manifest"), _dafny.Seq("encryp manifest command for test-vectors"), _dafny.Seq([GetOpt.Param_Opt(_dafny.Seq("encrypt-manifest-output"), _dafny.Seq("relative path of where to store the encrypt-manifest produced"), _dafny.Seq("arg"), GetOpt.default__.NullChar, GetOpt.Unused_Required(), False, GetOpt.Visibility_Normal(), _dafny.Seq([]), _dafny.Seq([]), GetOpt.Tri_No())])))]))
        if not((0) < (len(args))):
            raise _dafny.HaltException("dafny/TestVectors/src/Index.dfy(40,4): " + _dafny.string_of(_dafny.Seq("expectation violation")))
        d_1_parsedOptions_q_: Wrappers.Result
        d_1_parsedOptions_q_ = GetOpt.default__.GetOptions(d_0_vectorOptions_, args)
        if (d_1_parsedOptions_q_).is_Success:
            d_2_h_: Wrappers.Option
            d_2_h_ = GetOpt.default__.NeedsHelp(d_0_vectorOptions_, (d_1_parsedOptions_q_).value, _dafny.Seq(""))
            if (d_2_h_).is_Some:
                _dafny.print(_dafny.string_of((d_2_h_).value))
                return
            d_3_op_q_: Wrappers.Result
            d_3_op_q_ = default__.ParseCommandLineOptions((d_1_parsedOptions_q_).value)
            if (d_3_op_q_).is_Success:
                d_4_op_: EsdkManifestOptions.ManifestOptions
                d_4_op_ = (d_3_op_q_).value
                source0_ = d_4_op_
                with _dafny.label("match0"):
                    if True:
                        if source0_.is_Decrypt:
                            d_5_result_: Wrappers.Result
                            out0_: Wrappers.Result
                            out0_ = EsdkTestManifests.default__.StartDecryptVectors(d_4_op_)
                            d_5_result_ = out0_
                            if (d_5_result_).is_Failure:
                                _dafny.print(_dafny.string_of((d_5_result_).error))
                            if not((d_5_result_).is_Success):
                                raise _dafny.HaltException("dafny/TestVectors/src/Index.dfy(59,10): " + _dafny.string_of(_dafny.Seq("expectation violation")))
                            raise _dafny.Break("match0")
                    if True:
                        if source0_.is_Encrypt:
                            d_6_result_: Wrappers.Result
                            out1_: Wrappers.Result
                            out1_ = EsdkTestManifests.default__.StartEncryptVectors(d_4_op_)
                            d_6_result_ = out1_
                            if (d_6_result_).is_Failure:
                                _dafny.print(_dafny.string_of((d_6_result_).error))
                            if not((d_6_result_).is_Success):
                                raise _dafny.HaltException("dafny/TestVectors/src/Index.dfy(65,10): " + _dafny.string_of(_dafny.Seq("expectation violation")))
                            raise _dafny.Break("match0")
                    if True:
                        d_7_result_: Wrappers.Result
                        out2_: Wrappers.Result
                        out2_ = WriteVectors.default__.WriteTestVectors(d_4_op_)
                        d_7_result_ = out2_
                        if (d_7_result_).is_Failure:
                            _dafny.print(_dafny.string_of((d_7_result_).error))
                        if not((d_7_result_).is_Success):
                            raise _dafny.HaltException("dafny/TestVectors/src/Index.dfy(71,10): " + _dafny.string_of(_dafny.Seq("expectation violation")))
                    pass
            elif True:
                _dafny.print(_dafny.string_of(((d_3_op_q_).error) + (_dafny.Seq("\n"))))
                _dafny.print(_dafny.string_of(_dafny.Seq("help\n")))
        elif True:
            _dafny.print(_dafny.string_of(((d_1_parsedOptions_q_).error) + (_dafny.Seq("\n"))))

    @staticmethod
    def ParseCommandLineOptions(parsedOptions):
        d_0_valueOrError0_ = Wrappers.default__.Need(((parsedOptions).subcommand).is_Some, _dafny.Seq("Must supply subcommand\n"))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            source0_ = (((parsedOptions).subcommand).value).command
            if True:
                if (source0_) == (_dafny.Seq("decrypt")):
                    return default__.ParseDecryptCmd((((parsedOptions).subcommand).value).params)
            if True:
                if (source0_) == (_dafny.Seq("encrypt")):
                    return default__.ParseEncryptCmd((((parsedOptions).subcommand).value).params)
            if True:
                if (source0_) == (_dafny.Seq("encrypt-manifest")):
                    return default__.ParseEncryptManifestCmd((((parsedOptions).subcommand).value).params)
            if True:
                return Wrappers.Result_Failure(_dafny.Seq("Received unknown subcommand"))

    @staticmethod
    def ParseDecryptCmd(params):
        d_0_manifestPath_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("manifest-path"))
        d_1_testName_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("test-name"))
        d_2_manifestFileName_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("manifest-name"))
        d_3_manifestPath_ = ((d_0_manifestPath_q_).value if (d_0_manifestPath_q_).is_Some else _dafny.Seq("."))
        d_4_valueOrError0_ = Wrappers.default__.Need((0) < (len(d_3_manifestPath_)), _dafny.Seq("Invalid manifest path length\n"))
        if (d_4_valueOrError0_).IsFailure():
            return (d_4_valueOrError0_).PropagateFailure()
        elif True:
            d_5_valueOrError1_ = Wrappers.default__.Need((d_2_manifestFileName_q_).is_Some, _dafny.Seq("Must supply manifest file name"))
            if (d_5_valueOrError1_).IsFailure():
                return (d_5_valueOrError1_).PropagateFailure()
            elif True:
                d_6_manifestFileName_ = (d_2_manifestFileName_q_).value
                return Wrappers.Result_Success(EsdkManifestOptions.ManifestOptions_Decrypt((d_3_manifestPath_ if (Seq.default__.Last(d_3_manifestPath_)) == ('/') else (d_3_manifestPath_) + (_dafny.Seq("/"))), d_6_manifestFileName_, (d_1_testName_q_ if (d_1_testName_q_).is_Some else Wrappers.Option_None())))

    @staticmethod
    def ParseEncryptCmd(params):
        d_0_manifestPath_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("manifest-path"))
        d_1_manifestName_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("manifest"))
        d_2_decryptManifestPath_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("decrypt-manifest-path"))
        d_3_testName_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("test-name"))
        d_4_manifestPath_ = ((d_0_manifestPath_q_).value if (d_0_manifestPath_q_).is_Some else _dafny.Seq("."))
        d_5_manifestName_ = ((d_1_manifestName_q_).value if (d_1_manifestName_q_).is_Some else _dafny.Seq("encrypt-manifest.json"))
        d_6_decryptManifestPath_ = ((d_2_decryptManifestPath_q_).value if (d_2_decryptManifestPath_q_).is_Some else _dafny.Seq("."))
        d_7_valueOrError0_ = Wrappers.default__.Need(((0) < (len(d_4_manifestPath_))) and ((0) < (len(d_6_decryptManifestPath_))), _dafny.Seq("Invalid manifest or decrypt manifest path length\n"))
        if (d_7_valueOrError0_).IsFailure():
            return (d_7_valueOrError0_).PropagateFailure()
        elif True:
            return Wrappers.Result_Success(EsdkManifestOptions.ManifestOptions_Encrypt((d_4_manifestPath_ if (Seq.default__.Last(d_4_manifestPath_)) == ('/') else (d_4_manifestPath_) + (_dafny.Seq("/"))), d_5_manifestName_, (d_6_decryptManifestPath_ if (Seq.default__.Last(d_6_decryptManifestPath_)) == ('/') else (d_6_decryptManifestPath_) + (_dafny.Seq("/"))), (d_3_testName_q_ if (d_3_testName_q_).is_Some else Wrappers.Option_None())))

    @staticmethod
    def ParseEncryptManifestCmd(params):
        d_0_encryptManifestOutput_q_ = GetOpt.default__.OptValue(params, _dafny.Seq("encrypt-manifest-output"))
        d_1_encryptManifestOutput_ = ((d_0_encryptManifestOutput_q_).value if (d_0_encryptManifestOutput_q_).is_Some else _dafny.Seq("."))
        d_2_valueOrError0_ = Wrappers.default__.Need((0) < (len(d_1_encryptManifestOutput_)), _dafny.Seq("Invalid encrypt manifest output length"))
        if (d_2_valueOrError0_).IsFailure():
            return (d_2_valueOrError0_).PropagateFailure()
        elif True:
            return Wrappers.Result_Success(EsdkManifestOptions.ManifestOptions_EncryptManifest((d_1_encryptManifestOutput_ if (Seq.default__.Last(d_1_encryptManifestOutput_)) == ('/') else (d_1_encryptManifestOutput_) + (_dafny.Seq("/"))), 5))

