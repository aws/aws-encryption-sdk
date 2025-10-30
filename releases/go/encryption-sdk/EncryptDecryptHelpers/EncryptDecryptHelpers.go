// Package EncryptDecryptHelpers
// Dafny module EncryptDecryptHelpers compiled into Go

package EncryptDecryptHelpers

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
	m_UTF8 "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/UTF8"
	m_UnicodeStrings "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/UnicodeStrings"
	m__Unicode "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Unicode_"
	m_Utf16EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf16EncodingForm"
	m_Utf8EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf8EncodingForm"
	m_Wrappers "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	m_AwsCryptographyEncryptionSdkTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	m_EncryptedDataKeys "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptedDataKeys"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptionContext"
	m_Frames "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/Frames"
	m_Header "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/Header"
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderTypes"
	m_KeyDerivation "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/KeyDerivation"
	m_MessageBody "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/MessageBody"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SharedHeaderFunctions"
	m_V1HeaderBody "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/V1HeaderBody"
	m_V2HeaderBody "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/V2HeaderBody"
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
var _ m_V1HeaderBody.Dummy__
var _ m_V2HeaderBody.Dummy__
var _ m_HeaderAuth.Dummy__
var _ m_Header.Dummy__
var _ m_Frames.Dummy__
var _ m_MessageBody.Dummy__
var _ m_KeyDerivation.Dummy__

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
	return "EncryptDecryptHelpers.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) SerializeMessageWithSignature(framedMessage m_MessageBody.FramedMessageBody, signature _dafny.Sequence, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) m_Wrappers.Result {
	var _0_serializedSignature _dafny.Sequence = m_SerializeFunctions.Companion_Default___.WriteShortLengthSeq(signature)
	_ = _0_serializedSignature
	var _1_valueOrError0 m_Wrappers.Result = Companion_Default___.SerializeMessageWithoutSignature(framedMessage, suite)
	_ = _1_valueOrError0
	if (_1_valueOrError0).IsFailure() {
		return (_1_valueOrError0).PropagateFailure()
	} else {
		var _2_serializedMessage _dafny.Sequence = (_1_valueOrError0).Extract().(_dafny.Sequence)
		_ = _2_serializedMessage
		return m_Wrappers.Companion_Result_.Create_Success_(_dafny.Companion_Sequence_.Concatenate(_2_serializedMessage, _0_serializedSignature))
	}
}
func (_static *CompanionStruct_Default___) SerializeMessageWithoutSignature(framedMessage m_MessageBody.FramedMessageBody, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_HeaderAuth.Companion_Default___.WriteHeaderAuthTag((((framedMessage).Dtor_finalFrame()).Dtor_header()).Dtor_headerAuth(), suite)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_headerAuth _dafny.Sequence = (_0_valueOrError0).Extract().(_dafny.Sequence)
		_ = _1_headerAuth
		return m_Wrappers.Companion_Result_.Create_Success_(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate((((framedMessage).Dtor_finalFrame()).Dtor_header()).Dtor_rawHeader(), _1_headerAuth), m_MessageBody.Companion_Default___.WriteFramedMessageBody(framedMessage)))
	}
}
func (_static *CompanionStruct_Default___) VerifySignature(buffer m_SerializeFunctions.ReadableBuffer, msg _dafny.Sequence, decMat m_AwsCryptographyMaterialProvidersTypes.DecryptionMaterials, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_SerializeFunctions.Companion_ReadableBuffer_.Default())
	_ = res
	if ((decMat).Dtor_verificationKey()).Is_None() {
		res = m_Wrappers.Companion_Result_.Create_Success_(buffer)
		return res
	}
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_SerializeFunctions.Companion_SuccessfulRead_.Default(_dafny.EmptySeq))
	_ = _0_valueOrError0
	_0_valueOrError0 = (m_SerializeFunctions.Companion_Default___.ReadShortLengthSeq(buffer)).MapFailure(func(coer15 func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg16 interface{}) interface{} {
			return coer15(arg16.(m_SerializeFunctions.ReadProblems))
		}
	}(Companion_Default___.MapSerializeFailure(_dafny.SeqOfString(": ReadShortLengthSeq"))))
	if (_0_valueOrError0).IsFailure() {
		res = (_0_valueOrError0).PropagateFailure()
		return res
	}
	var _1_signature m_SerializeFunctions.SuccessfulRead
	_ = _1_signature
	_1_signature = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
	var _2_ecdsaParams m_AwsCryptographyPrimitivesTypes.ECDSASignatureAlgorithm
	_ = _2_ecdsaParams
	_2_ecdsaParams = ((((decMat).Dtor_algorithmSuite()).Dtor_signature()).Dtor_ECDSA()).Dtor_curve()
	var _3_maybeSignatureVerifiedResult m_Wrappers.Result
	_ = _3_maybeSignatureVerifiedResult
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (crypto).ECDSAVerify(m_AwsCryptographyPrimitivesTypes.Companion_ECDSAVerifyInput_.Create_ECDSAVerifyInput_(_2_ecdsaParams, ((decMat).Dtor_verificationKey()).Dtor_value().(_dafny.Sequence), msg, (_1_signature).Dtor_data().(_dafny.Sequence)))
	_3_maybeSignatureVerifiedResult = _out0
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(false)
	_ = _4_valueOrError1
	_4_valueOrError1 = (_3_maybeSignatureVerifiedResult).MapFailure(func(coer16 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg17 interface{}) interface{} {
			return coer16(arg17.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_5_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_5_e)
	}))
	if (_4_valueOrError1).IsFailure() {
		res = (_4_valueOrError1).PropagateFailure()
		return res
	}
	var _6_signatureVerifiedResult bool
	_ = _6_signatureVerifiedResult
	_6_signatureVerifiedResult = (_4_valueOrError1).Extract().(bool)
	if !(_6_signatureVerifiedResult) {
		res = m_Wrappers.Companion_Result_.Create_Failure_(m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Invalid signature")))
		return res
	}
	res = m_Wrappers.Companion_Result_.Create_Success_((_1_signature).Dtor_tail())
	return res
	return res
}
func (_static *CompanionStruct_Default___) MapSerializeFailure(s _dafny.Sequence) func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error {
	return (func(_0_s _dafny.Sequence) func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error {
		return func(_1_e m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error {
			return func() m_AwsCryptographyEncryptionSdkTypes.Error {
				var _source0 m_SerializeFunctions.ReadProblems = _1_e
				_ = _source0
				{
					if _source0.Is_Error() {
						var _2_e _dafny.Sequence = _source0.Get_().(m_SerializeFunctions.ReadProblems_Error).Message
						_ = _2_e
						return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_2_e)
					}
				}
				{
					return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.Companion_Sequence_.Concatenate(_dafny.SeqOfString("Incomplete message"), _0_s))
				}
			}()
		}
	})(s)
}
func (_static *CompanionStruct_Default___) ValidateEncryptionContext(input m_Wrappers.Option) m_Wrappers.Outcome {
	if ((input).Is_Some()) && (_dafny.Quantifier((((input).Dtor_value().(_dafny.Map)).Keys()).Elements(), false, func(_exists_var_0 _dafny.Sequence) bool {
		var _0_key _dafny.Sequence
		_0_key = interface{}(_exists_var_0).(_dafny.Sequence)
		if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_0_key) {
			return ((((input).Dtor_value().(_dafny.Map)).Keys()).Contains(_0_key)) && (_dafny.Companion_Sequence_.IsPrefixOf(Companion_Default___.RESERVED__ENCRYPTION__CONTEXT(), _0_key))
		} else {
			return false
		}
	})) {
		return m_Wrappers.Companion_Outcome_.Create_Fail_(m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Encryption context keys cannot contain reserved prefix 'aws-crypto-'")))
	} else {
		return m_Wrappers.Companion_Outcome_.Create_Pass_()
	}
}
func (_static *CompanionStruct_Default___) CreateCmmFromInput(inputCmm m_Wrappers.Option, inputKeyring m_Wrappers.Option) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _0_valueOrError0
	_0_valueOrError0 = m_Wrappers.Companion_Default___.Need(((inputCmm).Is_None()) || ((inputKeyring).Is_None()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Cannot provide both a keyring and a CMM")))
	if (_0_valueOrError0).IsFailure() {
		res = (_0_valueOrError0).PropagateFailure()
		return res
	}
	var _1_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _1_valueOrError1
	_1_valueOrError1 = m_Wrappers.Companion_Default___.Need(((inputCmm).Is_Some()) || ((inputKeyring).Is_Some()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Must provide either a keyring or a CMM")))
	if (_1_valueOrError1).IsFailure() {
		res = (_1_valueOrError1).PropagateFailure()
		return res
	}
	var _2_cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager = (m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager)(nil)
	_ = _2_cmm
	if (inputCmm).Is_Some() {
		res = m_Wrappers.Companion_Result_.Create_Success_(m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((inputCmm).Dtor_value()))
		return res
	} else {
		var _3_maybeMaterialsProviders m_Wrappers.Result
		_ = _3_maybeMaterialsProviders
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
		_3_maybeMaterialsProviders = _out0
		var _4_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
		_ = _4_valueOrError2
		_4_valueOrError2 = (_3_maybeMaterialsProviders).MapFailure(func(coer17 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
			return func(arg18 interface{}) interface{} {
				return coer17(arg18.(m_AwsCryptographyMaterialProvidersTypes.Error))
			}
		}(func(_5_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
			return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_5_e)
		}))
		if (_4_valueOrError2).IsFailure() {
			res = (_4_valueOrError2).PropagateFailure()
			return res
		}
		var _6_materialProviders *m_MaterialProviders.MaterialProvidersClient
		_ = _6_materialProviders
		_6_materialProviders = (_4_valueOrError2).Extract().(*m_MaterialProviders.MaterialProvidersClient)
		var _7_maybeCmm m_Wrappers.Result
		_ = _7_maybeCmm
		var _out1 m_Wrappers.Result
		_ = _out1
		_out1 = (_6_materialProviders).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((inputKeyring).Dtor_value())))
		_7_maybeCmm = _out1
		res = (_7_maybeCmm).MapFailure(func(coer18 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
			return func(arg19 interface{}) interface{} {
				return coer18(arg19.(m_AwsCryptographyMaterialProvidersTypes.Error))
			}
		}(func(_8_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
			return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_8_e)
		}))
		return res
	}
	return res
}
func (_static *CompanionStruct_Default___) ValidateMaxEncryptedDataKeys(maxEncryptedDataKeys m_Wrappers.Option, edks _dafny.Sequence) m_Wrappers.Outcome {
	if ((maxEncryptedDataKeys).Is_Some()) && ((uint64((edks).Cardinality())) > (uint64((maxEncryptedDataKeys).Dtor_value().(int64)))) {
		return m_Wrappers.Companion_Outcome_.Create_Fail_(m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Encrypted data keys exceed maxEncryptedDataKeys")))
	} else {
		return m_Wrappers.Companion_Outcome_.Create_Pass_()
	}
}
func (_static *CompanionStruct_Default___) GenerateMessageId(suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_maybeId m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _0_maybeId
	if ((suite).Dtor_messageVersion()) == (int32(1)) {
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = (crypto).GenerateRandomBytes(m_AwsCryptographyPrimitivesTypes.Companion_GenerateRandomBytesInput_.Create_GenerateRandomBytesInput_(int32(m_HeaderTypes.Companion_Default___.MESSAGE__ID__LEN__V1())))
		_0_maybeId = _out0
	} else {
		var _out1 m_Wrappers.Result
		_ = _out1
		_out1 = (crypto).GenerateRandomBytes(m_AwsCryptographyPrimitivesTypes.Companion_GenerateRandomBytesInput_.Create_GenerateRandomBytesInput_(int32(m_HeaderTypes.Companion_Default___.MESSAGE__ID__LEN__V2())))
		_0_maybeId = _out1
	}
	var _1_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _1_valueOrError0
	_1_valueOrError0 = (_0_maybeId).MapFailure(func(coer19 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg20 interface{}) interface{} {
			return coer19(arg20.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_2_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_2_e)
	}))
	if (_1_valueOrError0).IsFailure() {
		res = (_1_valueOrError0).PropagateFailure()
		return res
	}
	var _3_id _dafny.Sequence
	_ = _3_id
	_3_id = (_1_valueOrError0).Extract().(_dafny.Sequence)
	res = m_Wrappers.Companion_Result_.Create_Success_(_3_id)
	return res
	return res
}
func (_static *CompanionStruct_Default___) BuildHeaderForEncrypt(messageId _dafny.Sequence, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, encryptionContext _dafny.Map, requiredEncryptionContextKeys _dafny.Sequence, encryptedDataKeys _dafny.Sequence, frameLength uint32, derivedDataKeys m_KeyDerivation.ExpandedKeyMaterial, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_reqKeySet _dafny.Set
	_ = _0_reqKeySet
	_0_reqKeySet = func() _dafny.Set {
		var _coll0 = _dafny.NewBuilder()
		_ = _coll0
		for _iter2 := _dafny.Iterate((requiredEncryptionContextKeys).Elements()); ; {
			_compr_0, _ok2 := _iter2()
			if !_ok2 {
				break
			}
			var _1_k _dafny.Sequence
			_1_k = interface{}(_compr_0).(_dafny.Sequence)
			if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_1_k) {
				if _dafny.Companion_Sequence_.Contains(requiredEncryptionContextKeys, _1_k) {
					_coll0.Add(_1_k)
				}
			}
		}
		return _coll0.ToSet()
	}()
	var _2_storedEncryptionContext _dafny.Map
	_ = _2_storedEncryptionContext
	_2_storedEncryptionContext = func() _dafny.Map {
		var _coll1 = _dafny.NewMapBuilder()
		_ = _coll1
		for _iter3 := _dafny.Iterate(((encryptionContext).Subtract(_0_reqKeySet)).Keys().Elements()); ; {
			_compr_1, _ok3 := _iter3()
			if !_ok3 {
				break
			}
			var _3_f _dafny.Sequence
			_3_f = interface{}(_compr_1).(_dafny.Sequence)
			if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_3_f) {
				if ((encryptionContext).Subtract(_0_reqKeySet)).Contains(_3_f) {
					_coll1.Add(_3_f, (encryptionContext).Get(_3_f).(_dafny.Sequence))
				}
			}
		}
		return _coll1.ToMap()
	}()
	var _4_canonicalStoredEncryptionContext _dafny.Sequence
	_ = _4_canonicalStoredEncryptionContext
	_4_canonicalStoredEncryptionContext = m_EncryptionContext.Companion_Default___.GetCanonicalEncryptionContext(_2_storedEncryptionContext)
	var _5_body m_HeaderTypes.HeaderBody
	_ = _5_body
	var _out0 m_HeaderTypes.HeaderBody
	_ = _out0
	_out0 = Companion_Default___.BuildHeaderBody(messageId, suite, _4_canonicalStoredEncryptionContext, encryptedDataKeys, frameLength, (derivedDataKeys).Dtor_commitmentKey())
	_5_body = _out0
	var _6_requiredEncryptionContextMap _dafny.Map
	_ = _6_requiredEncryptionContextMap
	_6_requiredEncryptionContextMap = func() _dafny.Map {
		var _coll2 = _dafny.NewMapBuilder()
		_ = _coll2
		for _iter4 := _dafny.Iterate((_0_reqKeySet).Elements()); ; {
			_compr_2, _ok4 := _iter4()
			if !_ok4 {
				break
			}
			var _7_r _dafny.Sequence
			_7_r = interface{}(_compr_2).(_dafny.Sequence)
			if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_7_r) {
				if (_0_reqKeySet).Contains(_7_r) {
					_coll2.Add(_7_r, (encryptionContext).Get(_7_r).(_dafny.Sequence))
				}
			}
		}
		return _coll2.ToMap()
	}()
	var _8_canonicalReqEncryptionContext _dafny.Sequence
	_ = _8_canonicalReqEncryptionContext
	_8_canonicalReqEncryptionContext = m_EncryptionContext.Companion_Default___.GetCanonicalEncryptionContext(_6_requiredEncryptionContextMap)
	var _9_serializedReqEncryptionContext _dafny.Sequence
	_ = _9_serializedReqEncryptionContext
	_9_serializedReqEncryptionContext = m_EncryptionContext.Companion_Default___.WriteEmptyEcOrWriteAAD(_8_canonicalReqEncryptionContext)
	var _10_rawHeader _dafny.Sequence
	_ = _10_rawHeader
	_10_rawHeader = m_Header.Companion_Default___.WriteHeaderBody(_5_body)
	var _11_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_HeaderTypes.Companion_HeaderAuth_.Default())
	_ = _11_valueOrError0
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = Companion_Default___.BuildHeaderAuthTag(suite, (derivedDataKeys).Dtor_dataKey(), _10_rawHeader, _9_serializedReqEncryptionContext, crypto)
	_11_valueOrError0 = _out1
	if (_11_valueOrError0).IsFailure() {
		res = (_11_valueOrError0).PropagateFailure()
		return res
	}
	var _12_headerAuth m_HeaderTypes.HeaderAuth
	_ = _12_headerAuth
	_12_headerAuth = (_11_valueOrError0).Extract().(m_HeaderTypes.HeaderAuth)
	var _13_header m_Header.HeaderInfo
	_ = _13_header
	_13_header = m_Header.Companion_HeaderInfo_.Create_HeaderInfo_(_5_body, _10_rawHeader, encryptionContext, suite, _12_headerAuth)
	res = m_Wrappers.Companion_Result_.Create_Success_(_13_header)
	return res
	return res
}
func (_static *CompanionStruct_Default___) BuildHeaderBody(messageId _dafny.Sequence, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, encryptionContext _dafny.Sequence, encryptedDataKeys _dafny.Sequence, frameLength uint32, suiteData m_Wrappers.Option) m_HeaderTypes.HeaderBody {
	var res m_HeaderTypes.HeaderBody = m_HeaderTypes.HeaderBody{}
	_ = res
	var _0_contentType m_HeaderTypes.ContentType
	_ = _0_contentType
	_0_contentType = m_HeaderTypes.Companion_ContentType_.Create_Framed_()
	var _source0 m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm = (suite).Dtor_commitment()
	_ = _source0
	{
		{
			if _source0.Is_None() {
				res = m_HeaderTypes.Companion_HeaderBody_.Create_V1HeaderBody_(m_HeaderTypes.Companion_MessageType_.Create_TYPE__CUSTOMER__AED_(), suite, messageId, encryptionContext, encryptedDataKeys, _0_contentType, uint64(m_SerializableTypes.Companion_Default___.GetIvLength(suite)), frameLength)
				return res
				goto Lmatch0
			}
		}
		{
			res = m_HeaderTypes.Companion_HeaderBody_.Create_V2HeaderBody_(suite, messageId, encryptionContext, encryptedDataKeys, _0_contentType, frameLength, (suiteData).Dtor_value().(_dafny.Sequence))
			return res
		}
		goto Lmatch0
	}
Lmatch0:
	return res
}
func (_static *CompanionStruct_Default___) BuildHeaderAuthTag(suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, dataKey _dafny.Sequence, rawHeader _dafny.Sequence, serializedReqEncryptionContext _dafny.Sequence, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_HeaderTypes.Companion_HeaderAuth_.Default())
	_ = res
	var _0_keyLength int32
	_ = _0_keyLength
	_0_keyLength = m_SerializableTypes.Companion_Default___.GetEncryptKeyLength(suite)
	var _1_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _1_valueOrError0
	_1_valueOrError0 = m_Wrappers.Companion_Default___.Need((uint64((dataKey).Cardinality())) == (uint64(_0_keyLength)), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Incorrect data key length")))
	if (_1_valueOrError0).IsFailure() {
		res = (_1_valueOrError0).PropagateFailure()
		return res
	}
	var _2_iv _dafny.Sequence
	_ = _2_iv
	_2_iv = m_SerializableTypes.Companion_Default___.GetIvLengthZeros(suite)
	var _3_maybeEncryptionOutput m_Wrappers.Result
	_ = _3_maybeEncryptionOutput
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (crypto).AESEncrypt(m_AwsCryptographyPrimitivesTypes.Companion_AESEncryptInput_.Create_AESEncryptInput_(((suite).Dtor_encrypt()).Dtor_AES__GCM(), _2_iv, dataKey, _dafny.SeqOf(), _dafny.Companion_Sequence_.Concatenate(rawHeader, serializedReqEncryptionContext)))
	_3_maybeEncryptionOutput = _out0
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyPrimitivesTypes.Companion_AESEncryptOutput_.Default())
	_ = _4_valueOrError1
	_4_valueOrError1 = (_3_maybeEncryptionOutput).MapFailure(func(coer20 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg21 interface{}) interface{} {
			return coer20(arg21.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_5_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_5_e)
	}))
	if (_4_valueOrError1).IsFailure() {
		res = (_4_valueOrError1).PropagateFailure()
		return res
	}
	var _6_encryptionOutput m_AwsCryptographyPrimitivesTypes.AESEncryptOutput
	_ = _6_encryptionOutput
	_6_encryptionOutput = (_4_valueOrError1).Extract().(m_AwsCryptographyPrimitivesTypes.AESEncryptOutput)
	var _7_headerAuth m_HeaderTypes.HeaderAuth
	_ = _7_headerAuth
	_7_headerAuth = m_HeaderTypes.Companion_HeaderAuth_.Create_AESMac_(_2_iv, (_6_encryptionOutput).Dtor_authTag())
	res = m_Wrappers.Companion_Result_.Create_Success_(_7_headerAuth)
	return res
	return res
}
func (_static *CompanionStruct_Default___) GetEncryptionMaterials(cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager, algorithmSuiteId m_Wrappers.Option, encryptionContext _dafny.Map, maxPlaintextLength int64, commitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, mpl *m_MaterialProviders.MaterialProvidersClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_encMatRequest m_AwsCryptographyMaterialProvidersTypes.GetEncryptionMaterialsInput
	_ = _0_encMatRequest
	_0_encMatRequest = m_AwsCryptographyMaterialProvidersTypes.Companion_GetEncryptionMaterialsInput_.Create_GetEncryptionMaterialsInput_(encryptionContext, m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(commitmentPolicy), algorithmSuiteId, m_Wrappers.Companion_Option_.Create_Some_(maxPlaintextLength), m_Wrappers.Companion_Option_.Create_None_())
	var _1_getEncMatResult m_Wrappers.Result
	_ = _1_getEncMatResult
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (cmm).GetEncryptionMaterials(_0_encMatRequest)
	_1_getEncMatResult = _out0
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	_2_valueOrError0 = (_1_getEncMatResult).MapFailure(func(coer21 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg22 interface{}) interface{} {
			return coer21(arg22.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_3_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_3_e)
	}))
	if (_2_valueOrError0).IsFailure() {
		res = (_2_valueOrError0).PropagateFailure()
		return res
	}
	var _4_output m_AwsCryptographyMaterialProvidersTypes.GetEncryptionMaterialsOutput
	_ = _4_output
	_4_output = (_2_valueOrError0).Extract().(m_AwsCryptographyMaterialProvidersTypes.GetEncryptionMaterialsOutput)
	var _5_materials m_AwsCryptographyMaterialProvidersTypes.EncryptionMaterials
	_ = _5_materials
	_5_materials = (_4_output).Dtor_encryptionMaterials()
	var _6_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _6_valueOrError1
	_6_valueOrError1 = ((mpl).ValidateCommitmentPolicyOnEncrypt(m_AwsCryptographyMaterialProvidersTypes.Companion_ValidateCommitmentPolicyOnEncryptInput_.Create_ValidateCommitmentPolicyOnEncryptInput_(((_5_materials).Dtor_algorithmSuite()).Dtor_id(), m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(commitmentPolicy)))).MapFailure(func(coer22 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg23 interface{}) interface{} {
			return coer22(arg23.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_7_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_7_e)
	}))
	if (_6_valueOrError1).IsFailure() {
		res = (_6_valueOrError1).PropagateFailure()
		return res
	}
	var _8___v3 _dafny.Tuple
	_ = _8___v3
	_8___v3 = (_6_valueOrError1).Extract().(_dafny.Tuple)
	var _9_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _9_valueOrError2
	_9_valueOrError2 = ((mpl).EncryptionMaterialsHasPlaintextDataKey(_5_materials)).MapFailure(func(coer23 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg24 interface{}) interface{} {
			return coer23(arg24.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_10_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_10_e)
	}))
	if (_9_valueOrError2).IsFailure() {
		res = (_9_valueOrError2).PropagateFailure()
		return res
	}
	var _11___v4 _dafny.Tuple
	_ = _11___v4
	_11___v4 = (_9_valueOrError2).Extract().(_dafny.Tuple)
	var _12_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _12_valueOrError3
	_12_valueOrError3 = m_Wrappers.Companion_Default___.Need(m_SerializableTypes.Companion_Default___.IsESDKEncryptionContext((_5_materials).Dtor_encryptionContext()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("CMM failed to return serializable encryption materials.")))
	if (_12_valueOrError3).IsFailure() {
		res = (_12_valueOrError3).PropagateFailure()
		return res
	}
	var _13_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _13_valueOrError4
	_13_valueOrError4 = m_Wrappers.Companion_Default___.Need(m_StandardLibrary_UInt.Companion_Default___.HasUint16Len((_5_materials).Dtor_encryptedDataKeys()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("CMM returned EDKs that exceed the allowed maximum.")))
	if (_13_valueOrError4).IsFailure() {
		res = (_13_valueOrError4).PropagateFailure()
		return res
	}
	var _14_valueOrError5 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _14_valueOrError5
	_14_valueOrError5 = m_Wrappers.Companion_Default___.Need(_dafny.Quantifier(((_5_materials).Dtor_encryptedDataKeys()).UniqueElements(), true, func(_forall_var_0 m_AwsCryptographyMaterialProvidersTypes.EncryptedDataKey) bool {
		var _15_edk m_AwsCryptographyMaterialProvidersTypes.EncryptedDataKey
		_15_edk = interface{}(_forall_var_0).(m_AwsCryptographyMaterialProvidersTypes.EncryptedDataKey)
		return !(_dafny.Companion_Sequence_.Contains((_5_materials).Dtor_encryptedDataKeys(), _15_edk)) || (m_SerializableTypes.Companion_Default___.IsESDKEncryptedDataKey(_15_edk))
	}), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("CMM returned non-serializable encrypted data key.")))
	if (_14_valueOrError5).IsFailure() {
		res = (_14_valueOrError5).PropagateFailure()
		return res
	}
	res = m_Wrappers.Companion_Result_.Create_Success_(_5_materials)
	return res
	return res
}
func (_static *CompanionStruct_Default___) GetDecryptionMaterials(cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager, algorithmSuiteId m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteId, headerBody m_HeaderTypes.HeaderBody, reproducedEncryptionContext m_Wrappers.Option, commitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, mpl *m_MaterialProviders.MaterialProvidersClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_encryptionContext _dafny.Map
	_ = _0_encryptionContext
	_0_encryptionContext = m_EncryptionContext.Companion_Default___.GetEncryptionContext((headerBody).Dtor_encryptionContext())
	var _1_decMatRequest m_AwsCryptographyMaterialProvidersTypes.DecryptMaterialsInput
	_ = _1_decMatRequest
	_1_decMatRequest = m_AwsCryptographyMaterialProvidersTypes.Companion_DecryptMaterialsInput_.Create_DecryptMaterialsInput_(algorithmSuiteId, m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(commitmentPolicy), (headerBody).Dtor_encryptedDataKeys(), _0_encryptionContext, reproducedEncryptionContext)
	var _2_decMatResult m_Wrappers.Result
	_ = _2_decMatResult
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (cmm).DecryptMaterials(_1_decMatRequest)
	_2_decMatResult = _out0
	var _3_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _3_valueOrError0
	_3_valueOrError0 = (_2_decMatResult).MapFailure(func(coer24 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg25 interface{}) interface{} {
			return coer24(arg25.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_4_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_4_e)
	}))
	if (_3_valueOrError0).IsFailure() {
		res = (_3_valueOrError0).PropagateFailure()
		return res
	}
	var _5_output m_AwsCryptographyMaterialProvidersTypes.DecryptMaterialsOutput
	_ = _5_output
	_5_output = (_3_valueOrError0).Extract().(m_AwsCryptographyMaterialProvidersTypes.DecryptMaterialsOutput)
	var _6_materials m_AwsCryptographyMaterialProvidersTypes.DecryptionMaterials
	_ = _6_materials
	_6_materials = (_5_output).Dtor_decryptionMaterials()
	var _7_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _7_valueOrError1
	_7_valueOrError1 = ((mpl).ValidateCommitmentPolicyOnDecrypt(m_AwsCryptographyMaterialProvidersTypes.Companion_ValidateCommitmentPolicyOnDecryptInput_.Create_ValidateCommitmentPolicyOnDecryptInput_(((_6_materials).Dtor_algorithmSuite()).Dtor_id(), m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(commitmentPolicy)))).MapFailure(func(coer25 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg26 interface{}) interface{} {
			return coer25(arg26.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_8_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_8_e)
	}))
	if (_7_valueOrError1).IsFailure() {
		res = (_7_valueOrError1).PropagateFailure()
		return res
	}
	var _9___v5 _dafny.Tuple
	_ = _9___v5
	_9___v5 = (_7_valueOrError1).Extract().(_dafny.Tuple)
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _10_valueOrError2
	_10_valueOrError2 = ((mpl).DecryptionMaterialsWithPlaintextDataKey(_6_materials)).MapFailure(func(coer26 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg27 interface{}) interface{} {
			return coer26(arg27.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_11_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_11_e)
	}))
	if (_10_valueOrError2).IsFailure() {
		res = (_10_valueOrError2).PropagateFailure()
		return res
	}
	var _12___v6 _dafny.Tuple
	_ = _12___v6
	_12___v6 = (_10_valueOrError2).Extract().(_dafny.Tuple)
	var _13_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _13_valueOrError3
	_13_valueOrError3 = m_Wrappers.Companion_Default___.Need(m_SerializableTypes.Companion_Default___.IsESDKEncryptionContext((_6_materials).Dtor_encryptionContext()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("CMM failed to return serializable encryption materials.")))
	if (_13_valueOrError3).IsFailure() {
		res = (_13_valueOrError3).PropagateFailure()
		return res
	}
	res = m_Wrappers.Companion_Result_.Create_Success_(_6_materials)
	return res
	return res
}
func (_static *CompanionStruct_Default___) ValidateSuiteData(suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, header m_HeaderTypes.HeaderBody, expectedSuiteData _dafny.Sequence) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = res
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _0_valueOrError0
	_0_valueOrError0 = m_Wrappers.Companion_Default___.Need((uint64(((header).Dtor_suiteData()).Cardinality())) == (uint64((((suite).Dtor_commitment()).Dtor_HKDF()).Dtor_outputKeyLength())), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Commitment key is invalid")))
	if (_0_valueOrError0).IsFailure() {
		res = (_0_valueOrError0).PropagateFailure()
		return res
	}
	var _1_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _1_valueOrError1
	_1_valueOrError1 = m_Wrappers.Companion_Default___.Need(_dafny.Companion_Sequence_.Equal(expectedSuiteData, (header).Dtor_suiteData()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Commitment key does not match")))
	if (_1_valueOrError1).IsFailure() {
		res = (_1_valueOrError1).PropagateFailure()
		return res
	}
	res = m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf())
	return res
	return res
}
func (_static *CompanionStruct_Default___) ReadAndDecryptFramedMessageBody(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo, key _dafny.Sequence, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf(_dafny.EmptySeq, m_SerializeFunctions.Companion_ReadableBuffer_.Default()))
	_ = res
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	_0_valueOrError0 = (m_MessageBody.Companion_Default___.ReadFramedMessageBody(buffer, header, _dafny.SeqOf(), buffer)).MapFailure(func(coer27 func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg28 interface{}) interface{} {
			return coer27(arg28.(m_SerializeFunctions.ReadProblems))
		}
	}(Companion_Default___.MapSerializeFailure(_dafny.SeqOfString(": ReadFramedMessageBody"))))
	if (_0_valueOrError0).IsFailure() {
		res = (_0_valueOrError0).PropagateFailure()
		return res
	}
	var _1_messageBody m_SerializeFunctions.SuccessfulRead
	_ = _1_messageBody
	_1_messageBody = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
	var _2_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _2_valueOrError1
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_MessageBody.Companion_Default___.DecryptFramedMessageBody((_1_messageBody).Dtor_data().(m_MessageBody.FramedMessageBody), key, crypto)
	_2_valueOrError1 = _out0
	if (_2_valueOrError1).IsFailure() {
		res = (_2_valueOrError1).PropagateFailure()
		return res
	}
	var _3_plaintext _dafny.Sequence
	_ = _3_plaintext
	_3_plaintext = (_2_valueOrError1).Extract().(_dafny.Sequence)
	var _4_messageBodyTail m_SerializeFunctions.ReadableBuffer
	_ = _4_messageBodyTail
	_4_messageBodyTail = (_1_messageBody).Dtor_tail()
	res = m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf(_3_plaintext, _4_messageBodyTail))
	return res
	return res
}
func (_static *CompanionStruct_Default___) ReadAndDecryptNonFramedMessageBody(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo, key _dafny.Sequence, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf(_dafny.EmptySeq, m_SerializeFunctions.Companion_ReadableBuffer_.Default()))
	_ = res
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	_0_valueOrError0 = (m_MessageBody.Companion_Default___.ReadNonFramedMessageBody(buffer, header)).MapFailure(func(coer28 func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg29 interface{}) interface{} {
			return coer28(arg29.(m_SerializeFunctions.ReadProblems))
		}
	}(Companion_Default___.MapSerializeFailure(_dafny.SeqOfString(": ReadNonFramedMessageBody"))))
	if (_0_valueOrError0).IsFailure() {
		res = (_0_valueOrError0).PropagateFailure()
		return res
	}
	var _1_messageBody m_SerializeFunctions.SuccessfulRead
	_ = _1_messageBody
	_1_messageBody = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
	var _2_frame m_Frames.Frame
	_ = _2_frame
	_2_frame = (_1_messageBody).Dtor_data().(m_Frames.Frame)
	var _3_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _3_valueOrError1
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_MessageBody.Companion_Default___.DecryptFrame(_2_frame, key, crypto)
	_3_valueOrError1 = _out0
	if (_3_valueOrError1).IsFailure() {
		res = (_3_valueOrError1).PropagateFailure()
		return res
	}
	var _4_plaintext _dafny.Sequence
	_ = _4_plaintext
	_4_plaintext = (_3_valueOrError1).Extract().(_dafny.Sequence)
	var _5_messageBodyTail m_SerializeFunctions.ReadableBuffer
	_ = _5_messageBodyTail
	_5_messageBodyTail = (_1_messageBody).Dtor_tail()
	res = m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf(_4_plaintext, _5_messageBodyTail))
	return res
	return res
}
func (_static *CompanionStruct_Default___) DEFAULT__FRAME__LENGTH() int64 {
	return int64(4096)
}
func (_static *CompanionStruct_Default___) RESERVED__ENCRYPTION__CONTEXT() _dafny.Sequence {
	var _0_s _dafny.Sequence = _dafny.SeqOf(uint8(97), uint8(119), uint8(115), uint8(45), uint8(99), uint8(114), uint8(121), uint8(112), uint8(116), uint8(111), uint8(45))
	_ = _0_s
	return _0_s
}

// End of class Default__

// Definition of class FrameLength
type FrameLength struct {
}

func New_FrameLength_() *FrameLength {
	_this := FrameLength{}

	return &_this
}

type CompanionStruct_FrameLength_ struct {
}

var Companion_FrameLength_ = CompanionStruct_FrameLength_{}

func (*FrameLength) String() string {
	return "EncryptDecryptHelpers.FrameLength"
}

// End of class FrameLength

func Type_FrameLength_() _dafny.TypeDescriptor {
	return type_FrameLength_{}
}

type type_FrameLength_ struct {
}

func (_this type_FrameLength_) Default() interface{} {
	return int64(0)
}

func (_this type_FrameLength_) String() string {
	return "EncryptDecryptHelpers.FrameLength"
}
func (_this *CompanionStruct_FrameLength_) Is_(__source int64) bool {
	var _1_frameLength int64 = (__source)
	_ = _1_frameLength
	if true {
		return ((int64(0)) < (_1_frameLength)) && ((_1_frameLength) <= (int64(4294967295)))
	}
	return false
}
