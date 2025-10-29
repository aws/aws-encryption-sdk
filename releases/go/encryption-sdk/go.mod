module github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk

go 1.23.0

retract [v0.0.0-0, v0.1.1]

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.3.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.3.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.3.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.3.0
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.3.0
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.11.2

)

require (
	github.com/aws/aws-sdk-go-v2 v1.39.4 // indirect
	github.com/aws/aws-sdk-go-v2/config v1.31.15 // indirect
	github.com/aws/aws-sdk-go-v2/credentials v1.18.19 // indirect
	github.com/aws/aws-sdk-go-v2/feature/ec2/imds v1.18.11 // indirect
	github.com/aws/aws-sdk-go-v2/internal/configsources v1.4.11 // indirect
	github.com/aws/aws-sdk-go-v2/internal/endpoints/v2 v2.7.11 // indirect
	github.com/aws/aws-sdk-go-v2/internal/ini v1.8.4 // indirect
	github.com/aws/aws-sdk-go-v2/service/dynamodb v1.52.2 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/accept-encoding v1.13.2 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/endpoint-discovery v1.11.11 // indirect
	github.com/aws/aws-sdk-go-v2/service/internal/presigned-url v1.13.11 // indirect
	github.com/aws/aws-sdk-go-v2/service/kms v1.46.2 // indirect
	github.com/aws/aws-sdk-go-v2/service/sso v1.29.8 // indirect
	github.com/aws/aws-sdk-go-v2/service/ssooidc v1.35.3 // indirect
	github.com/aws/aws-sdk-go-v2/service/sts v1.38.9 // indirect
	github.com/aws/smithy-go v1.23.1 // indirect
	github.com/google/uuid v1.6.0 // indirect
)
