package WrappedESDK

import (
	"github.com/aws/aws-encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	"github.com/aws/aws-encryption-sdk/test/WrappedAwsCryptographyEncryptionSdkService"
	"github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
)

func (_static CompanionStruct_Default___) WrappedESDK(config AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig) Wrappers.Result {
	return WrappedAwsCryptographyEncryptionSdkService.WrappedESDK(config)
}
