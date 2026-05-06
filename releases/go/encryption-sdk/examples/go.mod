module github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/examples

go 1.24

toolchain go1.24.13

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.4.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.4.0
	github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk v0.3.0
	github.com/aws/aws-sdk-go-v2/config v1.32.17
	github.com/aws/aws-sdk-go-v2/credentials v1.19.16
	github.com/aws/aws-sdk-go-v2/service/dynamodb v1.57.3
	github.com/aws/aws-sdk-go-v2/service/kms v1.51.1
	github.com/aws/aws-sdk-go-v2/service/sts v1.42.1
	github.com/google/uuid v1.6.0
)

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.4.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.4.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.4.0 // indirect
	github.com/aws/aws-sdk-go-v2 v1.41.7 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.18.23 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.4.23 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.7.23 // indirect
	github.com/aws/aws-sdk-go-v2/internal/v4a v1.4.24 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.13.9 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/endpoint-discovery v1.11.23 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.13.23 // indirect
	github.com/aws/aws-sdk-go-v2/service/signin v1.0.11 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.30.17 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.35.21 // indirect
	github.com/aws/smithy-go v1.25.1 // indirect
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.11.3 // indirect
)

replace github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk => ../
