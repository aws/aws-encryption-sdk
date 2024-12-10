// Package EsdkTestVectors
// Dafny module EsdkTestVectors compiled into Go

package EsdkTestVectors

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
	m_EsdkManifestOptions "github.com/aws/aws-encryption-sdk/testvectors/EsdkManifestOptions"
	m_WrappedESDK "github.com/aws/aws-encryption-sdk/testvectors/WrappedESDK"
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
	return "EsdkTestVectors.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) SupportedGenerateManifestVersion_q(v _dafny.Int) bool {
	return (false) || ((v).Cmp(_dafny.IntOfInt64(4)) == 0)
}
func (_static *CompanionStruct_Default___) SupportedEncryptVersion_q(v _dafny.Int) bool {
	return (((v).Cmp(_dafny.One) == 0) || ((v).Cmp(_dafny.IntOfInt64(4)) == 0)) || ((v).Cmp(_dafny.IntOfInt64(5)) == 0)
}
func (_static *CompanionStruct_Default___) SupportedDecryptVersion_q(v _dafny.Int) bool {
	return (((((v).Cmp(_dafny.One) == 0) || ((v).Cmp(_dafny.IntOfInt64(2)) == 0)) || ((v).Cmp(_dafny.IntOfInt64(3)) == 0)) || ((v).Cmp(_dafny.IntOfInt64(4)) == 0)) || ((v).Cmp(_dafny.IntOfInt64(5)) == 0)
}
func (_static *CompanionStruct_Default___) TestDecrypt(keys *m_KeyVectors.KeyVectorsClient, vector EsdkDecryptTestVector) bool {
	var output bool = false
	_ = output
	if ((vector).Dtor_algorithmSuiteId()).Is_Some() {
		var _0_id _dafny.Sequence
		_ = _0_id
		_0_id = m_AllAlgorithmSuites.Companion_Default___.ToHex(((vector).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo))
		_dafny.Print((_dafny.SeqOfString("\nTEST-DECRYPT===> ")).SetString())
		_dafny.Print(((vector).Dtor_id()).SetString())
		_dafny.Print((_dafny.SeqOfString("\n")).SetString())
		_dafny.Print((_0_id).SetString())
		_dafny.Print((_dafny.SeqOfString(" ")).SetString())
		_dafny.Print(((vector).Dtor_description()).SetString())
		_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	} else {
		_dafny.Print((_dafny.SeqOfString("\nTEST-DECRYPT===> ")).SetString())
		_dafny.Print(((vector).Dtor_id()).SetString())
		_dafny.Print((_dafny.SeqOfString("\n")).SetString())
		_dafny.Print(((vector).Dtor_description()).SetString())
		_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	}
	var _1_test_q m_Wrappers.Result
	_ = _1_test_q
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = Companion_Default___.DecryptVectorToDecryptTest(keys, vector)
	_1_test_q = _out0
	if (_1_test_q).Is_Failure() {
		_dafny.Print((_1_test_q).Dtor_error().(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error))
		_dafny.Print((_dafny.SeqOfString("\n")).SetString())
		_dafny.Print((_dafny.SeqOfString("\nFAILED! <-----------\n")).SetString())
		output = false
		return output
	}
	var _2_test DecryptTest
	_ = _2_test
	_2_test = (_1_test_q).Dtor_value().(DecryptTest)
	var _3_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _3_valueOrError0
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = Companion_Default___.ReadVectorsFile(_dafny.Companion_Sequence_.Concatenate(((_2_test).Dtor_vector()).Dtor_manifestPath(), ((_2_test).Dtor_vector()).Dtor_ciphertextPath()))
	_3_valueOrError0 = _out1
	if !(!((_3_valueOrError0).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestVectors.dfy(221,22): " + (_3_valueOrError0).String())
	}
	var _4_ciphertext _dafny.Sequence
	_ = _4_ciphertext
	_4_ciphertext = (_3_valueOrError0).Extract().(_dafny.Sequence)
	var _5_plaintext _dafny.Sequence = _dafny.EmptySeq
	_ = _5_plaintext
	if ((((_2_test).Dtor_vector()).Is_PositiveDecryptTestVector()) || (((_2_test).Dtor_vector()).Is_PositiveV1OrV2DecryptTestVector())) || (((_2_test).Dtor_vector()).Is_PositiveV4DecryptTestVector()) {
		var _6_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
		_ = _6_valueOrError1
		var _out2 m_Wrappers.Result
		_ = _out2
		_out2 = Companion_Default___.ReadVectorsFile(_dafny.Companion_Sequence_.Concatenate(((_2_test).Dtor_vector()).Dtor_manifestPath(), ((_2_test).Dtor_vector()).Dtor_plaintextPath()))
		_6_valueOrError1 = _out2
		if !(!((_6_valueOrError1).IsFailure())) {
			panic("dafny/TestVectors/src/EsdkTestVectors.dfy(227,19): " + (_6_valueOrError1).String())
		}
		_5_plaintext = (_6_valueOrError1).Extract().(_dafny.Sequence)
	}
	var _7_input m_AwsCryptographyEncryptionSdkTypes.DecryptInput
	_ = _7_input
	_7_input = m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_4_ciphertext, m_Wrappers.Companion_Option_.Create_Some_((_2_test).Dtor_cmm()), m_Wrappers.Companion_Option_.Create_None_(), ((_2_test).Dtor_vector()).Dtor_reproducedEncryptionContext())
	var _8_result m_Wrappers.Result
	_ = _8_result
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = ((_2_test).Dtor_client()).Decrypt(_7_input)
	_8_result = _out3
	var _source0 EsdkDecryptTestVector = (_2_test).Dtor_vector()
	_ = _source0
	{
		{
			if _source0.Is_PositiveDecryptTestVector() {
				output = ((_8_result).Is_Success()) && (_dafny.Companion_Sequence_.Equal(((_8_result).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext(), _5_plaintext))
				goto Lmatch0
			}
		}
		{
			if _source0.Is_NegativeDecryptTestVector() {
				output = (true) && ((_8_result).Is_Failure())
				goto Lmatch0
			}
		}
		{
			if _source0.Is_PositiveV1OrV2DecryptTestVector() {
				output = ((_8_result).Is_Success()) && (_dafny.Companion_Sequence_.Equal(((_8_result).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext(), _5_plaintext))
				goto Lmatch0
			}
		}
		{
			output = ((_8_result).Is_Success()) && (_dafny.Companion_Sequence_.Equal(((_8_result).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext(), _5_plaintext))
		}
		goto Lmatch0
	}
Lmatch0:
	if !(output) {
		if (((((_2_test).Dtor_vector()).Is_PositiveDecryptTestVector()) || (((_2_test).Dtor_vector()).Is_PositiveV1OrV2DecryptTestVector())) || (((_2_test).Dtor_vector()).Is_PositiveV4DecryptTestVector())) && ((_8_result).Is_Failure()) {
			_dafny.Print((_8_result).Dtor_error().(m_AwsCryptographyEncryptionSdkTypes.Error))
			_dafny.Print((_dafny.SeqOfString("\n")).SetString())
			if (((_8_result).Dtor_error().(m_AwsCryptographyEncryptionSdkTypes.Error)).Is_AwsCryptographyMaterialProviders()) && ((((_8_result).Dtor_error().(m_AwsCryptographyEncryptionSdkTypes.Error)).Dtor_AwsCryptographyMaterialProviders()).Is_CollectionOfErrors()) {
				_dafny.Print((_dafny.SeqOfString("list:")).SetString())
				_dafny.Print((((_8_result).Dtor_error().(m_AwsCryptographyEncryptionSdkTypes.Error)).Dtor_AwsCryptographyMaterialProviders()).Dtor_list())
				_dafny.Print((_dafny.SeqOfString("\n")).SetString())
			}
		}
		_dafny.Print((_dafny.SeqOfString("\nFAILED! <-----------\n")).SetString())
	}
	return output
}
func (_static *CompanionStruct_Default___) DecryptVectorToDecryptTest(keys *m_KeyVectors.KeyVectorsClient, vector EsdkDecryptTestVector) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Result{}
	_ = output
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _0_valueOrError0
	_0_valueOrError0 = m_Wrappers.Companion_Default___.Need(!((vector).Is_NegativeDecryptTestVector()), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_KeyVectorException_(_dafny.SeqOfString("Negative Test Vectors not supported at this time")))
	if (_0_valueOrError0).IsFailure() {
		output = (_0_valueOrError0).PropagateFailure()
		return output
	}
	var _1_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _1_valueOrError1
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (keys).CreateWrappedTestVectorCmm(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_TestVectorCmmInput_.Create_TestVectorCmmInput_((vector).Dtor_decryptDescriptions(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_CmmOperation_.Create_DECRYPT_()))
	_1_valueOrError1 = _out0
	if (_1_valueOrError1).IsFailure() {
		output = (_1_valueOrError1).PropagateFailure()
		return output
	}
	var _2_cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _2_cmm
	_2_cmm = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_1_valueOrError1).Extract())
	var _3_commitmentPolicy m_AwsCryptographyMaterialProvidersTypes.CommitmentPolicy
	_ = _3_commitmentPolicy
	if ((vector).Dtor_algorithmSuiteId()).Is_Some() {
		_3_commitmentPolicy = m_AllAlgorithmSuites.Companion_Default___.GetCompatibleCommitmentPolicy(((vector).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo))
	} else {
		_3_commitmentPolicy = m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_FORBID__ENCRYPT__ALLOW__DECRYPT_())
	}
	var _4_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _4_valueOrError2
	_4_valueOrError2 = m_Wrappers.Companion_Default___.Need((_3_commitmentPolicy).Is_ESDK(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_KeyVectorException_(_dafny.SeqOfString("Compatible commitment policy is not for ESDK")))
	if (_4_valueOrError2).IsFailure() {
		output = (_4_valueOrError2).PropagateFailure()
		return output
	}
	var _5_config m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _5_config
	if (vector).Is_PositiveV4DecryptTestVector() {
		_5_config = m_WrappedESDK.Companion_Default___.WrappedAwsEncryptionSdkConfigWithSuppliedCommitmentRetryPolicy((_3_commitmentPolicy).Dtor_ESDK(), (vector).Dtor_retryPolicy())
	} else {
		_5_config = m_WrappedESDK.Companion_Default___.WrappedAwsEncryptionSdkConfigWithSuppliedCommitment((_3_commitmentPolicy).Dtor_ESDK())
	}
	var _6_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _6_valueOrError3
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_WrappedESDK.Companion_Default___.WrappedESDK(_5_config)
	_6_valueOrError3 = _out1
	if !(!((_6_valueOrError3).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestVectors.dfy(312,18): " + (_6_valueOrError3).String())
	}
	var _7_client m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
	_ = _7_client
	_7_client = m_AwsCryptographyEncryptionSdkTypes.Companion_IAwsEncryptionSdkClient_.CastTo_((_6_valueOrError3).Extract())
	var _8_test DecryptTest
	_ = _8_test
	_8_test = Companion_DecryptTest_.Create_DecryptTest_(_2_cmm, _7_client, vector)
	output = m_Wrappers.Companion_Result_.Create_Success_(_8_test)
	return output
}
func (_static *CompanionStruct_Default___) TestEncrypt(plaintexts _dafny.Map, keys *m_KeyVectors.KeyVectorsClient, test EncryptTest) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_EncryptTestOutput_.Default())
	_ = output
	var _0_id _dafny.Sequence
	_ = _0_id
	_0_id = m_AllAlgorithmSuites.Companion_Default___.ToHex((((test).Dtor_vector()).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo))
	_dafny.Print((_dafny.SeqOfString("\nTEST-ENCRYPT===> ")).SetString())
	_dafny.Print(((((test).Dtor_vector()).Dtor_id()).Dtor_value().(_dafny.Sequence)).SetString())
	_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	_dafny.Print((_0_id).SetString())
	_dafny.Print((_dafny.SeqOfString(" ")).SetString())
	_dafny.Print((((test).Dtor_vector()).Dtor_description()).SetString())
	_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	var _1_vector EsdkEncryptTestVector
	_ = _1_vector
	_1_vector = (test).Dtor_vector()
	if !((plaintexts).Contains(((test).Dtor_vector()).Dtor_plaintextPath())) {
		panic("dafny/TestVectors/src/EsdkTestVectors.dfy(355,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _2_plaintext _dafny.Sequence
	_ = _2_plaintext
	_2_plaintext = (plaintexts).Get(((test).Dtor_vector()).Dtor_plaintextPath()).(_dafny.Sequence)
	var _3_frameLength m_Wrappers.Option
	_ = _3_frameLength
	_3_frameLength = (_1_vector).Dtor_frameLength()
	var _4_input m_AwsCryptographyEncryptionSdkTypes.EncryptInput
	_ = _4_input
	_4_input = m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_2_plaintext, ((test).Dtor_vector()).Dtor_encryptionContext(), m_Wrappers.Companion_Option_.Create_Some_((test).Dtor_cmm()), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_((((((test).Dtor_vector()).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)).Dtor_id()).Dtor_ESDK()), _3_frameLength)
	var _5_result m_Wrappers.Result
	_ = _5_result
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = ((test).Dtor_client()).Encrypt(_4_input)
	_5_result = _out0
	if ((_5_result).Is_Success()) && ((((test).Dtor_vector()).Is_PositiveEncryptTestVector()) || (((test).Dtor_vector()).Is_PositiveEncryptNegativeDecryptTestVector())) {
		var _6_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_EsdkDecryptTestVector_.Default())
		_ = _6_valueOrError0
		var _out1 m_Wrappers.Result
		_ = _out1
		_out1 = Companion_Default___.EncryptTestToDecryptVector(test, (_5_result).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput))
		_6_valueOrError0 = _out1
		if (_6_valueOrError0).IsFailure() {
			output = (_6_valueOrError0).PropagateFailure()
			return output
		}
		var _7_decryptVector EsdkDecryptTestVector
		_ = _7_decryptVector
		_7_decryptVector = (_6_valueOrError0).Extract().(EsdkDecryptTestVector)
		output = m_Wrappers.Companion_Result_.Create_Success_(Companion_EncryptTestOutput_.Create_EncryptTestOutput_(true, m_Wrappers.Companion_Option_.Create_Some_(_7_decryptVector)))
	} else if ((_5_result).Is_Failure()) && (((test).Dtor_vector()).Is_NegativeEncryptTestVector()) {
		output = m_Wrappers.Companion_Result_.Create_Success_(Companion_EncryptTestOutput_.Create_EncryptTestOutput_(true, m_Wrappers.Companion_Option_.Create_None_()))
	} else {
		output = m_Wrappers.Companion_Result_.Create_Success_(Companion_EncryptTestOutput_.Create_EncryptTestOutput_(false, m_Wrappers.Companion_Option_.Create_None_()))
		if (!(((test).Dtor_vector()).Is_NegativeEncryptTestVector())) && ((_5_result).Is_Failure()) {
			_dafny.Print((_5_result).Dtor_error().(m_AwsCryptographyEncryptionSdkTypes.Error))
		}
		_dafny.Print((_dafny.SeqOfString("\nFAILED! <-----------\n")).SetString())
	}
	return output
}
func (_static *CompanionStruct_Default___) EncryptVectorToEncryptTest(keys *m_KeyVectors.KeyVectorsClient, vector EsdkEncryptTestVector) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Result{}
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (keys).CreateWrappedTestVectorCmm(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_TestVectorCmmInput_.Create_TestVectorCmmInput_((func() m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription {
		if (vector).Is_PositiveEncryptTestVector() {
			return (vector).Dtor_encryptDescriptions()
		}
		return (func() m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription {
			if (vector).Is_PositiveEncryptNegativeDecryptTestVector() {
				return (vector).Dtor_encryptDescriptions()
			}
			return (vector).Dtor_encryptDescriptions()
		})()
	})(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_CmmOperation_.Create_ENCRYPT_()))
	_0_valueOrError0 = _out0
	if (_0_valueOrError0).IsFailure() {
		output = (_0_valueOrError0).PropagateFailure()
		return output
	}
	var _1_cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _1_cmm
	_1_cmm = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_0_valueOrError0).Extract())
	var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _2_valueOrError1
	_2_valueOrError1 = m_Wrappers.Companion_Default___.Need(((vector).Dtor_algorithmSuiteId()).Is_Some(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_KeyVectorException_(_dafny.SeqOfString("Missing AlgorithmSuiteId in test vector")))
	if (_2_valueOrError1).IsFailure() {
		output = (_2_valueOrError1).PropagateFailure()
		return output
	}
	var _3_commitmentPolicy m_AwsCryptographyMaterialProvidersTypes.CommitmentPolicy
	_ = _3_commitmentPolicy
	_3_commitmentPolicy = m_AllAlgorithmSuites.Companion_Default___.GetCompatibleCommitmentPolicy(((vector).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo))
	var _4_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _4_valueOrError2
	_4_valueOrError2 = m_Wrappers.Companion_Default___.Need((_3_commitmentPolicy).Is_ESDK(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_KeyVectorException_(_dafny.SeqOfString("Compatible commitment policy is not for ESDK")))
	if (_4_valueOrError2).IsFailure() {
		output = (_4_valueOrError2).PropagateFailure()
		return output
	}
	var _5_config m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _5_config
	_5_config = m_WrappedESDK.Companion_Default___.WrappedAwsEncryptionSdkConfigWithSuppliedCommitment((_3_commitmentPolicy).Dtor_ESDK())
	var _6_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _6_valueOrError3
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_WrappedESDK.Companion_Default___.WrappedESDK(_5_config)
	_6_valueOrError3 = _out1
	if !(!((_6_valueOrError3).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestVectors.dfy(428,18): " + (_6_valueOrError3).String())
	}
	var _7_client m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
	_ = _7_client
	_7_client = m_AwsCryptographyEncryptionSdkTypes.Companion_IAwsEncryptionSdkClient_.CastTo_((_6_valueOrError3).Extract())
	var _8_test EncryptTest
	_ = _8_test
	_8_test = Companion_EncryptTest_.Create_EncryptTest_(_1_cmm, _7_client, vector)
	output = m_Wrappers.Companion_Result_.Create_Success_(_8_test)
	return output
}
func (_static *CompanionStruct_Default___) EncryptTestToDecryptVector(test EncryptTest, result m_AwsCryptographyEncryptionSdkTypes.EncryptOutput) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(Companion_EsdkDecryptTestVector_.Default())
	_ = output
	var _source0 EsdkEncryptTestVector = (test).Dtor_vector()
	_ = _source0
	{
		{
			if _source0.Is_PositiveEncryptTestVector() {
				output = m_Wrappers.Companion_Result_.Create_Success_(Companion_EsdkDecryptTestVector_.Create_PositiveDecryptTestVector_((((test).Dtor_vector()).Dtor_id()).Dtor_value().(_dafny.Sequence), _dafny.IntOfInt64(3), ((test).Dtor_vector()).Dtor_decryptManifestPath(), Companion_Default___.CiphertextPathPathRoot(), _dafny.Companion_Sequence_.Concatenate(Companion_Default___.PlaintextPathRoot(), ((test).Dtor_vector()).Dtor_plaintextPath()), ((test).Dtor_vector()).Dtor_reproducedEncryptionContext(), ((test).Dtor_vector()).Dtor_decryptDescriptions(), ((test).Dtor_vector()).Dtor_commitmentPolicy(), ((test).Dtor_vector()).Dtor_frameLength(), ((test).Dtor_vector()).Dtor_algorithmSuiteId(), ((test).Dtor_vector()).Dtor_description(), Companion_DecryptionMethod_.Create_OneShot_()))
				goto Lmatch0
			}
		}
		{
			output = m_Wrappers.Companion_Result_.Create_Failure_(_dafny.SeqOfString("Only postive tests supported"))
		}
		goto Lmatch0
	}
Lmatch0:
	var _0_decryptManifestCiphertext _dafny.Sequence
	_ = _0_decryptManifestCiphertext
	_0_decryptManifestCiphertext = _dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(((test).Dtor_vector()).Dtor_decryptManifestPath(), Companion_Default___.CiphertextPathPathRoot()), (((test).Dtor_vector()).Dtor_id()).Dtor_value().(_dafny.Sequence))
	var _1_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _1_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = Companion_Default___.WriteVectorsFile(_0_decryptManifestCiphertext, (result).Dtor_ciphertext())
	_1_valueOrError0 = _out0
	if !(!((_1_valueOrError0).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestVectors.dfy(470,13): " + (_1_valueOrError0).String())
	}
	var _2___v67 _dafny.Tuple
	_ = _2___v67
	_2___v67 = (_1_valueOrError0).Extract().(_dafny.Tuple)
	return output
}
func (_static *CompanionStruct_Default___) MplPrintErr(e m_AwsCryptographyMaterialProvidersTypes.Error) _dafny.Tuple {
	var _hresult _dafny.Tuple = _dafny.TupleOf()
	_ = _hresult
	_dafny.Print(e)
	_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	_hresult = _dafny.TupleOf()
	return _hresult
	return _hresult
}
func (_static *CompanionStruct_Default___) MplVectorPrintErr(e m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error) _dafny.Tuple {
	var _hresult _dafny.Tuple = _dafny.TupleOf()
	_ = _hresult
	_dafny.Print(e)
	_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	_hresult = _dafny.TupleOf()
	return _hresult
	return _hresult
}
func (_static *CompanionStruct_Default___) KeyDescriptionToCmm(keys *m_KeyVectors.KeyVectorsClient, keyDescriptions _dafny.Sequence) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Result{}
	_ = output
	var _0_keyringList _dafny.Sequence
	_ = _0_keyringList
	_0_keyringList = _dafny.SeqOf()
	var _hi0 _dafny.Int = _dafny.IntOfUint32((keyDescriptions).Cardinality())
	_ = _hi0
	for _1_i := _dafny.Zero; _1_i.Cmp(_hi0) < 0; _1_i = _1_i.Plus(_dafny.One) {
		var _2_keyDescription m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
		_ = _2_keyDescription
		_2_keyDescription = (keyDescriptions).Select((_1_i).Uint32()).(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription)
		var _3_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
		_ = _3_valueOrError0
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = (keys).CreateWrappedTestVectorKeyring(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_TestVectorKeyringInput_.Create_TestVectorKeyringInput_(_2_keyDescription))
		_3_valueOrError0 = _out0
		if (_3_valueOrError0).IsFailure() {
			output = (_3_valueOrError0).PropagateFailure()
			return output
		}
		var _4_keyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
		_ = _4_keyring
		_4_keyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_3_valueOrError0).Extract())
		_0_keyringList = _dafny.Companion_Sequence_.Concatenate(_0_keyringList, _dafny.SeqOf(_4_keyring))
	}
	var _5_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _5_valueOrError1
	_5_valueOrError1 = m_Wrappers.Companion_Default___.Need((_dafny.IntOfUint32((_0_keyringList).Cardinality())).Cmp(_dafny.One) == 0, m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_KeyVectorException_(_dafny.SeqOfString("Failed to create any keyrings")))
	if (_5_valueOrError1).IsFailure() {
		output = (_5_valueOrError1).PropagateFailure()
		return output
	}
	var _6_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _6_valueOrError2
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_WrappedMaterialProviders.Companion_Default___.WrappedMaterialProviders(m_WrappedMaterialProviders.Companion_Default___.WrappedDefaultMaterialProvidersConfig())
	_6_valueOrError2 = _out1
	if !(!((_6_valueOrError2).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestVectors.dfy(513,15): " + (_6_valueOrError2).String())
	}
	var _7_mpl m_AwsCryptographyMaterialProvidersTypes.IAwsCryptographicMaterialProvidersClient
	_ = _7_mpl
	_7_mpl = m_AwsCryptographyMaterialProvidersTypes.Companion_IAwsCryptographicMaterialProvidersClient_.CastTo_((_6_valueOrError2).Extract())
	var _8_generatorKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_generatorKeyring
	_8_generatorKeyring = (_0_keyringList).Select(0).(m_AwsCryptographyMaterialProvidersTypes.IKeyring)
	var _9_maybeMultiKeyring m_Wrappers.Result
	_ = _9_maybeMultiKeyring
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = (_7_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_generatorKeyring), (_0_keyringList).Drop(1)))
	_9_maybeMultiKeyring = _out2
	var _10_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError3
	_10_valueOrError3 = (_9_maybeMultiKeyring).MapFailure(func(coer0 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error) func(interface{}) interface{} {
		return func(arg0 interface{}) interface{} {
			return coer0(arg0.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_11_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error {
		return m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_11_e)
	}))
	if (_10_valueOrError3).IsFailure() {
		output = (_10_valueOrError3).PropagateFailure()
		return output
	}
	var _12_keyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _12_keyring
	_12_keyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError3).Extract())
	var _13_maybeCmm m_Wrappers.Result
	_ = _13_maybeCmm
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = (_7_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_9_maybeMultiKeyring).Dtor_value())))
	_13_maybeCmm = _out3
	output = (_13_maybeCmm).MapFailure(func(coer1 func(m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error) func(interface{}) interface{} {
		return func(arg1 interface{}) interface{} {
			return coer1(arg1.(m_AwsCryptographyMaterialProvidersTypes.Error))
		}
	}(func(_14_e m_AwsCryptographyMaterialProvidersTypes.Error) m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error {
		return m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_Error_.Create_AwsCryptographyMaterialProviders_(_14_e)
	}))
	return output
}
func (_static *CompanionStruct_Default___) ReadVectorsFile(location _dafny.Sequence) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_FileIO.Companion_Default___.ReadBytesFromFile(location)
	_0_valueOrError0 = _out0
	if (_0_valueOrError0).IsFailure() {
		output = (_0_valueOrError0).PropagateFailure()
		return output
	}
	var _1_fileBv _dafny.Sequence
	_ = _1_fileBv
	_1_fileBv = (_0_valueOrError0).Extract().(_dafny.Sequence)
	output = m_Wrappers.Companion_Result_.Create_Success_(m_JSONHelpers.Companion_Default___.BvToBytes(_1_fileBv))
	return output
}
func (_static *CompanionStruct_Default___) WriteVectorsFile(location _dafny.Sequence, bytes _dafny.Sequence) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = output
	var _0_bv _dafny.Sequence
	_ = _0_bv
	_0_bv = m_JSONHelpers.Companion_Default___.BytesBv(bytes)
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_FileIO.Companion_Default___.WriteBytesToFile(location, _0_bv)
	output = _out0
	return output
}
func (_static *CompanionStruct_Default___) CiphertextPathPathRoot() _dafny.Sequence {
	return _dafny.SeqOfString("ciphertexts/")
}
func (_static *CompanionStruct_Default___) PlaintextPathRoot() _dafny.Sequence {
	return _dafny.SeqOfString("plaintexts/")
}

// End of class Default__

// Definition of datatype EncryptTest
type EncryptTest struct {
	Data_EncryptTest_
}

func (_this EncryptTest) Get_() Data_EncryptTest_ {
	return _this.Data_EncryptTest_
}

type Data_EncryptTest_ interface {
	isEncryptTest()
}

type CompanionStruct_EncryptTest_ struct {
}

var Companion_EncryptTest_ = CompanionStruct_EncryptTest_{}

type EncryptTest_EncryptTest struct {
	Cmm    m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	Client m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
	Vector EsdkEncryptTestVector
}

func (EncryptTest_EncryptTest) isEncryptTest() {}

func (CompanionStruct_EncryptTest_) Create_EncryptTest_(Cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager, Client m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient, Vector EsdkEncryptTestVector) EncryptTest {
	return EncryptTest{EncryptTest_EncryptTest{Cmm, Client, Vector}}
}

func (_this EncryptTest) Is_EncryptTest() bool {
	_, ok := _this.Get_().(EncryptTest_EncryptTest)
	return ok
}

func (CompanionStruct_EncryptTest_) Default() EncryptTest {
	return Companion_EncryptTest_.Create_EncryptTest_((m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager)(nil), (m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient)(nil), Companion_EsdkEncryptTestVector_.Default())
}

func (_this EncryptTest) Dtor_cmm() m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager {
	return _this.Get_().(EncryptTest_EncryptTest).Cmm
}

func (_this EncryptTest) Dtor_client() m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient {
	return _this.Get_().(EncryptTest_EncryptTest).Client
}

func (_this EncryptTest) Dtor_vector() EsdkEncryptTestVector {
	return _this.Get_().(EncryptTest_EncryptTest).Vector
}

func (_this EncryptTest) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case EncryptTest_EncryptTest:
		{
			return "EsdkTestVectors.EncryptTest.EncryptTest" + "(" + _dafny.String(data.Cmm) + ", " + _dafny.String(data.Client) + ", " + _dafny.String(data.Vector) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this EncryptTest) Equals(other EncryptTest) bool {
	switch data1 := _this.Get_().(type) {
	case EncryptTest_EncryptTest:
		{
			data2, ok := other.Get_().(EncryptTest_EncryptTest)
			return ok && _dafny.AreEqual(data1.Cmm, data2.Cmm) && _dafny.AreEqual(data1.Client, data2.Client) && data1.Vector.Equals(data2.Vector)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this EncryptTest) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(EncryptTest)
	return ok && _this.Equals(typed)
}

func Type_EncryptTest_() _dafny.TypeDescriptor {
	return type_EncryptTest_{}
}

type type_EncryptTest_ struct {
}

func (_this type_EncryptTest_) Default() interface{} {
	return Companion_EncryptTest_.Default()
}

func (_this type_EncryptTest_) String() string {
	return "EsdkTestVectors.EncryptTest"
}
func (_this EncryptTest) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = EncryptTest{}

// End of datatype EncryptTest

// Definition of datatype DecryptTest
type DecryptTest struct {
	Data_DecryptTest_
}

func (_this DecryptTest) Get_() Data_DecryptTest_ {
	return _this.Data_DecryptTest_
}

type Data_DecryptTest_ interface {
	isDecryptTest()
}

type CompanionStruct_DecryptTest_ struct {
}

var Companion_DecryptTest_ = CompanionStruct_DecryptTest_{}

type DecryptTest_DecryptTest struct {
	Cmm    m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	Client m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
	Vector EsdkDecryptTestVector
}

func (DecryptTest_DecryptTest) isDecryptTest() {}

func (CompanionStruct_DecryptTest_) Create_DecryptTest_(Cmm m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager, Client m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient, Vector EsdkDecryptTestVector) DecryptTest {
	return DecryptTest{DecryptTest_DecryptTest{Cmm, Client, Vector}}
}

func (_this DecryptTest) Is_DecryptTest() bool {
	_, ok := _this.Get_().(DecryptTest_DecryptTest)
	return ok
}

func (CompanionStruct_DecryptTest_) Default() DecryptTest {
	return Companion_DecryptTest_.Create_DecryptTest_((m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager)(nil), (m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient)(nil), Companion_EsdkDecryptTestVector_.Default())
}

func (_this DecryptTest) Dtor_cmm() m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager {
	return _this.Get_().(DecryptTest_DecryptTest).Cmm
}

func (_this DecryptTest) Dtor_client() m_AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient {
	return _this.Get_().(DecryptTest_DecryptTest).Client
}

func (_this DecryptTest) Dtor_vector() EsdkDecryptTestVector {
	return _this.Get_().(DecryptTest_DecryptTest).Vector
}

func (_this DecryptTest) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case DecryptTest_DecryptTest:
		{
			return "EsdkTestVectors.DecryptTest.DecryptTest" + "(" + _dafny.String(data.Cmm) + ", " + _dafny.String(data.Client) + ", " + _dafny.String(data.Vector) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this DecryptTest) Equals(other DecryptTest) bool {
	switch data1 := _this.Get_().(type) {
	case DecryptTest_DecryptTest:
		{
			data2, ok := other.Get_().(DecryptTest_DecryptTest)
			return ok && _dafny.AreEqual(data1.Cmm, data2.Cmm) && _dafny.AreEqual(data1.Client, data2.Client) && data1.Vector.Equals(data2.Vector)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this DecryptTest) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(DecryptTest)
	return ok && _this.Equals(typed)
}

func Type_DecryptTest_() _dafny.TypeDescriptor {
	return type_DecryptTest_{}
}

type type_DecryptTest_ struct {
}

func (_this type_DecryptTest_) Default() interface{} {
	return Companion_DecryptTest_.Default()
}

func (_this type_DecryptTest_) String() string {
	return "EsdkTestVectors.DecryptTest"
}
func (_this DecryptTest) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = DecryptTest{}

// End of datatype DecryptTest

// Definition of class SupportedGenerateManifestVersion
type SupportedGenerateManifestVersion struct {
}

func New_SupportedGenerateManifestVersion_() *SupportedGenerateManifestVersion {
	_this := SupportedGenerateManifestVersion{}

	return &_this
}

type CompanionStruct_SupportedGenerateManifestVersion_ struct {
}

var Companion_SupportedGenerateManifestVersion_ = CompanionStruct_SupportedGenerateManifestVersion_{}

func (*SupportedGenerateManifestVersion) String() string {
	return "EsdkTestVectors.SupportedGenerateManifestVersion"
}
func (_this *CompanionStruct_SupportedGenerateManifestVersion_) Witness() _dafny.Int {
	return _dafny.IntOfInt64(4)
}

// End of class SupportedGenerateManifestVersion

func Type_SupportedGenerateManifestVersion_() _dafny.TypeDescriptor {
	return type_SupportedGenerateManifestVersion_{}
}

type type_SupportedGenerateManifestVersion_ struct {
}

func (_this type_SupportedGenerateManifestVersion_) Default() interface{} {
	return Companion_SupportedGenerateManifestVersion_.Witness()
}

func (_this type_SupportedGenerateManifestVersion_) String() string {
	return "EsdkTestVectors.SupportedGenerateManifestVersion"
}

// Definition of class SupportedEncryptVersion
type SupportedEncryptVersion struct {
}

func New_SupportedEncryptVersion_() *SupportedEncryptVersion {
	_this := SupportedEncryptVersion{}

	return &_this
}

type CompanionStruct_SupportedEncryptVersion_ struct {
}

var Companion_SupportedEncryptVersion_ = CompanionStruct_SupportedEncryptVersion_{}

func (*SupportedEncryptVersion) String() string {
	return "EsdkTestVectors.SupportedEncryptVersion"
}
func (_this *CompanionStruct_SupportedEncryptVersion_) Witness() _dafny.Int {
	return _dafny.One
}

// End of class SupportedEncryptVersion

func Type_SupportedEncryptVersion_() _dafny.TypeDescriptor {
	return type_SupportedEncryptVersion_{}
}

type type_SupportedEncryptVersion_ struct {
}

func (_this type_SupportedEncryptVersion_) Default() interface{} {
	return Companion_SupportedEncryptVersion_.Witness()
}

func (_this type_SupportedEncryptVersion_) String() string {
	return "EsdkTestVectors.SupportedEncryptVersion"
}

// Definition of datatype EsdkEncryptTestVector
type EsdkEncryptTestVector struct {
	Data_EsdkEncryptTestVector_
}

func (_this EsdkEncryptTestVector) Get_() Data_EsdkEncryptTestVector_ {
	return _this.Data_EsdkEncryptTestVector_
}

type Data_EsdkEncryptTestVector_ interface {
	isEsdkEncryptTestVector()
}

type CompanionStruct_EsdkEncryptTestVector_ struct {
}

var Companion_EsdkEncryptTestVector_ = CompanionStruct_EsdkEncryptTestVector_{}

type EsdkEncryptTestVector_PositiveEncryptTestVector struct {
	Id                          m_Wrappers.Option
	Version                     _dafny.Int
	ManifestPath                _dafny.Sequence
	DecryptManifestPath         _dafny.Sequence
	PlaintextPath               _dafny.Sequence
	EncryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	DecryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	EncryptionContext           m_Wrappers.Option
	ReproducedEncryptionContext m_Wrappers.Option
	CommitmentPolicy            m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                 m_Wrappers.Option
	AlgorithmSuiteId            m_Wrappers.Option
	Description                 _dafny.Sequence
	MaxEncryptedDataKeys        m_Wrappers.Option
}

func (EsdkEncryptTestVector_PositiveEncryptTestVector) isEsdkEncryptTestVector() {}

func (CompanionStruct_EsdkEncryptTestVector_) Create_PositiveEncryptTestVector_(Id m_Wrappers.Option, Version _dafny.Int, ManifestPath _dafny.Sequence, DecryptManifestPath _dafny.Sequence, PlaintextPath _dafny.Sequence, EncryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, DecryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, EncryptionContext m_Wrappers.Option, ReproducedEncryptionContext m_Wrappers.Option, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, Description _dafny.Sequence, MaxEncryptedDataKeys m_Wrappers.Option) EsdkEncryptTestVector {
	return EsdkEncryptTestVector{EsdkEncryptTestVector_PositiveEncryptTestVector{Id, Version, ManifestPath, DecryptManifestPath, PlaintextPath, EncryptDescriptions, DecryptDescriptions, EncryptionContext, ReproducedEncryptionContext, CommitmentPolicy, FrameLength, AlgorithmSuiteId, Description, MaxEncryptedDataKeys}}
}

func (_this EsdkEncryptTestVector) Is_PositiveEncryptTestVector() bool {
	_, ok := _this.Get_().(EsdkEncryptTestVector_PositiveEncryptTestVector)
	return ok
}

type EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector struct {
	Id                          m_Wrappers.Option
	Version                     _dafny.Int
	ManifestPath                _dafny.Sequence
	DecryptManifestPath         _dafny.Sequence
	PlaintextPath               _dafny.Sequence
	EncryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	DecryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	EncryptionContext           m_Wrappers.Option
	ReproducedEncryptionContext m_Wrappers.Option
	CommitmentPolicy            m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                 m_Wrappers.Option
	AlgorithmSuiteId            m_Wrappers.Option
	DecryptErrorDescription     _dafny.Sequence
	Description                 _dafny.Sequence
	MaxEncryptedDataKeys        m_Wrappers.Option
}

func (EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector) isEsdkEncryptTestVector() {}

func (CompanionStruct_EsdkEncryptTestVector_) Create_PositiveEncryptNegativeDecryptTestVector_(Id m_Wrappers.Option, Version _dafny.Int, ManifestPath _dafny.Sequence, DecryptManifestPath _dafny.Sequence, PlaintextPath _dafny.Sequence, EncryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, DecryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, EncryptionContext m_Wrappers.Option, ReproducedEncryptionContext m_Wrappers.Option, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, DecryptErrorDescription _dafny.Sequence, Description _dafny.Sequence, MaxEncryptedDataKeys m_Wrappers.Option) EsdkEncryptTestVector {
	return EsdkEncryptTestVector{EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector{Id, Version, ManifestPath, DecryptManifestPath, PlaintextPath, EncryptDescriptions, DecryptDescriptions, EncryptionContext, ReproducedEncryptionContext, CommitmentPolicy, FrameLength, AlgorithmSuiteId, DecryptErrorDescription, Description, MaxEncryptedDataKeys}}
}

func (_this EsdkEncryptTestVector) Is_PositiveEncryptNegativeDecryptTestVector() bool {
	_, ok := _this.Get_().(EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector)
	return ok
}

type EsdkEncryptTestVector_NegativeEncryptTestVector struct {
	Id                          m_Wrappers.Option
	Version                     _dafny.Int
	ManifestPath                _dafny.Sequence
	PlaintextPath               _dafny.Sequence
	EncryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	EncryptionContext           m_Wrappers.Option
	ReproducedEncryptionContext m_Wrappers.Option
	CommitmentPolicy            m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                 m_Wrappers.Option
	AlgorithmSuiteId            m_Wrappers.Option
	ErrorDescription            _dafny.Sequence
	Description                 _dafny.Sequence
	MaxEncryptedDataKeys        m_Wrappers.Option
}

func (EsdkEncryptTestVector_NegativeEncryptTestVector) isEsdkEncryptTestVector() {}

func (CompanionStruct_EsdkEncryptTestVector_) Create_NegativeEncryptTestVector_(Id m_Wrappers.Option, Version _dafny.Int, ManifestPath _dafny.Sequence, PlaintextPath _dafny.Sequence, EncryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, EncryptionContext m_Wrappers.Option, ReproducedEncryptionContext m_Wrappers.Option, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, ErrorDescription _dafny.Sequence, Description _dafny.Sequence, MaxEncryptedDataKeys m_Wrappers.Option) EsdkEncryptTestVector {
	return EsdkEncryptTestVector{EsdkEncryptTestVector_NegativeEncryptTestVector{Id, Version, ManifestPath, PlaintextPath, EncryptDescriptions, EncryptionContext, ReproducedEncryptionContext, CommitmentPolicy, FrameLength, AlgorithmSuiteId, ErrorDescription, Description, MaxEncryptedDataKeys}}
}

func (_this EsdkEncryptTestVector) Is_NegativeEncryptTestVector() bool {
	_, ok := _this.Get_().(EsdkEncryptTestVector_NegativeEncryptTestVector)
	return ok
}

func (CompanionStruct_EsdkEncryptTestVector_) Default() EsdkEncryptTestVector {
	return Companion_EsdkEncryptTestVector_.Create_PositiveEncryptTestVector_(m_Wrappers.Companion_Option_.Default(), Companion_SupportedEncryptVersion_.Witness(), _dafny.EmptySeq.SetString(), _dafny.EmptySeq.SetString(), _dafny.EmptySeq.SetString(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_KeyDescription_.Default(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_KeyDescription_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), _dafny.EmptySeq.SetString(), m_Wrappers.Companion_Option_.Default())
}

func (_this EsdkEncryptTestVector) Dtor_id() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.Id
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.Id
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).Id
	}
}

func (_this EsdkEncryptTestVector) Dtor_version() _dafny.Int {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.Version
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.Version
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).Version
	}
}

func (_this EsdkEncryptTestVector) Dtor_manifestPath() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.ManifestPath
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.ManifestPath
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).ManifestPath
	}
}

func (_this EsdkEncryptTestVector) Dtor_decryptManifestPath() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.DecryptManifestPath
	default:
		return data.(EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector).DecryptManifestPath
	}
}

func (_this EsdkEncryptTestVector) Dtor_plaintextPath() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.PlaintextPath
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.PlaintextPath
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).PlaintextPath
	}
}

func (_this EsdkEncryptTestVector) Dtor_encryptDescriptions() m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.EncryptDescriptions
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.EncryptDescriptions
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).EncryptDescriptions
	}
}

func (_this EsdkEncryptTestVector) Dtor_decryptDescriptions() m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.DecryptDescriptions
	default:
		return data.(EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector).DecryptDescriptions
	}
}

func (_this EsdkEncryptTestVector) Dtor_encryptionContext() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.EncryptionContext
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.EncryptionContext
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).EncryptionContext
	}
}

func (_this EsdkEncryptTestVector) Dtor_reproducedEncryptionContext() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.ReproducedEncryptionContext
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.ReproducedEncryptionContext
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).ReproducedEncryptionContext
	}
}

func (_this EsdkEncryptTestVector) Dtor_commitmentPolicy() m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.CommitmentPolicy
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.CommitmentPolicy
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).CommitmentPolicy
	}
}

func (_this EsdkEncryptTestVector) Dtor_frameLength() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.FrameLength
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.FrameLength
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).FrameLength
	}
}

func (_this EsdkEncryptTestVector) Dtor_algorithmSuiteId() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.AlgorithmSuiteId
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.AlgorithmSuiteId
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).AlgorithmSuiteId
	}
}

func (_this EsdkEncryptTestVector) Dtor_description() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.Description
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.Description
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).Description
	}
}

func (_this EsdkEncryptTestVector) Dtor_maxEncryptedDataKeys() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		return data.MaxEncryptedDataKeys
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		return data.MaxEncryptedDataKeys
	default:
		return data.(EsdkEncryptTestVector_NegativeEncryptTestVector).MaxEncryptedDataKeys
	}
}

func (_this EsdkEncryptTestVector) Dtor_decryptErrorDescription() _dafny.Sequence {
	return _this.Get_().(EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector).DecryptErrorDescription
}

func (_this EsdkEncryptTestVector) Dtor_errorDescription() _dafny.Sequence {
	return _this.Get_().(EsdkEncryptTestVector_NegativeEncryptTestVector).ErrorDescription
}

func (_this EsdkEncryptTestVector) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		{
			return "EsdkTestVectors.EsdkEncryptTestVector.PositiveEncryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.DecryptManifestPath) + ", " + _dafny.String(data.PlaintextPath) + ", " + _dafny.String(data.EncryptDescriptions) + ", " + _dafny.String(data.DecryptDescriptions) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.MaxEncryptedDataKeys) + ")"
		}
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		{
			return "EsdkTestVectors.EsdkEncryptTestVector.PositiveEncryptNegativeDecryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.DecryptManifestPath) + ", " + _dafny.String(data.PlaintextPath) + ", " + _dafny.String(data.EncryptDescriptions) + ", " + _dafny.String(data.DecryptDescriptions) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.DecryptErrorDescription) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.MaxEncryptedDataKeys) + ")"
		}
	case EsdkEncryptTestVector_NegativeEncryptTestVector:
		{
			return "EsdkTestVectors.EsdkEncryptTestVector.NegativeEncryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.PlaintextPath) + ", " + _dafny.String(data.EncryptDescriptions) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.ErrorDescription) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.MaxEncryptedDataKeys) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this EsdkEncryptTestVector) Equals(other EsdkEncryptTestVector) bool {
	switch data1 := _this.Get_().(type) {
	case EsdkEncryptTestVector_PositiveEncryptTestVector:
		{
			data2, ok := other.Get_().(EsdkEncryptTestVector_PositiveEncryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.DecryptManifestPath.Equals(data2.DecryptManifestPath) && data1.PlaintextPath.Equals(data2.PlaintextPath) && data1.EncryptDescriptions.Equals(data2.EncryptDescriptions) && data1.DecryptDescriptions.Equals(data2.DecryptDescriptions) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.Description.Equals(data2.Description) && data1.MaxEncryptedDataKeys.Equals(data2.MaxEncryptedDataKeys)
		}
	case EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector:
		{
			data2, ok := other.Get_().(EsdkEncryptTestVector_PositiveEncryptNegativeDecryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.DecryptManifestPath.Equals(data2.DecryptManifestPath) && data1.PlaintextPath.Equals(data2.PlaintextPath) && data1.EncryptDescriptions.Equals(data2.EncryptDescriptions) && data1.DecryptDescriptions.Equals(data2.DecryptDescriptions) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.DecryptErrorDescription.Equals(data2.DecryptErrorDescription) && data1.Description.Equals(data2.Description) && data1.MaxEncryptedDataKeys.Equals(data2.MaxEncryptedDataKeys)
		}
	case EsdkEncryptTestVector_NegativeEncryptTestVector:
		{
			data2, ok := other.Get_().(EsdkEncryptTestVector_NegativeEncryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.PlaintextPath.Equals(data2.PlaintextPath) && data1.EncryptDescriptions.Equals(data2.EncryptDescriptions) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.ErrorDescription.Equals(data2.ErrorDescription) && data1.Description.Equals(data2.Description) && data1.MaxEncryptedDataKeys.Equals(data2.MaxEncryptedDataKeys)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this EsdkEncryptTestVector) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(EsdkEncryptTestVector)
	return ok && _this.Equals(typed)
}

func Type_EsdkEncryptTestVector_() _dafny.TypeDescriptor {
	return type_EsdkEncryptTestVector_{}
}

type type_EsdkEncryptTestVector_ struct {
}

func (_this type_EsdkEncryptTestVector_) Default() interface{} {
	return Companion_EsdkEncryptTestVector_.Default()
}

func (_this type_EsdkEncryptTestVector_) String() string {
	return "EsdkTestVectors.EsdkEncryptTestVector"
}
func (_this EsdkEncryptTestVector) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = EsdkEncryptTestVector{}

// End of datatype EsdkEncryptTestVector

// Definition of class SupportedDecryptVersion
type SupportedDecryptVersion struct {
}

func New_SupportedDecryptVersion_() *SupportedDecryptVersion {
	_this := SupportedDecryptVersion{}

	return &_this
}

type CompanionStruct_SupportedDecryptVersion_ struct {
}

var Companion_SupportedDecryptVersion_ = CompanionStruct_SupportedDecryptVersion_{}

func (*SupportedDecryptVersion) String() string {
	return "EsdkTestVectors.SupportedDecryptVersion"
}
func (_this *CompanionStruct_SupportedDecryptVersion_) Witness() _dafny.Int {
	return _dafny.One
}

// End of class SupportedDecryptVersion

func Type_SupportedDecryptVersion_() _dafny.TypeDescriptor {
	return type_SupportedDecryptVersion_{}
}

type type_SupportedDecryptVersion_ struct {
}

func (_this type_SupportedDecryptVersion_) Default() interface{} {
	return Companion_SupportedDecryptVersion_.Witness()
}

func (_this type_SupportedDecryptVersion_) String() string {
	return "EsdkTestVectors.SupportedDecryptVersion"
}

// Definition of datatype EsdkDecryptTestVector
type EsdkDecryptTestVector struct {
	Data_EsdkDecryptTestVector_
}

func (_this EsdkDecryptTestVector) Get_() Data_EsdkDecryptTestVector_ {
	return _this.Data_EsdkDecryptTestVector_
}

type Data_EsdkDecryptTestVector_ interface {
	isEsdkDecryptTestVector()
}

type CompanionStruct_EsdkDecryptTestVector_ struct {
}

var Companion_EsdkDecryptTestVector_ = CompanionStruct_EsdkDecryptTestVector_{}

type EsdkDecryptTestVector_PositiveDecryptTestVector struct {
	Id                          _dafny.Sequence
	Version                     _dafny.Int
	ManifestPath                _dafny.Sequence
	CiphertextPath              _dafny.Sequence
	PlaintextPath               _dafny.Sequence
	ReproducedEncryptionContext m_Wrappers.Option
	DecryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	CommitmentPolicy            m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                 m_Wrappers.Option
	AlgorithmSuiteId            m_Wrappers.Option
	Description                 _dafny.Sequence
	DecryptionMethod            DecryptionMethod
}

func (EsdkDecryptTestVector_PositiveDecryptTestVector) isEsdkDecryptTestVector() {}

func (CompanionStruct_EsdkDecryptTestVector_) Create_PositiveDecryptTestVector_(Id _dafny.Sequence, Version _dafny.Int, ManifestPath _dafny.Sequence, CiphertextPath _dafny.Sequence, PlaintextPath _dafny.Sequence, ReproducedEncryptionContext m_Wrappers.Option, DecryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, Description _dafny.Sequence, DecryptionMethod DecryptionMethod) EsdkDecryptTestVector {
	return EsdkDecryptTestVector{EsdkDecryptTestVector_PositiveDecryptTestVector{Id, Version, ManifestPath, CiphertextPath, PlaintextPath, ReproducedEncryptionContext, DecryptDescriptions, CommitmentPolicy, FrameLength, AlgorithmSuiteId, Description, DecryptionMethod}}
}

func (_this EsdkDecryptTestVector) Is_PositiveDecryptTestVector() bool {
	_, ok := _this.Get_().(EsdkDecryptTestVector_PositiveDecryptTestVector)
	return ok
}

type EsdkDecryptTestVector_NegativeDecryptTestVector struct {
	Id                          _dafny.Sequence
	Version                     _dafny.Int
	ManifestPath                _dafny.Sequence
	CiphertextPath              _dafny.Sequence
	ErrorDescription            _dafny.Sequence
	ReproducedEncryptionContext m_Wrappers.Option
	DecryptDescriptions         m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	CommitmentPolicy            m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                 m_Wrappers.Option
	AlgorithmSuiteId            m_Wrappers.Option
	Description                 _dafny.Sequence
	DecryptionMethod            DecryptionMethod
}

func (EsdkDecryptTestVector_NegativeDecryptTestVector) isEsdkDecryptTestVector() {}

func (CompanionStruct_EsdkDecryptTestVector_) Create_NegativeDecryptTestVector_(Id _dafny.Sequence, Version _dafny.Int, ManifestPath _dafny.Sequence, CiphertextPath _dafny.Sequence, ErrorDescription _dafny.Sequence, ReproducedEncryptionContext m_Wrappers.Option, DecryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, Description _dafny.Sequence, DecryptionMethod DecryptionMethod) EsdkDecryptTestVector {
	return EsdkDecryptTestVector{EsdkDecryptTestVector_NegativeDecryptTestVector{Id, Version, ManifestPath, CiphertextPath, ErrorDescription, ReproducedEncryptionContext, DecryptDescriptions, CommitmentPolicy, FrameLength, AlgorithmSuiteId, Description, DecryptionMethod}}
}

func (_this EsdkDecryptTestVector) Is_NegativeDecryptTestVector() bool {
	_, ok := _this.Get_().(EsdkDecryptTestVector_NegativeDecryptTestVector)
	return ok
}

type EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector struct {
	Id                            _dafny.Sequence
	Version                       _dafny.Int
	ManifestPath                  _dafny.Sequence
	CiphertextPath                _dafny.Sequence
	PlaintextPath                 _dafny.Sequence
	ReproducedEncryptionContext   m_Wrappers.Option
	RequiredEncryptionContextKeys m_Wrappers.Option
	DecryptDescriptions           m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	CommitmentPolicy              m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                   m_Wrappers.Option
	AlgorithmSuiteId              m_Wrappers.Option
	Description                   _dafny.Sequence
	DecryptionMethod              DecryptionMethod
}

func (EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector) isEsdkDecryptTestVector() {}

func (CompanionStruct_EsdkDecryptTestVector_) Create_PositiveV1OrV2DecryptTestVector_(Id _dafny.Sequence, Version _dafny.Int, ManifestPath _dafny.Sequence, CiphertextPath _dafny.Sequence, PlaintextPath _dafny.Sequence, ReproducedEncryptionContext m_Wrappers.Option, RequiredEncryptionContextKeys m_Wrappers.Option, DecryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, Description _dafny.Sequence, DecryptionMethod DecryptionMethod) EsdkDecryptTestVector {
	return EsdkDecryptTestVector{EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector{Id, Version, ManifestPath, CiphertextPath, PlaintextPath, ReproducedEncryptionContext, RequiredEncryptionContextKeys, DecryptDescriptions, CommitmentPolicy, FrameLength, AlgorithmSuiteId, Description, DecryptionMethod}}
}

func (_this EsdkDecryptTestVector) Is_PositiveV1OrV2DecryptTestVector() bool {
	_, ok := _this.Get_().(EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector)
	return ok
}

type EsdkDecryptTestVector_PositiveV4DecryptTestVector struct {
	Id                            _dafny.Sequence
	Version                       _dafny.Int
	ManifestPath                  _dafny.Sequence
	CiphertextPath                _dafny.Sequence
	PlaintextPath                 _dafny.Sequence
	ReproducedEncryptionContext   m_Wrappers.Option
	RequiredEncryptionContextKeys m_Wrappers.Option
	DecryptDescriptions           m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription
	CommitmentPolicy              m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
	FrameLength                   m_Wrappers.Option
	AlgorithmSuiteId              m_Wrappers.Option
	Description                   _dafny.Sequence
	DecryptionMethod              DecryptionMethod
	Cmm                           _dafny.Sequence
	RetryPolicy                   m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy
}

func (EsdkDecryptTestVector_PositiveV4DecryptTestVector) isEsdkDecryptTestVector() {}

func (CompanionStruct_EsdkDecryptTestVector_) Create_PositiveV4DecryptTestVector_(Id _dafny.Sequence, Version _dafny.Int, ManifestPath _dafny.Sequence, CiphertextPath _dafny.Sequence, PlaintextPath _dafny.Sequence, ReproducedEncryptionContext m_Wrappers.Option, RequiredEncryptionContextKeys m_Wrappers.Option, DecryptDescriptions m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription, CommitmentPolicy m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy, FrameLength m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, Description _dafny.Sequence, DecryptionMethod DecryptionMethod, Cmm _dafny.Sequence, RetryPolicy m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy) EsdkDecryptTestVector {
	return EsdkDecryptTestVector{EsdkDecryptTestVector_PositiveV4DecryptTestVector{Id, Version, ManifestPath, CiphertextPath, PlaintextPath, ReproducedEncryptionContext, RequiredEncryptionContextKeys, DecryptDescriptions, CommitmentPolicy, FrameLength, AlgorithmSuiteId, Description, DecryptionMethod, Cmm, RetryPolicy}}
}

func (_this EsdkDecryptTestVector) Is_PositiveV4DecryptTestVector() bool {
	_, ok := _this.Get_().(EsdkDecryptTestVector_PositiveV4DecryptTestVector)
	return ok
}

func (CompanionStruct_EsdkDecryptTestVector_) Default() EsdkDecryptTestVector {
	return Companion_EsdkDecryptTestVector_.Create_PositiveDecryptTestVector_(_dafny.EmptySeq.SetString(), Companion_SupportedDecryptVersion_.Witness(), _dafny.EmptySeq.SetString(), _dafny.EmptySeq.SetString(), _dafny.EmptySeq.SetString(), m_Wrappers.Companion_Option_.Default(), m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_KeyDescription_.Default(), m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), _dafny.EmptySeq.SetString(), Companion_DecryptionMethod_.Default())
}

func (_this EsdkDecryptTestVector) Dtor_id() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.Id
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.Id
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.Id
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).Id
	}
}

func (_this EsdkDecryptTestVector) Dtor_version() _dafny.Int {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.Version
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.Version
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.Version
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).Version
	}
}

func (_this EsdkDecryptTestVector) Dtor_manifestPath() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.ManifestPath
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.ManifestPath
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.ManifestPath
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).ManifestPath
	}
}

func (_this EsdkDecryptTestVector) Dtor_ciphertextPath() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.CiphertextPath
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.CiphertextPath
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.CiphertextPath
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).CiphertextPath
	}
}

func (_this EsdkDecryptTestVector) Dtor_plaintextPath() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.PlaintextPath
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.PlaintextPath
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).PlaintextPath
	}
}

func (_this EsdkDecryptTestVector) Dtor_reproducedEncryptionContext() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.ReproducedEncryptionContext
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.ReproducedEncryptionContext
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.ReproducedEncryptionContext
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).ReproducedEncryptionContext
	}
}

func (_this EsdkDecryptTestVector) Dtor_decryptDescriptions() m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.KeyDescription {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.DecryptDescriptions
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.DecryptDescriptions
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.DecryptDescriptions
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).DecryptDescriptions
	}
}

func (_this EsdkDecryptTestVector) Dtor_commitmentPolicy() m_AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.CommitmentPolicy
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.CommitmentPolicy
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.CommitmentPolicy
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).CommitmentPolicy
	}
}

func (_this EsdkDecryptTestVector) Dtor_frameLength() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.FrameLength
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.FrameLength
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.FrameLength
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).FrameLength
	}
}

func (_this EsdkDecryptTestVector) Dtor_algorithmSuiteId() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.AlgorithmSuiteId
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.AlgorithmSuiteId
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.AlgorithmSuiteId
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).AlgorithmSuiteId
	}
}

func (_this EsdkDecryptTestVector) Dtor_description() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.Description
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.Description
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.Description
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).Description
	}
}

func (_this EsdkDecryptTestVector) Dtor_decryptionMethod() DecryptionMethod {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		return data.DecryptionMethod
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		return data.DecryptionMethod
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.DecryptionMethod
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).DecryptionMethod
	}
}

func (_this EsdkDecryptTestVector) Dtor_errorDescription() _dafny.Sequence {
	return _this.Get_().(EsdkDecryptTestVector_NegativeDecryptTestVector).ErrorDescription
}

func (_this EsdkDecryptTestVector) Dtor_requiredEncryptionContextKeys() m_Wrappers.Option {
	switch data := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		return data.RequiredEncryptionContextKeys
	default:
		return data.(EsdkDecryptTestVector_PositiveV4DecryptTestVector).RequiredEncryptionContextKeys
	}
}

func (_this EsdkDecryptTestVector) Dtor_cmm() _dafny.Sequence {
	return _this.Get_().(EsdkDecryptTestVector_PositiveV4DecryptTestVector).Cmm
}

func (_this EsdkDecryptTestVector) Dtor_retryPolicy() m_AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy {
	return _this.Get_().(EsdkDecryptTestVector_PositiveV4DecryptTestVector).RetryPolicy
}

func (_this EsdkDecryptTestVector) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		{
			return "EsdkTestVectors.EsdkDecryptTestVector.PositiveDecryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.CiphertextPath) + ", " + _dafny.String(data.PlaintextPath) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.DecryptDescriptions) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.DecryptionMethod) + ")"
		}
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		{
			return "EsdkTestVectors.EsdkDecryptTestVector.NegativeDecryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.CiphertextPath) + ", " + _dafny.String(data.ErrorDescription) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.DecryptDescriptions) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.DecryptionMethod) + ")"
		}
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		{
			return "EsdkTestVectors.EsdkDecryptTestVector.PositiveV1OrV2DecryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.CiphertextPath) + ", " + _dafny.String(data.PlaintextPath) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.RequiredEncryptionContextKeys) + ", " + _dafny.String(data.DecryptDescriptions) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.DecryptionMethod) + ")"
		}
	case EsdkDecryptTestVector_PositiveV4DecryptTestVector:
		{
			return "EsdkTestVectors.EsdkDecryptTestVector.PositiveV4DecryptTestVector" + "(" + _dafny.String(data.Id) + ", " + _dafny.String(data.Version) + ", " + _dafny.String(data.ManifestPath) + ", " + _dafny.String(data.CiphertextPath) + ", " + _dafny.String(data.PlaintextPath) + ", " + _dafny.String(data.ReproducedEncryptionContext) + ", " + _dafny.String(data.RequiredEncryptionContextKeys) + ", " + _dafny.String(data.DecryptDescriptions) + ", " + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.Description) + ", " + _dafny.String(data.DecryptionMethod) + ", " + _dafny.String(data.Cmm) + ", " + _dafny.String(data.RetryPolicy) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this EsdkDecryptTestVector) Equals(other EsdkDecryptTestVector) bool {
	switch data1 := _this.Get_().(type) {
	case EsdkDecryptTestVector_PositiveDecryptTestVector:
		{
			data2, ok := other.Get_().(EsdkDecryptTestVector_PositiveDecryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.CiphertextPath.Equals(data2.CiphertextPath) && data1.PlaintextPath.Equals(data2.PlaintextPath) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.DecryptDescriptions.Equals(data2.DecryptDescriptions) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.Description.Equals(data2.Description) && data1.DecryptionMethod.Equals(data2.DecryptionMethod)
		}
	case EsdkDecryptTestVector_NegativeDecryptTestVector:
		{
			data2, ok := other.Get_().(EsdkDecryptTestVector_NegativeDecryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.CiphertextPath.Equals(data2.CiphertextPath) && data1.ErrorDescription.Equals(data2.ErrorDescription) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.DecryptDescriptions.Equals(data2.DecryptDescriptions) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.Description.Equals(data2.Description) && data1.DecryptionMethod.Equals(data2.DecryptionMethod)
		}
	case EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector:
		{
			data2, ok := other.Get_().(EsdkDecryptTestVector_PositiveV1OrV2DecryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.CiphertextPath.Equals(data2.CiphertextPath) && data1.PlaintextPath.Equals(data2.PlaintextPath) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.RequiredEncryptionContextKeys.Equals(data2.RequiredEncryptionContextKeys) && data1.DecryptDescriptions.Equals(data2.DecryptDescriptions) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.Description.Equals(data2.Description) && data1.DecryptionMethod.Equals(data2.DecryptionMethod)
		}
	case EsdkDecryptTestVector_PositiveV4DecryptTestVector:
		{
			data2, ok := other.Get_().(EsdkDecryptTestVector_PositiveV4DecryptTestVector)
			return ok && data1.Id.Equals(data2.Id) && data1.Version.Cmp(data2.Version) == 0 && data1.ManifestPath.Equals(data2.ManifestPath) && data1.CiphertextPath.Equals(data2.CiphertextPath) && data1.PlaintextPath.Equals(data2.PlaintextPath) && data1.ReproducedEncryptionContext.Equals(data2.ReproducedEncryptionContext) && data1.RequiredEncryptionContextKeys.Equals(data2.RequiredEncryptionContextKeys) && data1.DecryptDescriptions.Equals(data2.DecryptDescriptions) && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.FrameLength.Equals(data2.FrameLength) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.Description.Equals(data2.Description) && data1.DecryptionMethod.Equals(data2.DecryptionMethod) && data1.Cmm.Equals(data2.Cmm) && data1.RetryPolicy.Equals(data2.RetryPolicy)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this EsdkDecryptTestVector) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(EsdkDecryptTestVector)
	return ok && _this.Equals(typed)
}

func Type_EsdkDecryptTestVector_() _dafny.TypeDescriptor {
	return type_EsdkDecryptTestVector_{}
}

type type_EsdkDecryptTestVector_ struct {
}

func (_this type_EsdkDecryptTestVector_) Default() interface{} {
	return Companion_EsdkDecryptTestVector_.Default()
}

func (_this type_EsdkDecryptTestVector_) String() string {
	return "EsdkTestVectors.EsdkDecryptTestVector"
}
func (_this EsdkDecryptTestVector) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = EsdkDecryptTestVector{}

// End of datatype EsdkDecryptTestVector

// Definition of datatype DecryptionMethod
type DecryptionMethod struct {
	Data_DecryptionMethod_
}

func (_this DecryptionMethod) Get_() Data_DecryptionMethod_ {
	return _this.Data_DecryptionMethod_
}

type Data_DecryptionMethod_ interface {
	isDecryptionMethod()
}

type CompanionStruct_DecryptionMethod_ struct {
}

var Companion_DecryptionMethod_ = CompanionStruct_DecryptionMethod_{}

type DecryptionMethod_StreamingUnsignedOnly struct {
}

func (DecryptionMethod_StreamingUnsignedOnly) isDecryptionMethod() {}

func (CompanionStruct_DecryptionMethod_) Create_StreamingUnsignedOnly_() DecryptionMethod {
	return DecryptionMethod{DecryptionMethod_StreamingUnsignedOnly{}}
}

func (_this DecryptionMethod) Is_StreamingUnsignedOnly() bool {
	_, ok := _this.Get_().(DecryptionMethod_StreamingUnsignedOnly)
	return ok
}

type DecryptionMethod_OneShot struct {
}

func (DecryptionMethod_OneShot) isDecryptionMethod() {}

func (CompanionStruct_DecryptionMethod_) Create_OneShot_() DecryptionMethod {
	return DecryptionMethod{DecryptionMethod_OneShot{}}
}

func (_this DecryptionMethod) Is_OneShot() bool {
	_, ok := _this.Get_().(DecryptionMethod_OneShot)
	return ok
}

func (CompanionStruct_DecryptionMethod_) Default() DecryptionMethod {
	return Companion_DecryptionMethod_.Create_StreamingUnsignedOnly_()
}

func (_ CompanionStruct_DecryptionMethod_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_DecryptionMethod_.Create_StreamingUnsignedOnly_(), true
		case 1:
			return Companion_DecryptionMethod_.Create_OneShot_(), true
		default:
			return DecryptionMethod{}, false
		}
	}
}

func (_this DecryptionMethod) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case DecryptionMethod_StreamingUnsignedOnly:
		{
			return "EsdkTestVectors.DecryptionMethod.StreamingUnsignedOnly"
		}
	case DecryptionMethod_OneShot:
		{
			return "EsdkTestVectors.DecryptionMethod.OneShot"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this DecryptionMethod) Equals(other DecryptionMethod) bool {
	switch _this.Get_().(type) {
	case DecryptionMethod_StreamingUnsignedOnly:
		{
			_, ok := other.Get_().(DecryptionMethod_StreamingUnsignedOnly)
			return ok
		}
	case DecryptionMethod_OneShot:
		{
			_, ok := other.Get_().(DecryptionMethod_OneShot)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this DecryptionMethod) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(DecryptionMethod)
	return ok && _this.Equals(typed)
}

func Type_DecryptionMethod_() _dafny.TypeDescriptor {
	return type_DecryptionMethod_{}
}

type type_DecryptionMethod_ struct {
}

func (_this type_DecryptionMethod_) Default() interface{} {
	return Companion_DecryptionMethod_.Default()
}

func (_this type_DecryptionMethod_) String() string {
	return "EsdkTestVectors.DecryptionMethod"
}
func (_this DecryptionMethod) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = DecryptionMethod{}

// End of datatype DecryptionMethod

// Definition of datatype EncryptTestOutput
type EncryptTestOutput struct {
	Data_EncryptTestOutput_
}

func (_this EncryptTestOutput) Get_() Data_EncryptTestOutput_ {
	return _this.Data_EncryptTestOutput_
}

type Data_EncryptTestOutput_ interface {
	isEncryptTestOutput()
}

type CompanionStruct_EncryptTestOutput_ struct {
}

var Companion_EncryptTestOutput_ = CompanionStruct_EncryptTestOutput_{}

type EncryptTestOutput_EncryptTestOutput struct {
	Output bool
	Vector m_Wrappers.Option
}

func (EncryptTestOutput_EncryptTestOutput) isEncryptTestOutput() {}

func (CompanionStruct_EncryptTestOutput_) Create_EncryptTestOutput_(Output bool, Vector m_Wrappers.Option) EncryptTestOutput {
	return EncryptTestOutput{EncryptTestOutput_EncryptTestOutput{Output, Vector}}
}

func (_this EncryptTestOutput) Is_EncryptTestOutput() bool {
	_, ok := _this.Get_().(EncryptTestOutput_EncryptTestOutput)
	return ok
}

func (CompanionStruct_EncryptTestOutput_) Default() EncryptTestOutput {
	return Companion_EncryptTestOutput_.Create_EncryptTestOutput_(false, m_Wrappers.Companion_Option_.Default())
}

func (_this EncryptTestOutput) Dtor_output() bool {
	return _this.Get_().(EncryptTestOutput_EncryptTestOutput).Output
}

func (_this EncryptTestOutput) Dtor_vector() m_Wrappers.Option {
	return _this.Get_().(EncryptTestOutput_EncryptTestOutput).Vector
}

func (_this EncryptTestOutput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case EncryptTestOutput_EncryptTestOutput:
		{
			return "EsdkTestVectors.EncryptTestOutput.EncryptTestOutput" + "(" + _dafny.String(data.Output) + ", " + _dafny.String(data.Vector) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this EncryptTestOutput) Equals(other EncryptTestOutput) bool {
	switch data1 := _this.Get_().(type) {
	case EncryptTestOutput_EncryptTestOutput:
		{
			data2, ok := other.Get_().(EncryptTestOutput_EncryptTestOutput)
			return ok && data1.Output == data2.Output && data1.Vector.Equals(data2.Vector)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this EncryptTestOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(EncryptTestOutput)
	return ok && _this.Equals(typed)
}

func Type_EncryptTestOutput_() _dafny.TypeDescriptor {
	return type_EncryptTestOutput_{}
}

type type_EncryptTestOutput_ struct {
}

func (_this type_EncryptTestOutput_) Default() interface{} {
	return Companion_EncryptTestOutput_.Default()
}

func (_this type_EncryptTestOutput_) String() string {
	return "EsdkTestVectors.EncryptTestOutput"
}
func (_this EncryptTestOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = EncryptTestOutput{}

// End of datatype EncryptTestOutput
