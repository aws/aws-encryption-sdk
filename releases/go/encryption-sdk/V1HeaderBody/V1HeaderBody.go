// Package V1HeaderBody
// Dafny module V1HeaderBody compiled into Go

package V1HeaderBody

import (
	os "os"

	m_ComAmazonawsDynamodbTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb/ComAmazonawsDynamodbTypes"
	m_Com_Amazonaws_Dynamodb "github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb/Com_Amazonaws_Dynamodb"
	m_ComAmazonawsKmsTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms/ComAmazonawsKmsTypes"
	m_Com_Amazonaws_Kms "github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms/Com_Amazonaws_Kms"
	m_AlgorithmSuites "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AlgorithmSuites"
	m_AwsArnParsing "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsArnParsing"
	m_AwsCryptographyKeyStoreOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyKeyStoreOperations"
	m_AwsCryptographyKeyStoreTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyKeyStoreTypes"
	m_AwsCryptographyMaterialProvidersOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyMaterialProvidersOperations"
	m_AwsCryptographyMaterialProvidersTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyMaterialProvidersTypes"
	m_AwsKmsDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsDiscoveryKeyring"
	m_AwsKmsEcdhKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsEcdhKeyring"
	m_AwsKmsHierarchicalKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsHierarchicalKeyring"
	m_AwsKmsKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsKeyring"
	m_AwsKmsMrkAreUnique "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkAreUnique"
	m_AwsKmsMrkDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkDiscoveryKeyring"
	m_AwsKmsMrkKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkKeyring"
	m_AwsKmsMrkMatchForDecrypt "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkMatchForDecrypt"
	m_AwsKmsRsaKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsRsaKeyring"
	m_AwsKmsUtils "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsUtils"
	m_CMM "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CMM"
	m_CacheConstants "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CacheConstants"
	m_CanonicalEncryptionContext "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CanonicalEncryptionContext"
	m_Commitment "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Commitment"
	m_Constants "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Constants"
	m_CreateKeyStoreTable "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CreateKeyStoreTable"
	m_CreateKeys "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CreateKeys"
	m_DDBKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DDBKeystoreOperations"
	m_DefaultCMM "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DefaultCMM"
	m_DefaultClientSupplier "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DefaultClientSupplier"
	m_Defaults "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Defaults"
	m_DiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DiscoveryMultiKeyring"
	m_EcdhEdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/EcdhEdkWrapping"
	m_EdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/EdkWrapping"
	m_ErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/ErrorMessages"
	m_GetKeys "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/GetKeys"
	m_IntermediateKeyWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/IntermediateKeyWrapping"
	m_KMSKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KMSKeystoreOperations"
	m_KeyStore "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KeyStore"
	m_KeyStoreErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KeyStoreErrorMessages"
	m_Keyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Keyring"
	m_KmsArn "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KmsArn"
	m_LocalCMC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/LocalCMC"
	m_MaterialProviders "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MaterialProviders"
	m_MaterialWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MaterialWrapping"
	m_Materials "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Materials"
	m_MrkAwareDiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MrkAwareDiscoveryMultiKeyring"
	m_MrkAwareStrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MrkAwareStrictMultiKeyring"
	m_MultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MultiKeyring"
	m_RawAESKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RawAESKeyring"
	m_RawECDHKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RawECDHKeyring"
	m_RawRSAKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RawRSAKeyring"
	m_RequiredEncryptionContextCMM "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RequiredEncryptionContextCMM"
	m_StormTracker "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/StormTracker"
	m_StormTrackingCMC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/StormTrackingCMC"
	m_StrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/StrictMultiKeyring"
	m_Structure "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Structure"
	m_SynchronizedLocalCMC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/SynchronizedLocalCMC"
	m_Utils "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Utils"
	m_AtomicPrimitives "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/AtomicPrimitives"
	m_AwsCryptographyPrimitivesOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/AwsCryptographyPrimitivesOperations"
	m_AwsCryptographyPrimitivesTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/AwsCryptographyPrimitivesTypes"
	m_Digest "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/Digest"
	m_HKDF "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/HKDF"
	m_KdfCtr "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/KdfCtr"
	m_Random "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/Random"
	m_WrappedHKDF "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/WrappedHKDF"
	m_WrappedHMAC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/WrappedHMAC"
	m_Actions "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Actions"
	m_Base64 "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Base64"
	m_Base64Lemmas "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Base64Lemmas"
	m_BoundedInts "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/BoundedInts"
	m_DivInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/DivInternals"
	m_DivInternalsNonlinear "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/DivInternalsNonlinear"
	m_DivMod "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/DivMod"
	m_FileIO "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/FileIO"
	m_FloatCompare "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/FloatCompare"
	m_Functions "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Functions"
	m_GeneralInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/GeneralInternals"
	m_GetOpt "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/GetOpt"
	m_HexStrings "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/HexStrings"
	m_Logarithm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Logarithm"
	m__Math "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Math_"
	m_ModInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/ModInternals"
	m_ModInternalsNonlinear "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/ModInternalsNonlinear"
	m_Mul "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Mul"
	m_MulInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/MulInternals"
	m_MulInternalsNonlinear "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/MulInternalsNonlinear"
	m_Power "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Power"
	m_Relations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Relations"
	m_Seq "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Seq"
	m_Seq_MergeSort "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Seq_MergeSort"
	m_Sorting "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Sorting"
	m_StandardLibrary "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary"
	m_StandardLibraryInterop "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibraryInterop"
	m_StandardLibrary_MemoryMath "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_MemoryMath"
	m_StandardLibrary_Sequence "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_Sequence"
	m_StandardLibrary_String "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_String"
	m_StandardLibrary_UInt "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_UInt"
	m_Streams "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Streams"
	m_UnicodeStrings "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/UnicodeStrings"
	m__Unicode "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Unicode_"
	m_Utf16EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf16EncodingForm"
	m_Utf8EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf8EncodingForm"
	m_Wrappers "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	m_AwsCryptographyEncryptionSdkTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	m_EncryptedDataKeys "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptedDataKeys"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptionContext"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderTypes"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SharedHeaderFunctions"
	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_BoundedInts.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary_MemoryMath.Dummy__
var _ m_StandardLibrary_Sequence.Dummy__
var _ m_StandardLibrary_String.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_AwsCryptographyPrimitivesTypes.Dummy__
var _ m_Random.Dummy__
var _ m_Digest.Dummy__
var _ m_WrappedHMAC.Dummy__
var _ m_HKDF.Dummy__
var _ m_WrappedHKDF.Dummy__
var _ m_KdfCtr.Dummy__
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
var _ m_Actions.Dummy__
var _ m_CanonicalEncryptionContext.Dummy__
var _ m_MaterialWrapping.Dummy__
var _ m_IntermediateKeyWrapping.Dummy__
var _ m_EdkWrapping.Dummy__
var _ m_ErrorMessages.Dummy__
var _ m_RawAESKeyring.Dummy__
var _ m_AwsArnParsing.Dummy__
var _ m_Constants.Dummy__
var _ m_EcdhEdkWrapping.Dummy__
var _ m_RawECDHKeyring.Dummy__
var _ m_RawRSAKeyring.Dummy__
var _ m_AwsKmsMrkMatchForDecrypt.Dummy__
var _ m_AwsKmsUtils.Dummy__
var _ m_AwsKmsKeyring.Dummy__
var _ m_AwsKmsDiscoveryKeyring.Dummy__
var _ m_AwsKmsEcdhKeyring.Dummy__
var _ m_FileIO.Dummy__
var _ m_LocalCMC.Dummy__
var _ m_SynchronizedLocalCMC.Dummy__
var _ m_StormTracker.Dummy__
var _ m_StormTrackingCMC.Dummy__
var _ m_CacheConstants.Dummy__
var _ m_AwsKmsHierarchicalKeyring.Dummy__
var _ m_AwsKmsMrkDiscoveryKeyring.Dummy__
var _ m_AwsKmsMrkKeyring.Dummy__
var _ m_AwsKmsRsaKeyring.Dummy__
var _ m_MultiKeyring.Dummy__
var _ m_AwsKmsMrkAreUnique.Dummy__
var _ m_StrictMultiKeyring.Dummy__
var _ m_Com_Amazonaws_Kms.Dummy__
var _ m_Com_Amazonaws_Dynamodb.Dummy__
var _ m_DiscoveryMultiKeyring.Dummy__
var _ m_MrkAwareDiscoveryMultiKeyring.Dummy__
var _ m_MrkAwareStrictMultiKeyring.Dummy__
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
	return "V1HeaderBody.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) WriteV1HeaderBody(body m_HeaderTypes.HeaderBody) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(m_SharedHeaderFunctions.Companion_Default___.WriteMessageFormatVersion(m_HeaderTypes.Companion_MessageFormatVersion_.Create_V1_()), Companion_Default___.WriteV1MessageType((body).Dtor_messageType())), m_SharedHeaderFunctions.Companion_Default___.WriteESDKSuiteId((body).Dtor_algorithmSuite())), m_SharedHeaderFunctions.Companion_Default___.WriteMessageId((body).Dtor_messageId())), m_EncryptionContext.Companion_Default___.WriteAADSection((body).Dtor_encryptionContext())), m_EncryptedDataKeys.Companion_Default___.WriteEncryptedDataKeysSection((body).Dtor_encryptedDataKeys())), m_SharedHeaderFunctions.Companion_Default___.WriteContentType((body).Dtor_contentType())), Companion_Default___.WriteV1ReservedBytes(Companion_Default___.RESERVED__BYTES())), Companion_Default___.WriteV1HeaderIvLength(m_SerializableTypes.Companion_Default___.GetIvLength((body).Dtor_algorithmSuite()))), m_SerializeFunctions.Companion_Default___.WriteUint32((body).Dtor_frameLength()))
}
func (_static *CompanionStruct_Default___) ReadV1HeaderBody(buffer m_SerializeFunctions.ReadableBuffer, maxEdks m_Wrappers.Option, mpl *m_MaterialProviders.MaterialProvidersClient) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadMessageFormatVersion(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_version m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_version
		var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_1_version).Dtor_data().(m_HeaderTypes.MessageFormatVersion)).Is_V1(), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Message version must be version 1.")))
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _3_valueOrError2 m_Wrappers.Result = Companion_Default___.ReadV1MessageType((_1_version).Dtor_tail())
			_ = _3_valueOrError2
			if (_3_valueOrError2).IsFailure() {
				return (_3_valueOrError2).PropagateFailure()
			} else {
				var _4_messageType m_SerializeFunctions.SuccessfulRead = (_3_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _4_messageType
				var _5_valueOrError3 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadESDKSuiteId((_4_messageType).Dtor_tail(), mpl)
				_ = _5_valueOrError3
				if (_5_valueOrError3).IsFailure() {
					return (_5_valueOrError3).PropagateFailure()
				} else {
					var _6_suite m_SerializeFunctions.SuccessfulRead = (_5_valueOrError3).Extract().(m_SerializeFunctions.SuccessfulRead)
					_ = _6_suite
					var _7_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((((_6_suite).Dtor_data().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)).Dtor_commitment()).Is_None(), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Algorithm suite must not support commitment.")))
					_ = _7_valueOrError4
					if (_7_valueOrError4).IsFailure() {
						return (_7_valueOrError4).PropagateFailure()
					} else {
						var _8_valueOrError5 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadMessageIdV1((_6_suite).Dtor_tail())
						_ = _8_valueOrError5
						if (_8_valueOrError5).IsFailure() {
							return (_8_valueOrError5).PropagateFailure()
						} else {
							var _9_messageId m_SerializeFunctions.SuccessfulRead = (_8_valueOrError5).Extract().(m_SerializeFunctions.SuccessfulRead)
							_ = _9_messageId
							var _10_valueOrError6 m_Wrappers.Result = m_EncryptionContext.Companion_Default___.ReadAADSection((_9_messageId).Dtor_tail())
							_ = _10_valueOrError6
							if (_10_valueOrError6).IsFailure() {
								return (_10_valueOrError6).PropagateFailure()
							} else {
								var _11_encryptionContext m_SerializeFunctions.SuccessfulRead = (_10_valueOrError6).Extract().(m_SerializeFunctions.SuccessfulRead)
								_ = _11_encryptionContext
								var _12_valueOrError7 m_Wrappers.Result = m_EncryptedDataKeys.Companion_Default___.ReadEncryptedDataKeysSection((_11_encryptionContext).Dtor_tail(), maxEdks)
								_ = _12_valueOrError7
								if (_12_valueOrError7).IsFailure() {
									return (_12_valueOrError7).PropagateFailure()
								} else {
									var _13_encryptedDataKeys m_SerializeFunctions.SuccessfulRead = (_12_valueOrError7).Extract().(m_SerializeFunctions.SuccessfulRead)
									_ = _13_encryptedDataKeys
									var _14_valueOrError8 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadContentType((_13_encryptedDataKeys).Dtor_tail())
									_ = _14_valueOrError8
									if (_14_valueOrError8).IsFailure() {
										return (_14_valueOrError8).PropagateFailure()
									} else {
										var _15_contentType m_SerializeFunctions.SuccessfulRead = (_14_valueOrError8).Extract().(m_SerializeFunctions.SuccessfulRead)
										_ = _15_contentType
										var _16_valueOrError9 m_Wrappers.Result = Companion_Default___.ReadV1ReservedBytes((_15_contentType).Dtor_tail())
										_ = _16_valueOrError9
										if (_16_valueOrError9).IsFailure() {
											return (_16_valueOrError9).PropagateFailure()
										} else {
											var _17_reservedBytes m_SerializeFunctions.SuccessfulRead = (_16_valueOrError9).Extract().(m_SerializeFunctions.SuccessfulRead)
											_ = _17_reservedBytes
											var _18_valueOrError10 m_Wrappers.Result = Companion_Default___.ReadV1HeaderIvLength((_17_reservedBytes).Dtor_tail(), (_6_suite).Dtor_data().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo))
											_ = _18_valueOrError10
											if (_18_valueOrError10).IsFailure() {
												return (_18_valueOrError10).PropagateFailure()
											} else {
												var _19_headerIvLength m_SerializeFunctions.SuccessfulRead = (_18_valueOrError10).Extract().(m_SerializeFunctions.SuccessfulRead)
												_ = _19_headerIvLength
												var _20_valueOrError11 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32((_19_headerIvLength).Dtor_tail())
												_ = _20_valueOrError11
												if (_20_valueOrError11).IsFailure() {
													return (_20_valueOrError11).PropagateFailure()
												} else {
													var _21_frameLength m_SerializeFunctions.SuccessfulRead = (_20_valueOrError11).Extract().(m_SerializeFunctions.SuccessfulRead)
													_ = _21_frameLength
													var _22_body m_HeaderTypes.HeaderBody = m_HeaderTypes.Companion_HeaderBody_.Create_V1HeaderBody_((_4_messageType).Dtor_data().(m_HeaderTypes.MessageType), (_6_suite).Dtor_data().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo), (_9_messageId).Dtor_data().(_dafny.Sequence), (_11_encryptionContext).Dtor_data().(_dafny.Sequence), (_13_encryptedDataKeys).Dtor_data().(_dafny.Sequence), (_15_contentType).Dtor_data().(m_HeaderTypes.ContentType), uint64((_19_headerIvLength).Dtor_data().(uint8)), (_21_frameLength).Dtor_data().(uint32))
													_ = _22_body
													return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_22_body, (_21_frameLength).Dtor_tail()))
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) WriteV1MessageType(messageType m_HeaderTypes.MessageType) _dafny.Sequence {
	return _dafny.SeqOf((messageType).Serialize())
}
func (_static *CompanionStruct_Default___) ReadV1MessageType(buffer m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read(buffer, uint64(1))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _let_tmp_rhs0 m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _let_tmp_rhs0
		var _1_raw _dafny.Sequence = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(_dafny.Sequence)
		_ = _1_raw
		var _2_tail m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
		_ = _2_tail
		var _3_valueOrError1 m_Wrappers.Result = (m_HeaderTypes.Companion_MessageType_.Get((_1_raw).Select(uint32(uint32(0))).(uint8))).MapFailure(func(coer6 func(_dafny.Sequence) m_SerializeFunctions.ReadProblems) func(interface{}) interface{} {
			return func(arg7 interface{}) interface{} {
				return coer6(arg7.(_dafny.Sequence))
			}
		}(func(_4_e _dafny.Sequence) m_SerializeFunctions.ReadProblems {
			return m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_4_e)
		}))
		_ = _3_valueOrError1
		if (_3_valueOrError1).IsFailure() {
			return (_3_valueOrError1).PropagateFailure()
		} else {
			var _5_messageType m_HeaderTypes.MessageType = (_3_valueOrError1).Extract().(m_HeaderTypes.MessageType)
			_ = _5_messageType
			return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_5_messageType, _2_tail))
		}
	}
}
func (_static *CompanionStruct_Default___) WriteV1ReservedBytes(reservedBytes _dafny.Sequence) _dafny.Sequence {
	return reservedBytes
}
func (_static *CompanionStruct_Default___) ReadV1ReservedBytes(buffer m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read(buffer, uint64((Companion_Default___.RESERVED__BYTES()).Cardinality()))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _let_tmp_rhs0 m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _let_tmp_rhs0
		var _1_raw _dafny.Sequence = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(_dafny.Sequence)
		_ = _1_raw
		var _2_tail m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
		_ = _2_tail
		var _3_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(_dafny.Companion_Sequence_.Equal(_1_raw, Companion_Default___.RESERVED__BYTES()), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Incorrect reserved bytes.")))
		_ = _3_valueOrError1
		if (_3_valueOrError1).IsFailure() {
			return (_3_valueOrError1).PropagateFailure()
		} else {
			var _4_reservedBytes _dafny.Sequence = _1_raw
			_ = _4_reservedBytes
			return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_4_reservedBytes, _2_tail))
		}
	}
}
func (_static *CompanionStruct_Default___) WriteV1HeaderIvLength(ivLength uint8) _dafny.Sequence {
	return _dafny.SeqOf(ivLength)
}
func (_static *CompanionStruct_Default___) ReadV1HeaderIvLength(buffer m_SerializeFunctions.ReadableBuffer, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read(buffer, uint64(1))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _let_tmp_rhs0 m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _let_tmp_rhs0
		var _1_raw _dafny.Sequence = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(_dafny.Sequence)
		_ = _1_raw
		var _2_tail m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
		_ = _2_tail
		var _3_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_1_raw).Select(uint32(uint32(0))).(uint8)) == (m_SerializableTypes.Companion_Default___.GetIvLength(suite)), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("HeaderIv Length does not match Algorithm Suite.")))
		_ = _3_valueOrError1
		if (_3_valueOrError1).IsFailure() {
			return (_3_valueOrError1).PropagateFailure()
		} else {
			return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_((_1_raw).Select(uint32(uint32(0))).(uint8), _2_tail))
		}
	}
}
func (_static *CompanionStruct_Default___) RESERVED__BYTES() _dafny.Sequence {
	return _dafny.SeqOf(uint8(0), uint8(0), uint8(0), uint8(0))
}

// End of class Default__

// Definition of class V1HeaderBody
type V1HeaderBody struct {
}

func New_V1HeaderBody_() *V1HeaderBody {
	_this := V1HeaderBody{}

	return &_this
}

type CompanionStruct_V1HeaderBody_ struct {
}

var Companion_V1HeaderBody_ = CompanionStruct_V1HeaderBody_{}

func (*V1HeaderBody) String() string {
	return "V1HeaderBody.V1HeaderBody"
}

// End of class V1HeaderBody

func Type_V1HeaderBody_() _dafny.TypeDescriptor {
	return type_V1HeaderBody_{}
}

type type_V1HeaderBody_ struct {
}

func (_this type_V1HeaderBody_) Default() interface{} {
	return m_HeaderTypes.Companion_HeaderBody_.Default()
}

func (_this type_V1HeaderBody_) String() string {
	return "V1HeaderBody.V1HeaderBody"
}
func (_this *CompanionStruct_V1HeaderBody_) Is_(__source m_HeaderTypes.HeaderBody) bool {
	var _0_h m_HeaderTypes.HeaderBody = (__source)
	_ = _0_h
	return (_0_h).Is_V1HeaderBody()
}

// Definition of class ReservedBytes
type ReservedBytes struct {
}

func New_ReservedBytes_() *ReservedBytes {
	_this := ReservedBytes{}

	return &_this
}

type CompanionStruct_ReservedBytes_ struct {
}

var Companion_ReservedBytes_ = CompanionStruct_ReservedBytes_{}

func (*ReservedBytes) String() string {
	return "V1HeaderBody.ReservedBytes"
}
func (_this *CompanionStruct_ReservedBytes_) Witness() _dafny.Sequence {
	return Companion_Default___.RESERVED__BYTES()
}

// End of class ReservedBytes

func Type_ReservedBytes_() _dafny.TypeDescriptor {
	return type_ReservedBytes_{}
}

type type_ReservedBytes_ struct {
}

func (_this type_ReservedBytes_) Default() interface{} {
	return Companion_ReservedBytes_.Witness()
}

func (_this type_ReservedBytes_) String() string {
	return "V1HeaderBody.ReservedBytes"
}
func (_this *CompanionStruct_ReservedBytes_) Is_(__source _dafny.Sequence) bool {
	var _1_s _dafny.Sequence = (__source)
	_ = _1_s
	return _dafny.Companion_Sequence_.Equal(_1_s, Companion_Default___.RESERVED__BYTES())
}
