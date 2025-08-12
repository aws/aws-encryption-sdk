module github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/examples

go 1.23.0

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.2.1
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.2.1
	github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk v0.2.0
	github.com/aws/aws-sdk-go-v2/config v1.31.0
	github.com/aws/aws-sdk-go-v2/credentials v1.18.4
	github.com/aws/aws-sdk-go-v2/service/dynamodb v1.47.0
	github.com/aws/aws-sdk-go-v2/service/kms v1.44.0
	github.com/aws/aws-sdk-go-v2/service/sts v1.37.0
	github.com/google/uuid v1.6.0
)

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.2.1 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.2.1 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.2.1 // indirect
	github.com/aws/aws-sdk-go-v2 v1.38.0 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.18.3 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.4.3 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.7.3 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.8.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.13.0 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/endpoint-discovery v1.11.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.13.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.28.0 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.33.0 // indirect
	github.com/aws/smithy-go v1.22.5 // indirect
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.10.1 // indirect
)

replace github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk => ../
