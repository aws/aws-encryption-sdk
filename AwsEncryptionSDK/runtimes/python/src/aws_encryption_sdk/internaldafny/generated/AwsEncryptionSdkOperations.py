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
import aws_encryption_sdk.internaldafny.generated.Frames as Frames
import aws_encryption_sdk.internaldafny.generated.MessageBody as MessageBody
import aws_encryption_sdk.internaldafny.generated.KeyDerivation as KeyDerivation
import aws_encryption_sdk.internaldafny.generated.EncryptDecryptHelpers as EncryptDecryptHelpers

# Module: AwsEncryptionSdkOperations

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def Encrypt(config, input):
        output: Wrappers.Result = Wrappers.Result.default(AwsCryptographyEncryptionSdkTypes.EncryptOutput.default())()
        d_0_valueOrError1_: Wrappers.Result = Wrappers.Result.default(BoundedInts.int64.default)()
        if ((input).frameLength).is_Some:
            d_1_valueOrError0_ = Wrappers.default__.Need(((0) < (((input).frameLength).value)) and ((((input).frameLength).value) <= (4294967295)), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("FrameLength must be greater than 0 and less than 2^32")))
            if (d_1_valueOrError0_).IsFailure():
                d_0_valueOrError1_ = (d_1_valueOrError0_).PropagateFailure()
            elif True:
                d_0_valueOrError1_ = Wrappers.Result_Success(((input).frameLength).value)
        elif True:
            d_0_valueOrError1_ = Wrappers.Result_Success(EncryptDecryptHelpers.default__.DEFAULT__FRAME__LENGTH)
        if (d_0_valueOrError1_).IsFailure():
            output = (d_0_valueOrError1_).PropagateFailure()
            return output
        d_2_frameLength_: int
        d_2_frameLength_ = (d_0_valueOrError1_).Extract()
        d_3_valueOrError2_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_3_valueOrError2_ = EncryptDecryptHelpers.default__.ValidateEncryptionContext((input).encryptionContext)
        if (d_3_valueOrError2_).IsFailure():
            output = (d_3_valueOrError2_).PropagateFailure()
            return output
        d_4_encryptionContext_: _dafny.Map
        if ((input).encryptionContext).is_Some:
            d_4_encryptionContext_ = ((input).encryptionContext).value
        elif True:
            d_4_encryptionContext_ = _dafny.Map({})
        d_5_valueOrError3_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptDecryptHelpers.default__.CreateCmmFromInput((input).materialsManager, (input).keyring)
        d_5_valueOrError3_ = out0_
        if (d_5_valueOrError3_).IsFailure():
            output = (d_5_valueOrError3_).PropagateFailure()
            return output
        d_6_cmm_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_6_cmm_ = (d_5_valueOrError3_).Extract()
        d_7_algorithmSuiteId_: Wrappers.Option
        if ((input).algorithmSuiteId).is_Some:
            d_7_algorithmSuiteId_ = Wrappers.Option_Some(AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteId_ESDK(((input).algorithmSuiteId).value))
        elif True:
            d_7_algorithmSuiteId_ = Wrappers.Option_None()
        if (d_7_algorithmSuiteId_).is_Some:
            d_8_valueOrError4_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
            def lambda0_(d_9_e_):
                return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_9_e_)

            d_8_valueOrError4_ = (((config).mpl).ValidateCommitmentPolicyOnEncrypt(AwsCryptographyMaterialProvidersTypes.ValidateCommitmentPolicyOnEncryptInput_ValidateCommitmentPolicyOnEncryptInput((d_7_algorithmSuiteId_).value, AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK((config).commitmentPolicy)))).MapFailure(lambda0_)
            if (d_8_valueOrError4_).IsFailure():
                output = (d_8_valueOrError4_).PropagateFailure()
                return output
            d_10___v0_: tuple
            d_10___v0_ = (d_8_valueOrError4_).Extract()
        d_11_valueOrError5_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_11_valueOrError5_ = Wrappers.default__.Need((len((input).plaintext)) < (StandardLibrary_UInt.default__.INT64__MAX__LIMIT), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Plaintext exceeds maximum allowed size")))
        if (d_11_valueOrError5_).IsFailure():
            output = (d_11_valueOrError5_).PropagateFailure()
            return output
        d_12_valueOrError6_: Wrappers.Result = None
        out1_: Wrappers.Result
        out1_ = EncryptDecryptHelpers.default__.GetEncryptionMaterials(d_6_cmm_, d_7_algorithmSuiteId_, d_4_encryptionContext_, len((input).plaintext), (config).commitmentPolicy, (config).mpl)
        d_12_valueOrError6_ = out1_
        if (d_12_valueOrError6_).IsFailure():
            output = (d_12_valueOrError6_).PropagateFailure()
            return output
        d_13_materials_: AwsCryptographyMaterialProvidersTypes.EncryptionMaterials
        d_13_materials_ = (d_12_valueOrError6_).Extract()
        d_14_valueOrError7_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_14_valueOrError7_ = Wrappers.default__.Need((((d_13_materials_).algorithmSuite).id).is_ESDK, AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Encryption materials contain incompatible algorithm suite for the AWS Encryption SDK.")))
        if (d_14_valueOrError7_).IsFailure():
            output = (d_14_valueOrError7_).PropagateFailure()
            return output
        d_15_valueOrError8_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_15_valueOrError8_ = EncryptDecryptHelpers.default__.ValidateMaxEncryptedDataKeys((config).maxEncryptedDataKeys, (d_13_materials_).encryptedDataKeys)
        if (d_15_valueOrError8_).IsFailure():
            output = (d_15_valueOrError8_).PropagateFailure()
            return output
        d_16_encryptedDataKeys_: _dafny.Seq
        d_16_encryptedDataKeys_ = (d_13_materials_).encryptedDataKeys
        d_17_valueOrError9_: Wrappers.Result = None
        out2_: Wrappers.Result
        out2_ = EncryptDecryptHelpers.default__.GenerateMessageId((d_13_materials_).algorithmSuite, (config).crypto)
        d_17_valueOrError9_ = out2_
        if (d_17_valueOrError9_).IsFailure():
            output = (d_17_valueOrError9_).PropagateFailure()
            return output
        d_18_messageId_: _dafny.Seq
        d_18_messageId_ = (d_17_valueOrError9_).Extract()
        d_19_maybeDerivedDataKeys_: Wrappers.Result
        out3_: Wrappers.Result
        out3_ = KeyDerivation.default__.DeriveKeys(d_18_messageId_, ((d_13_materials_).plaintextDataKey).value, (d_13_materials_).algorithmSuite, (config).crypto, (config).netV4__0__0__RetryPolicy, False)
        d_19_maybeDerivedDataKeys_ = out3_
        d_20_valueOrError10_: Wrappers.Result = Wrappers.Result.default(KeyDerivation.ExpandedKeyMaterial.default())()
        def lambda1_(d_21_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Failed to derive data keys"))

        d_20_valueOrError10_ = (d_19_maybeDerivedDataKeys_).MapFailure(lambda1_)
        if (d_20_valueOrError10_).IsFailure():
            output = (d_20_valueOrError10_).PropagateFailure()
            return output
        d_22_derivedDataKeys_: KeyDerivation.ExpandedKeyMaterial
        d_22_derivedDataKeys_ = (d_20_valueOrError10_).Extract()
        d_23_maybeHeader_: Wrappers.Result
        out4_: Wrappers.Result
        out4_ = EncryptDecryptHelpers.default__.BuildHeaderForEncrypt(d_18_messageId_, (d_13_materials_).algorithmSuite, (d_13_materials_).encryptionContext, (d_13_materials_).requiredEncryptionContextKeys, d_16_encryptedDataKeys_, d_2_frameLength_, d_22_derivedDataKeys_, (config).crypto)
        d_23_maybeHeader_ = out4_
        d_24_valueOrError11_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_24_valueOrError11_ = Wrappers.default__.Need((d_23_maybeHeader_).is_Success, AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Failed to build header body")))
        if (d_24_valueOrError11_).IsFailure():
            output = (d_24_valueOrError11_).PropagateFailure()
            return output
        d_25_header_: Header.HeaderInfo
        d_25_header_ = (d_23_maybeHeader_).value
        d_26_valueOrError12_: Wrappers.Result = None
        out5_: Wrappers.Result
        out5_ = MessageBody.default__.EncryptMessageBody((input).plaintext, d_25_header_, (d_22_derivedDataKeys_).dataKey, (config).crypto)
        d_26_valueOrError12_ = out5_
        if (d_26_valueOrError12_).IsFailure():
            output = (d_26_valueOrError12_).PropagateFailure()
            return output
        d_27_framedMessage_: MessageBody.FramedMessageBody
        d_27_framedMessage_ = (d_26_valueOrError12_).Extract()
        d_28_maybeSignedMessage_: Wrappers.Result
        out6_: Wrappers.Result
        out6_ = default__.SignAndSerializeMessage(config, d_25_header_, d_27_framedMessage_, d_13_materials_)
        d_28_maybeSignedMessage_ = out6_
        output = d_28_maybeSignedMessage_
        return output

    @staticmethod
    def SignAndSerializeMessage(config, header, framedMessage, materials):
        output: Wrappers.Result = Wrappers.Result.default(AwsCryptographyEncryptionSdkTypes.EncryptOutput.default())()
        if (((((framedMessage).finalFrame).header).suite).signature).is_ECDSA:
            d_0_valueOrError0_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            d_0_valueOrError0_ = EncryptDecryptHelpers.default__.SerializeMessageWithoutSignature(framedMessage, (materials).algorithmSuite)
            if (d_0_valueOrError0_).IsFailure():
                output = (d_0_valueOrError0_).PropagateFailure()
                return output
            d_1_msg_: _dafny.Seq
            d_1_msg_ = (d_0_valueOrError0_).Extract()
            d_2_ecdsaParams_: AwsCryptographyPrimitivesTypes.ECDSASignatureAlgorithm
            d_2_ecdsaParams_ = ((((((framedMessage).finalFrame).header).suite).signature).ECDSA).curve
            d_3_valueOrError1_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_3_valueOrError1_ = Wrappers.default__.Need(((materials).signingKey).is_Some, AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Missing signing key.")))
            if (d_3_valueOrError1_).IsFailure():
                output = (d_3_valueOrError1_).PropagateFailure()
                return output
            d_4_maybeBytes_: Wrappers.Result
            out0_: Wrappers.Result
            out0_ = ((config).crypto).ECDSASign(AwsCryptographyPrimitivesTypes.ECDSASignInput_ECDSASignInput(d_2_ecdsaParams_, ((materials).signingKey).value, d_1_msg_))
            d_4_maybeBytes_ = out0_
            d_5_valueOrError2_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            def lambda0_(d_6_e_):
                return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_6_e_)

            d_5_valueOrError2_ = (d_4_maybeBytes_).MapFailure(lambda0_)
            if (d_5_valueOrError2_).IsFailure():
                output = (d_5_valueOrError2_).PropagateFailure()
                return output
            d_7_bytes_: _dafny.Seq
            d_7_bytes_ = (d_5_valueOrError2_).Extract()
            d_8_valueOrError3_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_8_valueOrError3_ = Wrappers.default__.Need((len(d_7_bytes_)) < (StandardLibrary_UInt.default__.UINT16__LIMIT), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Length of signature bytes is larger than the uint16 limit.")))
            if (d_8_valueOrError3_).IsFailure():
                output = (d_8_valueOrError3_).PropagateFailure()
                return output
            d_9_signature_: _dafny.Seq
            d_9_signature_ = (StandardLibrary_UInt.default__.UInt16ToSeq(len(d_7_bytes_))) + (d_7_bytes_)
            d_1_msg_ = (d_1_msg_) + (d_9_signature_)
            output = Wrappers.Result_Success(AwsCryptographyEncryptionSdkTypes.EncryptOutput_EncryptOutput(d_1_msg_, (header).encryptionContext, (((header).suite).id).ESDK))
            return output
        elif True:
            d_10_valueOrError4_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
            d_10_valueOrError4_ = EncryptDecryptHelpers.default__.SerializeMessageWithoutSignature(framedMessage, (materials).algorithmSuite)
            if (d_10_valueOrError4_).IsFailure():
                output = (d_10_valueOrError4_).PropagateFailure()
                return output
            d_11_msg_: _dafny.Seq
            d_11_msg_ = (d_10_valueOrError4_).Extract()
            output = Wrappers.Result_Success(AwsCryptographyEncryptionSdkTypes.EncryptOutput_EncryptOutput(d_11_msg_, (header).encryptionContext, (((header).suite).id).ESDK))
            return output
        return output

    @staticmethod
    def Decrypt(config, input):
        output: Wrappers.Result = Wrappers.Result.default(AwsCryptographyEncryptionSdkTypes.DecryptOutput.default())()
        d_0_valueOrError0_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptDecryptHelpers.default__.CreateCmmFromInput((input).materialsManager, (input).keyring)
        d_0_valueOrError0_ = out0_
        if (d_0_valueOrError0_).IsFailure():
            output = (d_0_valueOrError0_).PropagateFailure()
            return output
        d_1_cmm_: AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
        d_1_cmm_ = (d_0_valueOrError0_).Extract()
        d_2_buffer_: SerializeFunctions.ReadableBuffer
        d_2_buffer_ = SerializeFunctions.ReadableBuffer_ReadableBuffer((input).ciphertext, 0)
        out1_: Wrappers.Result
        out1_ = default__.InternalDecrypt(config, d_1_cmm_, d_2_buffer_, (input).encryptionContext)
        output = out1_
        return output

    @staticmethod
    def InternalDecrypt(config, cmm, buffer, inputEncryptionContext):
        output: Wrappers.Result = Wrappers.Result.default(AwsCryptographyEncryptionSdkTypes.DecryptOutput.default())()
        d_0_v4Retry_: bool
        d_0_v4Retry_ = False
        d_1_valueOrError0_: Wrappers.Result = None
        d_1_valueOrError0_ = (Header.default__.ReadHeaderBody(buffer, (config).maxEncryptedDataKeys, (config).mpl)).MapFailure(EncryptDecryptHelpers.default__.MapSerializeFailure(_dafny.Seq(": ReadHeaderBody")))
        if (d_1_valueOrError0_).IsFailure():
            output = (d_1_valueOrError0_).PropagateFailure()
            return output
        d_2_headerBody_: SerializeFunctions.SuccessfulRead
        d_2_headerBody_ = (d_1_valueOrError0_).Extract()
        d_3_rawHeader_: _dafny.Seq
        d_3_rawHeader_ = _dafny.Seq(((buffer).bytes)[(buffer).start:((d_2_headerBody_).tail).start:])
        d_4_algorithmSuite_: AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
        d_4_algorithmSuite_ = ((d_2_headerBody_).data).algorithmSuite
        d_5_valueOrError1_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
        def lambda0_(d_6_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyMaterialProviders(d_6_e_)

        d_5_valueOrError1_ = (((config).mpl).ValidateCommitmentPolicyOnDecrypt(AwsCryptographyMaterialProvidersTypes.ValidateCommitmentPolicyOnDecryptInput_ValidateCommitmentPolicyOnDecryptInput((d_4_algorithmSuite_).id, AwsCryptographyMaterialProvidersTypes.CommitmentPolicy_ESDK((config).commitmentPolicy)))).MapFailure(lambda0_)
        if (d_5_valueOrError1_).IsFailure():
            output = (d_5_valueOrError1_).PropagateFailure()
            return output
        d_7___v1_: tuple
        d_7___v1_ = (d_5_valueOrError1_).Extract()
        d_8_valueOrError2_: Wrappers.Result = None
        out0_: Wrappers.Result
        out0_ = EncryptDecryptHelpers.default__.GetDecryptionMaterials(cmm, (d_4_algorithmSuite_).id, (d_2_headerBody_).data, inputEncryptionContext, (config).commitmentPolicy, (config).mpl)
        d_8_valueOrError2_ = out0_
        if (d_8_valueOrError2_).IsFailure():
            output = (d_8_valueOrError2_).PropagateFailure()
            return output
        d_9_decMat_: AwsCryptographyMaterialProvidersTypes.DecryptionMaterials
        d_9_decMat_ = (d_8_valueOrError2_).Extract()
        d_10_suite_: AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
        d_10_suite_ = (d_9_decMat_).algorithmSuite
        d_11_valueOrError3_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_11_valueOrError3_ = Wrappers.default__.Need((d_10_suite_) == (d_4_algorithmSuite_), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Stored header algorithm suite does not match decryption algorithm suite.")))
        if (d_11_valueOrError3_).IsFailure():
            output = (d_11_valueOrError3_).PropagateFailure()
            return output
        d_12_valueOrError4_: Wrappers.Result = None
        d_12_valueOrError4_ = (HeaderAuth.default__.ReadHeaderAuthTag((d_2_headerBody_).tail, d_10_suite_)).MapFailure(EncryptDecryptHelpers.default__.MapSerializeFailure(_dafny.Seq(": ReadHeaderAuthTag")))
        if (d_12_valueOrError4_).IsFailure():
            output = (d_12_valueOrError4_).PropagateFailure()
            return output
        d_13_headerAuth_: SerializeFunctions.SuccessfulRead
        d_13_headerAuth_ = (d_12_valueOrError4_).Extract()
        d_14_maybeDerivedDataKeys_: Wrappers.Result
        out1_: Wrappers.Result
        out1_ = KeyDerivation.default__.DeriveKeys(((d_2_headerBody_).data).messageId, ((d_9_decMat_).plaintextDataKey).value, d_10_suite_, (config).crypto, (config).netV4__0__0__RetryPolicy, False)
        d_14_maybeDerivedDataKeys_ = out1_
        d_15_valueOrError5_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_15_valueOrError5_ = Wrappers.default__.Need((d_14_maybeDerivedDataKeys_).is_Success, AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Failed to derive data keys")))
        if (d_15_valueOrError5_).IsFailure():
            output = (d_15_valueOrError5_).PropagateFailure()
            return output
        d_16_derivedDataKeys_: KeyDerivation.ExpandedKeyMaterial
        d_16_derivedDataKeys_ = (d_14_maybeDerivedDataKeys_).value
        d_17_valueOrError6_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_17_valueOrError6_ = Wrappers.default__.Need(Header.default__.HeaderVersionSupportsCommitment_q(d_10_suite_, (d_2_headerBody_).data), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Invalid commitment values found in header body")))
        if (d_17_valueOrError6_).IsFailure():
            output = (d_17_valueOrError6_).PropagateFailure()
            return output
        if ((d_10_suite_).commitment).is_HKDF:
            d_18_valueOrError7_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple())()
            out2_: Wrappers.Result
            out2_ = EncryptDecryptHelpers.default__.ValidateSuiteData(d_10_suite_, (d_2_headerBody_).data, ((d_16_derivedDataKeys_).commitmentKey).value)
            d_18_valueOrError7_ = out2_
            if (d_18_valueOrError7_).IsFailure():
                output = (d_18_valueOrError7_).PropagateFailure()
                return output
            d_19___v2_: tuple
            d_19___v2_ = (d_18_valueOrError7_).Extract()
        d_20_headerEncryptionContext_: _dafny.Map
        d_20_headerEncryptionContext_ = EncryptionContext.default__.GetEncryptionContext(((d_2_headerBody_).data).encryptionContext)
        d_21_encryptionContextToOnlyAuthenticate_: _dafny.Map
        d_21_encryptionContextToOnlyAuthenticate_ = default__.buildEncryptionContextToOnlyAuthenticate(d_9_decMat_)
        d_22_canonicalReqEncryptionContext_: _dafny.Seq
        d_22_canonicalReqEncryptionContext_ = EncryptionContext.default__.GetCanonicalEncryptionContext(d_21_encryptionContextToOnlyAuthenticate_)
        d_23_serializedReqEncryptionContext_: _dafny.Seq
        d_23_serializedReqEncryptionContext_ = EncryptionContext.default__.WriteEmptyEcOrWriteAAD(d_22_canonicalReqEncryptionContext_)
        d_24_maybeHeaderAuth_: Wrappers.Result
        out3_: Wrappers.Result
        out3_ = ((config).crypto).AESDecrypt(AwsCryptographyPrimitivesTypes.AESDecryptInput_AESDecryptInput(((d_10_suite_).encrypt).AES__GCM, (d_16_derivedDataKeys_).dataKey, _dafny.Seq([]), ((d_13_headerAuth_).data).headerAuthTag, ((d_13_headerAuth_).data).headerIv, (d_3_rawHeader_) + (d_23_serializedReqEncryptionContext_)))
        d_24_maybeHeaderAuth_ = out3_
        if (((d_24_maybeHeaderAuth_).is_Failure) and (((config).netV4__0__0__RetryPolicy) == (AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy_ALLOW__RETRY()))) and ((d_0_v4Retry_) == (False)):
            d_0_v4Retry_ = True
            out4_: Wrappers.Result
            out4_ = KeyDerivation.default__.DeriveKeys(((d_2_headerBody_).data).messageId, ((d_9_decMat_).plaintextDataKey).value, d_10_suite_, (config).crypto, (config).netV4__0__0__RetryPolicy, True)
            d_14_maybeDerivedDataKeys_ = out4_
            d_25_valueOrError8_: Wrappers.Outcome = Wrappers.Outcome.default()()
            d_25_valueOrError8_ = Wrappers.default__.Need((d_14_maybeDerivedDataKeys_).is_Success, AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Failed to derive data keys")))
            if (d_25_valueOrError8_).IsFailure():
                output = (d_25_valueOrError8_).PropagateFailure()
                return output
            d_16_derivedDataKeys_ = (d_14_maybeDerivedDataKeys_).value
            d_23_serializedReqEncryptionContext_ = EncryptionContext.default__.WriteAAD(d_22_canonicalReqEncryptionContext_)
            out5_: Wrappers.Result
            out5_ = ((config).crypto).AESDecrypt(AwsCryptographyPrimitivesTypes.AESDecryptInput_AESDecryptInput(((d_10_suite_).encrypt).AES__GCM, (d_16_derivedDataKeys_).dataKey, _dafny.Seq([]), ((d_13_headerAuth_).data).headerAuthTag, ((d_13_headerAuth_).data).headerIv, (d_3_rawHeader_) + (d_23_serializedReqEncryptionContext_)))
            d_24_maybeHeaderAuth_ = out5_
        d_26_valueOrError9_: Wrappers.Result = Wrappers.Result.default(_dafny.Seq)()
        def lambda1_(d_27_e_):
            return AwsCryptographyEncryptionSdkTypes.Error_AwsCryptographyPrimitives(d_27_e_)

        d_26_valueOrError9_ = (d_24_maybeHeaderAuth_).MapFailure(lambda1_)
        if (d_26_valueOrError9_).IsFailure():
            output = (d_26_valueOrError9_).PropagateFailure()
            return output
        d_28___v3_: _dafny.Seq
        d_28___v3_ = (d_26_valueOrError9_).Extract()
        d_29_header_: Header.HeaderInfo
        d_29_header_ = Header.HeaderInfo_HeaderInfo((d_2_headerBody_).data, d_3_rawHeader_, d_20_headerEncryptionContext_, d_10_suite_, (d_13_headerAuth_).data)
        d_30_key_: _dafny.Seq
        d_30_key_ = (d_16_derivedDataKeys_).dataKey
        d_31_plaintext_: _dafny.Seq = _dafny.Seq({})
        d_32_messageBodyTail_: SerializeFunctions.ReadableBuffer = SerializeFunctions.ReadableBuffer.default()()
        source0_ = ((d_29_header_).body).contentType
        with _dafny.label("match0"):
            if True:
                if source0_.is_NonFramed:
                    d_33_valueOrError10_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple(_dafny.Seq, SerializeFunctions.ReadableBuffer.default()))()
                    out6_: Wrappers.Result
                    out6_ = EncryptDecryptHelpers.default__.ReadAndDecryptNonFramedMessageBody((d_13_headerAuth_).tail, d_29_header_, d_30_key_, (config).crypto)
                    d_33_valueOrError10_ = out6_
                    if (d_33_valueOrError10_).IsFailure():
                        output = (d_33_valueOrError10_).PropagateFailure()
                        return output
                    d_34_decryptRes_: tuple
                    d_34_decryptRes_ = (d_33_valueOrError10_).Extract()
                    d_31_plaintext_ = (d_34_decryptRes_)[0]
                    d_32_messageBodyTail_ = (d_34_decryptRes_)[1]
                    raise _dafny.Break("match0")
            if True:
                d_35_valueOrError11_: Wrappers.Result = Wrappers.Result.default(_dafny.defaults.tuple(_dafny.Seq, SerializeFunctions.ReadableBuffer.default()))()
                out7_: Wrappers.Result
                out7_ = EncryptDecryptHelpers.default__.ReadAndDecryptFramedMessageBody((d_13_headerAuth_).tail, d_29_header_, d_30_key_, (config).crypto)
                d_35_valueOrError11_ = out7_
                if (d_35_valueOrError11_).IsFailure():
                    output = (d_35_valueOrError11_).PropagateFailure()
                    return output
                d_36_decryptRes_: tuple
                d_36_decryptRes_ = (d_35_valueOrError11_).Extract()
                d_31_plaintext_ = (d_36_decryptRes_)[0]
                d_32_messageBodyTail_ = (d_36_decryptRes_)[1]
            pass
        d_37_valueOrError12_: Wrappers.Result = Wrappers.Result.default(SerializeFunctions.ReadableBuffer.default())()
        out8_: Wrappers.Result
        out8_ = EncryptDecryptHelpers.default__.VerifySignature(d_32_messageBodyTail_, _dafny.Seq(((d_32_messageBodyTail_).bytes)[(buffer).start:(d_32_messageBodyTail_).start:]), d_9_decMat_, (config).crypto)
        d_37_valueOrError12_ = out8_
        if (d_37_valueOrError12_).IsFailure():
            output = (d_37_valueOrError12_).PropagateFailure()
            return output
        d_38_signature_: SerializeFunctions.ReadableBuffer
        d_38_signature_ = (d_37_valueOrError12_).Extract()
        d_39_valueOrError13_: Wrappers.Outcome = Wrappers.Outcome.default()()
        d_39_valueOrError13_ = Wrappers.default__.Need(((d_38_signature_).start) == (len((d_38_signature_).bytes)), AwsCryptographyEncryptionSdkTypes.Error_AwsEncryptionSdkException(_dafny.Seq("Data after message footer.")))
        if (d_39_valueOrError13_).IsFailure():
            output = (d_39_valueOrError13_).PropagateFailure()
            return output
        output = Wrappers.Result_Success(AwsCryptographyEncryptionSdkTypes.DecryptOutput_DecryptOutput(d_31_plaintext_, ((d_29_header_).encryptionContext) | (d_21_encryptionContextToOnlyAuthenticate_), (((d_29_header_).suite).id).ESDK))
        return output

    @staticmethod
    def buildEncryptionContextToOnlyAuthenticate(decMat):
        def iife0_():
            coll0_ = _dafny.Map()
            compr_0_: _dafny.Seq
            for compr_0_ in ((decMat).encryptionContext).keys.Elements:
                d_0_k_: _dafny.Seq = compr_0_
                if UTF8.ValidUTF8Bytes._Is(d_0_k_):
                    if ((d_0_k_) in ((decMat).encryptionContext)) and ((d_0_k_) in ((decMat).requiredEncryptionContextKeys)):
                        coll0_[d_0_k_] = ((decMat).encryptionContext)[d_0_k_]
            return _dafny.Map(coll0_)
        return iife0_()
        


class Config:
    @classmethod
    def default(cls, ):
        return lambda: Config_Config(None, None, AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy.default()(), Wrappers.Option.default()(), AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_Config(self) -> bool:
        return isinstance(self, Config_Config)

class Config_Config(Config, NamedTuple('Config', [('crypto', Any), ('mpl', Any), ('commitmentPolicy', Any), ('maxEncryptedDataKeys', Any), ('netV4__0__0__RetryPolicy', Any)])):
    def __dafnystr__(self) -> str:
        return f'AwsEncryptionSdkOperations.Config.Config({_dafny.string_of(self.crypto)}, {_dafny.string_of(self.mpl)}, {_dafny.string_of(self.commitmentPolicy)}, {_dafny.string_of(self.maxEncryptedDataKeys)}, {_dafny.string_of(self.netV4__0__0__RetryPolicy)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Config_Config) and self.crypto == __o.crypto and self.mpl == __o.mpl and self.commitmentPolicy == __o.commitmentPolicy and self.maxEncryptedDataKeys == __o.maxEncryptedDataKeys and self.netV4__0__0__RetryPolicy == __o.netV4__0__0__RetryPolicy
    def __hash__(self) -> int:
        return super().__hash__()


class FrameLength:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return int(0)
    def _Is(source__):
        d_0_frameLength_: int = source__
        if True:
            return ((0) < (d_0_frameLength_)) and ((d_0_frameLength_) <= (4294967295))
        return False
