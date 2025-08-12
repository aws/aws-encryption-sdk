// Package Header
// Dafny module Header compiled into Go

package Header

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
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderTypes"
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
	return "Header.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) HeaderVersionSupportsCommitment_q(suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, body m_HeaderTypes.HeaderBody) bool {
	return (!(((suite).Dtor_commitment()).Is_HKDF()) || (((body).Is_V2HeaderBody()) && ((uint64(((body).Dtor_suiteData()).Cardinality())) == (uint64((((suite).Dtor_commitment()).Dtor_HKDF()).Dtor_outputKeyLength()))))) && (!(!(((suite).Dtor_commitment()).Is_HKDF())) || ((true) && ((body).Is_V1HeaderBody())))
}
func (_static *CompanionStruct_Default___) ReadHeaderBody(buffer m_SerializeFunctions.ReadableBuffer, maxEdks m_Wrappers.Option, mpl *m_MaterialProviders.MaterialProvidersClient) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SharedHeaderFunctions.Companion_Default___.ReadMessageFormatVersion(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_version m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_version
		var _2_valueOrError1 m_Wrappers.Result = func() m_Wrappers.Result {
			var _source0 m_HeaderTypes.MessageFormatVersion = (_1_version).Dtor_data().(m_HeaderTypes.MessageFormatVersion)
			_ = _source0
			{
				if _source0.Is_V1() {
					var _3_valueOrError2 m_Wrappers.Result = m_V1HeaderBody.Companion_Default___.ReadV1HeaderBody(buffer, maxEdks, mpl)
					_ = _3_valueOrError2
					if (_3_valueOrError2).IsFailure() {
						return (_3_valueOrError2).PropagateFailure()
					} else {
						var _4_b m_SerializeFunctions.SuccessfulRead = (_3_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
						_ = _4_b
						var _5_body m_HeaderTypes.HeaderBody = (_4_b).Dtor_data().(m_HeaderTypes.HeaderBody)
						_ = _5_body
						return m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf(_5_body, (_4_b).Dtor_tail()))
					}
				}
			}
			{
				var _6_valueOrError3 m_Wrappers.Result = m_V2HeaderBody.Companion_Default___.ReadV2HeaderBody(buffer, maxEdks, mpl)
				_ = _6_valueOrError3
				if (_6_valueOrError3).IsFailure() {
					return (_6_valueOrError3).PropagateFailure()
				} else {
					var _7_b m_SerializeFunctions.SuccessfulRead = (_6_valueOrError3).Extract().(m_SerializeFunctions.SuccessfulRead)
					_ = _7_b
					var _8_body m_HeaderTypes.HeaderBody = (_7_b).Dtor_data().(m_HeaderTypes.HeaderBody)
					_ = _8_body
					return m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf(_8_body, (_7_b).Dtor_tail()))
				}
			}
		}()
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _let_tmp_rhs0 _dafny.Tuple = (_2_valueOrError1).Extract().(_dafny.Tuple)
			_ = _let_tmp_rhs0
			var _9_body m_HeaderTypes.HeaderBody = (*(_let_tmp_rhs0).IndexInt(0)).(m_HeaderTypes.HeaderBody)
			_ = _9_body
			var _10_tail m_SerializeFunctions.ReadableBuffer = (*(_let_tmp_rhs0).IndexInt(1)).(m_SerializeFunctions.ReadableBuffer)
			_ = _10_tail
			var _11_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((((_9_body).Dtor_contentType()).Is_Framed()) == (((_9_body).Dtor_frameLength()) > (uint32(0))), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Frame length must be positive if content is framed")))
			_ = _11_valueOrError4
			if (_11_valueOrError4).IsFailure() {
				return (_11_valueOrError4).PropagateFailure()
			} else {
				var _12_valueOrError5 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((((_9_body).Dtor_contentType()).Is_NonFramed()) == (((_9_body).Dtor_frameLength()) == (uint32(0))), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Frame length must be zero if content is non-framed")))
				_ = _12_valueOrError5
				if (_12_valueOrError5).IsFailure() {
					return (_12_valueOrError5).PropagateFailure()
				} else {
					return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_9_body, _10_tail))
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) WriteHeaderBody(body m_HeaderTypes.HeaderBody) _dafny.Sequence {
	var _source0 m_HeaderTypes.HeaderBody = body
	_ = _source0
	{
		if _source0.Is_V1HeaderBody() {
			return m_V1HeaderBody.Companion_Default___.WriteV1HeaderBody(body)
		}
	}
	{
		return m_V2HeaderBody.Companion_Default___.WriteV2HeaderBody(body)
	}
}

// End of class Default__

// Definition of datatype HeaderInfo
type HeaderInfo struct {
	Data_HeaderInfo_
}

func (_this HeaderInfo) Get_() Data_HeaderInfo_ {
	return _this.Data_HeaderInfo_
}

type Data_HeaderInfo_ interface {
	isHeaderInfo()
}

type CompanionStruct_HeaderInfo_ struct {
}

var Companion_HeaderInfo_ = CompanionStruct_HeaderInfo_{}

type HeaderInfo_HeaderInfo struct {
	Body              m_HeaderTypes.HeaderBody
	RawHeader         _dafny.Sequence
	EncryptionContext _dafny.Map
	Suite             m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
	HeaderAuth        m_HeaderTypes.HeaderAuth
}

func (HeaderInfo_HeaderInfo) isHeaderInfo() {}

func (CompanionStruct_HeaderInfo_) Create_HeaderInfo_(Body m_HeaderTypes.HeaderBody, RawHeader _dafny.Sequence, EncryptionContext _dafny.Map, Suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, HeaderAuth m_HeaderTypes.HeaderAuth) HeaderInfo {
	return HeaderInfo{HeaderInfo_HeaderInfo{Body, RawHeader, EncryptionContext, Suite, HeaderAuth}}
}

func (_this HeaderInfo) Is_HeaderInfo() bool {
	_, ok := _this.Get_().(HeaderInfo_HeaderInfo)
	return ok
}

func (CompanionStruct_HeaderInfo_) Default() HeaderInfo {
	return Companion_HeaderInfo_.Create_HeaderInfo_(m_HeaderTypes.Companion_HeaderBody_.Default(), _dafny.EmptySeq, _dafny.EmptyMap, m_AwsCryptographyMaterialProvidersTypes.Companion_AlgorithmSuiteInfo_.Default(), m_HeaderTypes.Companion_HeaderAuth_.Default())
}

func (_this HeaderInfo) Dtor_body() m_HeaderTypes.HeaderBody {
	return _this.Get_().(HeaderInfo_HeaderInfo).Body
}

func (_this HeaderInfo) Dtor_rawHeader() _dafny.Sequence {
	return _this.Get_().(HeaderInfo_HeaderInfo).RawHeader
}

func (_this HeaderInfo) Dtor_encryptionContext() _dafny.Map {
	return _this.Get_().(HeaderInfo_HeaderInfo).EncryptionContext
}

func (_this HeaderInfo) Dtor_suite() m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo {
	return _this.Get_().(HeaderInfo_HeaderInfo).Suite
}

func (_this HeaderInfo) Dtor_headerAuth() m_HeaderTypes.HeaderAuth {
	return _this.Get_().(HeaderInfo_HeaderInfo).HeaderAuth
}

func (_this HeaderInfo) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case HeaderInfo_HeaderInfo:
		{
			return "Header.HeaderInfo.HeaderInfo" + "(" + _dafny.String(data.Body) + ", " + _dafny.String(data.RawHeader) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.Suite) + ", " + _dafny.String(data.HeaderAuth) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this HeaderInfo) Equals(other HeaderInfo) bool {
	switch data1 := _this.Get_().(type) {
	case HeaderInfo_HeaderInfo:
		{
			data2, ok := other.Get_().(HeaderInfo_HeaderInfo)
			return ok && data1.Body.Equals(data2.Body) && data1.RawHeader.Equals(data2.RawHeader) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.Suite.Equals(data2.Suite) && data1.HeaderAuth.Equals(data2.HeaderAuth)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this HeaderInfo) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(HeaderInfo)
	return ok && _this.Equals(typed)
}

func Type_HeaderInfo_() _dafny.TypeDescriptor {
	return type_HeaderInfo_{}
}

type type_HeaderInfo_ struct {
}

func (_this type_HeaderInfo_) Default() interface{} {
	return Companion_HeaderInfo_.Default()
}

func (_this type_HeaderInfo_) String() string {
	return "Header.HeaderInfo"
}
func (_this HeaderInfo) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = HeaderInfo{}

// End of datatype HeaderInfo

// Definition of class Header
type Header struct {
}

func New_Header_() *Header {
	_this := Header{}

	return &_this
}

type CompanionStruct_Header_ struct {
}

var Companion_Header_ = CompanionStruct_Header_{}

func (*Header) String() string {
	return "Header.Header"
}

// End of class Header

func Type_Header_() _dafny.TypeDescriptor {
	return type_Header_{}
}

type type_Header_ struct {
}

func (_this type_Header_) Default() interface{} {
	return Companion_HeaderInfo_.Default()
}

func (_this type_Header_) String() string {
	return "Header.Header"
}
