package clientsupplier

import (
	"context"

	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/credentials/stscreds"
	"github.com/aws/aws-sdk-go-v2/service/kms"
	"github.com/aws/aws-sdk-go-v2/service/sts"
)

/*
 Example class demonstrating an implementation of a custom client supplier.
 This particular implementation will create KMS clients with different IAM roles,
 depending on the region passed.
*/

// RegionalRoleClientSupplier provides implementation for mpltypes.IClientSupplier
type RegionalRoleClientSupplier struct {
}

func (this *RegionalRoleClientSupplier) GetClient(input mpltypes.GetClientInput) (kms.Client, error) {
	region := input.Region
	// Check if the region is supported
	regionIamRoleMap := RegionIamRoleMap()
	var defaultVal kms.Client
	// Check if region is supported
	if _, exists := regionIamRoleMap[region]; !exists {
		return defaultVal, mpltypes.AwsCryptographicMaterialProvidersException{
			Message: "Region is not supported by this client supplier",
		}
	}
	// Get the IAM role ARN associated with the region
	arn := regionIamRoleMap[region]
	ctx := context.TODO()
	cfg, err := config.LoadDefaultConfig(ctx,
		config.WithRegion(region),
	)
	if err != nil {
		return defaultVal, err
	}
	stsClient := sts.NewFromConfig(cfg)
	// Create the AssumeRoleProvider
	provider := stscreds.NewAssumeRoleProvider(stsClient, arn, func(o *stscreds.AssumeRoleOptions) {
		o.RoleSessionName = "Go-ESDK-Client-Supplier-Example-Session"
	})
	// Load AWS SDK configuration with the AssumeRoleProvider
	sdkConfig, err := config.LoadDefaultConfig(context.Background(), config.WithRegion(region), config.WithCredentialsProvider(provider))
	if err != nil {
		return defaultVal, mpltypes.AwsCryptographicMaterialProvidersException{Message: "failed to load AWS SDK config"}
	}
	// Create the KMS client
	kmsClient := kms.NewFromConfig(sdkConfig)
	// Return the KMS client wrapped in a custom type
	return *kmsClient, nil
}
