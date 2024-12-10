// Package V2HeaderBody
// Dafny module V2HeaderBody compiled into Go

package V2HeaderBody

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
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/HeaderTypes"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/SharedHeaderFunctions"
	m_V1HeaderBody "github.com/aws/aws-encryption-sdk/V1HeaderBody"
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
	return "V2HeaderBody.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) WriteV2HeaderBody(body m_HeaderTypes.HeaderBody) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(m_SharedHeaderFunctions.Companion_Default___.WriteMessageFormatVersion(m_HeaderTypes.Companion_MessageFormatVersion_.Create_V2_()), m_SharedHeaderFunctions.Companion_Default___.WriteESDKSuiteId((body).Dtor_algorithmSuite())), m_SharedHeaderFunctions.Companion_Default___.WriteMessageId((body).Dtor_messageId())), m_EncryptionContext.Companion_Default___.WriteAADSection((body).Dtor_encryptionContext())), m_EncryptedDataKeys.Companion_Default___.WriteEncryptedDataKeysSection((body).Dtor_encryptedDataKeys())), m_SharedHeaderFunctions.Companion_Default___.WriteContentType((body).Dtor_contentType())), m_SerializeFunctions.Companion_Default___.WriteUint32((body).Dtor_frameLength())), m_SerializeFunctions.Companion_Default___.Write((body).Dtor_suiteData()))
}
func (_static *CompanionStruct_Default___) ReadV2HeaderBody(buffer m_SerializeFunctions.ReadableBuffer, maxEdks m_Wrappers.Option, mpl *m_MaterialProviders.MaterialProvidersClient) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadMessageFormatVersion(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_version m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_version
		var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_1_version).Dtor_data().(m_HeaderTypes.MessageFormatVersion)).Is_V2(), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Message version must be version 2.")))
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _3_valueOrError2 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadESDKSuiteId((_1_version).Dtor_tail(), mpl)
			_ = _3_valueOrError2
			if (_3_valueOrError2).IsFailure() {
				return (_3_valueOrError2).PropagateFailure()
			} else {
				var _4_suite m_SerializeFunctions.SuccessfulRead = (_3_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _4_suite
				var _5_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((((_4_suite).Dtor_data().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)).Dtor_commitment()).Is_HKDF(), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Algorithm suite must support commitment.")))
				_ = _5_valueOrError3
				if (_5_valueOrError3).IsFailure() {
					return (_5_valueOrError3).PropagateFailure()
				} else {
					var _6_valueOrError4 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadMessageIdV2((_4_suite).Dtor_tail())
					_ = _6_valueOrError4
					if (_6_valueOrError4).IsFailure() {
						return (_6_valueOrError4).PropagateFailure()
					} else {
						var _7_messageId m_SerializeFunctions.SuccessfulRead = (_6_valueOrError4).Extract().(m_SerializeFunctions.SuccessfulRead)
						_ = _7_messageId
						var _8_valueOrError5 m_Wrappers.Result = m_EncryptionContext.Companion_Default___.ReadAADSection((_7_messageId).Dtor_tail())
						_ = _8_valueOrError5
						if (_8_valueOrError5).IsFailure() {
							return (_8_valueOrError5).PropagateFailure()
						} else {
							var _9_encryptionContext m_SerializeFunctions.SuccessfulRead = (_8_valueOrError5).Extract().(m_SerializeFunctions.SuccessfulRead)
							_ = _9_encryptionContext
							var _10_valueOrError6 m_Wrappers.Result = m_EncryptedDataKeys.Companion_Default___.ReadEncryptedDataKeysSection((_9_encryptionContext).Dtor_tail(), maxEdks)
							_ = _10_valueOrError6
							if (_10_valueOrError6).IsFailure() {
								return (_10_valueOrError6).PropagateFailure()
							} else {
								var _11_encryptedDataKeys m_SerializeFunctions.SuccessfulRead = (_10_valueOrError6).Extract().(m_SerializeFunctions.SuccessfulRead)
								_ = _11_encryptedDataKeys
								var _12_valueOrError7 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadContentType((_11_encryptedDataKeys).Dtor_tail())
								_ = _12_valueOrError7
								if (_12_valueOrError7).IsFailure() {
									return (_12_valueOrError7).PropagateFailure()
								} else {
									var _13_contentType m_SerializeFunctions.SuccessfulRead = (_12_valueOrError7).Extract().(m_SerializeFunctions.SuccessfulRead)
									_ = _13_contentType
									var _14_valueOrError8 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32((_13_contentType).Dtor_tail())
									_ = _14_valueOrError8
									if (_14_valueOrError8).IsFailure() {
										return (_14_valueOrError8).PropagateFailure()
									} else {
										var _15_frameLength m_SerializeFunctions.SuccessfulRead = (_14_valueOrError8).Extract().(m_SerializeFunctions.SuccessfulRead)
										_ = _15_frameLength
										var _16_valueOrError9 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.Read((_15_frameLength).Dtor_tail(), _dafny.IntOfInt32(((((_4_suite).Dtor_data().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)).Dtor_commitment()).Dtor_HKDF()).Dtor_outputKeyLength()))
										_ = _16_valueOrError9
										if (_16_valueOrError9).IsFailure() {
											return (_16_valueOrError9).PropagateFailure()
										} else {
											var _17_suiteData m_SerializeFunctions.SuccessfulRead = (_16_valueOrError9).Extract().(m_SerializeFunctions.SuccessfulRead)
											_ = _17_suiteData
											var _18_body m_HeaderTypes.HeaderBody = m_HeaderTypes.Companion_HeaderBody_.Create_V2HeaderBody_((_4_suite).Dtor_data().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo), (_7_messageId).Dtor_data().(_dafny.Sequence), (_9_encryptionContext).Dtor_data().(_dafny.Sequence), (_11_encryptedDataKeys).Dtor_data().(_dafny.Sequence), (_13_contentType).Dtor_data().(m_HeaderTypes.ContentType), (_15_frameLength).Dtor_data().(uint32), (_17_suiteData).Dtor_data().(_dafny.Sequence))
											_ = _18_body
											return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_18_body, (_17_suiteData).Dtor_tail()))
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
func (_static *CompanionStruct_Default___) HeaderBytesToAADStart() _dafny.Int {
	return ((_dafny.One).Plus(_dafny.IntOfInt64(2))).Plus(_dafny.IntOfInt64(32))
}

// End of class Default__

// Definition of class V2HeaderBody
type V2HeaderBody struct {
}

func New_V2HeaderBody_() *V2HeaderBody {
	_this := V2HeaderBody{}

	return &_this
}

type CompanionStruct_V2HeaderBody_ struct {
}

var Companion_V2HeaderBody_ = CompanionStruct_V2HeaderBody_{}

func (*V2HeaderBody) String() string {
	return "V2HeaderBody.V2HeaderBody"
}

// End of class V2HeaderBody

func Type_V2HeaderBody_() _dafny.TypeDescriptor {
	return type_V2HeaderBody_{}
}

type type_V2HeaderBody_ struct {
}

func (_this type_V2HeaderBody_) Default() interface{} {
	return m_HeaderTypes.Companion_HeaderBody_.Default()
}

func (_this type_V2HeaderBody_) String() string {
	return "V2HeaderBody.V2HeaderBody"
}
func (_this *CompanionStruct_V2HeaderBody_) Is_(__source m_HeaderTypes.HeaderBody) bool {
	var _0_h m_HeaderTypes.HeaderBody = (__source)
	_ = _0_h
	return (_0_h).Is_V2HeaderBody()
}
