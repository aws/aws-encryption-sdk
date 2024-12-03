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
import aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import aws_encryption_sdk.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptionContext as EncryptionContext

# Module: HeaderTypes

class default__:
    def  __init__(self):
        pass

    @_dafny.classproperty
    def MESSAGE__ID__LEN__V1(instance):
        return 16
    @_dafny.classproperty
    def MESSAGE__ID__LEN__V2(instance):
        return 32

class MessageFormatVersion:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [MessageFormatVersion_V1(), MessageFormatVersion_V2()]
    @classmethod
    def default(cls, ):
        return lambda: MessageFormatVersion_V1()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_V1(self) -> bool:
        return isinstance(self, MessageFormatVersion_V1)
    @property
    def is_V2(self) -> bool:
        return isinstance(self, MessageFormatVersion_V2)
    def Serialize(self):
        source0_ = self
        if True:
            if source0_.is_V1:
                return _dafny.Seq([1])
        if True:
            return _dafny.Seq([2])

    @staticmethod
    def Get(x):
        d_0_valueOrError0_ = Wrappers.default__.Need(((x) == (_dafny.Seq([1]))) or ((x) == (_dafny.Seq([2]))), _dafny.Seq("Unsupported Version value."))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            def lambda0_():
                source0_ = (x)[0]
                if True:
                    if (source0_) == (1):
                        return MessageFormatVersion_V1()
                if True:
                    return MessageFormatVersion_V2()

            return Wrappers.Result_Success(lambda0_())


class MessageFormatVersion_V1(MessageFormatVersion, NamedTuple('V1', [])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.MessageFormatVersion.V1'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, MessageFormatVersion_V1)
    def __hash__(self) -> int:
        return super().__hash__()

class MessageFormatVersion_V2(MessageFormatVersion, NamedTuple('V2', [])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.MessageFormatVersion.V2'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, MessageFormatVersion_V2)
    def __hash__(self) -> int:
        return super().__hash__()


class ESDKAlgorithmSuite:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo.default()()

class HeaderBody:
    @classmethod
    def default(cls, ):
        return lambda: HeaderBody_V1HeaderBody(MessageType.default()(), AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo.default()(), _dafny.Seq({}), _dafny.Seq({}), _dafny.Seq({}), ContentType.default()(), int(0), int(0))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_V1HeaderBody(self) -> bool:
        return isinstance(self, HeaderBody_V1HeaderBody)
    @property
    def is_V2HeaderBody(self) -> bool:
        return isinstance(self, HeaderBody_V2HeaderBody)

class HeaderBody_V1HeaderBody(HeaderBody, NamedTuple('V1HeaderBody', [('messageType', Any), ('algorithmSuite', Any), ('messageId', Any), ('encryptionContext', Any), ('encryptedDataKeys', Any), ('contentType', Any), ('headerIvLength', Any), ('frameLength', Any)])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.HeaderBody.V1HeaderBody({_dafny.string_of(self.messageType)}, {_dafny.string_of(self.algorithmSuite)}, {_dafny.string_of(self.messageId)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.encryptedDataKeys)}, {_dafny.string_of(self.contentType)}, {_dafny.string_of(self.headerIvLength)}, {_dafny.string_of(self.frameLength)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, HeaderBody_V1HeaderBody) and self.messageType == __o.messageType and self.algorithmSuite == __o.algorithmSuite and self.messageId == __o.messageId and self.encryptionContext == __o.encryptionContext and self.encryptedDataKeys == __o.encryptedDataKeys and self.contentType == __o.contentType and self.headerIvLength == __o.headerIvLength and self.frameLength == __o.frameLength
    def __hash__(self) -> int:
        return super().__hash__()

class HeaderBody_V2HeaderBody(HeaderBody, NamedTuple('V2HeaderBody', [('algorithmSuite', Any), ('messageId', Any), ('encryptionContext', Any), ('encryptedDataKeys', Any), ('contentType', Any), ('frameLength', Any), ('suiteData', Any)])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.HeaderBody.V2HeaderBody({_dafny.string_of(self.algorithmSuite)}, {_dafny.string_of(self.messageId)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.encryptedDataKeys)}, {_dafny.string_of(self.contentType)}, {_dafny.string_of(self.frameLength)}, {_dafny.string_of(self.suiteData)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, HeaderBody_V2HeaderBody) and self.algorithmSuite == __o.algorithmSuite and self.messageId == __o.messageId and self.encryptionContext == __o.encryptionContext and self.encryptedDataKeys == __o.encryptedDataKeys and self.contentType == __o.contentType and self.frameLength == __o.frameLength and self.suiteData == __o.suiteData
    def __hash__(self) -> int:
        return super().__hash__()


class HeaderAuth:
    @classmethod
    def default(cls, ):
        return lambda: HeaderAuth_AESMac(_dafny.Seq({}), _dafny.Seq({}))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_AESMac(self) -> bool:
        return isinstance(self, HeaderAuth_AESMac)

class HeaderAuth_AESMac(HeaderAuth, NamedTuple('AESMac', [('headerIv', Any), ('headerAuthTag', Any)])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.HeaderAuth.AESMac({_dafny.string_of(self.headerIv)}, {_dafny.string_of(self.headerAuthTag)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, HeaderAuth_AESMac) and self.headerIv == __o.headerIv and self.headerAuthTag == __o.headerAuthTag
    def __hash__(self) -> int:
        return super().__hash__()


class MessageType:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [MessageType_TYPE__CUSTOMER__AED()]
    @classmethod
    def default(cls, ):
        return lambda: MessageType_TYPE__CUSTOMER__AED()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_TYPE__CUSTOMER__AED(self) -> bool:
        return isinstance(self, MessageType_TYPE__CUSTOMER__AED)
    def Serialize(self):
        source0_ = self
        if True:
            return 128

    @staticmethod
    def Get(x):
        d_0_valueOrError0_ = Wrappers.default__.Need((x) == (128), _dafny.Seq("Unsupported ContentType value."))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            def lambda0_():
                source0_ = x
                if True:
                    return MessageType_TYPE__CUSTOMER__AED()

            return Wrappers.Result_Success(lambda0_())


class MessageType_TYPE__CUSTOMER__AED(MessageType, NamedTuple('TYPE__CUSTOMER__AED', [])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.MessageType.TYPE_CUSTOMER_AED'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, MessageType_TYPE__CUSTOMER__AED)
    def __hash__(self) -> int:
        return super().__hash__()


class ContentType:
    @_dafny.classproperty
    def AllSingletonConstructors(cls):
        return [ContentType_NonFramed(), ContentType_Framed()]
    @classmethod
    def default(cls, ):
        return lambda: ContentType_NonFramed()
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_NonFramed(self) -> bool:
        return isinstance(self, ContentType_NonFramed)
    @property
    def is_Framed(self) -> bool:
        return isinstance(self, ContentType_Framed)
    def Serialize(self):
        source0_ = self
        if True:
            if source0_.is_NonFramed:
                return 1
        if True:
            return 2

    @staticmethod
    def Get(x):
        d_0_valueOrError0_ = Wrappers.default__.Need(((x) == (1)) or ((x) == (2)), _dafny.Seq("Unsupported ContentType value."))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            def lambda0_():
                source0_ = x
                if True:
                    if (source0_) == (1):
                        return ContentType_NonFramed()
                if True:
                    return ContentType_Framed()

            return Wrappers.Result_Success(lambda0_())


class ContentType_NonFramed(ContentType, NamedTuple('NonFramed', [])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.ContentType.NonFramed'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ContentType_NonFramed)
    def __hash__(self) -> int:
        return super().__hash__()

class ContentType_Framed(ContentType, NamedTuple('Framed', [])):
    def __dafnystr__(self) -> str:
        return f'HeaderTypes.ContentType.Framed'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ContentType_Framed)
    def __hash__(self) -> int:
        return super().__hash__()


class MessageId:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return _dafny.Seq({})
    def _Is(source__):
        d_1_x_: _dafny.Seq = source__
        return ((len(d_1_x_)) == (default__.MESSAGE__ID__LEN__V1)) or ((len(d_1_x_)) == (default__.MESSAGE__ID__LEN__V2))
