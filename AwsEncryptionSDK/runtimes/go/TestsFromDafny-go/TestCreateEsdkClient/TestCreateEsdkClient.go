// Package TestCreateEsdkClient
// Dafny module TestCreateEsdkClient compiled into Go

package TestCreateEsdkClient

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
	m_AwsEncryptionSdkOperations "github.com/aws/aws-encryption-sdk/AwsEncryptionSdkOperations"
	m_EncryptDecryptHelpers "github.com/aws/aws-encryption-sdk/EncryptDecryptHelpers"
	m_EncryptedDataKeys "github.com/aws/aws-encryption-sdk/EncryptedDataKeys"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/EncryptionContext"
	m_EncryptionSdk "github.com/aws/aws-encryption-sdk/EncryptionSdk"
	m_Frames "github.com/aws/aws-encryption-sdk/Frames"
	m_Header "github.com/aws/aws-encryption-sdk/Header"
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/HeaderTypes"
	m_KeyDerivation "github.com/aws/aws-encryption-sdk/KeyDerivation"
	m_MessageBody "github.com/aws/aws-encryption-sdk/MessageBody"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/SharedHeaderFunctions"
	m_V1HeaderBody "github.com/aws/aws-encryption-sdk/V1HeaderBody"
	m_V2HeaderBody "github.com/aws/aws-encryption-sdk/V2HeaderBody"
	m_Fixtures "github.com/aws/aws-encryption-sdk/test/Fixtures"
	m_TestEncryptDecrypt "github.com/aws/aws-encryption-sdk/test/TestEncryptDecrypt"
	m_TestReproducedEncryptionContext "github.com/aws/aws-encryption-sdk/test/TestReproducedEncryptionContext"
	m_TestRequiredEncryptionContext "github.com/aws/aws-encryption-sdk/test/TestRequiredEncryptionContext"
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
var _ m_ComAmazonawsDynamodbTypes.Dummy__
var _ m_ComAmazonawsKmsTypes.Dummy__
var _ m_AwsCryptographyKeyStoreTypes.Dummy__
var _ m_AwsCryptographyPrimitivesTypes.Dummy__
var _ m_AwsCryptographyMaterialProvidersTypes.Dummy__
var _ m_AwsCryptographyEncryptionSdkTypes.Dummy__
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
var _ m_Streams.Dummy__
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
var _ m_AwsEncryptionSdkOperations.Dummy__
var _ m_EncryptionSdk.Dummy__
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
var _ m_Sorting.Dummy__
var _ m_HexStrings.Dummy__
var _ m_GetOpt.Dummy__
var _ m_FloatCompare.Dummy__
var _ m_Base64Lemmas.Dummy__
var _ m_Fixtures.Dummy__
var _ m_TestRequiredEncryptionContext.Dummy__
var _ m_TestReproducedEncryptionContext.Dummy__
var _ m_TestEncryptDecrypt.Dummy__

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
	return "TestCreateEsdkClient.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) TestClientCreation() {
	var _0_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _0_defaultConfig
	_0_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _1_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _1_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_0_defaultConfig)
	_1_valueOrError0 = _out0
	if !(!((_1_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(55,51): " + (_1_valueOrError0).String())
	}
	var _2_esdk m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
	_ = _2_esdk
	_2_esdk = (_1_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	if !(func(_is_0 m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient) bool {
		return _dafny.InstanceOf(_is_0, (*m_EncryptionSdk.ESDKClient)(nil))
	}(_2_esdk)) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(56,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _3_esdkClient *m_EncryptionSdk.ESDKClient
	_ = _3_esdkClient
	_3_esdkClient = _2_esdk.(*m_EncryptionSdk.ESDKClient)
	if !((((_3_esdkClient).Config()).Dtor_commitmentPolicy()).Equals(((_0_defaultConfig).Dtor_commitmentPolicy()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy))) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(59,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !((((_3_esdkClient).Config()).Dtor_maxEncryptedDataKeys()).Equals((_0_defaultConfig).Dtor_maxEncryptedDataKeys())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(60,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !((((_3_esdkClient).Config()).Dtor_netV4__0__0__RetryPolicy()).Equals(m_AwsCryptographyEncryptionSdkTypes.Companion_NetV4__0__0__RetryPolicy_.Create_ALLOW__RETRY_())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(61,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestNetRetryFlag() {
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_0_valueOrError0 = _out0
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(65,19): " + (_0_valueOrError0).String())
	}
	var _1_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _1_mpl
	_1_mpl = (_0_valueOrError0).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _2_keyNamespace _dafny.Sequence
	_ = _2_keyNamespace
	_2_keyNamespace = _dafny.SeqOfString("Some managed raw keys")
	var _3_keyName _dafny.Sequence
	_ = _3_keyName
	_3_keyName = _dafny.SeqOfString("My 256-bit AES wrapping key")
	var _4_expectedMessage _dafny.Sequence
	_ = _4_expectedMessage
	_4_expectedMessage = _dafny.SeqOf(uint8(84), uint8(104), uint8(105), uint8(115), uint8(32), uint8(105), uint8(115), uint8(32), uint8(97), uint8(32), uint8(116), uint8(101), uint8(115), uint8(116), uint8(46))
	var _5_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _5_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = (_1_mpl).CreateRawAesKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRawAesKeyringInput_.Create_CreateRawAesKeyringInput_(_2_keyNamespace, _3_keyName, _dafny.SeqCreate(32, func(coer2 func(_dafny.Int) uint8) func(_dafny.Int) interface{} {
		return func(arg2 _dafny.Int) interface{} {
			return coer2(arg2)
		}
	}(func(_6_i _dafny.Int) uint8 {
		return uint8(0)
	})), m_AwsCryptographyMaterialProvidersTypes.Companion_AesWrappingAlg_.Create_ALG__AES256__GCM__IV12__TAG16_()))
	_5_valueOrError1 = _out1
	if !(!((_5_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(70,29): " + (_5_valueOrError1).String())
	}
	var _7_rawAesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_rawAesKeyring
	_7_rawAesKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_5_valueOrError1).Extract())
	var _8_esdkConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _8_esdkConfig
	_8_esdkConfig = m_AwsCryptographyEncryptionSdkTypes.Companion_AwsEncryptionSdkConfig_.Create_AwsEncryptionSdkConfig_(m_Wrappers.Companion_Option_.Create_Some_(m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_REQUIRE__ENCRYPT__REQUIRE__DECRYPT_()), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(m_AwsCryptographyEncryptionSdkTypes.Companion_NetV4__0__0__RetryPolicy_.Create_FORBID__RETRY_()))
	var _9_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _9_valueOrError2
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = m_EncryptionSdk.Companion_Default___.ESDK(_8_esdkConfig)
	_9_valueOrError2 = _out2
	if !(!((_9_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(85,27): " + (_9_valueOrError2).String())
	}
	var _10_noRetryEsdk *m_EncryptionSdk.ESDKClient
	_ = _10_noRetryEsdk
	_10_noRetryEsdk = (_9_valueOrError2).Extract().(*m_EncryptionSdk.ESDKClient)
	var _11_expectFailureDecryptOutput m_Wrappers.Result
	_ = _11_expectFailureDecryptOutput
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = (_10_noRetryEsdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(Companion_Default___.ESDK__NET__V400__MESSAGE(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_rawAesKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_11_expectFailureDecryptOutput = _out3
	if !((_11_expectFailureDecryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(94,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _12_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _12_defaultConfig
	_12_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _13_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _13_valueOrError3
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = m_EncryptionSdk.Companion_Default___.ESDK(_12_defaultConfig)
	_13_valueOrError3 = _out4
	if !(!((_13_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(99,20): " + (_13_valueOrError3).String())
	}
	var _14_esdk *m_EncryptionSdk.ESDKClient
	_ = _14_esdk
	_14_esdk = (_13_valueOrError3).Extract().(*m_EncryptionSdk.ESDKClient)
	var _15_decryptOutput m_Wrappers.Result
	_ = _15_decryptOutput
	var _out5 m_Wrappers.Result
	_ = _out5
	_out5 = (_14_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(Companion_Default___.ESDK__NET__V400__MESSAGE(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_rawAesKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_15_decryptOutput = _out5
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(108,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	if !(_dafny.Companion_Sequence_.Equal(((_15_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext(), _4_expectedMessage)) {
		panic("dafny/AwsEncryptionSdk/test/TestCreateEsdkClient.dfy(109,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) ESDK__NET__V400__MESSAGE() _dafny.Sequence {
	return _dafny.SeqOf(uint8(2), uint8(5), uint8(120), uint8(238), uint8(5), uint8(239), uint8(107), uint8(129), uint8(136), uint8(211), uint8(103), uint8(75), uint8(18), uint8(140), uint8(11), uint8(74), uint8(26), uint8(191), uint8(92), uint8(27), uint8(202), uint8(170), uint8(33), uint8(28), uint8(9), uint8(117), uint8(252), uint8(29), uint8(29), uint8(92), uint8(213), uint8(21), uint8(231), uint8(172), uint8(234), uint8(0), uint8(95), uint8(0), uint8(1), uint8(0), uint8(21), uint8(97), uint8(119), uint8(115), uint8(45), uint8(99), uint8(114), uint8(121), uint8(112), uint8(116), uint8(111), uint8(45), uint8(112), uint8(117), uint8(98), uint8(108), uint8(105), uint8(99), uint8(45), uint8(107), uint8(101), uint8(121), uint8(0), uint8(68), uint8(65), uint8(119), uint8(102), uint8(117), uint8(103), uint8(90), uint8(99), uint8(107), uint8(57), uint8(116), uint8(100), uint8(53), uint8(104), uint8(78), uint8(108), uint8(49), uint8(78), uint8(108), uint8(75), uint8(111), uint8(47), uint8(104), uint8(105), uint8(114), uint8(53), uint8(85), uint8(47), uint8(48), uint8(81), uint8(109), uint8(98), uint8(73), uint8(111), uint8(107), uint8(79), uint8(72), uint8(81), uint8(87), uint8(97), uint8(72), uint8(83), uint8(43), uint8(115), uint8(117), uint8(119), uint8(75), uint8(73), uint8(77), uint8(82), uint8(76), uint8(99), uint8(67), uint8(80), uint8(49), uint8(54), uint8(55), uint8(56), uint8(43), uint8(49), uint8(82), uint8(75), uint8(49), uint8(48), uint8(82), uint8(101), uint8(119), uint8(61), uint8(61), uint8(0), uint8(1), uint8(0), uint8(21), uint8(83), uint8(111), uint8(109), uint8(101), uint8(32), uint8(109), uint8(97), uint8(110), uint8(97), uint8(103), uint8(101), uint8(100), uint8(32), uint8(114), uint8(97), uint8(119), uint8(32), uint8(107), uint8(101), uint8(121), uint8(115), uint8(0), uint8(47), uint8(77), uint8(121), uint8(32), uint8(50), uint8(53), uint8(54), uint8(45), uint8(98), uint8(105), uint8(116), uint8(32), uint8(65), uint8(69), uint8(83), uint8(32), uint8(119), uint8(114), uint8(97), uint8(112), uint8(112), uint8(105), uint8(110), uint8(103), uint8(32), uint8(107), uint8(101), uint8(121), uint8(0), uint8(0), uint8(0), uint8(128), uint8(0), uint8(0), uint8(0), uint8(12), uint8(229), uint8(254), uint8(197), uint8(205), uint8(110), uint8(124), uint8(222), uint8(48), uint8(217), uint8(121), uint8(252), uint8(11), uint8(0), uint8(48), uint8(64), uint8(60), uint8(232), uint8(232), uint8(76), uint8(229), uint8(15), uint8(118), uint8(224), uint8(152), uint8(79), uint8(93), uint8(113), uint8(166), uint8(255), uint8(172), uint8(255), uint8(148), uint8(185), uint8(150), uint8(195), uint8(179), uint8(78), uint8(52), uint8(186), uint8(38), uint8(216), uint8(48), uint8(118), uint8(45), uint8(113), uint8(204), uint8(71), uint8(102), uint8(116), uint8(148), uint8(199), uint8(109), uint8(178), uint8(19), uint8(2), uint8(203), uint8(150), uint8(201), uint8(65), uint8(32), uint8(199), uint8(180), uint8(2), uint8(0), uint8(0), uint8(16), uint8(0), uint8(67), uint8(72), uint8(208), uint8(112), uint8(230), uint8(137), uint8(188), uint8(187), uint8(0), uint8(28), uint8(183), uint8(198), uint8(192), uint8(45), uint8(248), uint8(108), uint8(2), uint8(129), uint8(34), uint8(42), uint8(59), uint8(155), uint8(70), uint8(117), uint8(182), uint8(216), uint8(239), uint8(27), uint8(210), uint8(78), uint8(62), uint8(104), uint8(181), uint8(247), uint8(141), uint8(50), uint8(133), uint8(42), uint8(72), uint8(200), uint8(185), uint8(57), uint8(20), uint8(49), uint8(193), uint8(240), uint8(171), uint8(140), uint8(255), uint8(255), uint8(255), uint8(255), uint8(0), uint8(0), uint8(0), uint8(1), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(1), uint8(0), uint8(0), uint8(0), uint8(15), uint8(67), uint8(37), uint8(106), uint8(11), uint8(15), uint8(23), uint8(78), uint8(239), uint8(208), uint8(185), uint8(4), uint8(36), uint8(182), uint8(9), uint8(63), uint8(62), uint8(83), uint8(97), uint8(42), uint8(250), uint8(252), uint8(185), uint8(165), uint8(14), uint8(182), uint8(231), uint8(83), uint8(176), uint8(227), uint8(191), uint8(92), uint8(0), uint8(103), uint8(48), uint8(101), uint8(2), uint8(49), uint8(0), uint8(193), uint8(152), uint8(7), uint8(169), uint8(197), uint8(137), uint8(244), uint8(88), uint8(9), uint8(1), uint8(6), uint8(56), uint8(96), uint8(13), uint8(220), uint8(201), uint8(56), uint8(16), uint8(50), uint8(68), uint8(70), uint8(36), uint8(174), uint8(38), uint8(14), uint8(241), uint8(207), uint8(11), uint8(139), uint8(154), uint8(166), uint8(224), uint8(191), uint8(20), uint8(12), uint8(175), uint8(56), uint8(117), uint8(183), uint8(120), uint8(119), uint8(228), uint8(173), uint8(130), uint8(71), uint8(110), uint8(211), uint8(189), uint8(2), uint8(48), uint8(99), uint8(98), uint8(250), uint8(36), uint8(53), uint8(182), uint8(2), uint8(204), uint8(198), uint8(55), uint8(150), uint8(51), uint8(159), uint8(101), uint8(231), uint8(34), uint8(42), uint8(30), uint8(57), uint8(204), uint8(88), uint8(114), uint8(138), uint8(94), uint8(12), uint8(79), uint8(52), uint8(71), uint8(178), uint8(34), uint8(61), uint8(246), uint8(55), uint8(163), uint8(145), uint8(95), uint8(80), uint8(61), uint8(85), uint8(143), uint8(32), uint8(0), uint8(98), uint8(20), uint8(88), uint8(251), uint8(204), uint8(5))
}

// End of class Default__
