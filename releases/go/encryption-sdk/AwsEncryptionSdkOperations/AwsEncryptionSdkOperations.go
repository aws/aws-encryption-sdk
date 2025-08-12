// Package AwsEncryptionSdkOperations
// Dafny module AwsEncryptionSdkOperations compiled into Go

package AwsEncryptionSdkOperations

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
	m_EncryptDecryptHelpers "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptDecryptHelpers"
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
var _ m_EncryptDecryptHelpers.Dummy__

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
	return "AwsEncryptionSdkOperations.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) Encrypt(config Config, input m_AwsCryptographyEncryptionSdkTypes.EncryptInput) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptOutput_.Default())
	_ = output
	var _0_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(int64(0))
	_ = _0_valueOrError1
	if ((input).Dtor_frameLength()).Is_Some() {
		var _1_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((int64(0)) < (((input).Dtor_frameLength()).Dtor_value().(int64))) && ((((input).Dtor_frameLength()).Dtor_value().(int64)) <= (int64(4294967295))), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("FrameLength must be greater than 0 and less than 2^32")))
		_ = _1_valueOrError0
		if (_1_valueOrError0).IsFailure() {
			_0_valueOrError1 = (_1_valueOrError0).PropagateFailure()
		} else {
			_0_valueOrError1 = m_Wrappers.Companion_Result_.Create_Success_(((input).Dtor_frameLength()).Dtor_value().(int64))
		}
	} else {
		_0_valueOrError1 = m_Wrappers.Companion_Result_.Create_Success_(m_EncryptDecryptHelpers.Companion_Default___.DEFAULT__FRAME__LENGTH())
	}
	if (_0_valueOrError1).IsFailure() {
		output = (_0_valueOrError1).PropagateFailure()
		return output
	}
	var _2_frameLength int64
	_ = _2_frameLength
	_2_frameLength = (_0_valueOrError1).Extract().(int64)
	var _3_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _3_valueOrError2
	_3_valueOrError2 = m_EncryptDecryptHelpers.Companion_Default___.ValidateEncryptionContext((input).Dtor_encryptionContext())
	if (_3_valueOrError2).IsFailure() {
		output = (_3_valueOrError2).PropagateFailure()
		return output
	}
	var _4_encryptionContext _dafny.Map
	_ = _4_encryptionContext
	if ((input).Dtor_encryptionContext()).Is_Some() {
		_4_encryptionContext = ((input).Dtor_encryptionContext()).Dtor_value().(_dafny.Map)
	} else {
		_4_encryptionContext = _dafny.NewMapBuilder().ToMap()
	}
	var _5_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _5_valueOrError3
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptDecryptHelpers.Companion_Default___.CreateCmmFromInput((input).Dtor_materialsManager(), (input).Dtor_keyring())
	_5_valueOrError3 = _out0
	if (_5_valueOrError3).IsFailure() {
		output = (_5_valueOrError3).PropagateFailure()
		return output
	}
	var _6_cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _6_cmm
	_6_cmm = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_5_valueOrError3).Extract())
	var _7_algorithmSuiteId m_Wrappers.Option
	_ = _7_algorithmSuiteId
	if ((input).Dtor_algorithmSuiteId()).Is_Some() {
		_7_algorithmSuiteId = m_Wrappers.Companion_Option_.Create_Some_(m_AwsCryptographyMaterialProvidersTypes.Companion_AlgorithmSuiteId_.Create_ESDK_(((input).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId)))
	} else {
		_7_algorithmSuiteId = m_Wrappers.Companion_Option_.Create_None_()
	}
	if (_7_algorithmSuiteId).Is_Some() {
		var _8_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
		_ = _8_valueOrError4
		_8_valueOrError4 = (((config).Dtor_mpl()).ValidateCommitmentPolicyOnEncrypt(m_AwsCryptographyMaterialProvidersTypes.Companion_ValidateCommitmentPolicyOnEncryptInput_.Create_ValidateCommitmentPolicyOnEncryptInput_((_7_algorithmSuiteId).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteId), m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_((config).Dtor_commitmentPolicy())))).MapFailure(func(coer29 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
			return func(arg30 interface{}) interface{} {
				return coer29(arg30.(m_AwsCryptographyMaterialProvidersTypes.Error))
			}
		}(func(_9_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
			return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_9_e)
		}))
		if (_8_valueOrError4).IsFailure() {
			output = (_8_valueOrError4).PropagateFailure()
			return output
		}
		var _10___v0 _dafny.Tuple
		_ = _10___v0
		_10___v0 = (_8_valueOrError4).Extract().(_dafny.Tuple)
	}
	var _11_valueOrError5 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _11_valueOrError5
	_11_valueOrError5 = m_Wrappers.Companion_Default___.Need((uint64(((input).Dtor_plaintext()).Cardinality())) < ((m_StandardLibrary_UInt.Companion_Default___.INT64__MAX__LIMIT()).Uint64()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Plaintext exceeds maximum allowed size")))
	if (_11_valueOrError5).IsFailure() {
		output = (_11_valueOrError5).PropagateFailure()
		return output
	}
	var _12_valueOrError6 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _12_valueOrError6
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_EncryptDecryptHelpers.Companion_Default___.GetEncryptionMaterials(_6_cmm, _7_algorithmSuiteId, _4_encryptionContext, int64(((input).Dtor_plaintext()).Cardinality()), (config).Dtor_commitmentPolicy(), (config).Dtor_mpl())
	_12_valueOrError6 = _out1
	if (_12_valueOrError6).IsFailure() {
		output = (_12_valueOrError6).PropagateFailure()
		return output
	}
	var _13_materials m_AwsCryptographyMaterialProvidersTypes.EncryptionMaterials
	_ = _13_materials
	_13_materials = (_12_valueOrError6).Extract().(m_AwsCryptographyMaterialProvidersTypes.EncryptionMaterials)
	var _14_valueOrError7 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _14_valueOrError7
	_14_valueOrError7 = m_Wrappers.Companion_Default___.Need((((_13_materials).Dtor_algorithmSuite()).Dtor_id()).Is_ESDK(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Encryption materials contain incompatible algorithm suite for the AWS Encryption SDK.")))
	if (_14_valueOrError7).IsFailure() {
		output = (_14_valueOrError7).PropagateFailure()
		return output
	}
	var _15_valueOrError8 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _15_valueOrError8
	_15_valueOrError8 = m_EncryptDecryptHelpers.Companion_Default___.ValidateMaxEncryptedDataKeys((config).Dtor_maxEncryptedDataKeys(), (_13_materials).Dtor_encryptedDataKeys())
	if (_15_valueOrError8).IsFailure() {
		output = (_15_valueOrError8).PropagateFailure()
		return output
	}
	var _16_encryptedDataKeys _dafny.Sequence
	_ = _16_encryptedDataKeys
	_16_encryptedDataKeys = (_13_materials).Dtor_encryptedDataKeys()
	var _17_valueOrError9 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _17_valueOrError9
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = m_EncryptDecryptHelpers.Companion_Default___.GenerateMessageId((_13_materials).Dtor_algorithmSuite(), (config).Dtor_crypto())
	_17_valueOrError9 = _out2
	if (_17_valueOrError9).IsFailure() {
		output = (_17_valueOrError9).PropagateFailure()
		return output
	}
	var _18_messageId _dafny.Sequence
	_ = _18_messageId
	_18_messageId = (_17_valueOrError9).Extract().(_dafny.Sequence)
	var _19_maybeDerivedDataKeys m_Wrappers.Result
	_ = _19_maybeDerivedDataKeys
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = m_KeyDerivation.Companion_Default___.DeriveKeys(_18_messageId, ((_13_materials).Dtor_plaintextDataKey()).Dtor_value().(_dafny.Sequence), (_13_materials).Dtor_algorithmSuite(), (config).Dtor_crypto(), (config).Dtor_netV4__0__0__RetryPolicy(), false)
	_19_maybeDerivedDataKeys = _out3
	var _20_valueOrError10 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_KeyDerivation.Companion_ExpandedKeyMaterial_.Default())
	_ = _20_valueOrError10
	_20_valueOrError10 = (_19_maybeDerivedDataKeys).MapFailure(func(coer30 func(m_AwsCryptographyEncryptionSdkTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg31 interface{}) interface{} {
			return coer30(arg31.(m_AwsCryptographyEncryptionSdkTypes.Error))
		}
	}(func(_21_e m_AwsCryptographyEncryptionSdkTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Failed to derive data keys"))
	}))
	if (_20_valueOrError10).IsFailure() {
		output = (_20_valueOrError10).PropagateFailure()
		return output
	}
	var _22_derivedDataKeys m_KeyDerivation.ExpandedKeyMaterial
	_ = _22_derivedDataKeys
	_22_derivedDataKeys = (_20_valueOrError10).Extract().(m_KeyDerivation.ExpandedKeyMaterial)
	var _23_maybeHeader m_Wrappers.Result
	_ = _23_maybeHeader
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = m_EncryptDecryptHelpers.Companion_Default___.BuildHeaderForEncrypt(_18_messageId, (_13_materials).Dtor_algorithmSuite(), (_13_materials).Dtor_encryptionContext(), (_13_materials).Dtor_requiredEncryptionContextKeys(), _16_encryptedDataKeys, uint32(_2_frameLength), _22_derivedDataKeys, (config).Dtor_crypto())
	_23_maybeHeader = _out4
	var _24_valueOrError11 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _24_valueOrError11
	_24_valueOrError11 = m_Wrappers.Companion_Default___.Need((_23_maybeHeader).Is_Success(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Failed to build header body")))
	if (_24_valueOrError11).IsFailure() {
		output = (_24_valueOrError11).PropagateFailure()
		return output
	}
	var _25_header m_Header.HeaderInfo
	_ = _25_header
	_25_header = (_23_maybeHeader).Dtor_value().(m_Header.HeaderInfo)
	var _26_valueOrError12 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _26_valueOrError12
	var _out5 m_Wrappers.Result
	_ = _out5
	_out5 = m_MessageBody.Companion_Default___.EncryptMessageBody((input).Dtor_plaintext(), _25_header, (_22_derivedDataKeys).Dtor_dataKey(), (config).Dtor_crypto())
	_26_valueOrError12 = _out5
	if (_26_valueOrError12).IsFailure() {
		output = (_26_valueOrError12).PropagateFailure()
		return output
	}
	var _27_framedMessage m_MessageBody.FramedMessageBody
	_ = _27_framedMessage
	_27_framedMessage = (_26_valueOrError12).Extract().(m_MessageBody.FramedMessageBody)
	var _28_maybeSignedMessage m_Wrappers.Result
	_ = _28_maybeSignedMessage
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = Companion_Default___.SignAndSerializeMessage(config, _25_header, _27_framedMessage, _13_materials)
	_28_maybeSignedMessage = _out6
	output = _28_maybeSignedMessage
	return output
}
func (_static *CompanionStruct_Default___) SignAndSerializeMessage(config Config, header m_Header.HeaderInfo, framedMessage m_MessageBody.FramedMessageBody, materials m_AwsCryptographyMaterialProvidersTypes.EncryptionMaterials) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptOutput_.Default())
	_ = output
	if (((((framedMessage).Dtor_finalFrame()).Dtor_header()).Dtor_suite()).Dtor_signature()).Is_ECDSA() {
		var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
		_ = _0_valueOrError0
		_0_valueOrError0 = m_EncryptDecryptHelpers.Companion_Default___.SerializeMessageWithoutSignature(framedMessage, (materials).Dtor_algorithmSuite())
		if (_0_valueOrError0).IsFailure() {
			output = (_0_valueOrError0).PropagateFailure()
			return output
		}
		var _1_msg _dafny.Sequence
		_ = _1_msg
		_1_msg = (_0_valueOrError0).Extract().(_dafny.Sequence)
		var _2_ecdsaParams m_AwsCryptographyPrimitivesTypes.ECDSASignatureAlgorithm
		_ = _2_ecdsaParams
		_2_ecdsaParams = ((((((framedMessage).Dtor_finalFrame()).Dtor_header()).Dtor_suite()).Dtor_signature()).Dtor_ECDSA()).Dtor_curve()
		var _3_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _3_valueOrError1
		_3_valueOrError1 = m_Wrappers.Companion_Default___.Need(((materials).Dtor_signingKey()).Is_Some(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Missing signing key.")))
		if (_3_valueOrError1).IsFailure() {
			output = (_3_valueOrError1).PropagateFailure()
			return output
		}
		var _4_maybeBytes m_Wrappers.Result
		_ = _4_maybeBytes
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = ((config).Dtor_crypto()).ECDSASign(m_AwsCryptographyPrimitivesTypes.Companion_ECDSASignInput_.Create_ECDSASignInput_(_2_ecdsaParams, ((materials).Dtor_signingKey()).Dtor_value().(_dafny.Sequence), _1_msg))
		_4_maybeBytes = _out0
		var _5_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
		_ = _5_valueOrError2
		_5_valueOrError2 = (_4_maybeBytes).MapFailure(func(coer31 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
			return func(arg32 interface{}) interface{} {
				return coer31(arg32.(m_AwsCryptographyPrimitivesTypes.Error))
			}
		}(func(_6_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
			return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_6_e)
		}))
		if (_5_valueOrError2).IsFailure() {
			output = (_5_valueOrError2).PropagateFailure()
			return output
		}
		var _7_bytes _dafny.Sequence
		_ = _7_bytes
		_7_bytes = (_5_valueOrError2).Extract().(_dafny.Sequence)
		var _8_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _8_valueOrError3
		_8_valueOrError3 = m_Wrappers.Companion_Default___.Need((uint64((_7_bytes).Cardinality())) < ((m_StandardLibrary_UInt.Companion_Default___.UINT16__LIMIT()).Uint64()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Length of signature bytes is larger than the uint16 limit.")))
		if (_8_valueOrError3).IsFailure() {
			output = (_8_valueOrError3).PropagateFailure()
			return output
		}
		var _9_signature _dafny.Sequence
		_ = _9_signature
		_9_signature = _dafny.Companion_Sequence_.Concatenate(m_StandardLibrary_UInt.Companion_Default___.UInt16ToSeq(uint16((_7_bytes).Cardinality())), _7_bytes)
		_1_msg = _dafny.Companion_Sequence_.Concatenate(_1_msg, _9_signature)
		output = m_Wrappers.Companion_Result_.Create_Success_(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptOutput_.Create_EncryptOutput_(_1_msg, (header).Dtor_encryptionContext(), (((header).Dtor_suite()).Dtor_id()).Dtor_ESDK()))
		return output
	} else {
		var _10_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
		_ = _10_valueOrError4
		_10_valueOrError4 = m_EncryptDecryptHelpers.Companion_Default___.SerializeMessageWithoutSignature(framedMessage, (materials).Dtor_algorithmSuite())
		if (_10_valueOrError4).IsFailure() {
			output = (_10_valueOrError4).PropagateFailure()
			return output
		}
		var _11_msg _dafny.Sequence
		_ = _11_msg
		_11_msg = (_10_valueOrError4).Extract().(_dafny.Sequence)
		output = m_Wrappers.Companion_Result_.Create_Success_(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptOutput_.Create_EncryptOutput_(_11_msg, (header).Dtor_encryptionContext(), (((header).Dtor_suite()).Dtor_id()).Dtor_ESDK()))
		return output
	}
	return output
}
func (_static *CompanionStruct_Default___) Decrypt(config Config, input m_AwsCryptographyEncryptionSdkTypes.DecryptInput) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptOutput_.Default())
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptDecryptHelpers.Companion_Default___.CreateCmmFromInput((input).Dtor_materialsManager(), (input).Dtor_keyring())
	_0_valueOrError0 = _out0
	if (_0_valueOrError0).IsFailure() {
		output = (_0_valueOrError0).PropagateFailure()
		return output
	}
	var _1_cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _1_cmm
	_1_cmm = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_0_valueOrError0).Extract())
	var _2_buffer m_SerializeFunctions.ReadableBuffer
	_ = _2_buffer
	_2_buffer = m_SerializeFunctions.Companion_ReadableBuffer_.Create_ReadableBuffer_((input).Dtor_ciphertext(), uint64(0))
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = Companion_Default___.InternalDecrypt(config, _1_cmm, _2_buffer, (input).Dtor_encryptionContext())
	output = _out1
	return output
}
func (_static *CompanionStruct_Default___) InternalDecrypt(config Config, cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager, buffer m_SerializeFunctions.ReadableBuffer, inputEncryptionContext m_Wrappers.Option) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptOutput_.Default())
	_ = output
	var _0_v4Retry bool
	_ = _0_v4Retry
	_0_v4Retry = false
	var _1_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _1_valueOrError0
	_1_valueOrError0 = (m_Header.Companion_Default___.ReadHeaderBody(buffer, (config).Dtor_maxEncryptedDataKeys(), (config).Dtor_mpl())).MapFailure(func(coer32 func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg33 interface{}) interface{} {
			return coer32(arg33.(m_SerializeFunctions.ReadProblems))
		}
	}(m_EncryptDecryptHelpers.Companion_Default___.MapSerializeFailure(_dafny.SeqOfString(": ReadHeaderBody"))))
	if (_1_valueOrError0).IsFailure() {
		output = (_1_valueOrError0).PropagateFailure()
		return output
	}
	var _2_headerBody m_SerializeFunctions.SuccessfulRead
	_ = _2_headerBody
	_2_headerBody = (_1_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
	var _3_rawHeader _dafny.Sequence
	_ = _3_rawHeader
	_3_rawHeader = ((buffer).Dtor_bytes()).Subsequence(uint32((buffer).Dtor_start()), uint32(((_2_headerBody).Dtor_tail()).Dtor_start()))
	var _4_algorithmSuite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
	_ = _4_algorithmSuite
	_4_algorithmSuite = ((_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody)).Dtor_algorithmSuite()
	var _5_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _5_valueOrError1
	_5_valueOrError1 = (((config).Dtor_mpl()).ValidateCommitmentPolicyOnDecrypt(m_AwsCryptographyMaterialProvidersTypes.Companion_ValidateCommitmentPolicyOnDecryptInput_.Create_ValidateCommitmentPolicyOnDecryptInput_((_4_algorithmSuite).Dtor_id(), m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_((config).Dtor_commitmentPolicy())))).MapFailure(func(coer33 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg34 interface{}) interface{} {
			return coer33(arg34.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_6_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_6_e)
	}))
	if (_5_valueOrError1).IsFailure() {
		output = (_5_valueOrError1).PropagateFailure()
		return output
	}
	var _7___v1 _dafny.Tuple
	_ = _7___v1
	_7___v1 = (_5_valueOrError1).Extract().(_dafny.Tuple)
	var _8_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _8_valueOrError2
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptDecryptHelpers.Companion_Default___.GetDecryptionMaterials(cmm, (_4_algorithmSuite).Dtor_id(), (_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody), inputEncryptionContext, (config).Dtor_commitmentPolicy(), (config).Dtor_mpl())
	_8_valueOrError2 = _out0
	if (_8_valueOrError2).IsFailure() {
		output = (_8_valueOrError2).PropagateFailure()
		return output
	}
	var _9_decMat m_AwsCryptographyMaterialProvidersTypes.DecryptionMaterials
	_ = _9_decMat
	_9_decMat = (_8_valueOrError2).Extract().(m_AwsCryptographyMaterialProvidersTypes.DecryptionMaterials)
	var _10_suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
	_ = _10_suite
	_10_suite = (_9_decMat).Dtor_algorithmSuite()
	var _11_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _11_valueOrError3
	_11_valueOrError3 = m_Wrappers.Companion_Default___.Need((_10_suite).Equals(_4_algorithmSuite), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Stored header algorithm suite does not match decryption algorithm suite.")))
	if (_11_valueOrError3).IsFailure() {
		output = (_11_valueOrError3).PropagateFailure()
		return output
	}
	var _12_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _12_valueOrError4
	_12_valueOrError4 = (m_HeaderAuth.Companion_Default___.ReadHeaderAuthTag((_2_headerBody).Dtor_tail(), _10_suite)).MapFailure(func(coer34 func(m_SerializeFunctions.ReadProblems) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg35 interface{}) interface{} {
			return coer34(arg35.(m_SerializeFunctions.ReadProblems))
		}
	}(m_EncryptDecryptHelpers.Companion_Default___.MapSerializeFailure(_dafny.SeqOfString(": ReadHeaderAuthTag"))))
	if (_12_valueOrError4).IsFailure() {
		output = (_12_valueOrError4).PropagateFailure()
		return output
	}
	var _13_headerAuth m_SerializeFunctions.SuccessfulRead
	_ = _13_headerAuth
	_13_headerAuth = (_12_valueOrError4).Extract().(m_SerializeFunctions.SuccessfulRead)
	var _14_maybeDerivedDataKeys m_Wrappers.Result
	_ = _14_maybeDerivedDataKeys
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_KeyDerivation.Companion_Default___.DeriveKeys(((_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody)).Dtor_messageId(), ((_9_decMat).Dtor_plaintextDataKey()).Dtor_value().(_dafny.Sequence), _10_suite, (config).Dtor_crypto(), (config).Dtor_netV4__0__0__RetryPolicy(), false)
	_14_maybeDerivedDataKeys = _out1
	var _15_valueOrError5 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _15_valueOrError5
	_15_valueOrError5 = m_Wrappers.Companion_Default___.Need((_14_maybeDerivedDataKeys).Is_Success(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Failed to derive data keys")))
	if (_15_valueOrError5).IsFailure() {
		output = (_15_valueOrError5).PropagateFailure()
		return output
	}
	var _16_derivedDataKeys m_KeyDerivation.ExpandedKeyMaterial
	_ = _16_derivedDataKeys
	_16_derivedDataKeys = (_14_maybeDerivedDataKeys).Dtor_value().(m_KeyDerivation.ExpandedKeyMaterial)
	var _17_valueOrError6 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _17_valueOrError6
	_17_valueOrError6 = m_Wrappers.Companion_Default___.Need(m_Header.Companion_Default___.HeaderVersionSupportsCommitment_q(_10_suite, (_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody)), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Invalid commitment values found in header body")))
	if (_17_valueOrError6).IsFailure() {
		output = (_17_valueOrError6).PropagateFailure()
		return output
	}
	if ((_10_suite).Dtor_commitment()).Is_HKDF() {
		var _18_valueOrError7 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
		_ = _18_valueOrError7
		var _out2 m_Wrappers.Result
		_ = _out2
		_out2 = m_EncryptDecryptHelpers.Companion_Default___.ValidateSuiteData(_10_suite, (_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody), ((_16_derivedDataKeys).Dtor_commitmentKey()).Dtor_value().(_dafny.Sequence))
		_18_valueOrError7 = _out2
		if (_18_valueOrError7).IsFailure() {
			output = (_18_valueOrError7).PropagateFailure()
			return output
		}
		var _19___v2 _dafny.Tuple
		_ = _19___v2
		_19___v2 = (_18_valueOrError7).Extract().(_dafny.Tuple)
	}
	var _20_headerEncryptionContext _dafny.Map
	_ = _20_headerEncryptionContext
	_20_headerEncryptionContext = m_EncryptionContext.Companion_Default___.GetEncryptionContext(((_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody)).Dtor_encryptionContext())
	var _21_encryptionContextToOnlyAuthenticate _dafny.Map
	_ = _21_encryptionContextToOnlyAuthenticate
	_21_encryptionContextToOnlyAuthenticate = Companion_Default___.BuildEncryptionContextToOnlyAuthenticate(_9_decMat)
	var _22_canonicalReqEncryptionContext _dafny.Sequence
	_ = _22_canonicalReqEncryptionContext
	_22_canonicalReqEncryptionContext = m_EncryptionContext.Companion_Default___.GetCanonicalEncryptionContext(_21_encryptionContextToOnlyAuthenticate)
	var _23_serializedReqEncryptionContext _dafny.Sequence
	_ = _23_serializedReqEncryptionContext
	_23_serializedReqEncryptionContext = m_EncryptionContext.Companion_Default___.WriteEmptyEcOrWriteAAD(_22_canonicalReqEncryptionContext)
	var _24_maybeHeaderAuth m_Wrappers.Result
	_ = _24_maybeHeaderAuth
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = ((config).Dtor_crypto()).AESDecrypt(m_AwsCryptographyPrimitivesTypes.Companion_AESDecryptInput_.Create_AESDecryptInput_(((_10_suite).Dtor_encrypt()).Dtor_AES__GCM(), (_16_derivedDataKeys).Dtor_dataKey(), _dafny.SeqOf(), ((_13_headerAuth).Dtor_data().(m_HeaderTypes.HeaderAuth)).Dtor_headerAuthTag(), ((_13_headerAuth).Dtor_data().(m_HeaderTypes.HeaderAuth)).Dtor_headerIv(), _dafny.Companion_Sequence_.Concatenate(_3_rawHeader, _23_serializedReqEncryptionContext)))
	_24_maybeHeaderAuth = _out3
	if (((_24_maybeHeaderAuth).Is_Failure()) && (((config).Dtor_netV4__0__0__RetryPolicy()).Equals(m_AwsCryptographyEncryptionSdkTypes.Companion_NetV4__0__0__RetryPolicy_.Create_ALLOW__RETRY_()))) && ((_0_v4Retry) == (false)) {
		_0_v4Retry = true
		var _out4 m_Wrappers.Result
		_ = _out4
		_out4 = m_KeyDerivation.Companion_Default___.DeriveKeys(((_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody)).Dtor_messageId(), ((_9_decMat).Dtor_plaintextDataKey()).Dtor_value().(_dafny.Sequence), _10_suite, (config).Dtor_crypto(), (config).Dtor_netV4__0__0__RetryPolicy(), true)
		_14_maybeDerivedDataKeys = _out4
		var _25_valueOrError8 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _25_valueOrError8
		_25_valueOrError8 = m_Wrappers.Companion_Default___.Need((_14_maybeDerivedDataKeys).Is_Success(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Failed to derive data keys")))
		if (_25_valueOrError8).IsFailure() {
			output = (_25_valueOrError8).PropagateFailure()
			return output
		}
		_16_derivedDataKeys = (_14_maybeDerivedDataKeys).Dtor_value().(m_KeyDerivation.ExpandedKeyMaterial)
		_23_serializedReqEncryptionContext = m_EncryptionContext.Companion_Default___.WriteAAD(_22_canonicalReqEncryptionContext)
		var _out5 m_Wrappers.Result
		_ = _out5
		_out5 = ((config).Dtor_crypto()).AESDecrypt(m_AwsCryptographyPrimitivesTypes.Companion_AESDecryptInput_.Create_AESDecryptInput_(((_10_suite).Dtor_encrypt()).Dtor_AES__GCM(), (_16_derivedDataKeys).Dtor_dataKey(), _dafny.SeqOf(), ((_13_headerAuth).Dtor_data().(m_HeaderTypes.HeaderAuth)).Dtor_headerAuthTag(), ((_13_headerAuth).Dtor_data().(m_HeaderTypes.HeaderAuth)).Dtor_headerIv(), _dafny.Companion_Sequence_.Concatenate(_3_rawHeader, _23_serializedReqEncryptionContext)))
		_24_maybeHeaderAuth = _out5
	}
	var _26_valueOrError9 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _26_valueOrError9
	_26_valueOrError9 = (_24_maybeHeaderAuth).MapFailure(func(coer35 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg36 interface{}) interface{} {
			return coer35(arg36.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_27_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_27_e)
	}))
	if (_26_valueOrError9).IsFailure() {
		output = (_26_valueOrError9).PropagateFailure()
		return output
	}
	var _28___v3 _dafny.Sequence
	_ = _28___v3
	_28___v3 = (_26_valueOrError9).Extract().(_dafny.Sequence)
	var _29_header m_Header.HeaderInfo
	_ = _29_header
	_29_header = m_Header.Companion_HeaderInfo_.Create_HeaderInfo_((_2_headerBody).Dtor_data().(m_HeaderTypes.HeaderBody), _3_rawHeader, _20_headerEncryptionContext, _10_suite, (_13_headerAuth).Dtor_data().(m_HeaderTypes.HeaderAuth))
	var _30_key _dafny.Sequence
	_ = _30_key
	_30_key = (_16_derivedDataKeys).Dtor_dataKey()
	var _31_plaintext _dafny.Sequence = _dafny.EmptySeq
	_ = _31_plaintext
	var _32_messageBodyTail m_SerializeFunctions.ReadableBuffer = m_SerializeFunctions.Companion_ReadableBuffer_.Default()
	_ = _32_messageBodyTail
	var _source0 m_HeaderTypes.ContentType = ((_29_header).Dtor_body()).Dtor_contentType()
	_ = _source0
	{
		{
			if _source0.Is_NonFramed() {
				var _33_valueOrError10 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf(_dafny.EmptySeq, m_SerializeFunctions.Companion_ReadableBuffer_.Default()))
				_ = _33_valueOrError10
				var _out6 m_Wrappers.Result
				_ = _out6
				_out6 = m_EncryptDecryptHelpers.Companion_Default___.ReadAndDecryptNonFramedMessageBody((_13_headerAuth).Dtor_tail(), _29_header, _30_key, (config).Dtor_crypto())
				_33_valueOrError10 = _out6
				if (_33_valueOrError10).IsFailure() {
					output = (_33_valueOrError10).PropagateFailure()
					return output
				}
				var _34_decryptRes _dafny.Tuple
				_ = _34_decryptRes
				_34_decryptRes = (_33_valueOrError10).Extract().(_dafny.Tuple)
				_31_plaintext = (*(_34_decryptRes).IndexInt(0)).(_dafny.Sequence)
				_32_messageBodyTail = (*(_34_decryptRes).IndexInt(1)).(m_SerializeFunctions.ReadableBuffer)
				goto Lmatch0
			}
		}
		{
			var _35_valueOrError11 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf(_dafny.EmptySeq, m_SerializeFunctions.Companion_ReadableBuffer_.Default()))
			_ = _35_valueOrError11
			var _out7 m_Wrappers.Result
			_ = _out7
			_out7 = m_EncryptDecryptHelpers.Companion_Default___.ReadAndDecryptFramedMessageBody((_13_headerAuth).Dtor_tail(), _29_header, _30_key, (config).Dtor_crypto())
			_35_valueOrError11 = _out7
			if (_35_valueOrError11).IsFailure() {
				output = (_35_valueOrError11).PropagateFailure()
				return output
			}
			var _36_decryptRes _dafny.Tuple
			_ = _36_decryptRes
			_36_decryptRes = (_35_valueOrError11).Extract().(_dafny.Tuple)
			_31_plaintext = (*(_36_decryptRes).IndexInt(0)).(_dafny.Sequence)
			_32_messageBodyTail = (*(_36_decryptRes).IndexInt(1)).(m_SerializeFunctions.ReadableBuffer)
		}
		goto Lmatch0
	}
Lmatch0:
	var _37_valueOrError12 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_SerializeFunctions.Companion_ReadableBuffer_.Default())
	_ = _37_valueOrError12
	var _out8 m_Wrappers.Result
	_ = _out8
	_out8 = m_EncryptDecryptHelpers.Companion_Default___.VerifySignature(_32_messageBodyTail, ((_32_messageBodyTail).Dtor_bytes()).Subsequence(uint32((buffer).Dtor_start()), uint32((_32_messageBodyTail).Dtor_start())), _9_decMat, (config).Dtor_crypto())
	_37_valueOrError12 = _out8
	if (_37_valueOrError12).IsFailure() {
		output = (_37_valueOrError12).PropagateFailure()
		return output
	}
	var _38_signature m_SerializeFunctions.ReadableBuffer
	_ = _38_signature
	_38_signature = (_37_valueOrError12).Extract().(m_SerializeFunctions.ReadableBuffer)
	var _39_valueOrError13 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _39_valueOrError13
	_39_valueOrError13 = m_Wrappers.Companion_Default___.Need(((_38_signature).Dtor_start()) == (uint64(((_38_signature).Dtor_bytes()).Cardinality())), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Data after message footer.")))
	if (_39_valueOrError13).IsFailure() {
		output = (_39_valueOrError13).PropagateFailure()
		return output
	}
	output = m_Wrappers.Companion_Result_.Create_Success_(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptOutput_.Create_DecryptOutput_(_31_plaintext, ((_29_header).Dtor_encryptionContext()).Merge(_21_encryptionContextToOnlyAuthenticate), (((_29_header).Dtor_suite()).Dtor_id()).Dtor_ESDK()))
	return output
}
func (_static *CompanionStruct_Default___) BuildEncryptionContextToOnlyAuthenticate(decMat m_AwsCryptographyMaterialProvidersTypes.DecryptionMaterials) _dafny.Map {
	return func() _dafny.Map {
		var _coll0 = _dafny.NewMapBuilder()
		_ = _coll0
		for _iter5 := _dafny.Iterate(((decMat).Dtor_encryptionContext()).Keys().Elements()); ; {
			_compr_0, _ok5 := _iter5()
			if !_ok5 {
				break
			}
			var _0_k _dafny.Sequence
			_0_k = interface{}(_compr_0).(_dafny.Sequence)
			if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_0_k) {
				if (((decMat).Dtor_encryptionContext()).Contains(_0_k)) && (_dafny.Companion_Sequence_.Contains((decMat).Dtor_requiredEncryptionContextKeys(), _0_k)) {
					_coll0.Add(_0_k, ((decMat).Dtor_encryptionContext()).Get(_0_k).(_dafny.Sequence))
				}
			}
		}
		return _coll0.ToMap()
	}()
}

// End of class Default__

// Definition of datatype Config
type Config struct {
	Data_Config_
}

func (_this Config) Get_() Data_Config_ {
	return _this.Data_Config_
}

type Data_Config_ interface {
	isConfig()
}

type CompanionStruct_Config_ struct {
}

var Companion_Config_ = CompanionStruct_Config_{}

type Config_Config struct {
	Crypto                   *m_AtomicPrimitives.AtomicPrimitivesClient
	Mpl                      *m_MaterialProviders.MaterialProvidersClient
	CommitmentPolicy         m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	MaxEncryptedDataKeys     m_Wrappers.Option
	NetV4__0__0__RetryPolicy m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy
}

func (Config_Config) isConfig() {}

func (CompanionStruct_Config_) Create_Config_(Crypto *m_AtomicPrimitives.AtomicPrimitivesClient, Mpl *m_MaterialProviders.MaterialProvidersClient, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, MaxEncryptedDataKeys m_Wrappers.Option, NetV4__0__0__RetryPolicy m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy) Config {
	return Config{Config_Config{Crypto, Mpl, CommitmentPolicy, MaxEncryptedDataKeys, NetV4__0__0__RetryPolicy}}
}

func (_this Config) Is_Config() bool {
	_, ok := _this.Get_().(Config_Config)
	return ok
}

func (CompanionStruct_Config_) Default() Config {
	return Companion_Config_.Create_Config_((*m_AtomicPrimitives.AtomicPrimitivesClient)(nil), (*m_MaterialProviders.MaterialProvidersClient)(nil), m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Default(), m_Wrappers.Companion_Option_.Default(), m_AwsCryptographyEncryptionSdkTypes.Companion_NetV4__0__0__RetryPolicy_.Default())
}

func (_this Config) Dtor_crypto() *m_AtomicPrimitives.AtomicPrimitivesClient {
	return _this.Get_().(Config_Config).Crypto
}

func (_this Config) Dtor_mpl() *m_MaterialProviders.MaterialProvidersClient {
	return _this.Get_().(Config_Config).Mpl
}

func (_this Config) Dtor_commitmentPolicy() m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy {
	return _this.Get_().(Config_Config).CommitmentPolicy
}

func (_this Config) Dtor_maxEncryptedDataKeys() m_Wrappers.Option {
	return _this.Get_().(Config_Config).MaxEncryptedDataKeys
}

func (_this Config) Dtor_netV4__0__0__RetryPolicy() m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy {
	return _this.Get_().(Config_Config).NetV4__0__0__RetryPolicy
}

func (_this Config) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case Config_Config:
		{
			return "AwsEncryptionSdkOperations.Config.Config" + "(" + _dafny.String(data.Crypto) + ", " + _dafny.String(data.Mpl) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.MaxEncryptedDataKeys) + ", " + _dafny.String(data.NetV4__0__0__RetryPolicy) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this Config) Equals(other Config) bool {
	switch data1 := _this.Get_().(type) {
	case Config_Config:
		{
			data2, ok := other.Get_().(Config_Config)
			return ok && data1.Crypto == data2.Crypto && data1.Mpl == data2.Mpl && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.MaxEncryptedDataKeys.Equals(data2.MaxEncryptedDataKeys) && data1.NetV4__0__0__RetryPolicy.Equals(data2.NetV4__0__0__RetryPolicy)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this Config) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(Config)
	return ok && _this.Equals(typed)
}

func Type_Config_() _dafny.TypeDescriptor {
	return type_Config_{}
}

type type_Config_ struct {
}

func (_this type_Config_) Default() interface{} {
	return Companion_Config_.Default()
}

func (_this type_Config_) String() string {
	return "AwsEncryptionSdkOperations.Config"
}
func (_this Config) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = Config{}

// End of datatype Config

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
	return "AwsEncryptionSdkOperations.FrameLength"
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
	return "AwsEncryptionSdkOperations.FrameLength"
}
func (_this *CompanionStruct_FrameLength_) Is_(__source int64) bool {
	var _0_frameLength int64 = (__source)
	_ = _0_frameLength
	if true {
		return ((int64(0)) < (_0_frameLength)) && ((_0_frameLength) <= (int64(4294967295)))
	}
	return false
}
