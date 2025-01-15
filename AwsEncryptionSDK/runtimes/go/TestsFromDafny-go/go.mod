module github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk/test

go 1.23.0

require github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.0.1

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.0.0 => ../../../../mpl/AwsCryptographicMaterialProviders/runtimes/go/ImplementationFromDafny-go/

replace github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk v0.0.0 => ../ImplementationFromDafny-go

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.0.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.0.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.0.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.0.0
	github.com/aws/aws-encryption-sdk-dafny/releases/go/encryption-sdk v0.0.0
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.9.2

)

require (
	github.com/aws/aws-sdk-go-v2 v1.32.8 // indirect
	github.com/aws/aws-sdk-go-v2/config v1.28.10 // indirect
	github.com/aws/aws-sdk-go-v2/credentials v1.17.51 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.16.23 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.3.27 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.6.27 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.8.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/dynamodb v1.39.2 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.12.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/endpoint-discovery v1.10.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.12.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/kms v1.36.0 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.24.9 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.28.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/sts v1.33.6 // indirect
	github.com/aws/smithy-go v1.22.1 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/jmespath/go-jmespath v0.4.0 // indirect
)

replace (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.0.0 => ../../../../mpl/ComAmazonawsDynamodb/runtimes/go/ImplementationFromDafny-go/
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.0.0 => ../../../../mpl/ComAmazonawsKms/runtimes/go/ImplementationFromDafny-go/
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.0.0 => ../../../../mpl/AwsCryptographyPrimitives/runtimes/go/ImplementationFromDafny-go/

)

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library => ../../../../mpl/StandardLibrary/runtimes/go/ImplementationFromDafny-go/
