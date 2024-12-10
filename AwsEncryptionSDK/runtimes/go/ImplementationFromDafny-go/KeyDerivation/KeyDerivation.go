// Package KeyDerivation
// Dafny module KeyDerivation compiled into Go

package KeyDerivation

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
	m_Frames "github.com/aws/aws-encryption-sdk/Frames"
	m_Header "github.com/aws/aws-encryption-sdk/Header"
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/HeaderTypes"
	m_MessageBody "github.com/aws/aws-encryption-sdk/MessageBody"
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
var _ m_Frames.Dummy__
var _ m_MessageBody.Dummy__

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
	return "KeyDerivation.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) DeriveKey(messageId _dafny.Sequence, plaintextDataKey _dafny.Sequence, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, crypto *m_AtomicPrimitives.AtomicPrimitivesClient, onNetV4Retry bool) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_ExpandedKeyMaterial_.Default())
	_ = res
	var _source0 m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm = (suite).Dtor_kdf()
	_ = _source0
	{
		{
			if _source0.Is_IDENTITY() {
				var _0_i m_AwsCryptographyMaterialProvidersTypes.IDENTITY = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm_IDENTITY).IDENTITY
				_ = _0_i
				{
					res = m_Wrappers.Companion_Result_.Create_Success_(Companion_ExpandedKeyMaterial_.Create_ExpandedKeyMaterial_(plaintextDataKey, m_Wrappers.Companion_Option_.Create_None_()))
					return res
				}
				goto Lmatch0
			}
		}
		{
			if _source0.Is_HKDF() {
				var _1_hkdf m_AwsCryptographyMaterialProvidersTypes.HKDF = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm_HKDF).HKDF
				_ = _1_hkdf
				{
					var _2_hkdfInput m_AwsCryptographyPrimitivesTypes.HkdfInput
					_ = _2_hkdfInput
					_2_hkdfInput = m_AwsCryptographyPrimitivesTypes.Companion_HkdfInput_.Create_HkdfInput_((_1_hkdf).Dtor_hmac(), m_Wrappers.Companion_Option_.Create_None_(), plaintextDataKey, _dafny.Companion_Sequence_.Concatenate((suite).Dtor_binaryId(), messageId), (_1_hkdf).Dtor_outputKeyLength())
					if onNetV4Retry {
						_2_hkdfInput = m_AwsCryptographyPrimitivesTypes.Companion_HkdfInput_.Create_HkdfInput_((_1_hkdf).Dtor_hmac(), m_Wrappers.Companion_Option_.Create_None_(), plaintextDataKey, (suite).Dtor_binaryId(), (_1_hkdf).Dtor_outputKeyLength())
					}
					var _3_maybeDerivedKey m_Wrappers.Result
					_ = _3_maybeDerivedKey
					var _out0 m_Wrappers.Result
					_ = _out0
					_out0 = (crypto).Hkdf(_2_hkdfInput)
					_3_maybeDerivedKey = _out0
					var _4_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
					_ = _4_valueOrError0
					_4_valueOrError0 = (_3_maybeDerivedKey).MapFailure(func(coer11 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
						return func(arg12 interface{}) interface{} {
							return coer11(arg12.(m_AwsCryptographyPrimitivesTypes.Error))
						}
					}(func(_5_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
						return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_5_e)
					}))
					if (_4_valueOrError0).IsFailure() {
						res = (_4_valueOrError0).PropagateFailure()
						return res
					}
					var _6_derivedKey _dafny.Sequence
					_ = _6_derivedKey
					_6_derivedKey = (_4_valueOrError0).Extract().(_dafny.Sequence)
					res = m_Wrappers.Companion_Result_.Create_Success_(Companion_ExpandedKeyMaterial_.Create_ExpandedKeyMaterial_(_6_derivedKey, m_Wrappers.Companion_Option_.Create_None_()))
					return res
				}
				goto Lmatch0
			}
		}
		{
			var _7_None m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm = _source0
			_ = _7_None
			{
				res = m_Wrappers.Companion_Result_.Create_Failure_(m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("None is not a valid Key Derivation Function")))
				return res
			}
		}
		goto Lmatch0
	}
Lmatch0:
	return res
}
func (_static *CompanionStruct_Default___) ExpandKeyMaterial(messageId _dafny.Sequence, plaintextKey _dafny.Sequence, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_ExpandedKeyMaterial_.Default())
	_ = res
	var _0_digest m_AwsCryptographyPrimitivesTypes.DigestAlgorithm
	_ = _0_digest
	_0_digest = (((suite).Dtor_commitment()).Dtor_HKDF()).Dtor_hmac()
	var _1_info _dafny.Sequence
	_ = _1_info
	_1_info = _dafny.Companion_Sequence_.Concatenate((suite).Dtor_binaryId(), Companion_Default___.KEY__LABEL())
	var _2_hkdfExtractInput m_AwsCryptographyPrimitivesTypes.HkdfExtractInput
	_ = _2_hkdfExtractInput
	_2_hkdfExtractInput = m_AwsCryptographyPrimitivesTypes.Companion_HkdfExtractInput_.Create_HkdfExtractInput_(_0_digest, m_Wrappers.Companion_Option_.Create_Some_(messageId), plaintextKey)
	var _3_maybePseudoRandomKey m_Wrappers.Result
	_ = _3_maybePseudoRandomKey
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (crypto).HkdfExtract(_2_hkdfExtractInput)
	_3_maybePseudoRandomKey = _out0
	var _4_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _4_valueOrError0
	_4_valueOrError0 = (_3_maybePseudoRandomKey).MapFailure(func(coer12 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg13 interface{}) interface{} {
			return coer12(arg13.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_5_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_5_e)
	}))
	if (_4_valueOrError0).IsFailure() {
		res = (_4_valueOrError0).PropagateFailure()
		return res
	}
	var _6_pseudoRandomKey _dafny.Sequence
	_ = _6_pseudoRandomKey
	_6_pseudoRandomKey = (_4_valueOrError0).Extract().(_dafny.Sequence)
	var _7_encryptKeyInput m_AwsCryptographyPrimitivesTypes.HkdfExpandInput
	_ = _7_encryptKeyInput
	_7_encryptKeyInput = m_AwsCryptographyPrimitivesTypes.Companion_HkdfExpandInput_.Create_HkdfExpandInput_(_0_digest, _6_pseudoRandomKey, _1_info, (((suite).Dtor_kdf()).Dtor_HKDF()).Dtor_outputKeyLength())
	var _8_commitKeyInput m_AwsCryptographyPrimitivesTypes.HkdfExpandInput
	_ = _8_commitKeyInput
	var _9_dt__update__tmp_h0 m_AwsCryptographyPrimitivesTypes.HkdfExpandInput = _7_encryptKeyInput
	_ = _9_dt__update__tmp_h0
	var _10_dt__update_hexpectedLength_h0 int32 = (((suite).Dtor_commitment()).Dtor_HKDF()).Dtor_outputKeyLength()
	_ = _10_dt__update_hexpectedLength_h0
	var _11_dt__update_hinfo_h0 _dafny.Sequence = Companion_Default___.COMMIT__LABEL()
	_ = _11_dt__update_hinfo_h0
	_8_commitKeyInput = m_AwsCryptographyPrimitivesTypes.Companion_HkdfExpandInput_.Create_HkdfExpandInput_((_9_dt__update__tmp_h0).Dtor_digestAlgorithm(), (_9_dt__update__tmp_h0).Dtor_prk(), _11_dt__update_hinfo_h0, _10_dt__update_hexpectedLength_h0)
	var _12_maybeEncryptKey m_Wrappers.Result
	_ = _12_maybeEncryptKey
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = (crypto).HkdfExpand(_7_encryptKeyInput)
	_12_maybeEncryptKey = _out1
	var _13_maybeCommitKey m_Wrappers.Result
	_ = _13_maybeCommitKey
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = (crypto).HkdfExpand(_8_commitKeyInput)
	_13_maybeCommitKey = _out2
	var _14_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _14_valueOrError1
	_14_valueOrError1 = (_12_maybeEncryptKey).MapFailure(func(coer13 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg14 interface{}) interface{} {
			return coer13(arg14.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_15_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_15_e)
	}))
	if (_14_valueOrError1).IsFailure() {
		res = (_14_valueOrError1).PropagateFailure()
		return res
	}
	var _16_encryptKey _dafny.Sequence
	_ = _16_encryptKey
	_16_encryptKey = (_14_valueOrError1).Extract().(_dafny.Sequence)
	var _17_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _17_valueOrError2
	_17_valueOrError2 = (_13_maybeCommitKey).MapFailure(func(coer14 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg15 interface{}) interface{} {
			return coer14(arg15.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_18_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_18_e)
	}))
	if (_17_valueOrError2).IsFailure() {
		res = (_17_valueOrError2).PropagateFailure()
		return res
	}
	var _19_commitKey _dafny.Sequence
	_ = _19_commitKey
	_19_commitKey = (_17_valueOrError2).Extract().(_dafny.Sequence)
	res = m_Wrappers.Companion_Result_.Create_Success_(Companion_ExpandedKeyMaterial_.Create_ExpandedKeyMaterial_(_16_encryptKey, m_Wrappers.Companion_Option_.Create_Some_(_19_commitKey)))
	return res
	return res
}
func (_static *CompanionStruct_Default___) DeriveKeys(messageId _dafny.Sequence, plaintextKey _dafny.Sequence, suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, crypto *m_AtomicPrimitives.AtomicPrimitivesClient, netV4__0__0__RetryPolicy m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy, onNetV4Retry bool) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_ExpandedKeyMaterial_.Default())
	_ = res
	var _0_keys ExpandedKeyMaterial = Companion_ExpandedKeyMaterial_.Default()
	_ = _0_keys
	if ((suite).Dtor_messageVersion()) == (int32(2)) {
		var _1_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _1_valueOrError0
		_1_valueOrError0 = m_Wrappers.Companion_Default___.Need((((suite).Dtor_commitment()).Is_HKDF()) && (((suite).Dtor_kdf()).Equals((suite).Dtor_commitment())), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Suites with message version 2 must have commitment")))
		if (_1_valueOrError0).IsFailure() {
			res = (_1_valueOrError0).PropagateFailure()
			return res
		}
		var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _2_valueOrError1
		_2_valueOrError1 = m_Wrappers.Companion_Default___.Need(((m_SerializableTypes.Companion_Default___.GetEncryptKeyLength(suite)) == ((((suite).Dtor_kdf()).Dtor_HKDF()).Dtor_outputKeyLength())) && ((_dafny.IntOfUint32((plaintextKey).Cardinality())).Cmp(_dafny.IntOfInt32((((suite).Dtor_kdf()).Dtor_HKDF()).Dtor_inputKeyLength())) == 0), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Invalid Materials")))
		if (_2_valueOrError1).IsFailure() {
			res = (_2_valueOrError1).PropagateFailure()
			return res
		}
		var _3_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_ExpandedKeyMaterial_.Default())
		_ = _3_valueOrError2
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = Companion_Default___.ExpandKeyMaterial(messageId, plaintextKey, suite, crypto)
		_3_valueOrError2 = _out0
		if (_3_valueOrError2).IsFailure() {
			res = (_3_valueOrError2).PropagateFailure()
			return res
		}
		_0_keys = (_3_valueOrError2).Extract().(ExpandedKeyMaterial)
	} else if ((suite).Dtor_messageVersion()) == (int32(1)) {
		var _4_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _4_valueOrError3
		_4_valueOrError3 = m_Wrappers.Companion_Default___.Need(((suite).Dtor_commitment()).Is_None(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Suites with message version 1 must not have commitment")))
		if (_4_valueOrError3).IsFailure() {
			res = (_4_valueOrError3).PropagateFailure()
			return res
		}
		var _5_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _5_valueOrError4
		_5_valueOrError4 = m_Wrappers.Companion_Default___.Need(func() bool {
			var _source0 m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm = (suite).Dtor_kdf()
			_ = _source0
			{
				if _source0.Is_IDENTITY() {
					var _6_i m_AwsCryptographyMaterialProvidersTypes.IDENTITY = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm_IDENTITY).IDENTITY
					_ = _6_i
					return (_dafny.IntOfUint32((plaintextKey).Cardinality())).Cmp(_dafny.IntOfInt32(m_SerializableTypes.Companion_Default___.GetEncryptKeyLength(suite))) == 0
				}
			}
			{
				if _source0.Is_HKDF() {
					var _7_hkdf m_AwsCryptographyMaterialProvidersTypes.HKDF = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm_HKDF).HKDF
					_ = _7_hkdf
					return ((_dafny.IntOfUint32((plaintextKey).Cardinality())).Cmp(_dafny.IntOfInt32((((suite).Dtor_kdf()).Dtor_HKDF()).Dtor_inputKeyLength())) == 0) && (((((suite).Dtor_kdf()).Dtor_HKDF()).Dtor_outputKeyLength()) == (m_SerializableTypes.Companion_Default___.GetEncryptKeyLength(suite)))
				}
			}
			{
				var _8_None m_AwsCryptographyMaterialProvidersTypes.DerivationAlgorithm = _source0
				_ = _8_None
				return false
			}
		}(), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Suites with message version 1 must not have commitment")))
		if (_5_valueOrError4).IsFailure() {
			res = (_5_valueOrError4).PropagateFailure()
			return res
		}
		if ((netV4__0__0__RetryPolicy).Equals(m_AwsCryptographyEncryptionSdkTypes.Companion_NetV4__0__0__RetryPolicy_.Create_ALLOW__RETRY_())) && (onNetV4Retry) {
			var _9_valueOrError5 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_ExpandedKeyMaterial_.Default())
			_ = _9_valueOrError5
			var _out1 m_Wrappers.Result
			_ = _out1
			_out1 = Companion_Default___.DeriveKey(messageId, plaintextKey, suite, crypto, true)
			_9_valueOrError5 = _out1
			if (_9_valueOrError5).IsFailure() {
				res = (_9_valueOrError5).PropagateFailure()
				return res
			}
			_0_keys = (_9_valueOrError5).Extract().(ExpandedKeyMaterial)
		} else {
			var _10_valueOrError6 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_ExpandedKeyMaterial_.Default())
			_ = _10_valueOrError6
			var _out2 m_Wrappers.Result
			_ = _out2
			_out2 = Companion_Default___.DeriveKey(messageId, plaintextKey, suite, crypto, false)
			_10_valueOrError6 = _out2
			if (_10_valueOrError6).IsFailure() {
				res = (_10_valueOrError6).PropagateFailure()
				return res
			}
			_0_keys = (_10_valueOrError6).Extract().(ExpandedKeyMaterial)
		}
	} else {
		res = m_Wrappers.Companion_Result_.Create_Failure_(m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("Unknown message version")))
		return res
	}
	res = m_Wrappers.Companion_Result_.Create_Success_(_0_keys)
	return res
	return res
}
func (_static *CompanionStruct_Default___) KEY__LABEL() _dafny.Sequence {
	var _0_s _dafny.Sequence = _dafny.SeqOf(uint8(68), uint8(69), uint8(82), uint8(73), uint8(86), uint8(69), uint8(75), uint8(69), uint8(89))
	_ = _0_s
	return _0_s
}
func (_static *CompanionStruct_Default___) COMMIT__LABEL() _dafny.Sequence {
	var _0_s _dafny.Sequence = _dafny.SeqOf(uint8(67), uint8(79), uint8(77), uint8(77), uint8(73), uint8(84), uint8(75), uint8(69), uint8(89))
	_ = _0_s
	return _0_s
}

// End of class Default__

// Definition of datatype ExpandedKeyMaterial
type ExpandedKeyMaterial struct {
	Data_ExpandedKeyMaterial_
}

func (_this ExpandedKeyMaterial) Get_() Data_ExpandedKeyMaterial_ {
	return _this.Data_ExpandedKeyMaterial_
}

type Data_ExpandedKeyMaterial_ interface {
	isExpandedKeyMaterial()
}

type CompanionStruct_ExpandedKeyMaterial_ struct {
}

var Companion_ExpandedKeyMaterial_ = CompanionStruct_ExpandedKeyMaterial_{}

type ExpandedKeyMaterial_ExpandedKeyMaterial struct {
	DataKey       _dafny.Sequence
	CommitmentKey m_Wrappers.Option
}

func (ExpandedKeyMaterial_ExpandedKeyMaterial) isExpandedKeyMaterial() {}

func (CompanionStruct_ExpandedKeyMaterial_) Create_ExpandedKeyMaterial_(DataKey _dafny.Sequence, CommitmentKey m_Wrappers.Option) ExpandedKeyMaterial {
	return ExpandedKeyMaterial{ExpandedKeyMaterial_ExpandedKeyMaterial{DataKey, CommitmentKey}}
}

func (_this ExpandedKeyMaterial) Is_ExpandedKeyMaterial() bool {
	_, ok := _this.Get_().(ExpandedKeyMaterial_ExpandedKeyMaterial)
	return ok
}

func (CompanionStruct_ExpandedKeyMaterial_) Default() ExpandedKeyMaterial {
	return Companion_ExpandedKeyMaterial_.Create_ExpandedKeyMaterial_(_dafny.EmptySeq, m_Wrappers.Companion_Option_.Default())
}

func (_this ExpandedKeyMaterial) Dtor_dataKey() _dafny.Sequence {
	return _this.Get_().(ExpandedKeyMaterial_ExpandedKeyMaterial).DataKey
}

func (_this ExpandedKeyMaterial) Dtor_commitmentKey() m_Wrappers.Option {
	return _this.Get_().(ExpandedKeyMaterial_ExpandedKeyMaterial).CommitmentKey
}

func (_this ExpandedKeyMaterial) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case ExpandedKeyMaterial_ExpandedKeyMaterial:
		{
			return "KeyDerivation.ExpandedKeyMaterial.ExpandedKeyMaterial" + "(" + _dafny.String(data.DataKey) + ", " + _dafny.String(data.CommitmentKey) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this ExpandedKeyMaterial) Equals(other ExpandedKeyMaterial) bool {
	switch data1 := _this.Get_().(type) {
	case ExpandedKeyMaterial_ExpandedKeyMaterial:
		{
			data2, ok := other.Get_().(ExpandedKeyMaterial_ExpandedKeyMaterial)
			return ok && data1.DataKey.Equals(data2.DataKey) && data1.CommitmentKey.Equals(data2.CommitmentKey)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this ExpandedKeyMaterial) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(ExpandedKeyMaterial)
	return ok && _this.Equals(typed)
}

func Type_ExpandedKeyMaterial_() _dafny.TypeDescriptor {
	return type_ExpandedKeyMaterial_{}
}

type type_ExpandedKeyMaterial_ struct {
}

func (_this type_ExpandedKeyMaterial_) Default() interface{} {
	return Companion_ExpandedKeyMaterial_.Default()
}

func (_this type_ExpandedKeyMaterial_) String() string {
	return "KeyDerivation.ExpandedKeyMaterial"
}
func (_this ExpandedKeyMaterial) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = ExpandedKeyMaterial{}

// End of datatype ExpandedKeyMaterial
