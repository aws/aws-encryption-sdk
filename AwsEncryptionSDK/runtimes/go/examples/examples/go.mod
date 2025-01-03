module github.com/aws/aws-encryption-sdk/aws-esdk-go-preview

go 1.23.0

replace github.com/aws/aws-cryptographic-material-providers-library/mpl v0.0.0 => ../mpl

replace (
	github.com/aws/aws-cryptographic-material-providers-library/dynamodb v0.0.0 => ../dynamodb
	github.com/aws/aws-cryptographic-material-providers-library/kms v0.0.0 => ../kms
	github.com/aws/aws-cryptographic-material-providers-library/primitives v0.0.0 => ../primitives

)

replace github.com/dafny-lang/DafnyStandardLibGo => ../StandardLibrary/

replace github.com/aws/aws-encryption-sdk => ./../esdk

require (
	github.com/aws/aws-cryptographic-material-providers-library/mpl v0.0.0
	github.com/aws/aws-cryptographic-material-providers-library/primitives v0.0.0
	github.com/aws/aws-encryption-sdk v0.0.0-00010101000000-000000000000
	github.com/aws/aws-sdk-go-v2/config v1.27.37
	github.com/aws/aws-sdk-go-v2/service/dynamodb v1.35.1
	github.com/aws/aws-sdk-go-v2/credentials v1.17.35
	github.com/aws/aws-sdk-go-v2/service/kms v1.36.0
	github.com/aws/aws-sdk-go-v2/service/sts v1.31.1
)

require (
	github.com/aws/aws-cryptographic-material-providers-library/dynamodb v0.0.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/kms v0.0.0 // indirect
	github.com/aws/aws-sdk-go-v2 v1.31.0 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.16.14 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.3.18 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.6.18 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.8.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.11.5 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/endpoint-discovery v1.9.19 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.11.20 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.23.1 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.27.1 // indirect
	github.com/aws/smithy-go v1.21.0 // indirect
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.9.1 // indirect
	github.com/dafny-lang/DafnyStandardLibGo v0.0.0 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/jmespath/go-jmespath v0.4.0 // indirect
)
