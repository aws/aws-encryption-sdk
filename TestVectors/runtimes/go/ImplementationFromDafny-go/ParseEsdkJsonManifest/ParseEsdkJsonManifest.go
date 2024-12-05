// Package ParseEsdkJsonManifest
// Dafny module ParseEsdkJsonManifest compiled into Go

package ParseEsdkJsonManifest

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
	m_AllAlgorithmSuites "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllAlgorithmSuites"
	m_AllDefaultCmm "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllDefaultCmm"
	m_AllHierarchy "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllHierarchy"
	m_AllKms "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKms"
	m_AllKmsEcdh "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsEcdh"
	m_AllKmsMrkAware "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsMrkAware"
	m_AllKmsMrkAwareDiscovery "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsMrkAwareDiscovery"
	m_AllKmsRsa "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsRsa"
	m_AllMulti "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllMulti"
	m_AllRawAES "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRawAES"
	m_AllRawECDH "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRawECDH"
	m_AllRawRSA "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRawRSA"
	m_AllRequiredEncryptionContextCmm "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRequiredEncryptionContextCmm"
	m_AwsCryptographyMaterialProvidersTestVectorKeysTypes "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AwsCryptographyMaterialProvidersTestVectorKeysTypes"
	m_CmmFromKeyDescription "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CmmFromKeyDescription"
	m_CompleteVectors "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CompleteVectors"
	m_CreateStaticKeyStores "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CreateStaticKeyStores"
	m_CreateStaticKeyrings "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CreateStaticKeyrings"
	m_JSONHelpers "github.com/aws/aws-cryptographic-material-providers-library/testvectors/JSONHelpers"
	m_KeyDescription "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyDescription"
	m_KeyMaterial "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyMaterial"
	m_KeyVectors "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyVectors"
	m_KeyringFromKeyDescription "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyringFromKeyDescription"
	m_KeysVectorOperations "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeysVectorOperations"
	m_MplManifestOptions "github.com/aws/aws-cryptographic-material-providers-library/testvectors/MplManifestOptions"
	m_ParseJsonManifests "github.com/aws/aws-cryptographic-material-providers-library/testvectors/ParseJsonManifests"
	m_TestManifests "github.com/aws/aws-cryptographic-material-providers-library/testvectors/TestManifests"
	m_TestVectors "github.com/aws/aws-cryptographic-material-providers-library/testvectors/TestVectors"
	m_WrappedMaterialProviders "github.com/aws/aws-cryptographic-material-providers-library/testvectors/WrappedMaterialProviders"
	m_WrappedMaterialProvidersMain "github.com/aws/aws-cryptographic-material-providers-library/testvectors/WrappedMaterialProvidersMain"
	m_WriteJsonManifests "github.com/aws/aws-cryptographic-material-providers-library/testvectors/WriteJsonManifests"
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
	m_AllEsdkV4NoReqEc "github.com/aws/aws-encryption-sdk/testvectors/AllEsdkV4NoReqEc"
	m_AllEsdkV4WithReqEc "github.com/aws/aws-encryption-sdk/testvectors/AllEsdkV4WithReqEc"
	m_EsdkManifestOptions "github.com/aws/aws-encryption-sdk/testvectors/EsdkManifestOptions"
	m_EsdkTestVectors "github.com/aws/aws-encryption-sdk/testvectors/EsdkTestVectors"
	m_WrappedESDK "github.com/aws/aws-encryption-sdk/testvectors/WrappedESDK"
	m_WriteEsdkJsonManifests "github.com/aws/aws-encryption-sdk/testvectors/WriteEsdkJsonManifests"
	m_WriteVectors "github.com/aws/aws-encryption-sdk/testvectors/WriteVectors"
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
	m_JSON_API "github.com/dafny-lang/DafnyStandardLibGo/JSON_API"
	m_JSON_ConcreteSyntax_Spec "github.com/dafny-lang/DafnyStandardLibGo/JSON_ConcreteSyntax_Spec"
	m_JSON_ConcreteSyntax_SpecProperties "github.com/dafny-lang/DafnyStandardLibGo/JSON_ConcreteSyntax_SpecProperties"
	m_JSON_Deserializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_Deserializer"
	m_JSON_Deserializer_ByteStrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Deserializer_ByteStrConversion"
	m_JSON_Deserializer_Uint16StrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Deserializer_Uint16StrConversion"
	m_JSON_Errors "github.com/dafny-lang/DafnyStandardLibGo/JSON_Errors"
	m_JSON_Grammar "github.com/dafny-lang/DafnyStandardLibGo/JSON_Grammar"
	m_JSON_Serializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_Serializer"
	m_JSON_Serializer_ByteStrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Serializer_ByteStrConversion"
	m_JSON_Spec "github.com/dafny-lang/DafnyStandardLibGo/JSON_Spec"
	m_JSON_Utils_Cursors "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Cursors"
	m_JSON_Utils_Lexers_Core "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Lexers_Core"
	m_JSON_Utils_Lexers_Strings "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Lexers_Strings"
	m_JSON_Utils_Parsers "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Parsers"
	m_JSON_Utils_Seq "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Seq"
	m_JSON_Utils_Str "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Str"
	m_JSON_Utils_Str_CharStrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Str_CharStrConversion"
	m_JSON_Utils_Str_CharStrEscaping "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Str_CharStrEscaping"
	m_JSON_Utils_Vectors "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Vectors"
	m_JSON_Utils_Views_Core "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Views_Core"
	m_JSON_Utils_Views_Writers "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Views_Writers"
	m_JSON_Values "github.com/dafny-lang/DafnyStandardLibGo/JSON_Values"
	m_JSON_ZeroCopy_API "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_API"
	m_JSON_ZeroCopy_Deserializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer"
	m_JSON_ZeroCopy_Deserializer_API "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_API"
	m_JSON_ZeroCopy_Deserializer_ArrayParams "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_ArrayParams"
	m_JSON_ZeroCopy_Deserializer_Arrays "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Arrays"
	m_JSON_ZeroCopy_Deserializer_Constants "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Constants"
	m_JSON_ZeroCopy_Deserializer_Core "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Core"
	m_JSON_ZeroCopy_Deserializer_Numbers "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Numbers"
	m_JSON_ZeroCopy_Deserializer_ObjectParams "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_ObjectParams"
	m_JSON_ZeroCopy_Deserializer_Objects "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Objects"
	m_JSON_ZeroCopy_Deserializer_Strings "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Strings"
	m_JSON_ZeroCopy_Deserializer_Values "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Values"
	m_JSON_ZeroCopy_Serializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Serializer"
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
var _ m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Dummy__
var _ m_JSON_Utils_Views_Core.Dummy__
var _ m_JSON_Utils_Views_Writers.Dummy__
var _ m_JSON_Utils_Lexers_Core.Dummy__
var _ m_JSON_Utils_Lexers_Strings.Dummy__
var _ m_JSON_Utils_Cursors.Dummy__
var _ m_JSON_Utils_Parsers.Dummy__
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
var _ m_JSON_Utils_Str_CharStrConversion.Dummy__
var _ m_JSON_Utils_Str_CharStrEscaping.Dummy__
var _ m_JSON_Utils_Str.Dummy__
var _ m_JSON_Utils_Seq.Dummy__
var _ m_JSON_Utils_Vectors.Dummy__
var _ m_JSON_Errors.Dummy__
var _ m_JSON_Values.Dummy__
var _ m__Unicode.Dummy__
var _ m_Functions.Dummy__
var _ m_Utf8EncodingForm.Dummy__
var _ m_Utf16EncodingForm.Dummy__
var _ m_UnicodeStrings.Dummy__
var _ m_JSON_Spec.Dummy__
var _ m_JSON_Grammar.Dummy__
var _ m_JSON_Serializer_ByteStrConversion.Dummy__
var _ m_JSON_Serializer.Dummy__
var _ m_JSON_Deserializer_Uint16StrConversion.Dummy__
var _ m_JSON_Deserializer_ByteStrConversion.Dummy__
var _ m_JSON_Deserializer.Dummy__
var _ m_JSON_ConcreteSyntax_Spec.Dummy__
var _ m_JSON_ConcreteSyntax_SpecProperties.Dummy__
var _ m_JSON_ZeroCopy_Serializer.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Core.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Strings.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Numbers.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_ObjectParams.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Objects.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_ArrayParams.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Arrays.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Constants.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Values.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_API.Dummy__
var _ m_JSON_ZeroCopy_Deserializer.Dummy__
var _ m_JSON_ZeroCopy_API.Dummy__
var _ m_JSON_API.Dummy__
var _ m_JSONHelpers.Dummy__
var _ m_KeyDescription.Dummy__
var _ m_HexStrings.Dummy__
var _ m_KeyMaterial.Dummy__
var _ m_CreateStaticKeyrings.Dummy__
var _ m_CreateStaticKeyStores.Dummy__
var _ m_KeyringFromKeyDescription.Dummy__
var _ m_CmmFromKeyDescription.Dummy__
var _ m_WrappedMaterialProviders.Dummy__
var _ m_KeysVectorOperations.Dummy__
var _ m_FileIO.Dummy__
var _ m_KeyVectors.Dummy__
var _ m_AwsCryptographyEncryptionSdkTypes.Dummy__
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
var _ m_MplManifestOptions.Dummy__
var _ m_GetOpt.Dummy__
var _ m_AllAlgorithmSuites.Dummy__
var _ m_TestVectors.Dummy__
var _ m_AllHierarchy.Dummy__
var _ m_AllKms.Dummy__
var _ m_AllKmsMrkAware.Dummy__
var _ m_AllKmsMrkAwareDiscovery.Dummy__
var _ m_AllKmsRsa.Dummy__
var _ m_AllKmsEcdh.Dummy__
var _ m_AllRawAES.Dummy__
var _ m_AllRawRSA.Dummy__
var _ m_AllRawECDH.Dummy__
var _ m_AllDefaultCmm.Dummy__
var _ m_AllRequiredEncryptionContextCmm.Dummy__
var _ m_AllMulti.Dummy__
var _ m_WriteJsonManifests.Dummy__
var _ m_CompleteVectors.Dummy__
var _ m_ParseJsonManifests.Dummy__
var _ m_TestManifests.Dummy__
var _ m_WrappedMaterialProvidersMain.Dummy__
var _ m_StandardLibraryInterop.Dummy__
var _ m_Sorting.Dummy__
var _ m_FloatCompare.Dummy__
var _ m_Base64Lemmas.Dummy__
var _ m_WrappedESDK.Dummy__
var _ m_EsdkManifestOptions.Dummy__
var _ m_EsdkTestVectors.Dummy__
var _ m_AllEsdkV4NoReqEc.Dummy__
var _ m_AllEsdkV4WithReqEc.Dummy__
var _ m_WriteEsdkJsonManifests.Dummy__
var _ m_WriteVectors.Dummy__

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
	return "ParseEsdkJsonManifest.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) BuildDecryptTestVector(op m_EsdkManifestOptions.ManifestOptions, version _dafny.Int, keys *m_KeyVectors.KeyVectorsClient, obj _dafny.Sequence) m_Wrappers.Result {
	var _hresult m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _hresult
	var _0_i _dafny.Int
	_ = _0_i
	_0_i = _dafny.IntOfUint32((obj).Cardinality())
	var _1_vectors _dafny.Sequence
	_ = _1_vectors
	_1_vectors = _dafny.SeqOf()
	for (_0_i).Sign() != 0 {
		_0_i = (_0_i).Minus(_dafny.One)
		var _2_test m_Wrappers.Result
		_ = _2_test
		_2_test = Companion_Default___.ToDecryptTestVectors(op, version, keys, (*((obj).Select((_0_i).Uint32()).(_dafny.Tuple)).IndexInt(0)).(_dafny.Sequence), (*((obj).Select((_0_i).Uint32()).(_dafny.Tuple)).IndexInt(1)).(m_JSON_Values.JSON))
		if ((_2_test).Is_Failure()) && (!_dafny.Companion_Sequence_.Equal((_2_test).Dtor_error().(_dafny.Sequence), Companion_Default___.NegativeTestVectorFound())) {
			_hresult = m_Wrappers.Companion_Result_.Create_Failure_(Companion_Default___.BuildTestVectorError())
			return _hresult
		}
		if (_2_test).Is_Success() {
			_1_vectors = _dafny.Companion_Sequence_.Concatenate(_dafny.SeqOf((_2_test).Dtor_value().(m_EsdkTestVectors.EsdkDecryptTestVector)), _1_vectors)
		}
		if ((_2_test).Is_Failure()) && (_dafny.Companion_Sequence_.Equal((_2_test).Dtor_error().(_dafny.Sequence), Companion_Default___.NegativeTestVectorFound())) {
			_1_vectors = _1_vectors
		}
	}
	_hresult = m_Wrappers.Companion_Result_.Create_Success_(_1_vectors)
	return _hresult
	return _hresult
}
func (_static *CompanionStruct_Default___) ToDecryptTestVectors(op m_EsdkManifestOptions.ManifestOptions, version _dafny.Int, keys *m_KeyVectors.KeyVectorsClient, name _dafny.Sequence, json m_JSON_Values.JSON) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((json).Is_Object(), _dafny.SeqOfString("Vector is not an object"))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_obj _dafny.Sequence = (json).Dtor_obj()
		_ = _1_obj
		var _source0 _dafny.Int = version
		_ = _source0
		{
			if (_source0).Cmp(_dafny.IntOfInt64(3)) == 0 {
				var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((op).Is_Decrypt(), _dafny.SeqOfString("Err parsing manifest expected Decrypt"))
				_ = _2_valueOrError1
				if (_2_valueOrError1).IsFailure() {
					return (_2_valueOrError1).PropagateFailure()
				} else {
					return Companion_Default___.V3ToDecryptTestVector(op, keys, name, _1_obj, version)
				}
			}
		}
		{
			if (_source0).Cmp(_dafny.IntOfInt64(2)) == 0 {
				var _3_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((op).Is_Decrypt(), _dafny.SeqOfString("Err parsing manifest expected Decrypt"))
				_ = _3_valueOrError2
				if (_3_valueOrError2).IsFailure() {
					return (_3_valueOrError2).PropagateFailure()
				} else {
					return Companion_Default___.V2ToDecryptTestVector(op, keys, name, _1_obj, version)
				}
			}
		}
		{
			if (_source0).Cmp(_dafny.One) == 0 {
				var _4_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((op).Is_Decrypt(), _dafny.SeqOfString("Err parsing manifest expected Decrypt"))
				_ = _4_valueOrError3
				if (_4_valueOrError3).IsFailure() {
					return (_4_valueOrError3).PropagateFailure()
				} else {
					return Companion_Default___.V1ToDecryptTestVector(op, keys, name, _1_obj, version)
				}
			}
		}
		{
			return m_Wrappers.Companion_Result_.Create_Failure_(_dafny.SeqOfString("Version not supported\n"))
		}
	}
}
func (_static *CompanionStruct_Default___) BuildEncryptTestVector(op m_EsdkManifestOptions.ManifestOptions, version _dafny.Int, keys *m_KeyVectors.KeyVectorsClient, obj _dafny.Sequence) m_Wrappers.Result {
	var _hresult m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _hresult
	var _0_i _dafny.Int
	_ = _0_i
	_0_i = _dafny.IntOfUint32((obj).Cardinality())
	var _1_vectors _dafny.Sequence
	_ = _1_vectors
	_1_vectors = _dafny.SeqOf()
	for (_0_i).Sign() != 0 {
		_0_i = (_0_i).Minus(_dafny.One)
		var _2_test m_Wrappers.Result
		_ = _2_test
		_2_test = Companion_Default___.ToEncryptTestVector(op, version, keys, (*((obj).Select((_0_i).Uint32()).(_dafny.Tuple)).IndexInt(0)).(_dafny.Sequence), (*((obj).Select((_0_i).Uint32()).(_dafny.Tuple)).IndexInt(1)).(m_JSON_Values.JSON))
		if (_2_test).Is_Failure() {
			_hresult = m_Wrappers.Companion_Result_.Create_Failure_((_2_test).Dtor_error().(_dafny.Sequence))
			return _hresult
		}
		_1_vectors = _dafny.Companion_Sequence_.Concatenate(_dafny.SeqOf((_2_test).Dtor_value().(m_EsdkTestVectors.EsdkEncryptTestVector)), _1_vectors)
	}
	_hresult = m_Wrappers.Companion_Result_.Create_Success_(_1_vectors)
	return _hresult
	return _hresult
}
func (_static *CompanionStruct_Default___) ToEncryptTestVector(op m_EsdkManifestOptions.ManifestOptions, version _dafny.Int, keys *m_KeyVectors.KeyVectorsClient, name _dafny.Sequence, json m_JSON_Values.JSON) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((json).Is_Object(), _dafny.SeqOfString("EncryptTestVector not an object"))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_obj _dafny.Sequence = (json).Dtor_obj()
		_ = _1_obj
		var _source0 _dafny.Int = version
		_ = _source0
		{
			if (_source0).Cmp(_dafny.IntOfInt64(5)) == 0 {
				return Companion_Default___.V5ToEncryptTestVector(op, keys, name, _1_obj, version)
			}
		}
		{
			return m_Wrappers.Companion_Result_.Create_Failure_(_dafny.SeqOfString("Version not supported"))
		}
	}
}
func (_static *CompanionStruct_Default___) V5ToEncryptTestVector(op m_EsdkManifestOptions.ManifestOptions, keys *m_KeyVectors.KeyVectorsClient, name _dafny.Sequence, obj _dafny.Sequence, version _dafny.Int) m_Wrappers.Result {
	var _0_scenarioString _dafny.Sequence = _dafny.SeqOfString("encryption-scenario")
	_ = _0_scenarioString
	var _1_valueOrError0 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetObject(_0_scenarioString, obj)
	_ = _1_valueOrError0
	if (_1_valueOrError0).IsFailure() {
		return (_1_valueOrError0).PropagateFailure()
	} else {
		var _2_scenario _dafny.Sequence = (_1_valueOrError0).Extract().(_dafny.Sequence)
		_ = _2_scenario
		var _3_typeString _dafny.Sequence = _dafny.SeqOfString("type")
		_ = _3_typeString
		var _4_valueOrError1 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_3_typeString, _2_scenario)
		_ = _4_valueOrError1
		if (_4_valueOrError1).IsFailure() {
			return (_4_valueOrError1).PropagateFailure()
		} else {
			var _5_typ _dafny.Sequence = (_4_valueOrError1).Extract().(_dafny.Sequence)
			_ = _5_typ
			var _6_valueOrError2 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(Companion_Default___.PlaintextJsonKey(), _2_scenario)
			_ = _6_valueOrError2
			if (_6_valueOrError2).IsFailure() {
				return (_6_valueOrError2).PropagateFailure()
			} else {
				var _7_plaintextLoc _dafny.Sequence = (_6_valueOrError2).Extract().(_dafny.Sequence)
				_ = _7_plaintextLoc
				var _8_valueOrError3 m_Wrappers.Result = m_ParseJsonManifests.Companion_Default___.GetAlgorithmSuiteInfo(_2_scenario)
				_ = _8_valueOrError3
				if (_8_valueOrError3).IsFailure() {
					return (_8_valueOrError3).PropagateFailure()
				} else {
					var _9_algorithmSuite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo = (_8_valueOrError3).Extract().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)
					_ = _9_algorithmSuite
					var _10_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_9_algorithmSuite).Dtor_id()).Is_ESDK(), _dafny.SeqOfString("Unsupported algorithmSuiteId"))
					_ = _10_valueOrError4
					if (_10_valueOrError4).IsFailure() {
						return (_10_valueOrError4).PropagateFailure()
					} else {
						var _11_valueOrError5 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetOptionalPositiveLong(Companion_Default___.FrameSizeJsonKey(), _2_scenario)
						_ = _11_valueOrError5
						if (_11_valueOrError5).IsFailure() {
							return (_11_valueOrError5).PropagateFailure()
						} else {
							var _12_frameLength m_Wrappers.Option = (_11_valueOrError5).Extract().(m_Wrappers.Option)
							_ = _12_frameLength
							var _13_valueOrError6 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.SmallObjectToStringStringMap(Companion_Default___.EncryptionContextJsonKey(), _2_scenario)
							_ = _13_valueOrError6
							if (_13_valueOrError6).IsFailure() {
								return (_13_valueOrError6).PropagateFailure()
							} else {
								var _14_encryptionContextStrings _dafny.Map = (_13_valueOrError6).Extract().(_dafny.Map)
								_ = _14_encryptionContextStrings
								var _15_valueOrError7 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.Utf8EncodeMap(_14_encryptionContextStrings)
								_ = _15_valueOrError7
								if (_15_valueOrError7).IsFailure() {
									return (_15_valueOrError7).PropagateFailure()
								} else {
									var _16_encryptionContext _dafny.Map = (_15_valueOrError7).Extract().(_dafny.Map)
									_ = _16_encryptionContext
									var _17_valueOrError8 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.SmallObjectToStringStringMap(Companion_Default___.ReproducedEncryptionContextJsonKey(), _2_scenario)
									_ = _17_valueOrError8
									if (_17_valueOrError8).IsFailure() {
										return (_17_valueOrError8).PropagateFailure()
									} else {
										var _18_reproducedEncryptionContextString _dafny.Map = (_17_valueOrError8).Extract().(_dafny.Map)
										_ = _18_reproducedEncryptionContextString
										var _19_valueOrError9 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.Utf8EncodeMap(_18_reproducedEncryptionContextString)
										_ = _19_valueOrError9
										if (_19_valueOrError9).IsFailure() {
											return (_19_valueOrError9).PropagateFailure()
										} else {
											var _20_reproducedEncryptionContext _dafny.Map = (_19_valueOrError9).Extract().(_dafny.Map)
											_ = _20_reproducedEncryptionContext
											var _21_valueOrError10 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("description"), _2_scenario)
											_ = _21_valueOrError10
											if (_21_valueOrError10).IsFailure() {
												return (_21_valueOrError10).PropagateFailure()
											} else {
												var _22_description _dafny.Sequence = (_21_valueOrError10).Extract().(_dafny.Sequence)
												_ = _22_description
												var _source0 _dafny.Sequence = _5_typ
												_ = _source0
												{
													if (_source0).Equals(_dafny.SeqOfString("positive-esdk")) {
														var _23_valueOrError11 m_Wrappers.Result = m_ParseJsonManifests.Companion_Default___.GetKeyDescription(keys, Companion_Default___.EncryptKeyDescription(), _2_scenario)
														_ = _23_valueOrError11
														if (_23_valueOrError11).IsFailure() {
															return (_23_valueOrError11).PropagateFailure()
														} else {
															var _24_encryptKeyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription = (_23_valueOrError11).Extract().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)
															_ = _24_encryptKeyDescription
															var _25_valueOrError12 m_Wrappers.Result = m_ParseJsonManifests.Companion_Default___.GetKeyDescription(keys, Companion_Default___.DecryptKeyDescription(), _2_scenario)
															_ = _25_valueOrError12
															if (_25_valueOrError12).IsFailure() {
																return (_25_valueOrError12).PropagateFailure()
															} else {
																var _26_decryptKeyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription = (_25_valueOrError12).Extract().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)
																_ = _26_decryptKeyDescription
																return m_Wrappers.Companion_Result_.Create_Success_(m_EsdkTestVectors.Companion_EsdkEncryptTestVector_.Create_PositiveEncryptTestVector_(m_Wrappers.Companion_Option_.Create_Some_(name), version, (op).Dtor_manifestPath(), (op).Dtor_decryptManifestOutput(), _7_plaintextLoc, _24_encryptKeyDescription, _26_decryptKeyDescription, m_Wrappers.Companion_Option_.Create_Some_(_16_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_20_reproducedEncryptionContext), m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_FORBID__ENCRYPT__ALLOW__DECRYPT_(), _12_frameLength, m_Wrappers.Companion_Option_.Create_Some_(_9_algorithmSuite), _22_description, m_Wrappers.Companion_Option_.Create_Some_(int64(1))))
															}
														}
													}
												}
												{
													return m_Wrappers.Companion_Result_.Create_Failure_(_dafny.Companion_Sequence_.Concatenate(_dafny.SeqOfString("Unsupported ESDK TestVector type: "), _5_typ))
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
func (_static *CompanionStruct_Default___) V1ToDecryptTestVector(op m_EsdkManifestOptions.ManifestOptions, keys *m_KeyVectors.KeyVectorsClient, name _dafny.Sequence, obj _dafny.Sequence, version _dafny.Int) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("plaintext"), obj)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_plaintextLoc _dafny.Sequence = (_0_valueOrError0).Extract().(_dafny.Sequence)
		_ = _1_plaintextLoc
		var _2_valueOrError1 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("ciphertext"), obj)
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			var _3_ciphertextLoc _dafny.Sequence = (_2_valueOrError1).Extract().(_dafny.Sequence)
			_ = _3_ciphertextLoc
			var _4_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _3_ciphertextLoc)) && (_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _1_plaintextLoc)), _dafny.SeqOfString("Invalid file prefix in test vector"))
			_ = _4_valueOrError2
			if (_4_valueOrError2).IsFailure() {
				return (_4_valueOrError2).PropagateFailure()
			} else {
				var _5_valueOrError3 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetArray(_dafny.SeqOfString("master-keys"), obj)
				_ = _5_valueOrError3
				if (_5_valueOrError3).IsFailure() {
					return (_5_valueOrError3).PropagateFailure()
				} else {
					var _6_masterKeys _dafny.Sequence = (_5_valueOrError3).Extract().(_dafny.Sequence)
					_ = _6_masterKeys
					var _7_valueOrError4 m_Wrappers.Result = Companion_Default___.GetKeyDescriptions(_6_masterKeys, keys)
					_ = _7_valueOrError4
					if (_7_valueOrError4).IsFailure() {
						return (_7_valueOrError4).PropagateFailure()
					} else {
						var _8_keyDescriptions _dafny.Sequence = (_7_valueOrError4).Extract().(_dafny.Sequence)
						_ = _8_keyDescriptions
						var _9_valueOrError5 m_Wrappers.Result = Companion_Default___.ToMultiKeyDescription(_8_keyDescriptions)
						_ = _9_valueOrError5
						if (_9_valueOrError5).IsFailure() {
							return (_9_valueOrError5).PropagateFailure()
						} else {
							var _10_keyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription = (_9_valueOrError5).Extract().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)
							_ = _10_keyDescription
							return m_Wrappers.Companion_Result_.Create_Success_(m_EsdkTestVectors.Companion_EsdkDecryptTestVector_.Create_PositiveV1OrV2DecryptTestVector_(name, version, (op).Dtor_manifestPath(), (_3_ciphertextLoc).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()), (_1_plaintextLoc).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), _10_keyDescription, m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_FORBID__ENCRYPT__ALLOW__DECRYPT_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), name, m_EsdkTestVectors.Companion_DecryptionMethod_.Create_OneShot_()))
						}
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) V2ToDecryptTestVector(op m_EsdkManifestOptions.ManifestOptions, keys *m_KeyVectors.KeyVectorsClient, name _dafny.Sequence, obj _dafny.Sequence, version _dafny.Int) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetObject(_dafny.SeqOfString("result"), obj)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_resultLoc _dafny.Sequence = (_0_valueOrError0).Extract().(_dafny.Sequence)
		_ = _1_resultLoc
		var _2_errorLoc_q m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetObject(_dafny.SeqOfString("error"), _1_resultLoc)
		_ = _2_errorLoc_q
		if (_2_errorLoc_q).Is_Success() {
			return m_Wrappers.Companion_Result_.Create_Failure_(Companion_Default___.NegativeTestVectorFound())
		} else {
			var _3_valueOrError1 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetObject(_dafny.SeqOfString("output"), _1_resultLoc)
			_ = _3_valueOrError1
			if (_3_valueOrError1).IsFailure() {
				return (_3_valueOrError1).PropagateFailure()
			} else {
				var _4_outputLoc _dafny.Sequence = (_3_valueOrError1).Extract().(_dafny.Sequence)
				_ = _4_outputLoc
				var _5_valueOrError2 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("plaintext"), _4_outputLoc)
				_ = _5_valueOrError2
				if (_5_valueOrError2).IsFailure() {
					return (_5_valueOrError2).PropagateFailure()
				} else {
					var _6_plaintextLoc _dafny.Sequence = (_5_valueOrError2).Extract().(_dafny.Sequence)
					_ = _6_plaintextLoc
					var _7_valueOrError3 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("ciphertext"), obj)
					_ = _7_valueOrError3
					if (_7_valueOrError3).IsFailure() {
						return (_7_valueOrError3).PropagateFailure()
					} else {
						var _8_ciphertextLoc _dafny.Sequence = (_7_valueOrError3).Extract().(_dafny.Sequence)
						_ = _8_ciphertextLoc
						var _9_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _8_ciphertextLoc)) && (_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _6_plaintextLoc)), _dafny.SeqOfString("Invalid file prefix in test vector"))
						_ = _9_valueOrError4
						if (_9_valueOrError4).IsFailure() {
							return (_9_valueOrError4).PropagateFailure()
						} else {
							var _10_valueOrError5 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetArray(_dafny.SeqOfString("master-keys"), obj)
							_ = _10_valueOrError5
							if (_10_valueOrError5).IsFailure() {
								return (_10_valueOrError5).PropagateFailure()
							} else {
								var _11_masterKeys _dafny.Sequence = (_10_valueOrError5).Extract().(_dafny.Sequence)
								_ = _11_masterKeys
								var _12_valueOrError6 m_Wrappers.Result = Companion_Default___.GetKeyDescriptions(_11_masterKeys, keys)
								_ = _12_valueOrError6
								if (_12_valueOrError6).IsFailure() {
									return (_12_valueOrError6).PropagateFailure()
								} else {
									var _13_keyDescriptions _dafny.Sequence = (_12_valueOrError6).Extract().(_dafny.Sequence)
									_ = _13_keyDescriptions
									var _14_valueOrError7 m_Wrappers.Result = Companion_Default___.ToMultiKeyDescription(_13_keyDescriptions)
									_ = _14_valueOrError7
									if (_14_valueOrError7).IsFailure() {
										return (_14_valueOrError7).PropagateFailure()
									} else {
										var _15_keyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription = (_14_valueOrError7).Extract().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)
										_ = _15_keyDescription
										return m_Wrappers.Companion_Result_.Create_Success_(m_EsdkTestVectors.Companion_EsdkDecryptTestVector_.Create_PositiveV1OrV2DecryptTestVector_(name, version, (op).Dtor_manifestPath(), (_8_ciphertextLoc).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()), (_6_plaintextLoc).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), _15_keyDescription, m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_FORBID__ENCRYPT__ALLOW__DECRYPT_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), name, m_EsdkTestVectors.Companion_DecryptionMethod_.Create_OneShot_()))
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
func (_static *CompanionStruct_Default___) V3ToDecryptTestVector(op m_EsdkManifestOptions.ManifestOptions, keys *m_KeyVectors.KeyVectorsClient, name _dafny.Sequence, obj _dafny.Sequence, version _dafny.Int) m_Wrappers.Result {
	var _0_scenarioString _dafny.Sequence = _dafny.SeqOfString("decryption-scenario")
	_ = _0_scenarioString
	var _1_valueOrError0 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetObject(_0_scenarioString, obj)
	_ = _1_valueOrError0
	if (_1_valueOrError0).IsFailure() {
		return (_1_valueOrError0).PropagateFailure()
	} else {
		var _2_scenario _dafny.Sequence = (_1_valueOrError0).Extract().(_dafny.Sequence)
		_ = _2_scenario
		var _3_typeString _dafny.Sequence = _dafny.SeqOfString("type")
		_ = _3_typeString
		var _4_valueOrError1 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_3_typeString, _2_scenario)
		_ = _4_valueOrError1
		if (_4_valueOrError1).IsFailure() {
			return (_4_valueOrError1).PropagateFailure()
		} else {
			var _5_typ _dafny.Sequence = (_4_valueOrError1).Extract().(_dafny.Sequence)
			_ = _5_typ
			var _6_valueOrError2 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(Companion_Default___.CiphertextJsonKey(), _2_scenario)
			_ = _6_valueOrError2
			if (_6_valueOrError2).IsFailure() {
				return (_6_valueOrError2).PropagateFailure()
			} else {
				var _7_ciphertextLoc _dafny.Sequence = (_6_valueOrError2).Extract().(_dafny.Sequence)
				_ = _7_ciphertextLoc
				var _8_valueOrError3 m_Wrappers.Result = m_ParseJsonManifests.Companion_Default___.GetAlgorithmSuiteInfo(_2_scenario)
				_ = _8_valueOrError3
				if (_8_valueOrError3).IsFailure() {
					return (_8_valueOrError3).PropagateFailure()
				} else {
					var _9_algorithmSuite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo = (_8_valueOrError3).Extract().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)
					_ = _9_algorithmSuite
					var _10_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_9_algorithmSuite).Dtor_id()).Is_ESDK(), _dafny.SeqOfString("Unsupported algorithmSuiteId"))
					_ = _10_valueOrError4
					if (_10_valueOrError4).IsFailure() {
						return (_10_valueOrError4).PropagateFailure()
					} else {
						var _11_valueOrError5 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetOptionalPositiveLong(Companion_Default___.FrameSizeJsonKey(), _2_scenario)
						_ = _11_valueOrError5
						if (_11_valueOrError5).IsFailure() {
							return (_11_valueOrError5).PropagateFailure()
						} else {
							var _12_frameLength m_Wrappers.Option = (_11_valueOrError5).Extract().(m_Wrappers.Option)
							_ = _12_frameLength
							var _13_valueOrError6 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.SmallObjectToStringStringMap(Companion_Default___.ReproducedEncryptionContextJsonKey(), _2_scenario)
							_ = _13_valueOrError6
							if (_13_valueOrError6).IsFailure() {
								return (_13_valueOrError6).PropagateFailure()
							} else {
								var _14_reproducedEncryptionContextStrings _dafny.Map = (_13_valueOrError6).Extract().(_dafny.Map)
								_ = _14_reproducedEncryptionContextStrings
								var _15_valueOrError7 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.Utf8EncodeMap(_14_reproducedEncryptionContextStrings)
								_ = _15_valueOrError7
								if (_15_valueOrError7).IsFailure() {
									return (_15_valueOrError7).PropagateFailure()
								} else {
									var _16_reproducedEncryptionContext _dafny.Map = (_15_valueOrError7).Extract().(_dafny.Map)
									_ = _16_reproducedEncryptionContext
									var _17_valueOrError8 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("description"), _2_scenario)
									_ = _17_valueOrError8
									if (_17_valueOrError8).IsFailure() {
										return (_17_valueOrError8).PropagateFailure()
									} else {
										var _18_description _dafny.Sequence = (_17_valueOrError8).Extract().(_dafny.Sequence)
										_ = _18_description
										var _19_valueOrError9 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("result"), _2_scenario)
										_ = _19_valueOrError9
										if (_19_valueOrError9).IsFailure() {
											return (_19_valueOrError9).PropagateFailure()
										} else {
											var _20_result _dafny.Sequence = (_19_valueOrError9).Extract().(_dafny.Sequence)
											_ = _20_result
											var _21_valueOrError10 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _7_ciphertextLoc)) && (_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _20_result)), _dafny.SeqOfString("Invalid file prefix in test vector"))
											_ = _21_valueOrError10
											if (_21_valueOrError10).IsFailure() {
												return (_21_valueOrError10).PropagateFailure()
											} else {
												var _source0 _dafny.Sequence = _5_typ
												_ = _source0
												{
													if (_source0).Equals(_dafny.SeqOfString("positive-esdk")) {
														var _22_valueOrError11 m_Wrappers.Result = m_ParseJsonManifests.Companion_Default___.GetKeyDescription(keys, Companion_Default___.DecryptKeyDescription(), _2_scenario)
														_ = _22_valueOrError11
														if (_22_valueOrError11).IsFailure() {
															return (_22_valueOrError11).PropagateFailure()
														} else {
															var _23_decryptKeyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription = (_22_valueOrError11).Extract().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)
															_ = _23_decryptKeyDescription
															return m_Wrappers.Companion_Result_.Create_Success_(m_EsdkTestVectors.Companion_EsdkDecryptTestVector_.Create_PositiveDecryptTestVector_(name, version, (op).Dtor_manifestPath(), (_7_ciphertextLoc).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()), (_20_result).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()), m_Wrappers.Companion_Option_.Create_Some_(_16_reproducedEncryptionContext), _23_decryptKeyDescription, m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_FORBID__ENCRYPT__ALLOW__DECRYPT_(), _12_frameLength, m_Wrappers.Companion_Option_.Create_Some_(_9_algorithmSuite), _18_description, m_EsdkTestVectors.Companion_DecryptionMethod_.Create_OneShot_()))
														}
													}
												}
												{
													return m_Wrappers.Companion_Result_.Create_Failure_(_dafny.Companion_Sequence_.Concatenate(_dafny.SeqOfString("Unsupported ESDK TestVector type: "), _5_typ))
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
func (_static *CompanionStruct_Default___) GetKeyDescriptions(keyArray _dafny.Sequence, keys *m_KeyVectors.KeyVectorsClient) m_Wrappers.Result {
	if (_dafny.IntOfUint32((keyArray).Cardinality())).Sign() == 0 {
		return m_Wrappers.Companion_Result_.Create_Success_(_dafny.SeqOf())
	} else {
		var _0_currKey m_JSON_Values.JSON = (keyArray).Select(0).(m_JSON_Values.JSON)
		_ = _0_currKey
		var _1_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_0_currKey).Is_Object(), _dafny.SeqOfString("Not an object"))
		_ = _1_valueOrError0
		if (_1_valueOrError0).IsFailure() {
			return (_1_valueOrError0).PropagateFailure()
		} else {
			var _2_valueOrError1 m_Wrappers.Result = (m_JSON_API.Companion_Default___.Serialize(_0_currKey)).MapFailure(func(coer6 func(m_JSON_Errors.SerializationError) _dafny.Sequence) func(interface{}) interface{} {
				return func(arg8 interface{}) interface{} {
					return coer6(arg8.(m_JSON_Errors.SerializationError))
				}
			}(func(_3_e m_JSON_Errors.SerializationError) _dafny.Sequence {
				return (_3_e).ToString()
			}))
			_ = _2_valueOrError1
			if (_2_valueOrError1).IsFailure() {
				return (_2_valueOrError1).PropagateFailure()
			} else {
				var _4_encryptStr _dafny.Sequence = (_2_valueOrError1).Extract().(_dafny.Sequence)
				_ = _4_encryptStr
				var _5_valueOrError2 m_Wrappers.Result = ((keys).GetKeyDescription(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_GetKeyDescriptionInput_.Create_GetKeyDescriptionInput_(_4_encryptStr))).MapFailure(func(coer7 func(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error) _dafny.Sequence) func(interface{}) interface{} {
					return func(arg9 interface{}) interface{} {
						return coer7(arg9.(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error))
					}
				}(m_ParseJsonManifests.Companion_Default___.ErrorToString))
				_ = _5_valueOrError2
				if (_5_valueOrError2).IsFailure() {
					return (_5_valueOrError2).PropagateFailure()
				} else {
					var _6_encryptDecryptKeyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.GetKeyDescriptionOutput = (_5_valueOrError2).Extract().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.GetKeyDescriptionOutput)
					_ = _6_encryptDecryptKeyDescription
					var _7_valueOrError3 m_Wrappers.Result = Companion_Default___.GetKeyDescriptions((keyArray).Drop(1), keys)
					_ = _7_valueOrError3
					if (_7_valueOrError3).IsFailure() {
						return (_7_valueOrError3).PropagateFailure()
					} else {
						var _8_tail _dafny.Sequence = (_7_valueOrError3).Extract().(_dafny.Sequence)
						_ = _8_tail
						return m_Wrappers.Companion_Result_.Create_Success_(_dafny.Companion_Sequence_.Concatenate(_dafny.SeqOf((_6_encryptDecryptKeyDescription).Dtor_keyDescription()), _8_tail))
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) ToMultiKeyDescription(keyDescriptions _dafny.Sequence) m_Wrappers.Result {
	if (_dafny.IntOfUint32((keyDescriptions).Cardinality())).Cmp(_dafny.One) == 0 {
		return m_Wrappers.Companion_Result_.Create_Success_((keyDescriptions).Select(0).(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription))
	} else {
		var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_dafny.IntOfUint32((keyDescriptions).Cardinality())).Cmp(_dafny.One) > 0, _dafny.SeqOfString("Received invalid key description length"))
		_ = _0_valueOrError0
		if (_0_valueOrError0).IsFailure() {
			return (_0_valueOrError0).PropagateFailure()
		} else {
			return m_Wrappers.Companion_Result_.Create_Success_(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_KeyDescription_.Create_Multi_(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_MultiKeyring_.Create_MultiKeyring_(m_Wrappers.Companion_Option_.Create_Some_((keyDescriptions).Select(0).(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)), (keyDescriptions).Drop(1))))
		}
	}
}
func (_static *CompanionStruct_Default___) GetPath(key _dafny.Sequence, obj _dafny.Sequence) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_JSONHelpers.Companion_Default___.GetString(key, obj)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_path _dafny.Sequence = (_0_valueOrError0).Extract().(_dafny.Sequence)
		_ = _1_path
		var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(_dafny.Companion_Sequence_.IsProperPrefixOf(Companion_Default___.FILE__PREPEND(), _1_path), _dafny.SeqOfString("Received Invalid location for plaintext or ciphertext."))
		_ = _2_valueOrError1
		if (_2_valueOrError1).IsFailure() {
			return (_2_valueOrError1).PropagateFailure()
		} else {
			return m_Wrappers.Companion_Result_.Create_Success_((_1_path).Drop((_dafny.IntOfUint32((Companion_Default___.FILE__PREPEND()).Cardinality())).Uint32()))
		}
	}
}
func (_static *CompanionStruct_Default___) Result_q(key _dafny.Sequence) bool {
	return (_dafny.Companion_Sequence_.Equal(key, _dafny.SeqOfString("output"))) || (_dafny.Companion_Sequence_.Equal(key, _dafny.SeqOfString("error")))
}
func (_static *CompanionStruct_Default___) CiphertextJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("ciphertext")
}
func (_static *CompanionStruct_Default___) FrameSizeJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("frame-size")
}
func (_static *CompanionStruct_Default___) ReproducedEncryptionContextJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("reproduced-encryption-context")
}
func (_static *CompanionStruct_Default___) DecryptKeyDescription() _dafny.Sequence {
	return _dafny.SeqOfString("decryptKeyDescription")
}
func (_static *CompanionStruct_Default___) FILE__PREPEND() _dafny.Sequence {
	return _dafny.SeqOfString("file://")
}
func (_static *CompanionStruct_Default___) NegativeTestVectorFound() _dafny.Sequence {
	return _dafny.SeqOfString("Negative test vector found; not supported yet.")
}
func (_static *CompanionStruct_Default___) BuildTestVectorError() _dafny.Sequence {
	return _dafny.SeqOfString("Error other than negative test vector found thrown")
}
func (_static *CompanionStruct_Default___) PlaintextJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("plaintext")
}
func (_static *CompanionStruct_Default___) EncryptionContextJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("encryption-context")
}
func (_static *CompanionStruct_Default___) EncryptKeyDescription() _dafny.Sequence {
	return _dafny.SeqOfString("encryptKeyDescription")
}
func (_static *CompanionStruct_Default___) MasterKeysJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("master-keys")
}
func (_static *CompanionStruct_Default___) DecryptionMethodJsonKey() _dafny.Sequence {
	return _dafny.SeqOfString("decryption-method")
}

// End of class Default__
