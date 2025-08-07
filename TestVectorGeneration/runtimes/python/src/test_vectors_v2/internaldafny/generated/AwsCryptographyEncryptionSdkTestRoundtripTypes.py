import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import test_vectors_v2.internaldafny.generated.module_ as module_
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
import aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import smithy_dafny_standard_library.internaldafny.generated.Streams as Streams
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
import aws_encryption_sdk_dafny.internaldafny.generated.KeyDerivation as KeyDerivation
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptDecryptHelpers as EncryptDecryptHelpers
import aws_encryption_sdk_dafny.internaldafny.generated.AwsEncryptionSdkOperations as AwsEncryptionSdkOperations
import aws_encryption_sdk_dafny.internaldafny.generated.ESDK as ESDK
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
import smithy_dafny_standard_library.internaldafny.generated.Sorting as Sorting
import smithy_dafny_standard_library.internaldafny.generated.HexStrings as HexStrings
import smithy_dafny_standard_library.internaldafny.generated.GetOpt as GetOpt
import smithy_dafny_standard_library.internaldafny.generated.FloatCompare as FloatCompare
import smithy_dafny_standard_library.internaldafny.generated.ConcurrentCall as ConcurrentCall
import smithy_dafny_standard_library.internaldafny.generated.Base64Lemmas as Base64Lemmas
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Views_Core as JSON_Utils_Views_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Views_Writers as JSON_Utils_Views_Writers
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Lexers_Core as JSON_Utils_Lexers_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Lexers_Strings as JSON_Utils_Lexers_Strings
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Cursors as JSON_Utils_Cursors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Parsers as JSON_Utils_Parsers
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str_CharStrConversion as JSON_Utils_Str_CharStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str_CharStrEscaping as JSON_Utils_Str_CharStrEscaping
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str as JSON_Utils_Str
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Seq as JSON_Utils_Seq
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Vectors as JSON_Utils_Vectors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Errors as JSON_Errors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Values as JSON_Values
import smithy_dafny_standard_library.internaldafny.generated.JSON_Spec as JSON_Spec
import smithy_dafny_standard_library.internaldafny.generated.JSON_Grammar as JSON_Grammar
import smithy_dafny_standard_library.internaldafny.generated.JSON_Serializer_ByteStrConversion as JSON_Serializer_ByteStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Serializer as JSON_Serializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer_Uint16StrConversion as JSON_Deserializer_Uint16StrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer_ByteStrConversion as JSON_Deserializer_ByteStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer as JSON_Deserializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ConcreteSyntax_Spec as JSON_ConcreteSyntax_Spec
import smithy_dafny_standard_library.internaldafny.generated.JSON_ConcreteSyntax_SpecProperties as JSON_ConcreteSyntax_SpecProperties
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Serializer as JSON_ZeroCopy_Serializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Core as JSON_ZeroCopy_Deserializer_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Strings as JSON_ZeroCopy_Deserializer_Strings
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Numbers as JSON_ZeroCopy_Deserializer_Numbers
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_ObjectParams as JSON_ZeroCopy_Deserializer_ObjectParams
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Objects as JSON_ZeroCopy_Deserializer_Objects
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_ArrayParams as JSON_ZeroCopy_Deserializer_ArrayParams
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Arrays as JSON_ZeroCopy_Deserializer_Arrays
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Constants as JSON_ZeroCopy_Deserializer_Constants
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Values as JSON_ZeroCopy_Deserializer_Values
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_API as JSON_ZeroCopy_Deserializer_API
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer as JSON_ZeroCopy_Deserializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_API as JSON_ZeroCopy_API
import smithy_dafny_standard_library.internaldafny.generated.JSON_API as JSON_API
import test_vectors_v2.internaldafny.generated.TestVectorGeneration as TestVectorGeneration
import test_vectors_v2.internaldafny.generated.EmptyStringPartition as EmptyStringPartition
import test_vectors_v2.internaldafny.generated.NonemptyStringPartition as NonemptyStringPartition
import test_vectors_v2.internaldafny.generated.StringLengthPartitioning as StringLengthPartitioning
import test_vectors_v2.internaldafny.generated.AsciiPartition as AsciiPartition
import test_vectors_v2.internaldafny.generated.NonAsciiPartition as NonAsciiPartition
import test_vectors_v2.internaldafny.generated.BasicStringPartitioning as BasicStringPartitioning
import test_vectors_v2.internaldafny.generated.OptionStringNone as OptionStringNone
import test_vectors_v2.internaldafny.generated.OptionStringSome as OptionStringSome
import test_vectors_v2.internaldafny.generated.OptionStringPartitioning as OptionStringPartitioning
import test_vectors_v2.internaldafny.generated.Aes128WrappingKeyPartition as Aes128WrappingKeyPartition
import test_vectors_v2.internaldafny.generated.Aes192WrappingKeyPartition as Aes192WrappingKeyPartition
import test_vectors_v2.internaldafny.generated.Aes256WrappingKeyPartition as Aes256WrappingKeyPartition
import test_vectors_v2.internaldafny.generated.ValidAesWrappingKeyPartition as ValidAesWrappingKeyPartition
import test_vectors_v2.internaldafny.generated.InvalidAesWrappingKeyPartition as InvalidAesWrappingKeyPartition
import test_vectors_v2.internaldafny.generated.AesWrappingKeyPartitioningScheme as AesWrappingKeyPartitioningScheme
import test_vectors_v2.internaldafny.generated.WrappingAlgValues as WrappingAlgValues
import test_vectors_v2.internaldafny.generated.InvalidRawAesKeyNamespacePartition as InvalidRawAesKeyNamespacePartition
import test_vectors_v2.internaldafny.generated.ValidRawAesKeyNamespacePartition as ValidRawAesKeyNamespacePartition
import test_vectors_v2.internaldafny.generated.KeyNamespacePartitioningScheme as KeyNamespacePartitioningScheme
import test_vectors_v2.internaldafny.generated.KeyNamePartitioningScheme as KeyNamePartitioningScheme
import test_vectors_v2.internaldafny.generated.InvalidAes128PruningConfiguration as InvalidAes128PruningConfiguration
import test_vectors_v2.internaldafny.generated.InvalidAes192PruningConfiguration as InvalidAes192PruningConfiguration
import test_vectors_v2.internaldafny.generated.InvalidAes256PruningConfiguration as InvalidAes256PruningConfiguration
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringInput as CreateRawAesKeyringInput
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringSmokeTest as CreateRawAesKeyringSmokeTest
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringTestService as CreateRawAesKeyringTestService
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringInputTests as CreateRawAesKeyringInputTests
import test_vectors_v2.internaldafny.generated.EmptyPlaintextPartitioning as EmptyPlaintextPartitioning
import test_vectors_v2.internaldafny.generated.NonemptyPlaintextPartitioning as NonemptyPlaintextPartitioning
import test_vectors_v2.internaldafny.generated.PlaintextPartitioning as PlaintextPartitioning
import test_vectors_v2.internaldafny.generated.EmptyEncryptionContextPartition as EmptyEncryptionContextPartition
import test_vectors_v2.internaldafny.generated.NomemptyEncryptionContextPartition as NomemptyEncryptionContextPartition
import test_vectors_v2.internaldafny.generated.EncryptionContextPartitioning as EncryptionContextPartitioning
import test_vectors_v2.internaldafny.generated.ValidKeyIdentifierPruningConfiguration as ValidKeyIdentifierPruningConfiguration
import test_vectors_v2.internaldafny.generated.InvalidKeyIdentifierPruningConfiguration as InvalidKeyIdentifierPruningConfiguration

# Module: AwsCryptographyEncryptionSdkTestRoundtripTypes

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def IsDummySubsetType(x):
        return (0) < (x)


class DafnyCallEvent:
    @classmethod
    def default(cls, default_I, default_O):
        return lambda: DafnyCallEvent_DafnyCallEvent(default_I(), default_O())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_DafnyCallEvent(self) -> bool:
        return isinstance(self, DafnyCallEvent_DafnyCallEvent)

class DafnyCallEvent_DafnyCallEvent(DafnyCallEvent, NamedTuple('DafnyCallEvent', [('input', Any), ('output', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.DafnyCallEvent.DafnyCallEvent({_dafny.string_of(self.input)}, {_dafny.string_of(self.output)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DafnyCallEvent_DafnyCallEvent) and self.input == __o.input and self.output == __o.output
    def __hash__(self) -> int:
        return super().__hash__()


class ESDKTestRoundTripConfig:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [ESDKTestRoundTripConfig_ESDKTestRoundTripConfig()]
    @classmethod
    def default(cls, ):
        return lambda: ESDKTestRoundTripConfig_ESDKTestRoundTripConfig()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_ESDKTestRoundTripConfig(self) -> bool:
        return isinstance(self, ESDKTestRoundTripConfig_ESDKTestRoundTripConfig)

class ESDKTestRoundTripConfig_ESDKTestRoundTripConfig(ESDKTestRoundTripConfig, NamedTuple('ESDKTestRoundTripConfig', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ESDKTestRoundTripConfig.ESDKTestRoundTripConfig'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ESDKTestRoundTripConfig_ESDKTestRoundTripConfig)
    def __hash__(self) -> int:
        return super().__hash__()


class IESDKTestRoundTripServiceClientCallHistory:
    def  __init__(self):
        pass

    def __dafnystr__(self) -> str:
        return "AwsCryptographyEncryptionSdkTestRoundtripTypes.IESDKTestRoundTripServiceClientCallHistory"

class IESDKTestRoundTripServiceClient:
    pass
    def TestCrossLanguageRoundTrip(self, input):
        pass


class ImplementationLanguage:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [ImplementationLanguage_JAVA(), ImplementationLanguage_PYTHON(), ImplementationLanguage_DOTNET(), ImplementationLanguage_GO(), ImplementationLanguage_JAVASCRIPT(), ImplementationLanguage_RUST()]
    @classmethod
    def default(cls, ):
        return lambda: ImplementationLanguage_JAVA()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_JAVA(self) -> bool:
        return isinstance(self, ImplementationLanguage_JAVA)
    @property
    def is_PYTHON(self) -> bool:
        return isinstance(self, ImplementationLanguage_PYTHON)
    @property
    def is_DOTNET(self) -> bool:
        return isinstance(self, ImplementationLanguage_DOTNET)
    @property
    def is_GO(self) -> bool:
        return isinstance(self, ImplementationLanguage_GO)
    @property
    def is_JAVASCRIPT(self) -> bool:
        return isinstance(self, ImplementationLanguage_JAVASCRIPT)
    @property
    def is_RUST(self) -> bool:
        return isinstance(self, ImplementationLanguage_RUST)

class ImplementationLanguage_JAVA(ImplementationLanguage, NamedTuple('JAVA', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.JAVA'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ImplementationLanguage_JAVA)
    def __hash__(self) -> int:
        return super().__hash__()

class ImplementationLanguage_PYTHON(ImplementationLanguage, NamedTuple('PYTHON', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.PYTHON'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ImplementationLanguage_PYTHON)
    def __hash__(self) -> int:
        return super().__hash__()

class ImplementationLanguage_DOTNET(ImplementationLanguage, NamedTuple('DOTNET', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.DOTNET'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ImplementationLanguage_DOTNET)
    def __hash__(self) -> int:
        return super().__hash__()

class ImplementationLanguage_GO(ImplementationLanguage, NamedTuple('GO', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.GO'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ImplementationLanguage_GO)
    def __hash__(self) -> int:
        return super().__hash__()

class ImplementationLanguage_JAVASCRIPT(ImplementationLanguage, NamedTuple('JAVASCRIPT', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.JAVASCRIPT'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ImplementationLanguage_JAVASCRIPT)
    def __hash__(self) -> int:
        return super().__hash__()

class ImplementationLanguage_RUST(ImplementationLanguage, NamedTuple('RUST', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.ImplementationLanguage.RUST'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ImplementationLanguage_RUST)
    def __hash__(self) -> int:
        return super().__hash__()


class RoundTripDecryptInput:
    @classmethod
    def default(cls, ):
        return lambda: RoundTripDecryptInput_RoundTripDecryptInput(Wrappers.Option.default()(), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_RoundTripDecryptInput(self) -> bool:
        return isinstance(self, RoundTripDecryptInput_RoundTripDecryptInput)

class RoundTripDecryptInput_RoundTripDecryptInput(RoundTripDecryptInput, NamedTuple('RoundTripDecryptInput', [('encryptionContext', Any), ('keyring', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripDecryptInput.RoundTripDecryptInput({_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.keyring)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, RoundTripDecryptInput_RoundTripDecryptInput) and self.encryptionContext == __o.encryptionContext and self.keyring == __o.keyring
    def __hash__(self) -> int:
        return super().__hash__()


class RoundTripEncryptInput:
    @classmethod
    def default(cls, ):
        return lambda: RoundTripEncryptInput_RoundTripEncryptInput(_dafny.Seq({}), Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_RoundTripEncryptInput(self) -> bool:
        return isinstance(self, RoundTripEncryptInput_RoundTripEncryptInput)

class RoundTripEncryptInput_RoundTripEncryptInput(RoundTripEncryptInput, NamedTuple('RoundTripEncryptInput', [('plaintext', Any), ('encryptionContext', Any), ('keyring', Any), ('algorithmSuiteId', Any), ('frameLength', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.RoundTripEncryptInput.RoundTripEncryptInput({_dafny.string_of(self.plaintext)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.keyring)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.frameLength)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, RoundTripEncryptInput_RoundTripEncryptInput) and self.plaintext == __o.plaintext and self.encryptionContext == __o.encryptionContext and self.keyring == __o.keyring and self.algorithmSuiteId == __o.algorithmSuiteId and self.frameLength == __o.frameLength
    def __hash__(self) -> int:
        return super().__hash__()


class SupportedKeyringCreateInputs:
    @classmethod
    def default(cls, ):
        return lambda: SupportedKeyringCreateInputs_RawAes(AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_RawAes(self) -> bool:
        return isinstance(self, SupportedKeyringCreateInputs_RawAes)

class SupportedKeyringCreateInputs_RawAes(SupportedKeyringCreateInputs, NamedTuple('RawAes', [('RawAes', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.SupportedKeyringCreateInputs.RawAes({_dafny.string_of(self.RawAes)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SupportedKeyringCreateInputs_RawAes) and self.RawAes == __o.RawAes
    def __hash__(self) -> int:
        return super().__hash__()


class TestCrossLanguageRoundTripInput:
    @classmethod
    def default(cls, ):
        return lambda: TestCrossLanguageRoundTripInput_TestCrossLanguageRoundTripInput(ImplementationLanguage.default()(), ImplementationLanguage.default()(), RoundTripEncryptInput.default()(), RoundTripDecryptInput.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_TestCrossLanguageRoundTripInput(self) -> bool:
        return isinstance(self, TestCrossLanguageRoundTripInput_TestCrossLanguageRoundTripInput)

class TestCrossLanguageRoundTripInput_TestCrossLanguageRoundTripInput(TestCrossLanguageRoundTripInput, NamedTuple('TestCrossLanguageRoundTripInput', [('encryptLanguage', Any), ('decryptLanguage', Any), ('encryptInput', Any), ('decryptInput', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripInput.TestCrossLanguageRoundTripInput({_dafny.string_of(self.encryptLanguage)}, {_dafny.string_of(self.decryptLanguage)}, {_dafny.string_of(self.encryptInput)}, {_dafny.string_of(self.decryptInput)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, TestCrossLanguageRoundTripInput_TestCrossLanguageRoundTripInput) and self.encryptLanguage == __o.encryptLanguage and self.decryptLanguage == __o.decryptLanguage and self.encryptInput == __o.encryptInput and self.decryptInput == __o.decryptInput
    def __hash__(self) -> int:
        return super().__hash__()


class TestCrossLanguageRoundTripOutput:
    @classmethod
    def default(cls, ):
        return lambda: TestCrossLanguageRoundTripOutput_TestCrossLanguageRoundTripOutput(_dafny.Seq(""))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_TestCrossLanguageRoundTripOutput(self) -> bool:
        return isinstance(self, TestCrossLanguageRoundTripOutput_TestCrossLanguageRoundTripOutput)

class TestCrossLanguageRoundTripOutput_TestCrossLanguageRoundTripOutput(TestCrossLanguageRoundTripOutput, NamedTuple('TestCrossLanguageRoundTripOutput', [('status', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.TestCrossLanguageRoundTripOutput.TestCrossLanguageRoundTripOutput({_dafny.string_of(self.status)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, TestCrossLanguageRoundTripOutput_TestCrossLanguageRoundTripOutput) and self.status == __o.status
    def __hash__(self) -> int:
        return super().__hash__()


class Error:
    @classmethod
    def default(cls, ):
        return lambda: Error_AwsCryptographyEncryptionSdk(AwsCryptographyEncryptionSdkTypes.Error.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_AwsCryptographyEncryptionSdk(self) -> bool:
        return isinstance(self, Error_AwsCryptographyEncryptionSdk)
    @property
    def is_AwsCryptographyMaterialProviders(self) -> bool:
        return isinstance(self, Error_AwsCryptographyMaterialProviders)
    @property
    def is_CollectionOfErrors(self) -> bool:
        return isinstance(self, Error_CollectionOfErrors)
    @property
    def is_Opaque(self) -> bool:
        return isinstance(self, Error_Opaque)
    @property
    def is_OpaqueWithText(self) -> bool:
        return isinstance(self, Error_OpaqueWithText)

class Error_AwsCryptographyEncryptionSdk(Error, NamedTuple('AwsCryptographyEncryptionSdk', [('AwsCryptographyEncryptionSdk', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.Error.AwsCryptographyEncryptionSdk({_dafny.string_of(self.AwsCryptographyEncryptionSdk)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_AwsCryptographyEncryptionSdk) and self.AwsCryptographyEncryptionSdk == __o.AwsCryptographyEncryptionSdk
    def __hash__(self) -> int:
        return super().__hash__()

class Error_AwsCryptographyMaterialProviders(Error, NamedTuple('AwsCryptographyMaterialProviders', [('AwsCryptographyMaterialProviders', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.Error.AwsCryptographyMaterialProviders({_dafny.string_of(self.AwsCryptographyMaterialProviders)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_AwsCryptographyMaterialProviders) and self.AwsCryptographyMaterialProviders == __o.AwsCryptographyMaterialProviders
    def __hash__(self) -> int:
        return super().__hash__()

class Error_CollectionOfErrors(Error, NamedTuple('CollectionOfErrors', [('list', Any), ('message', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.Error.CollectionOfErrors({_dafny.string_of(self.list)}, {_dafny.string_of(self.message)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_CollectionOfErrors) and self.list == __o.list and self.message == __o.message
    def __hash__(self) -> int:
        return super().__hash__()

class Error_Opaque(Error, NamedTuple('Opaque', [('obj', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.Error.Opaque({_dafny.string_of(self.obj)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_Opaque) and self.obj == __o.obj
    def __hash__(self) -> int:
        return super().__hash__()

class Error_OpaqueWithText(Error, NamedTuple('OpaqueWithText', [('obj', Any), ('objMessage', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTestRoundtripTypes.Error.OpaqueWithText({_dafny.string_of(self.obj)}, {_dafny.string_of(self.objMessage)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_OpaqueWithText) and self.obj == __o.obj and self.objMessage == __o.objMessage
    def __hash__(self) -> int:
        return super().__hash__()


class OpaqueError:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Error.default()()
    def _Is(source__):
        d_0_e_: Error = source__
        return ((d_0_e_).is_Opaque) or ((d_0_e_).is_OpaqueWithText)

class DummySubsetType:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return 1
    def _Is(source__):
        d_1_x_: int = source__
        return default__.IsDummySubsetType(d_1_x_)
