module github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/examples

go 1.23.0

replace (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb => ../../../../mpl/ComAmazonawsDynamodb/runtimes/go/ImplementationFromDafny-go/
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms => ../../../../mpl/ComAmazonawsKms/runtimes/go/ImplementationFromDafny-go/
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl => ../../../../mpl/AwsCryptographicMaterialProviders/runtimes/go/ImplementationFromDafny-go
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives => ../../../../mpl/AwsCryptographyPrimitives/runtimes/go/ImplementationFromDafny-go/
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library => ../../../../mpl/StandardLibrary/runtimes/go/ImplementationFromDafny-go/
	github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk => ../ImplementationFromDafny-go/
)

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.3.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.3.0
	github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk v0.3.0
	github.com/aws/aws-sdk-go-v2/config v1.29.0
	github.com/aws/aws-sdk-go-v2/credentials v1.17.53
	github.com/aws/aws-sdk-go-v2/service/dynamodb v1.39.4
	github.com/aws/aws-sdk-go-v2/service/kms v1.37.12
	github.com/aws/aws-sdk-go-v2/service/sts v1.33.8
	github.com/google/uuid v1.6.0
)

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.3.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.3.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.3.0 // indirect
	github.com/aws/aws-sdk-go-v2 v1.33.0 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.16.24 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.3.28 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.6.28 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.8.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.12.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/endpoint-discovery v1.10.9 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.12.9 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.24.10 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.28.9 // indirect
	github.com/aws/smithy-go v1.22.1 // indirect
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.11.3 // indirect
	github.com/jmespath/go-jmespath v0.4.0 // indirect
)
