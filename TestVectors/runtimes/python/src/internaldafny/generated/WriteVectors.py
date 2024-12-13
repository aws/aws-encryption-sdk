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

# Module: WriteVectors

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def GetCommitmentPolicyString(algorithmSuite):
        source0_ = (algorithmSuite).id
        if True:
            if source0_.is_ESDK:
                if ((algorithmSuite).commitment).is_None:
                    return _dafny.Seq("FORBID_ENCRYPT_ALLOW_DECRYPT")
                elif True:
                    return _dafny.Seq("REQUIRE_ENCRYPT_REQUIRE_DECRYPT")
        if True:
            return _dafny.Seq("NOT SUPPORTED FOR UNSTRUCTURED ENCRYPTION")

    @staticmethod
    def GetCommitmentPolicyType(commitmentPolicy):
        if (commitmentPolicy) == (_dafny.Seq("FORBID_ENCRYPT_ALLOW_DECRYPT")):
            return AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_FORBID__ENCRYPT__ALLOW__DECRYPT())
        elif True:
            return AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy_REQUIRE__ENCRYPT__REQUIRE__DECRYPT())

    @staticmethod
    def WriteTestVectors(op):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_version_: int
        d_0_version_ = (op).version
        d_1_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Set)()
        d_1_valueOrError0_ = default__.getVersionTests(d_0_version_)
        if (d_1_valueOrError0_).IsFailure():
            output = (d_1_valueOrError0_).PropagateFailure()
            return output
        d_2_allTests_: _dafny.Set
        d_2_allTests_ = (d_1_valueOrError0_).Extract()
        d_3_tests_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = SortedSets.default__.SetToSequence(d_2_allTests_)
        d_3_tests_ = out0_
        d_4_sortedTests_: _dafny.Seq
        d_4_sortedTests_ = Seq_MergeSort.default__.MergeSortBy(d_3_tests_, default__.DescriptionLessThan)
        d_5_testsJSON_: _dafny.Seq
        d_5_testsJSON_ = _dafny.Seq([])
        hi0_ = len(d_4_sortedTests_)
        for d_6_i_ in range(0, hi0_):
            d_7_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_7_valueOrError1_ = Wrappers.default__.Need((True) and ((((d_4_sortedTests_)[d_6_i_]).algorithmSuiteId).is_Some), _dafny.Seq("No algorithm suite defined in test"))
            if (d_7_valueOrError1_).IsFailure():
                output = (d_7_valueOrError1_).PropagateFailure()
                return output
            d_8_id_: _dafny.Seq
            d_8_id_ = AllAlgorithmSuites.default__.ToHex((((d_4_sortedTests_)[d_6_i_]).algorithmSuiteId).value)
            d_9_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            out1_: Wrappers.Result
            out1_ = UUID.default__.GenerateUUID()
            d_9_valueOrError2_ = out1_
            if not(not((d_9_valueOrError2_).IsFailure())):
                raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(82,18): " + _dafny.string_of(d_9_valueOrError2_))
            d_10_uuid_: _dafny.Seq
            d_10_uuid_ = (d_9_valueOrError2_).Extract()
            d_11_valueOrError3_: Wrappers.Result = Wrappers.Result.default(JSON_Values.JSON.default())()
            d_11_valueOrError3_ = WriteEsdkJsonManifests.default__.EncryptTestVectorToJson((d_4_sortedTests_)[d_6_i_])
            if (d_11_valueOrError3_).IsFailure():
                output = (d_11_valueOrError3_).PropagateFailure()
                return output
            d_12_test_: JSON_Values.JSON
            d_12_test_ = (d_11_valueOrError3_).Extract()
            d_5_testsJSON_ = (d_5_testsJSON_) + (_dafny.Seq([(d_10_uuid_, d_12_test_)]))
        d_13_manifestJson_: JSON_Values.JSON
        d_13_manifestJson_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("type"), JSON_Values.JSON_String(_dafny.Seq("awses-encrypt"))), (_dafny.Seq("version"), JSON_Values.JSON_Number(JSON_Values.default__.Int(5)))]))
        d_14_plaintexts_: JSON_Values.JSON
        d_14_plaintexts_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("small"), JSON_Values.JSON_Number(JSON_Values.default__.Int(10240)))]))
        d_15_esdkEncryptManifests_: JSON_Values.JSON
        d_15_esdkEncryptManifests_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("manifest"), d_13_manifestJson_), (_dafny.Seq("keys"), JSON_Values.JSON_String(_dafny.Seq("file://keys.json"))), (_dafny.Seq("plaintexts"), d_14_plaintexts_), (_dafny.Seq("tests"), JSON_Values.JSON_Object(d_5_testsJSON_))]))
        d_16_valueOrError4_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_16_valueOrError4_ = JSON_API.default__.Serialize(d_15_esdkEncryptManifests_)
        if not(not((d_16_valueOrError4_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(102,36): " + _dafny.string_of(d_16_valueOrError4_))
        d_17_esdkEncryptManifestBytes_: _dafny.Seq
        d_17_esdkEncryptManifestBytes_ = (d_16_valueOrError4_).Extract()
        d_18_esdkEncryptManifestBv_: _dafny.Seq
        d_18_esdkEncryptManifestBv_ = JSONHelpers.default__.BytesBv(d_17_esdkEncryptManifestBytes_)
        d_19_valueOrError5_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        out2_: Wrappers.Result
        out2_ = FileIO.default__.WriteBytesToFile(((op).encryptManifestOutput) + (_dafny.Seq("encrypt-manifest.json")), d_18_esdkEncryptManifestBv_)
        d_19_valueOrError5_ = out2_
        if not(not((d_19_valueOrError5_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(105,13): " + _dafny.string_of(d_19_valueOrError5_))
        d_20___v2_: tuple
        d_20___v2_ = (d_19_valueOrError5_).Extract()
        output = Wrappers.Result_Success(())
        return output

    @staticmethod
    def WriteDecryptManifest(op, keys, tests):
        output: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        d_0_testsJSON_: _dafny.Seq
        d_0_testsJSON_ = _dafny.Seq([])
        hi0_ = len(tests)
        for d_1_i_ in range(0, hi0_):
            d_2_name_: _dafny.Seq
            d_2_name_ = ((tests)[d_1_i_]).id
            d_3_valueOrError0_: Wrappers.Result = Wrappers.Result.default(JSON_Values.JSON.default())()
            d_3_valueOrError0_ = WriteEsdkJsonManifests.default__.DecryptTestVectorToJson((tests)[d_1_i_])
            if (d_3_valueOrError0_).IsFailure():
                output = (d_3_valueOrError0_).PropagateFailure()
                return output
            d_4_test_: JSON_Values.JSON
            d_4_test_ = (d_3_valueOrError0_).Extract()
            d_0_testsJSON_ = (d_0_testsJSON_) + (_dafny.Seq([(d_2_name_, d_4_test_)]))
        d_5_manifestJson_: JSON_Values.JSON
        d_5_manifestJson_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("type"), JSON_Values.JSON_String(_dafny.Seq("awses-decrypt"))), (_dafny.Seq("version"), JSON_Values.JSON_Number(JSON_Values.default__.Int(3)))]))
        d_6_esdkDecryptManifest_: JSON_Values.JSON
        d_6_esdkDecryptManifest_ = JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("manifest"), d_5_manifestJson_), (_dafny.Seq("client"), JSON_Values.JSON_String(_dafny.Seq("aws-encryption-sdk-dafny"))), (_dafny.Seq("keys"), JSON_Values.JSON_String(_dafny.Seq("file://keys.json"))), (_dafny.Seq("tests"), JSON_Values.JSON_Object(d_0_testsJSON_))]))
        d_7_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        d_7_valueOrError1_ = JSON_API.default__.Serialize(d_6_esdkDecryptManifest_)
        if not(not((d_7_valueOrError1_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(146,36): " + _dafny.string_of(d_7_valueOrError1_))
        d_8_esdkDecryptManifestBytes_: _dafny.Seq
        d_8_esdkDecryptManifestBytes_ = (d_7_valueOrError1_).Extract()
        d_9_esdkDecryptManifestBv_: _dafny.Seq
        d_9_esdkDecryptManifestBv_ = JSONHelpers.default__.BytesBv(d_8_esdkDecryptManifestBytes_)
        d_10_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        out0_: Wrappers.Result
        out0_ = FileIO.default__.WriteBytesToFile(((op).decryptManifestOutput) + (_dafny.Seq("decrypt-manifest.json")), d_9_esdkDecryptManifestBv_)
        d_10_valueOrError2_ = out0_
        if not(not((d_10_valueOrError2_).IsFailure())):
            raise _dafny.HaltException("dafny/TestVectors/src/WriteVectors.dfy(149,13): " + _dafny.string_of(d_10_valueOrError2_))
        d_11___v3_: tuple
        d_11___v3_ = (d_10_valueOrError2_).Extract()
        output = Wrappers.Result_Success(())
        return output

    @staticmethod
    def getVersionTests(version):
        source0_ = version
        if True:
            if (source0_) == (5):
                return Wrappers.Result_Success((AllEsdkV4NoReqEc.default__.Tests) | (AllEsdkV4WithReqEc.default__.Tests))
        if True:
            return Wrappers.Result_Failure(_dafny.Seq("Only version 4 of generate manifest is supported\n"))

    @staticmethod
    def DescriptionLessThan(x, y):
        return default__.Below((x).description, (y).description)

    @staticmethod
    def Below(x, y):
        return not ((len(x)) != (0)) or ((((len(y)) != (0)) and (((x)[0]) <= ((y)[0]))) and (not (((x)[0]) == ((y)[0])) or (default__.Below(_dafny.Seq((x)[1::]), _dafny.Seq((y)[1::])))))

