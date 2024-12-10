// Package Frames
// Dafny module Frames compiled into Go

package Frames

import (
	os "os"

	m_ComAmazonawsDynamodbTypes "github.com/aws/aws-cryptographic-material-providers-library/dynamodb/ComAmazonawsDynamodbTypes"
	m_Com_Amazonaws_Dynamodb "github.com/aws/aws-cryptographic-material-providers-library/dynamodb/Com_Amazonaws_Dynamodb"
	m_ComAmazonawsKmsTypes "github.com/aws/aws-cryptographic-material-providers-library/kms/ComAmazonawsKmsTypes"
	m_Com_Amazonaws_Kms "github.com/aws/aws-cryptographic-material-providers-library/kms/Com_Amazonaws_Kms"
	m_AlgorithmSuites "github.com/aws/aws-cryptographic-material-providers-library/mpl/AlgorithmSuites"
	m_AwsArnParsing "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsArnParsing"
	m_AwsCryptographyKeyStoreOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyKeyStoreOperations"
	m_AwsCryptographyKeyStoreTypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyKeyStoreTypes"
	m_AwsCryptographyMaterialProvidersOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyMaterialProvidersOperations"
	m_AwsCryptographyMaterialProvidersTypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyMaterialProvidersTypes"
	m_AwsKmsDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsDiscoveryKeyring"
	m_AwsKmsEcdhKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsEcdhKeyring"
	m_AwsKmsHierarchicalKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsHierarchicalKeyring"
	m_AwsKmsKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsKeyring"
	m_AwsKmsMrkAreUnique "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkAreUnique"
	m_AwsKmsMrkDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkDiscoveryKeyring"
	m_AwsKmsMrkKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkKeyring"
	m_AwsKmsMrkMatchForDecrypt "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkMatchForDecrypt"
	m_AwsKmsRsaKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsRsaKeyring"
	m_AwsKmsUtils "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsUtils"
	m_CMM "github.com/aws/aws-cryptographic-material-providers-library/mpl/CMM"
	m_CacheConstants "github.com/aws/aws-cryptographic-material-providers-library/mpl/CacheConstants"
	m_CanonicalEncryptionContext "github.com/aws/aws-cryptographic-material-providers-library/mpl/CanonicalEncryptionContext"
	m_Commitment "github.com/aws/aws-cryptographic-material-providers-library/mpl/Commitment"
	m_Constants "github.com/aws/aws-cryptographic-material-providers-library/mpl/Constants"
	m_CreateKeyStoreTable "github.com/aws/aws-cryptographic-material-providers-library/mpl/CreateKeyStoreTable"
	m_CreateKeys "github.com/aws/aws-cryptographic-material-providers-library/mpl/CreateKeys"
	m_DDBKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/DDBKeystoreOperations"
	m_DefaultCMM "github.com/aws/aws-cryptographic-material-providers-library/mpl/DefaultCMM"
	m_DefaultClientSupplier "github.com/aws/aws-cryptographic-material-providers-library/mpl/DefaultClientSupplier"
	m_Defaults "github.com/aws/aws-cryptographic-material-providers-library/mpl/Defaults"
	m_DiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/DiscoveryMultiKeyring"
	m_EcdhEdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/EcdhEdkWrapping"
	m_EdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/EdkWrapping"
	m_ErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/mpl/ErrorMessages"
	m_GetKeys "github.com/aws/aws-cryptographic-material-providers-library/mpl/GetKeys"
	m_IntermediateKeyWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/IntermediateKeyWrapping"
	m_KMSKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/KMSKeystoreOperations"
	m_KeyStore "github.com/aws/aws-cryptographic-material-providers-library/mpl/KeyStore"
	m_KeyStoreErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/mpl/KeyStoreErrorMessages"
	m_Keyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/Keyring"
	m_KmsArn "github.com/aws/aws-cryptographic-material-providers-library/mpl/KmsArn"
	m_LocalCMC "github.com/aws/aws-cryptographic-material-providers-library/mpl/LocalCMC"
	m_MaterialProviders "github.com/aws/aws-cryptographic-material-providers-library/mpl/MaterialProviders"
	m_MaterialWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/MaterialWrapping"
	m_Materials "github.com/aws/aws-cryptographic-material-providers-library/mpl/Materials"
	m_MrkAwareDiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/MrkAwareDiscoveryMultiKeyring"
	m_MrkAwareStrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/MrkAwareStrictMultiKeyring"
	m_MultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/MultiKeyring"
	m_RawAESKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/RawAESKeyring"
	m_RawECDHKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/RawECDHKeyring"
	m_RawRSAKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/RawRSAKeyring"
	m_RequiredEncryptionContextCMM "github.com/aws/aws-cryptographic-material-providers-library/mpl/RequiredEncryptionContextCMM"
	m_StormTracker "github.com/aws/aws-cryptographic-material-providers-library/mpl/StormTracker"
	m_StormTrackingCMC "github.com/aws/aws-cryptographic-material-providers-library/mpl/StormTrackingCMC"
	m_StrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/StrictMultiKeyring"
	m_Structure "github.com/aws/aws-cryptographic-material-providers-library/mpl/Structure"
	m_SynchronizedLocalCMC "github.com/aws/aws-cryptographic-material-providers-library/mpl/SynchronizedLocalCMC"
	m_Utils "github.com/aws/aws-cryptographic-material-providers-library/mpl/Utils"
	m_AESEncryption "github.com/aws/aws-cryptographic-material-providers-library/primitives/AESEncryption"
	m_AtomicPrimitives "github.com/aws/aws-cryptographic-material-providers-library/primitives/AtomicPrimitives"
	m_AwsCryptographyPrimitivesOperations "github.com/aws/aws-cryptographic-material-providers-library/primitives/AwsCryptographyPrimitivesOperations"
	m_AwsCryptographyPrimitivesTypes "github.com/aws/aws-cryptographic-material-providers-library/primitives/AwsCryptographyPrimitivesTypes"
	m_Digest "github.com/aws/aws-cryptographic-material-providers-library/primitives/Digest"
	m_ECDH "github.com/aws/aws-cryptographic-material-providers-library/primitives/ECDH"
	m_HKDF "github.com/aws/aws-cryptographic-material-providers-library/primitives/HKDF"
	m_HMAC "github.com/aws/aws-cryptographic-material-providers-library/primitives/HMAC"
	m_KdfCtr "github.com/aws/aws-cryptographic-material-providers-library/primitives/KdfCtr"
	m_RSAEncryption "github.com/aws/aws-cryptographic-material-providers-library/primitives/RSAEncryption"
	m_Random "github.com/aws/aws-cryptographic-material-providers-library/primitives/Random"
	m_Signature "github.com/aws/aws-cryptographic-material-providers-library/primitives/Signature"
	m_WrappedHKDF "github.com/aws/aws-cryptographic-material-providers-library/primitives/WrappedHKDF"
	m_WrappedHMAC "github.com/aws/aws-cryptographic-material-providers-library/primitives/WrappedHMAC"
	m_AwsCryptographyEncryptionSdkTypes "github.com/aws/aws-encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	m_EncryptedDataKeys "github.com/aws/aws-encryption-sdk/EncryptedDataKeys"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/EncryptionContext"
	m_Header "github.com/aws/aws-encryption-sdk/Header"
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/HeaderTypes"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/SharedHeaderFunctions"
	m_V1HeaderBody "github.com/aws/aws-encryption-sdk/V1HeaderBody"
	m_V2HeaderBody "github.com/aws/aws-encryption-sdk/V2HeaderBody"
	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	m_Actions "github.com/dafny-lang/DafnyStandardLibGo/Actions"
	m_Base64 "github.com/dafny-lang/DafnyStandardLibGo/Base64"
	m_Base64Lemmas "github.com/dafny-lang/DafnyStandardLibGo/Base64Lemmas"
	m_BoundedInts "github.com/dafny-lang/DafnyStandardLibGo/BoundedInts"
	m_DivInternals "github.com/dafny-lang/DafnyStandardLibGo/DivInternals"
	m_DivInternalsNonlinear "github.com/dafny-lang/DafnyStandardLibGo/DivInternalsNonlinear"
	m_DivMod "github.com/dafny-lang/DafnyStandardLibGo/DivMod"
	m_FileIO "github.com/dafny-lang/DafnyStandardLibGo/FileIO"
	m_FloatCompare "github.com/dafny-lang/DafnyStandardLibGo/FloatCompare"
	m_Functions "github.com/dafny-lang/DafnyStandardLibGo/Functions"
	m_GeneralInternals "github.com/dafny-lang/DafnyStandardLibGo/GeneralInternals"
	m_GetOpt "github.com/dafny-lang/DafnyStandardLibGo/GetOpt"
	m_HexStrings "github.com/dafny-lang/DafnyStandardLibGo/HexStrings"
	m_Logarithm "github.com/dafny-lang/DafnyStandardLibGo/Logarithm"
	m__Math "github.com/dafny-lang/DafnyStandardLibGo/Math_"
	m_ModInternals "github.com/dafny-lang/DafnyStandardLibGo/ModInternals"
	m_ModInternalsNonlinear "github.com/dafny-lang/DafnyStandardLibGo/ModInternalsNonlinear"
	m_Mul "github.com/dafny-lang/DafnyStandardLibGo/Mul"
	m_MulInternals "github.com/dafny-lang/DafnyStandardLibGo/MulInternals"
	m_MulInternalsNonlinear "github.com/dafny-lang/DafnyStandardLibGo/MulInternalsNonlinear"
	m_Power "github.com/dafny-lang/DafnyStandardLibGo/Power"
	m_Relations "github.com/dafny-lang/DafnyStandardLibGo/Relations"
	m_Seq "github.com/dafny-lang/DafnyStandardLibGo/Seq"
	m_Seq_MergeSort "github.com/dafny-lang/DafnyStandardLibGo/Seq_MergeSort"
	m_Sorting "github.com/dafny-lang/DafnyStandardLibGo/Sorting"
	m_StandardLibrary "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary"
	m_StandardLibraryInterop "github.com/dafny-lang/DafnyStandardLibGo/StandardLibraryInterop"
	m_StandardLibrary_Sequence "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_Sequence"
	m_StandardLibrary_String "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_String"
	m_StandardLibrary_UInt "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_UInt"
	m_Streams "github.com/dafny-lang/DafnyStandardLibGo/Streams"
	m_UnicodeStrings "github.com/dafny-lang/DafnyStandardLibGo/UnicodeStrings"
	m__Unicode "github.com/dafny-lang/DafnyStandardLibGo/Unicode_"
	m_Utf16EncodingForm "github.com/dafny-lang/DafnyStandardLibGo/Utf16EncodingForm"
	m_Utf8EncodingForm "github.com/dafny-lang/DafnyStandardLibGo/Utf8EncodingForm"
	m_Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_BoundedInts.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary_Sequence.Dummy__
var _ m_StandardLibrary_String.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_AwsCryptographyPrimitivesTypes.Dummy__
var _ m_Random.Dummy__
var _ m_AESEncryption.Dummy__
var _ m_Digest.Dummy__
var _ m_HMAC.Dummy__
var _ m_WrappedHMAC.Dummy__
var _ m_HKDF.Dummy__
var _ m_WrappedHKDF.Dummy__
var _ m_Signature.Dummy__
var _ m_KdfCtr.Dummy__
var _ m_RSAEncryption.Dummy__
var _ m_ECDH.Dummy__
var _ m_AwsCryptographyPrimitivesOperations.Dummy__
var _ m_AtomicPrimitives.Dummy__
var _ m_ComAmazonawsDynamodbTypes.Dummy__
var _ m_ComAmazonawsKmsTypes.Dummy__
var _ m_AwsCryptographyKeyStoreTypes.Dummy__
var _ m_AwsCryptographyMaterialProvidersTypes.Dummy__
var _ m_Base64.Dummy__
var _ m_AlgorithmSuites.Dummy__
var _ m_Materials.Dummy__
var _ m_Keyring.Dummy__
var _ m_Relations.Dummy__
var _ m_Seq_MergeSort.Dummy__
var _ m__Math.Dummy__
var _ m_Seq.Dummy__
var _ m_MultiKeyring.Dummy__
var _ m_AwsArnParsing.Dummy__
var _ m_AwsKmsMrkAreUnique.Dummy__
var _ m_Actions.Dummy__
var _ m_AwsKmsMrkMatchForDecrypt.Dummy__
var _ m_AwsKmsUtils.Dummy__
var _ m_Constants.Dummy__
var _ m_MaterialWrapping.Dummy__
var _ m_CanonicalEncryptionContext.Dummy__
var _ m_IntermediateKeyWrapping.Dummy__
var _ m_EdkWrapping.Dummy__
var _ m_ErrorMessages.Dummy__
var _ m_AwsKmsKeyring.Dummy__
var _ m_StrictMultiKeyring.Dummy__
var _ m_AwsKmsDiscoveryKeyring.Dummy__
var _ m_Com_Amazonaws_Kms.Dummy__
var _ m_Com_Amazonaws_Dynamodb.Dummy__
var _ m_DiscoveryMultiKeyring.Dummy__
var _ m_AwsKmsMrkDiscoveryKeyring.Dummy__
var _ m_MrkAwareDiscoveryMultiKeyring.Dummy__
var _ m_AwsKmsMrkKeyring.Dummy__
var _ m_MrkAwareStrictMultiKeyring.Dummy__
var _ m_LocalCMC.Dummy__
var _ m_SynchronizedLocalCMC.Dummy__
var _ m_StormTracker.Dummy__
var _ m_StormTrackingCMC.Dummy__
var _ m_CacheConstants.Dummy__
var _ m_AwsKmsHierarchicalKeyring.Dummy__
var _ m_AwsKmsRsaKeyring.Dummy__
var _ m_EcdhEdkWrapping.Dummy__
var _ m_RawECDHKeyring.Dummy__
var _ m_AwsKmsEcdhKeyring.Dummy__
var _ m_RawAESKeyring.Dummy__
var _ m_RawRSAKeyring.Dummy__
var _ m_CMM.Dummy__
var _ m_Defaults.Dummy__
var _ m_Commitment.Dummy__
var _ m_DefaultCMM.Dummy__
var _ m_DefaultClientSupplier.Dummy__
var _ m_Utils.Dummy__
var _ m_RequiredEncryptionContextCMM.Dummy__
var _ m_AwsCryptographyMaterialProvidersOperations.Dummy__
var _ m_MaterialProviders.Dummy__
var _ m_KeyStoreErrorMessages.Dummy__
var _ m_KmsArn.Dummy__
var _ m_Structure.Dummy__
var _ m_KMSKeystoreOperations.Dummy__
var _ m_DDBKeystoreOperations.Dummy__
var _ m_CreateKeys.Dummy__
var _ m_CreateKeyStoreTable.Dummy__
var _ m_GetKeys.Dummy__
var _ m_AwsCryptographyKeyStoreOperations.Dummy__
var _ m_KeyStore.Dummy__
var _ m__Unicode.Dummy__
var _ m_Functions.Dummy__
var _ m_Utf8EncodingForm.Dummy__
var _ m_Utf16EncodingForm.Dummy__
var _ m_UnicodeStrings.Dummy__
var _ m_FileIO.Dummy__
var _ m_GeneralInternals.Dummy__
var _ m_MulInternalsNonlinear.Dummy__
var _ m_MulInternals.Dummy__
var _ m_Mul.Dummy__
var _ m_ModInternalsNonlinear.Dummy__
var _ m_DivInternalsNonlinear.Dummy__
var _ m_ModInternals.Dummy__
var _ m_DivInternals.Dummy__
var _ m_DivMod.Dummy__
var _ m_Power.Dummy__
var _ m_Logarithm.Dummy__
var _ m_StandardLibraryInterop.Dummy__
var _ m_Streams.Dummy__
var _ m_Sorting.Dummy__
var _ m_HexStrings.Dummy__
var _ m_GetOpt.Dummy__
var _ m_FloatCompare.Dummy__
var _ m_Base64Lemmas.Dummy__
var _ m_AwsCryptographyEncryptionSdkTypes.Dummy__
var _ m_SerializableTypes.Dummy__
var _ m_SerializeFunctions.Dummy__
var _ m_EncryptionContext.Dummy__
var _ m_HeaderTypes.Dummy__
var _ m_SharedHeaderFunctions.Dummy__
var _ m_EncryptedDataKeys.Dummy__
var _ m_V1HeaderBody.Dummy__
var _ m_V2HeaderBody.Dummy__
var _ m_HeaderAuth.Dummy__
var _ m_Header.Dummy__

type Dummy__ struct{}

// Definition of class Default__
type Default__ struct {
	dummy byte
}

func New_Default___() *Default__ {
	_this := Default__{}

	return &_this
}

type CompanionStruct_Default___ struct {
}

var Companion_Default___ = CompanionStruct_Default___{}

func (_this *Default__) Equals(other *Default__) bool {
	return _this == other
}

func (_this *Default__) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*Default__)
	return ok && _this.Equals(other)
}

func (*Default__) String() string {
	return "Frames.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) WriteRegularFrame(regularFrame Frame) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(m_SerializeFunctions.Companion_Default___.WriteUint32((regularFrame).Dtor_seqNum()), m_SerializeFunctions.Companion_Default___.Write((regularFrame).Dtor_iv())), m_SerializeFunctions.Companion_Default___.Write((regularFrame).Dtor_encContent())), m_SerializeFunctions.Companion_Default___.Write((regularFrame).Dtor_authTag()))
}
func (_static *CompanionStruct_Default___) ReadRegularFrame(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_sequenceNumber m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_sequenceNumber
		var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_1_sequenceNumber).Dtor_data().(uint32)) < (Companion_Default___.ENDFRAME__SEQUENCE__NUMBER()), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Regular frame sequence number can not equal or exceed the final frame.")))
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _3_valueOrError2 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_1_sequenceNumber).Dtor_tail(), _dafny.IntOfUint8(m_SerializableTypes.Companion_Default___.GetIvLength((header).Dtor_suite())))
			_ = _3_valueOrError2
			if (_3_valueOrError2).IsFailure() {
				return (_3_valueOrError2).PropagateFailure()
			} else {
				var _4_iv m_SerializeFunctions.SuccessfulRead = (_3_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _4_iv
				var _5_valueOrError3 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_4_iv).Dtor_tail(), _dafny.IntOfUint32(((header).Dtor_body()).Dtor_frameLength()))
				_ = _5_valueOrError3
				if (_5_valueOrError3).IsFailure() {
					return (_5_valueOrError3).PropagateFailure()
				} else {
					var _6_encContent m_SerializeFunctions.SuccessfulRead = (_5_valueOrError3).Extract().(m_SerializeFunctions.SuccessfulRead)
					_ = _6_encContent
					var _7_valueOrError4 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_6_encContent).Dtor_tail(), _dafny.IntOfUint8(m_SerializableTypes.Companion_Default___.GetTagLength((header).Dtor_suite())))
					_ = _7_valueOrError4
					if (_7_valueOrError4).IsFailure() {
						return (_7_valueOrError4).PropagateFailure()
					} else {
						var _8_authTag m_SerializeFunctions.SuccessfulRead = (_7_valueOrError4).Extract().(m_SerializeFunctions.SuccessfulRead)
						_ = _8_authTag
						var _9_regularFrame Frame = Companion_Frame_.Create_RegularFrame_(header, (_1_sequenceNumber).Dtor_data().(uint32), (_4_iv).Dtor_data().(_dafny.Sequence), (_6_encContent).Dtor_data().(_dafny.Sequence), (_8_authTag).Dtor_data().(_dafny.Sequence))
						_ = _9_regularFrame
						return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_9_regularFrame, (_8_authTag).Dtor_tail()))
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) WriteFinalFrame(finalFrame Frame) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(m_SerializeFunctions.Companion_Default___.WriteUint32(Companion_Default___.ENDFRAME__SEQUENCE__NUMBER()), m_SerializeFunctions.Companion_Default___.WriteUint32((finalFrame).Dtor_seqNum())), m_SerializeFunctions.Companion_Default___.Write((finalFrame).Dtor_iv())), m_SerializeFunctions.Companion_Default___.WriteUint32Seq((finalFrame).Dtor_encContent())), m_SerializeFunctions.Companion_Default___.Write((finalFrame).Dtor_authTag()))
}
func (_static *CompanionStruct_Default___) ReadFinalFrame(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_finalFrameSignal m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_finalFrameSignal
		var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_1_finalFrameSignal).Dtor_data().(uint32)) == (Companion_Default___.ENDFRAME__SEQUENCE__NUMBER()), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Final frame sequence number MUST be the end-frame sequence number.")))
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _3_valueOrError2 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32((_1_finalFrameSignal).Dtor_tail())
			_ = _3_valueOrError2
			if (_3_valueOrError2).IsFailure() {
				return (_3_valueOrError2).PropagateFailure()
			} else {
				var _4_sequenceNumber m_SerializeFunctions.SuccessfulRead = (_3_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _4_sequenceNumber
				var _5_valueOrError3 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_4_sequenceNumber).Dtor_tail(), _dafny.IntOfUint8(m_SerializableTypes.Companion_Default___.GetIvLength((header).Dtor_suite())))
				_ = _5_valueOrError3
				if (_5_valueOrError3).IsFailure() {
					return (_5_valueOrError3).PropagateFailure()
				} else {
					var _6_iv m_SerializeFunctions.SuccessfulRead = (_5_valueOrError3).Extract().(m_SerializeFunctions.SuccessfulRead)
					_ = _6_iv
					var _7_valueOrError4 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32((_6_iv).Dtor_tail())
					_ = _7_valueOrError4
					if (_7_valueOrError4).IsFailure() {
						return (_7_valueOrError4).PropagateFailure()
					} else {
						var _8_contentLength m_SerializeFunctions.SuccessfulRead = (_7_valueOrError4).Extract().(m_SerializeFunctions.SuccessfulRead)
						_ = _8_contentLength
						var _9_valueOrError5 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_8_contentLength).Dtor_data().(uint32)) <= (((header).Dtor_body()).Dtor_frameLength()), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Content length MUST NOT exceed the frame length.")))
						_ = _9_valueOrError5
						if (_9_valueOrError5).IsFailure() {
							return (_9_valueOrError5).PropagateFailure()
						} else {
							var _10_valueOrError6 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUint32Seq((_6_iv).Dtor_tail())
							_ = _10_valueOrError6
							if (_10_valueOrError6).IsFailure() {
								return (_10_valueOrError6).PropagateFailure()
							} else {
								var _11_encContent m_SerializeFunctions.SuccessfulRead = (_10_valueOrError6).Extract().(m_SerializeFunctions.SuccessfulRead)
								_ = _11_encContent
								var _12_valueOrError7 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_11_encContent).Dtor_tail(), _dafny.IntOfUint8(m_SerializableTypes.Companion_Default___.GetTagLength((header).Dtor_suite())))
								_ = _12_valueOrError7
								if (_12_valueOrError7).IsFailure() {
									return (_12_valueOrError7).PropagateFailure()
								} else {
									var _13_authTag m_SerializeFunctions.SuccessfulRead = (_12_valueOrError7).Extract().(m_SerializeFunctions.SuccessfulRead)
									_ = _13_authTag
									var _14_finalFrame Frame = Companion_Frame_.Create_FinalFrame_(header, (_4_sequenceNumber).Dtor_data().(uint32), (_6_iv).Dtor_data().(_dafny.Sequence), (_11_encContent).Dtor_data().(_dafny.Sequence), (_13_authTag).Dtor_data().(_dafny.Sequence))
									_ = _14_finalFrame
									return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_14_finalFrame, (_13_authTag).Dtor_tail()))
								}
							}
						}
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) ReadNonFrame(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read(buffer, _dafny.IntOfUint8(m_SerializableTypes.Companion_Default___.GetIvLength((header).Dtor_suite())))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_iv m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_iv
		var _2_valueOrError1 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt64((_1_iv).Dtor_tail())
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _3_contentLength m_SerializeFunctions.SuccessfulRead = (_2_valueOrError1).Extract().(m_SerializeFunctions.SuccessfulRead)
			_ = _3_contentLength
			var _4_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_dafny.IntOfUint64((_3_contentLength).Dtor_data().(uint64))).Cmp(Companion_Default___.SAFE__MAX__ENCRYPT()) < 0, m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Frame exceeds AES-GCM cryptographic safety for a single key/iv.")))
			_ = _4_valueOrError2
			if (_4_valueOrError2).IsFailure() {
				return (_4_valueOrError2).PropagateFailure()
			} else {
				var _5_valueOrError3 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUint64Seq((_1_iv).Dtor_tail())
				_ = _5_valueOrError3
				if (_5_valueOrError3).IsFailure() {
					return (_5_valueOrError3).PropagateFailure()
				} else {
					var _6_encContent m_SerializeFunctions.SuccessfulRead = (_5_valueOrError3).Extract().(m_SerializeFunctions.SuccessfulRead)
					_ = _6_encContent
					var _7_valueOrError4 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_6_encContent).Dtor_tail(), _dafny.IntOfUint8(m_SerializableTypes.Companion_Default___.GetTagLength((header).Dtor_suite())))
					_ = _7_valueOrError4
					if (_7_valueOrError4).IsFailure() {
						return (_7_valueOrError4).PropagateFailure()
					} else {
						var _8_authTag m_SerializeFunctions.SuccessfulRead = (_7_valueOrError4).Extract().(m_SerializeFunctions.SuccessfulRead)
						_ = _8_authTag
						var _9_nonFramed Frame = Companion_Frame_.Create_NonFramed_(header, (_1_iv).Dtor_data().(_dafny.Sequence), (_6_encContent).Dtor_data().(_dafny.Sequence), (_8_authTag).Dtor_data().(_dafny.Sequence))
						_ = _9_nonFramed
						return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_9_nonFramed, (_8_authTag).Dtor_tail()))
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) ENDFRAME__SEQUENCE__NUMBER() uint32 {
	return uint32(4294967295)
}
func (_static *CompanionStruct_Default___) SAFE__MAX__ENCRYPT() _dafny.Int {
	return _dafny.IntOfInt64(68719476704)
}
func (_static *CompanionStruct_Default___) START__SEQUENCE__NUMBER() uint32 {
	return uint32(1)
}
func (_static *CompanionStruct_Default___) NONFRAMED__SEQUENCE__NUMBER() uint32 {
	return uint32(1)
}

// End of class Default__

// Definition of class FramedHeader
type FramedHeader struct {
}

func New_FramedHeader_() *FramedHeader {
	_this := FramedHeader{}

	return &_this
}

type CompanionStruct_FramedHeader_ struct {
}

var Companion_FramedHeader_ = CompanionStruct_FramedHeader_{}

func (*FramedHeader) String() string {
	return "Frames.FramedHeader"
}

// End of class FramedHeader

func Type_FramedHeader_() _dafny.TypeDescriptor {
	return type_FramedHeader_{}
}

type type_FramedHeader_ struct {
}

func (_this type_FramedHeader_) Default() interface{} {
	return m_Header.Companion_HeaderInfo_.Default()
}

func (_this type_FramedHeader_) String() string {
	return "Frames.FramedHeader"
}

// Definition of class NonFramedHeader
type NonFramedHeader struct {
}

func New_NonFramedHeader_() *NonFramedHeader {
	_this := NonFramedHeader{}

	return &_this
}

type CompanionStruct_NonFramedHeader_ struct {
}

var Companion_NonFramedHeader_ = CompanionStruct_NonFramedHeader_{}

func (*NonFramedHeader) String() string {
	return "Frames.NonFramedHeader"
}

// End of class NonFramedHeader

func Type_NonFramedHeader_() _dafny.TypeDescriptor {
	return type_NonFramedHeader_{}
}

type type_NonFramedHeader_ struct {
}

func (_this type_NonFramedHeader_) Default() interface{} {
	return m_Header.Companion_HeaderInfo_.Default()
}

func (_this type_NonFramedHeader_) String() string {
	return "Frames.NonFramedHeader"
}

// Definition of datatype Frame
type Frame struct {
	Data_Frame_
}

func (_this Frame) Get_() Data_Frame_ {
	return _this.Data_Frame_
}

type Data_Frame_ interface {
	isFrame()
}

type CompanionStruct_Frame_ struct {
}

var Companion_Frame_ = CompanionStruct_Frame_{}

type Frame_RegularFrame struct {
	Header     m_Header.HeaderInfo
	SeqNum     uint32
	Iv         _dafny.Sequence
	EncContent _dafny.Sequence
	AuthTag    _dafny.Sequence
}

func (Frame_RegularFrame) isFrame() {}

func (CompanionStruct_Frame_) Create_RegularFrame_(Header m_Header.HeaderInfo, SeqNum uint32, Iv _dafny.Sequence, EncContent _dafny.Sequence, AuthTag _dafny.Sequence) Frame {
	return Frame{Frame_RegularFrame{Header, SeqNum, Iv, EncContent, AuthTag}}
}

func (_this Frame) Is_RegularFrame() bool {
	_, ok := _this.Get_().(Frame_RegularFrame)
	return ok
}

type Frame_FinalFrame struct {
	Header     m_Header.HeaderInfo
	SeqNum     uint32
	Iv         _dafny.Sequence
	EncContent _dafny.Sequence
	AuthTag    _dafny.Sequence
}

func (Frame_FinalFrame) isFrame() {}

func (CompanionStruct_Frame_) Create_FinalFrame_(Header m_Header.HeaderInfo, SeqNum uint32, Iv _dafny.Sequence, EncContent _dafny.Sequence, AuthTag _dafny.Sequence) Frame {
	return Frame{Frame_FinalFrame{Header, SeqNum, Iv, EncContent, AuthTag}}
}

func (_this Frame) Is_FinalFrame() bool {
	_, ok := _this.Get_().(Frame_FinalFrame)
	return ok
}

type Frame_NonFramed struct {
	Header     m_Header.HeaderInfo
	Iv         _dafny.Sequence
	EncContent _dafny.Sequence
	AuthTag    _dafny.Sequence
}

func (Frame_NonFramed) isFrame() {}

func (CompanionStruct_Frame_) Create_NonFramed_(Header m_Header.HeaderInfo, Iv _dafny.Sequence, EncContent _dafny.Sequence, AuthTag _dafny.Sequence) Frame {
	return Frame{Frame_NonFramed{Header, Iv, EncContent, AuthTag}}
}

func (_this Frame) Is_NonFramed() bool {
	_, ok := _this.Get_().(Frame_NonFramed)
	return ok
}

func (CompanionStruct_Frame_) Default() Frame {
	return Companion_Frame_.Create_RegularFrame_(m_Header.Companion_HeaderInfo_.Default(), uint32(0), _dafny.EmptySeq, _dafny.EmptySeq, _dafny.EmptySeq)
}

func (_this Frame) Dtor_header() m_Header.HeaderInfo {
	switch data := _this.Get_().(type) {
	case Frame_RegularFrame:
		return data.Header
	case Frame_FinalFrame:
		return data.Header
	default:
		return data.(Frame_NonFramed).Header
	}
}

func (_this Frame) Dtor_seqNum() uint32 {
	switch data := _this.Get_().(type) {
	case Frame_RegularFrame:
		return data.SeqNum
	default:
		return data.(Frame_FinalFrame).SeqNum
	}
}

func (_this Frame) Dtor_iv() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case Frame_RegularFrame:
		return data.Iv
	case Frame_FinalFrame:
		return data.Iv
	default:
		return data.(Frame_NonFramed).Iv
	}
}

func (_this Frame) Dtor_encContent() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case Frame_RegularFrame:
		return data.EncContent
	case Frame_FinalFrame:
		return data.EncContent
	default:
		return data.(Frame_NonFramed).EncContent
	}
}

func (_this Frame) Dtor_authTag() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case Frame_RegularFrame:
		return data.AuthTag
	case Frame_FinalFrame:
		return data.AuthTag
	default:
		return data.(Frame_NonFramed).AuthTag
	}
}

func (_this Frame) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case Frame_RegularFrame:
		{
			return "Frames.Frame.RegularFrame" + "(" + _dafny.String(data.Header) + ", " + _dafny.String(data.SeqNum) + ", " + _dafny.String(data.Iv) + ", " + _dafny.String(data.EncContent) + ", " + _dafny.String(data.AuthTag) + ")"
		}
	case Frame_FinalFrame:
		{
			return "Frames.Frame.FinalFrame" + "(" + _dafny.String(data.Header) + ", " + _dafny.String(data.SeqNum) + ", " + _dafny.String(data.Iv) + ", " + _dafny.String(data.EncContent) + ", " + _dafny.String(data.AuthTag) + ")"
		}
	case Frame_NonFramed:
		{
			return "Frames.Frame.NonFramed" + "(" + _dafny.String(data.Header) + ", " + _dafny.String(data.Iv) + ", " + _dafny.String(data.EncContent) + ", " + _dafny.String(data.AuthTag) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this Frame) Equals(other Frame) bool {
	switch data1 := _this.Get_().(type) {
	case Frame_RegularFrame:
		{
			data2, ok := other.Get_().(Frame_RegularFrame)
			return ok && data1.Header.Equals(data2.Header) && data1.SeqNum == data2.SeqNum && data1.Iv.Equals(data2.Iv) && data1.EncContent.Equals(data2.EncContent) && data1.AuthTag.Equals(data2.AuthTag)
		}
	case Frame_FinalFrame:
		{
			data2, ok := other.Get_().(Frame_FinalFrame)
			return ok && data1.Header.Equals(data2.Header) && data1.SeqNum == data2.SeqNum && data1.Iv.Equals(data2.Iv) && data1.EncContent.Equals(data2.EncContent) && data1.AuthTag.Equals(data2.AuthTag)
		}
	case Frame_NonFramed:
		{
			data2, ok := other.Get_().(Frame_NonFramed)
			return ok && data1.Header.Equals(data2.Header) && data1.Iv.Equals(data2.Iv) && data1.EncContent.Equals(data2.EncContent) && data1.AuthTag.Equals(data2.AuthTag)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this Frame) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(Frame)
	return ok && _this.Equals(typed)
}

func Type_Frame_() _dafny.TypeDescriptor {
	return type_Frame_{}
}

type type_Frame_ struct {
}

func (_this type_Frame_) Default() interface{} {
	return Companion_Frame_.Default()
}

func (_this type_Frame_) String() string {
	return "Frames.Frame"
}
func (_this Frame) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = Frame{}

// End of datatype Frame

// Definition of class RegularFrame
type RegularFrame struct {
}

func New_RegularFrame_() *RegularFrame {
	_this := RegularFrame{}

	return &_this
}

type CompanionStruct_RegularFrame_ struct {
}

var Companion_RegularFrame_ = CompanionStruct_RegularFrame_{}

func (*RegularFrame) String() string {
	return "Frames.RegularFrame"
}

// End of class RegularFrame

func Type_RegularFrame_() _dafny.TypeDescriptor {
	return type_RegularFrame_{}
}

type type_RegularFrame_ struct {
}

func (_this type_RegularFrame_) Default() interface{} {
	return Companion_Frame_.Default()
}

func (_this type_RegularFrame_) String() string {
	return "Frames.RegularFrame"
}

// Definition of class FinalFrame
type FinalFrame struct {
}

func New_FinalFrame_() *FinalFrame {
	_this := FinalFrame{}

	return &_this
}

type CompanionStruct_FinalFrame_ struct {
}

var Companion_FinalFrame_ = CompanionStruct_FinalFrame_{}

func (*FinalFrame) String() string {
	return "Frames.FinalFrame"
}

// End of class FinalFrame

func Type_FinalFrame_() _dafny.TypeDescriptor {
	return type_FinalFrame_{}
}

type type_FinalFrame_ struct {
}

func (_this type_FinalFrame_) Default() interface{} {
	return Companion_Frame_.Default()
}

func (_this type_FinalFrame_) String() string {
	return "Frames.FinalFrame"
}

// Definition of class NonFramed
type NonFramed struct {
}

func New_NonFramed_() *NonFramed {
	_this := NonFramed{}

	return &_this
}

type CompanionStruct_NonFramed_ struct {
}

var Companion_NonFramed_ = CompanionStruct_NonFramed_{}

func (*NonFramed) String() string {
	return "Frames.NonFramed"
}

// End of class NonFramed

func Type_NonFramed_() _dafny.TypeDescriptor {
	return type_NonFramed_{}
}

type type_NonFramed_ struct {
}

func (_this type_NonFramed_) Default() interface{} {
	return Companion_Frame_.Default()
}

func (_this type_NonFramed_) String() string {
	return "Frames.NonFramed"
}
