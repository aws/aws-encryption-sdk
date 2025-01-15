package WrappedESDK

import (
	"github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	"github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/test/WrappedAwsCryptographyEncryptionSdkService"
	"github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
)

func (_static CompanionStruct_Default___) WrappedESDK(config AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig) Wrappers.Result {
	return WrappedAwsCryptographyEncryptionSdkService.WrappedESDK(config)
}
