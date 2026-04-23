module github.com/aws/aws-encryption-sdk/esdk-performance-testing/benchmarks/go

go 1.23.0

replace github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk => ../../../AwsEncryptionSDK/runtimes/go/ImplementationFromDafny-go

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb => ../../../mpl/ComAmazonawsDynamodb/runtimes/go/ImplementationFromDafny-go

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms => ../../../mpl/ComAmazonawsKms/runtimes/go/ImplementationFromDafny-go

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives => ../../../mpl/AwsCryptographyPrimitives/runtimes/go/ImplementationFromDafny-go

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library => ../../../mpl/StandardLibrary/runtimes/go/ImplementationFromDafny-go

replace github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl => ../../../mpl/AwsCryptographicMaterialProviders/runtimes/go/ImplementationFromDafny-go

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl v0.3.0
	github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk v0.3.0
	github.com/schollz/progressbar/v3 v3.18.0
	github.com/shirou/gopsutil/v3 v3.23.12
	gopkg.in/yaml.v3 v3.0.1
)

require (
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb v0.3.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms v0.3.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives v0.3.0 // indirect
	github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library v0.3.0 // indirect
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
	github.com/dafny-lang/DafnyRuntimeGo/v4 v4.11.3 // indirect
	github.com/go-ole/go-ole v1.2.6 // indirect
	github.com/google/uuid v1.6.0 // indirect
	github.com/lufia/plan9stats v0.0.0-20211012122336-39d0f177ccd0 // indirect
	github.com/mitchellh/colorstring v0.0.0-20190213212951-d06e56a500db // indirect
	github.com/power-devops/perfstat v0.0.0-20210106213030-5aafc221ea8c // indirect
	github.com/rivo/uniseg v0.4.7 // indirect
	github.com/tklauser/go-sysconf v0.3.12 // indirect
	github.com/tklauser/numcpus v0.6.1 // indirect
	github.com/yusufpapurcu/wmi v1.2.3 // indirect
	golang.org/x/sys v0.29.0 // indirect
	golang.org/x/term v0.28.0 // indirect
)
