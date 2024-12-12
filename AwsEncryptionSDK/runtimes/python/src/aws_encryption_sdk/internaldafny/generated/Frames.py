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
import aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import aws_encryption_sdk.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptionContext as EncryptionContext
import aws_encryption_sdk.internaldafny.generated.HeaderTypes as HeaderTypes
import aws_encryption_sdk.internaldafny.generated.SharedHeaderFunctions as SharedHeaderFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptedDataKeys as EncryptedDataKeys
import aws_encryption_sdk.internaldafny.generated.V1HeaderBody as V1HeaderBody
import aws_encryption_sdk.internaldafny.generated.V2HeaderBody as V2HeaderBody
import aws_encryption_sdk.internaldafny.generated.HeaderAuth as HeaderAuth
import aws_encryption_sdk.internaldafny.generated.Header as Header

# Module: Frames

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def WriteRegularFrame(regularFrame):
        return (((SerializeFunctions.default__.WriteUint32((regularFrame).seqNum)) + (SerializeFunctions.default__.Write((regularFrame).iv))) + (SerializeFunctions.default__.Write((regularFrame).encContent))) + (SerializeFunctions.default__.Write((regularFrame).authTag))

    @staticmethod
    def ReadRegularFrame(buffer, header):
        d_0_valueOrError0_ = SerializeFunctions.default__.ReadUInt32(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_sequenceNumber_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = Wrappers.default__.Need(((d_1_sequenceNumber_).data) < (default__.ENDFRAME__SEQUENCE__NUMBER), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Regular frame sequence number can not equal or exceed the final frame.")))
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_valueOrError2_ = SerializeFunctions.default__.Read((d_1_sequenceNumber_).tail, SerializableTypes.default__.GetIvLength((header).suite))
                if (d_3_valueOrError2_).IsFailure():
                    return (d_3_valueOrError2_).PropagateFailure()
                elif True:
                    d_4_iv_ = (d_3_valueOrError2_).Extract()
                    d_5_valueOrError3_ = SerializeFunctions.default__.Read((d_4_iv_).tail, ((header).body).frameLength)
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_encContent_ = (d_5_valueOrError3_).Extract()
                        d_7_valueOrError4_ = SerializeFunctions.default__.Read((d_6_encContent_).tail, SerializableTypes.default__.GetTagLength((header).suite))
                        if (d_7_valueOrError4_).IsFailure():
                            return (d_7_valueOrError4_).PropagateFailure()
                        elif True:
                            d_8_authTag_ = (d_7_valueOrError4_).Extract()
                            d_9_regularFrame_ = Frame_RegularFrame(header, (d_1_sequenceNumber_).data, (d_4_iv_).data, (d_6_encContent_).data, (d_8_authTag_).data)
                            return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_9_regularFrame_, (d_8_authTag_).tail))

    @staticmethod
    def WriteFinalFrame(finalFrame):
        return ((((SerializeFunctions.default__.WriteUint32(default__.ENDFRAME__SEQUENCE__NUMBER)) + (SerializeFunctions.default__.WriteUint32((finalFrame).seqNum))) + (SerializeFunctions.default__.Write((finalFrame).iv))) + (SerializeFunctions.default__.WriteUint32Seq((finalFrame).encContent))) + (SerializeFunctions.default__.Write((finalFrame).authTag))

    @staticmethod
    def ReadFinalFrame(buffer, header):
        d_0_valueOrError0_ = SerializeFunctions.default__.ReadUInt32(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_finalFrameSignal_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = Wrappers.default__.Need(((d_1_finalFrameSignal_).data) == (default__.ENDFRAME__SEQUENCE__NUMBER), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Final frame sequence number MUST be the end-frame sequence number.")))
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_valueOrError2_ = SerializeFunctions.default__.ReadUInt32((d_1_finalFrameSignal_).tail)
                if (d_3_valueOrError2_).IsFailure():
                    return (d_3_valueOrError2_).PropagateFailure()
                elif True:
                    d_4_sequenceNumber_ = (d_3_valueOrError2_).Extract()
                    d_5_valueOrError3_ = SerializeFunctions.default__.Read((d_4_sequenceNumber_).tail, SerializableTypes.default__.GetIvLength((header).suite))
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_iv_ = (d_5_valueOrError3_).Extract()
                        d_7_valueOrError4_ = SerializeFunctions.default__.ReadUInt32((d_6_iv_).tail)
                        if (d_7_valueOrError4_).IsFailure():
                            return (d_7_valueOrError4_).PropagateFailure()
                        elif True:
                            d_8_contentLength_ = (d_7_valueOrError4_).Extract()
                            d_9_valueOrError5_ = Wrappers.default__.Need(((d_8_contentLength_).data) <= (((header).body).frameLength), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Content length MUST NOT exceed the frame length.")))
                            if (d_9_valueOrError5_).IsFailure():
                                return (d_9_valueOrError5_).PropagateFailure()
                            elif True:
                                d_10_valueOrError6_ = SerializeFunctions.default__.ReadUint32Seq((d_6_iv_).tail)
                                if (d_10_valueOrError6_).IsFailure():
                                    return (d_10_valueOrError6_).PropagateFailure()
                                elif True:
                                    d_11_encContent_ = (d_10_valueOrError6_).Extract()
                                    d_12_valueOrError7_ = SerializeFunctions.default__.Read((d_11_encContent_).tail, SerializableTypes.default__.GetTagLength((header).suite))
                                    if (d_12_valueOrError7_).IsFailure():
                                        return (d_12_valueOrError7_).PropagateFailure()
                                    elif True:
                                        d_13_authTag_ = (d_12_valueOrError7_).Extract()
                                        d_14_finalFrame_ = Frame_FinalFrame(header, (d_4_sequenceNumber_).data, (d_6_iv_).data, (d_11_encContent_).data, (d_13_authTag_).data)
                                        return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_14_finalFrame_, (d_13_authTag_).tail))

    @staticmethod
    def ReadNonFrame(buffer, header):
        d_0_valueOrError0_ = SerializeFunctions.default__.Read(buffer, SerializableTypes.default__.GetIvLength((header).suite))
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_iv_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = SerializeFunctions.default__.ReadUInt64((d_1_iv_).tail)
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_contentLength_ = (d_2_valueOrError1_).Extract()
                d_4_valueOrError2_ = Wrappers.default__.Need(((d_3_contentLength_).data) < (default__.SAFE__MAX__ENCRYPT), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Frame exceeds AES-GCM cryptographic safety for a single key/iv.")))
                if (d_4_valueOrError2_).IsFailure():
                    return (d_4_valueOrError2_).PropagateFailure()
                elif True:
                    d_5_valueOrError3_ = SerializeFunctions.default__.ReadUint64Seq((d_1_iv_).tail)
                    if (d_5_valueOrError3_).IsFailure():
                        return (d_5_valueOrError3_).PropagateFailure()
                    elif True:
                        d_6_encContent_ = (d_5_valueOrError3_).Extract()
                        d_7_valueOrError4_ = SerializeFunctions.default__.Read((d_6_encContent_).tail, SerializableTypes.default__.GetTagLength((header).suite))
                        if (d_7_valueOrError4_).IsFailure():
                            return (d_7_valueOrError4_).PropagateFailure()
                        elif True:
                            d_8_authTag_ = (d_7_valueOrError4_).Extract()
                            d_9_nonFramed_ = Frame_NonFramed(header, (d_1_iv_).data, (d_6_encContent_).data, (d_8_authTag_).data)
                            return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_9_nonFramed_, (d_8_authTag_).tail))

    @_dafny.classproperty
    def ENDFRAME__SEQUENCE__NUMBER(instance):
        return 4294967295
    @_dafny.classproperty
    def SAFE__MAX__ENCRYPT(instance):
        return 68719476704
    @_dafny.classproperty
    def START__SEQUENCE__NUMBER(instance):
        return 1
    @_dafny.classproperty
    def NONFRAMED__SEQUENCE__NUMBER(instance):
        return 1

class FramedHeader:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Header.HeaderInfo.default()()

class NonFramedHeader:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Header.HeaderInfo.default()()

class Frame:
    @classmethod
    def default(cls, ):
        return lambda: Frame_RegularFrame(Header.HeaderInfo.default()(), int(0), _dafny.Seq({}), _dafny.Seq({}), _dafny.Seq({}))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_RegularFrame(self) -> bool:
        return isinstance(self, Frame_RegularFrame)
    @property
    def is_FinalFrame(self) -> bool:
        return isinstance(self, Frame_FinalFrame)
    @property
    def is_NonFramed(self) -> bool:
        return isinstance(self, Frame_NonFramed)

class Frame_RegularFrame(Frame, NamedTuple('RegularFrame', [('header', Any), ('seqNum', Any), ('iv', Any), ('encContent', Any), ('authTag', Any)])):
    def __dafnystr__(self) -> str:
        return f'Frames.Frame.RegularFrame({_dafny.string_of(self.header)}, {_dafny.string_of(self.seqNum)}, {_dafny.string_of(self.iv)}, {_dafny.string_of(self.encContent)}, {_dafny.string_of(self.authTag)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Frame_RegularFrame) and self.header == __o.header and self.seqNum == __o.seqNum and self.iv == __o.iv and self.encContent == __o.encContent and self.authTag == __o.authTag
    def __hash__(self) -> int:
        return super().__hash__()

class Frame_FinalFrame(Frame, NamedTuple('FinalFrame', [('header', Any), ('seqNum', Any), ('iv', Any), ('encContent', Any), ('authTag', Any)])):
    def __dafnystr__(self) -> str:
        return f'Frames.Frame.FinalFrame({_dafny.string_of(self.header)}, {_dafny.string_of(self.seqNum)}, {_dafny.string_of(self.iv)}, {_dafny.string_of(self.encContent)}, {_dafny.string_of(self.authTag)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Frame_FinalFrame) and self.header == __o.header and self.seqNum == __o.seqNum and self.iv == __o.iv and self.encContent == __o.encContent and self.authTag == __o.authTag
    def __hash__(self) -> int:
        return super().__hash__()

class Frame_NonFramed(Frame, NamedTuple('NonFramed', [('header', Any), ('iv', Any), ('encContent', Any), ('authTag', Any)])):
    def __dafnystr__(self) -> str:
        return f'Frames.Frame.NonFramed({_dafny.string_of(self.header)}, {_dafny.string_of(self.iv)}, {_dafny.string_of(self.encContent)}, {_dafny.string_of(self.authTag)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Frame_NonFramed) and self.header == __o.header and self.iv == __o.iv and self.encContent == __o.encContent and self.authTag == __o.authTag
    def __hash__(self) -> int:
        return super().__hash__()


class RegularFrame:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Frame.default()()

class FinalFrame:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Frame.default()()

class NonFramed:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return Frame.default()()
