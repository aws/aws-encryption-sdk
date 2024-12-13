import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import aws_encryption_sdk.internaldafny.generated.module_ as module_
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

# Module: AwsCryptographyEncryptionSdkTypes

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def IsValid__CountingNumbers(x):
        return (1) <= (x)

    @staticmethod
    def IsValid__FrameLength(x):
        return ((1) <= (x)) and ((x) <= (4294967296))

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
        return f'AwsCryptographyEncryptionSdkTypes.DafnyCallEvent.DafnyCallEvent({_dafny.string_of(self.input)}, {_dafny.string_of(self.output)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DafnyCallEvent_DafnyCallEvent) and self.input == __o.input and self.output == __o.output
    def __hash__(self) -> int:
        return super().__hash__()


class IAwsEncryptionSdkClientCallHistory:
    def  __init__(self):
        pass

    def __dafnystr__(self) -> str:
        return "AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClientCallHistory"

class IAwsEncryptionSdkClient:
    pass
    def Encrypt(self, input):
        pass

    def Decrypt(self, input):
        pass


class AwsEncryptionSdkConfig:
    @classmethod
    def default(cls, ):
        return lambda: AwsEncryptionSdkConfig_AwsEncryptionSdkConfig(Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_AwsEncryptionSdkConfig(self) -> bool:
        return isinstance(self, AwsEncryptionSdkConfig_AwsEncryptionSdkConfig)

class AwsEncryptionSdkConfig_AwsEncryptionSdkConfig(AwsEncryptionSdkConfig, NamedTuple('AwsEncryptionSdkConfig', [('commitmentPolicy', Any), ('maxEncryptedDataKeys', Any), ('netV4__0__0__RetryPolicy', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig.AwsEncryptionSdkConfig({_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.maxEncryptedDataKeys)}, {_dafny.string_of(self.netV4__0__0__RetryPolicy)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, AwsEncryptionSdkConfig_AwsEncryptionSdkConfig) and self.commitmentPolicy == __o.commitmentPolicy and self.maxEncryptedDataKeys == __o.maxEncryptedDataKeys and self.netV4__0__0__RetryPolicy == __o.netV4__0__0__RetryPolicy
    def __hash__(self) -> int:
        return super().__hash__()


class CountingNumbers:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return int(0)
    def _Is(source__):
        d_0_x_: int = source__
        if True:
            return default__.IsValid__CountingNumbers(d_0_x_)
        return False

class DecryptInput:
    @classmethod
    def default(cls, ):
        return lambda: DecryptInput_DecryptInput(_dafny.Seq({}), Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_DecryptInput(self) -> bool:
        return isinstance(self, DecryptInput_DecryptInput)

class DecryptInput_DecryptInput(DecryptInput, NamedTuple('DecryptInput', [('ciphertext', Any), ('materialsManager', Any), ('keyring', Any), ('encryptionContext', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.DecryptInput.DecryptInput({_dafny.string_of(self.ciphertext)}, {_dafny.string_of(self.materialsManager)}, {_dafny.string_of(self.keyring)}, {_dafny.string_of(self.encryptionContext)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DecryptInput_DecryptInput) and self.ciphertext == __o.ciphertext and self.materialsManager == __o.materialsManager and self.keyring == __o.keyring and self.encryptionContext == __o.encryptionContext
    def __hash__(self) -> int:
        return super().__hash__()


class DecryptOutput:
    @classmethod
    def default(cls, ):
        return lambda: DecryptOutput_DecryptOutput(_dafny.Seq({}), _dafny.Map({}), AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_DecryptOutput(self) -> bool:
        return isinstance(self, DecryptOutput_DecryptOutput)

class DecryptOutput_DecryptOutput(DecryptOutput, NamedTuple('DecryptOutput', [('plaintext', Any), ('encryptionContext', Any), ('algorithmSuiteId', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.DecryptOutput.DecryptOutput({_dafny.string_of(self.plaintext)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.algorithmSuiteId)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, DecryptOutput_DecryptOutput) and self.plaintext == __o.plaintext and self.encryptionContext == __o.encryptionContext and self.algorithmSuiteId == __o.algorithmSuiteId
    def __hash__(self) -> int:
        return super().__hash__()


class EncryptInput:
    @classmethod
    def default(cls, ):
        return lambda: EncryptInput_EncryptInput(_dafny.Seq({}), Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()(), Wrappers.Option.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_EncryptInput(self) -> bool:
        return isinstance(self, EncryptInput_EncryptInput)

class EncryptInput_EncryptInput(EncryptInput, NamedTuple('EncryptInput', [('plaintext', Any), ('encryptionContext', Any), ('materialsManager', Any), ('keyring', Any), ('algorithmSuiteId', Any), ('frameLength', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.EncryptInput.EncryptInput({_dafny.string_of(self.plaintext)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.materialsManager)}, {_dafny.string_of(self.keyring)}, {_dafny.string_of(self.algorithmSuiteId)}, {_dafny.string_of(self.frameLength)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EncryptInput_EncryptInput) and self.plaintext == __o.plaintext and self.encryptionContext == __o.encryptionContext and self.materialsManager == __o.materialsManager and self.keyring == __o.keyring and self.algorithmSuiteId == __o.algorithmSuiteId and self.frameLength == __o.frameLength
    def __hash__(self) -> int:
        return super().__hash__()


class EncryptOutput:
    @classmethod
    def default(cls, ):
        return lambda: EncryptOutput_EncryptOutput(_dafny.Seq({}), _dafny.Map({}), AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_EncryptOutput(self) -> bool:
        return isinstance(self, EncryptOutput_EncryptOutput)

class EncryptOutput_EncryptOutput(EncryptOutput, NamedTuple('EncryptOutput', [('ciphertext', Any), ('encryptionContext', Any), ('algorithmSuiteId', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.EncryptOutput.EncryptOutput({_dafny.string_of(self.ciphertext)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.algorithmSuiteId)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, EncryptOutput_EncryptOutput) and self.ciphertext == __o.ciphertext and self.encryptionContext == __o.encryptionContext and self.algorithmSuiteId == __o.algorithmSuiteId
    def __hash__(self) -> int:
        return super().__hash__()


class FrameLength:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return int(0)
    def _Is(source__):
        d_1_x_: int = source__
        if True:
            return default__.IsValid__FrameLength(d_1_x_)
        return False

class NetV4__0__0__RetryPolicy:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [NetV4__0__0__RetryPolicy_FORBID__RETRY(), NetV4__0__0__RetryPolicy_ALLOW__RETRY()]
    @classmethod
    def default(cls, ):
        return lambda: NetV4__0__0__RetryPolicy_FORBID__RETRY()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_FORBID__RETRY(self) -> bool:
        return isinstance(self, NetV4__0__0__RetryPolicy_FORBID__RETRY)
    @property
    def is_ALLOW__RETRY(self) -> bool:
        return isinstance(self, NetV4__0__0__RetryPolicy_ALLOW__RETRY)

class NetV4__0__0__RetryPolicy_FORBID__RETRY(NetV4__0__0__RetryPolicy, NamedTuple('FORBID__RETRY', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.NetV4_0_0_RetryPolicy.FORBID_RETRY'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, NetV4__0__0__RetryPolicy_FORBID__RETRY)
    def __hash__(self) -> int:
        return super().__hash__()

class NetV4__0__0__RetryPolicy_ALLOW__RETRY(NetV4__0__0__RetryPolicy, NamedTuple('ALLOW__RETRY', [])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.NetV4_0_0_RetryPolicy.ALLOW_RETRY'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, NetV4__0__0__RetryPolicy_ALLOW__RETRY)
    def __hash__(self) -> int:
        return super().__hash__()


class Error:
    @classmethod
    def default(cls, ):
        return lambda: Error_AwsEncryptionSdkException(_dafny.Seq(""))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_AwsEncryptionSdkException(self) -> bool:
        return isinstance(self, Error_AwsEncryptionSdkException)
    @property
    def is_AwsCryptographyMaterialProviders(self) -> bool:
        return isinstance(self, Error_AwsCryptographyMaterialProviders)
    @property
    def is_AwsCryptographyPrimitives(self) -> bool:
        return isinstance(self, Error_AwsCryptographyPrimitives)
    @property
    def is_CollectionOfErrors(self) -> bool:
        return isinstance(self, Error_CollectionOfErrors)
    @property
    def is_Opaque(self) -> bool:
        return isinstance(self, Error_Opaque)
    @property
    def is_OpaqueWithText(self) -> bool:
        return isinstance(self, Error_OpaqueWithText)

class Error_AwsEncryptionSdkException(Error, NamedTuple('AwsEncryptionSdkException', [('message', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.Error.AwsEncryptionSdkException({_dafny.string_of(self.message)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_AwsEncryptionSdkException) and self.message == __o.message
    def __hash__(self) -> int:
        return super().__hash__()

class Error_AwsCryptographyMaterialProviders(Error, NamedTuple('AwsCryptographyMaterialProviders', [('AwsCryptographyMaterialProviders', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.Error.AwsCryptographyMaterialProviders({_dafny.string_of(self.AwsCryptographyMaterialProviders)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_AwsCryptographyMaterialProviders) and self.AwsCryptographyMaterialProviders == __o.AwsCryptographyMaterialProviders
    def __hash__(self) -> int:
        return super().__hash__()

class Error_AwsCryptographyPrimitives(Error, NamedTuple('AwsCryptographyPrimitives', [('AwsCryptographyPrimitives', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.Error.AwsCryptographyPrimitives({_dafny.string_of(self.AwsCryptographyPrimitives)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_AwsCryptographyPrimitives) and self.AwsCryptographyPrimitives == __o.AwsCryptographyPrimitives
    def __hash__(self) -> int:
        return super().__hash__()

class Error_CollectionOfErrors(Error, NamedTuple('CollectionOfErrors', [('list', Any), ('message', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.Error.CollectionOfErrors({_dafny.string_of(self.list)}, {_dafny.string_of(self.message)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_CollectionOfErrors) and self.list == __o.list and self.message == __o.message
    def __hash__(self) -> int:
        return super().__hash__()

class Error_Opaque(Error, NamedTuple('Opaque', [('obj', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.Error.Opaque({_dafny.string_of(self.obj)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Error_Opaque) and self.obj == __o.obj
    def __hash__(self) -> int:
        return super().__hash__()

class Error_OpaqueWithText(Error, NamedTuple('OpaqueWithText', [('obj', Any), ('objMessage', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsCryptographyEncryptionSdkTypes.Error.OpaqueWithText({_dafny.string_of(self.obj)}, {_dafny.string_of(self.objMessage)})'
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
        d_2_e_: Error = source__
        return ((d_2_e_).is_Opaque) or ((d_2_e_).is_OpaqueWithText)

class DummySubsetType:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return 1
    def _Is(source__):
        d_3_x_: int = source__
        return default__.IsDummySubsetType(d_3_x_)
