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

# Module: SerializeFunctions

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def Write(data):
        return data

    @staticmethod
    def Read(buffer, length):
        d_0_end_ = ((buffer).start) + (length)
        d_1_valueOrError0_ = Wrappers.default__.Need((len((buffer).bytes)) >= (d_0_end_), ReadProblems_MoreNeeded(d_0_end_))
        if (d_1_valueOrError0_).IsFailure():
            return (d_1_valueOrError0_).PropagateFailure()
        elif True:
            def iife0_(_pat_let0_0):
                def iife1_(d_2_dt__update__tmp_h0_):
                    def iife2_(_pat_let1_0):
                        def iife3_(d_3_dt__update_hstart_h0_):
                            return ReadableBuffer_ReadableBuffer((d_2_dt__update__tmp_h0_).bytes, d_3_dt__update_hstart_h0_)
                        return iife3_(_pat_let1_0)
                    return iife2_(d_0_end_)
                return iife1_(_pat_let0_0)
            return Wrappers.Result_Success(SuccessfulRead_SuccessfulRead(_dafny.Seq(((buffer).bytes)[(buffer).start:d_0_end_:]), iife0_(buffer)))

    @staticmethod
    def WriteUint16(number):
        return default__.Write(StandardLibrary_UInt.default__.UInt16ToSeq(number))

    @staticmethod
    def ReadUInt16(buffer):
        d_0_valueOrError0_ = default__.Read(buffer, 2)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_uint16Bytes_ = let_tmp_rhs0_.data
            d_2_tail_ = let_tmp_rhs0_.tail
            return Wrappers.Result_Success(SuccessfulRead_SuccessfulRead(StandardLibrary_UInt.default__.SeqToUInt16(d_1_uint16Bytes_), d_2_tail_))

    @staticmethod
    def WriteUint32(number):
        return default__.Write(StandardLibrary_UInt.default__.UInt32ToSeq(number))

    @staticmethod
    def ReadUInt32(buffer):
        d_0_valueOrError0_ = default__.Read(buffer, 4)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_uint32Bytes_ = let_tmp_rhs0_.data
            d_2_tail_ = let_tmp_rhs0_.tail
            return Wrappers.Result_Success(SuccessfulRead_SuccessfulRead(StandardLibrary_UInt.default__.SeqToUInt32(d_1_uint32Bytes_), d_2_tail_))

    @staticmethod
    def WriteUint64(number):
        return default__.Write(StandardLibrary_UInt.default__.UInt64ToSeq(number))

    @staticmethod
    def ReadUInt64(buffer):
        d_0_valueOrError0_ = default__.Read(buffer, 8)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_uint64Bytes_ = let_tmp_rhs0_.data
            d_2_tail_ = let_tmp_rhs0_.tail
            return Wrappers.Result_Success(SuccessfulRead_SuccessfulRead(StandardLibrary_UInt.default__.SeqToUInt64(d_1_uint64Bytes_), d_2_tail_))

    @staticmethod
    def WriteShortLengthSeq(d):
        return (default__.WriteUint16(len(d))) + (default__.Write(d))

    @staticmethod
    def ReadShortLengthSeq(buffer):
        d_0_valueOrError0_ = default__.ReadUInt16(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_length_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = default__.Read((d_1_length_).tail, (d_1_length_).data)
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_d_ = (d_2_valueOrError1_).Extract()
                return Wrappers.Result_Success(d_3_d_)

    @staticmethod
    def WriteUint32Seq(d):
        return (default__.WriteUint32(len(d))) + (default__.Write(d))

    @staticmethod
    def ReadUint32Seq(buffer):
        d_0_valueOrError0_ = default__.ReadUInt32(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_length_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = default__.Read((d_1_length_).tail, (d_1_length_).data)
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_d_ = (d_2_valueOrError1_).Extract()
                return Wrappers.Result_Success(d_3_d_)

    @staticmethod
    def WriteUint64Seq(d):
        return (default__.WriteUint64(len(d))) + (default__.Write(d))

    @staticmethod
    def ReadUint64Seq(buffer):
        d_0_valueOrError0_ = default__.ReadUInt64(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_length_ = (d_0_valueOrError0_).Extract()
            d_2_valueOrError1_ = default__.Read((d_1_length_).tail, (d_1_length_).data)
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                d_3_d_ = (d_2_valueOrError1_).Extract()
                return Wrappers.Result_Success(d_3_d_)


class ReadProblems:
    @classmethod
    def default(cls, ):
        return lambda: ReadProblems_MoreNeeded(int(0))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_MoreNeeded(self) -> bool:
        return isinstance(self, ReadProblems_MoreNeeded)
    @property
    def is_Error(self) -> bool:
        return isinstance(self, ReadProblems_Error)

class ReadProblems_MoreNeeded(ReadProblems, NamedTuple('MoreNeeded', [('pos', Any)])):
    def __dafnystr__(self) -> str:
        return f'SerializeFunctions.ReadProblems.MoreNeeded({_dafny.string_of(self.pos)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ReadProblems_MoreNeeded) and self.pos == __o.pos
    def __hash__(self) -> int:
        return super().__hash__()

class ReadProblems_Error(ReadProblems, NamedTuple('Error', [('message', Any)])):
    def __dafnystr__(self) -> str:
        return f'SerializeFunctions.ReadProblems.Error({_dafny.string_of(self.message)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ReadProblems_Error) and self.message == __o.message
    def __hash__(self) -> int:
        return super().__hash__()


class MoreNeeded:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return ReadProblems.default()()
    def _Is(source__):
        d_0_p_: ReadProblems = source__
        return (d_0_p_).is_MoreNeeded

class ReadableBuffer:
    @classmethod
    def default(cls, ):
        return lambda: ReadableBuffer_ReadableBuffer(_dafny.Seq({}), int(0))
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_ReadableBuffer(self) -> bool:
        return isinstance(self, ReadableBuffer_ReadableBuffer)

class ReadableBuffer_ReadableBuffer(ReadableBuffer, NamedTuple('ReadableBuffer', [('bytes', Any), ('start', Any)])):
    def __dafnystr__(self) -> str:
        return f'SerializeFunctions.ReadableBuffer.ReadableBuffer({_dafny.string_of(self.bytes)}, {_dafny.string_of(self.start)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, ReadableBuffer_ReadableBuffer) and self.bytes == __o.bytes and self.start == __o.start
    def __hash__(self) -> int:
        return super().__hash__()


class SuccessfulRead:
    @classmethod
    def default(cls, default_T):
        return lambda: SuccessfulRead_SuccessfulRead(default_T(), ReadableBuffer.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_SuccessfulRead(self) -> bool:
        return isinstance(self, SuccessfulRead_SuccessfulRead)

class SuccessfulRead_SuccessfulRead(SuccessfulRead, NamedTuple('SuccessfulRead', [('data', Any), ('tail', Any)])):
    def __dafnystr__(self) -> str:
        return f'SerializeFunctions.SuccessfulRead.SuccessfulRead({_dafny.string_of(self.data)}, {_dafny.string_of(self.tail)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, SuccessfulRead_SuccessfulRead) and self.data == __o.data and self.tail == __o.tail
    def __hash__(self) -> int:
        return super().__hash__()

