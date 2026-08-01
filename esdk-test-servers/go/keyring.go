// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Translation from the modeled CMM/Keyring configuration to real MPL
// cryptographic materials managers and keyrings. Enforces the "exactly one
// variant member set" invariant the tagged-union-via-optional-members shapes
// cannot express in the type system; everything else is delegated to the real
// MPL constructors so their validation failures surface as-is.

package main

import (
	"bytes"
	"context"
	"encoding/pem"
	"fmt"
	"os"
	"strings"

	keystore "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygenerated"
	keystoretypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographykeystoresmithygeneratedtypes"
	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/aws"
	awsconfig "github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/kms"
)

// buildCMM resolves a modeled CryptographicMaterialsManager to a real MPL
// materials manager.
func buildCMM(ctx context.Context, matProv *mpl.Client, cmm *CMMConfig) (mpltypes.ICryptographicMaterialsManager, error) {
	set := 0
	for _, present := range []bool{cmm.Default != nil, cmm.RequiredEncryptionContext != nil, cmm.Caching != nil} {
		if present {
			set++
		}
	}
	if set != 1 {
		return nil, fmt.Errorf("CryptographicMaterialsManager must set exactly one variant, found %d", set)
	}

	switch {
	case cmm.Default != nil:
		keyring, err := buildKeyring(ctx, matProv, cmm.Default.Keyring)
		if err != nil {
			return nil, err
		}
		return matProv.CreateDefaultCryptographicMaterialsManager(ctx,
			mpltypes.CreateDefaultCryptographicMaterialsManagerInput{Keyring: keyring})
	case cmm.RequiredEncryptionContext != nil:
		underlying, err := buildCMM(ctx, matProv, cmm.RequiredEncryptionContext.UnderlyingCMM)
		if err != nil {
			return nil, err
		}
		return matProv.CreateRequiredEncryptionContextCMM(ctx,
			mpltypes.CreateRequiredEncryptionContextCMMInput{
				UnderlyingCMM:                 underlying,
				RequiredEncryptionContextKeys: cmm.RequiredEncryptionContext.RequiredEncryptionContextKeys,
			})
	default:
		return nil, fmt.Errorf("the Caching CMM is not supported: no caching CMM exists in the MPL")
	}
}

// buildKeyring resolves a modeled Keyring to a real MPL keyring, recursing
// through Multi variants.
func buildKeyring(ctx context.Context, matProv *mpl.Client, keyring *KeyringConfig) (mpltypes.IKeyring, error) {
	set := 0
	for _, present := range []bool{
		keyring.AwsKms != nil,
		keyring.AwsKmsMrk != nil,
		keyring.AwsKmsMultiKeyring != nil,
		keyring.AwsKmsMrkMultiKeyring != nil,
		keyring.AwsKmsDiscovery != nil,
		keyring.AwsKmsMrkDiscovery != nil,
		keyring.AwsKmsRsa != nil,
		keyring.RawAes != nil,
		keyring.RawRsa != nil,
		keyring.AwsKmsHierarchical != nil,
		keyring.Multi != nil,
	} {
		if present {
			set++
		}
	}
	if set != 1 {
		return nil, fmt.Errorf("Keyring must set exactly one variant, found %d", set)
	}

	switch {
	case keyring.RawAes != nil:
		cfg := keyring.RawAes
		return matProv.CreateRawAesKeyring(ctx, mpltypes.CreateRawAesKeyringInput{
			KeyNamespace: cfg.KeyNamespace,
			KeyName:      cfg.KeyName,
			WrappingKey:  cfg.WrappingKey,
			WrappingAlg:  cfg.WrappingAlg,
		})
	case keyring.RawRsa != nil:
		cfg := keyring.RawRsa
		return matProv.CreateRawRsaKeyring(ctx, mpltypes.CreateRawRsaKeyringInput{
			KeyNamespace:  cfg.KeyNamespace,
			KeyName:       cfg.KeyName,
			PaddingScheme: cfg.PaddingScheme,
			PublicKey:     cfg.PublicKey,
			PrivateKey:    cfg.PrivateKey,
		})
	case keyring.AwsKms != nil:
		cfg := keyring.AwsKms
		client, err := kmsClientForKey(ctx, cfg.KmsKeyID)
		if err != nil {
			return nil, err
		}
		return matProv.CreateAwsKmsKeyring(ctx, mpltypes.CreateAwsKmsKeyringInput{
			KmsClient:   client,
			KmsKeyId:    cfg.KmsKeyID,
			GrantTokens: cfg.GrantTokens,
		})
	case keyring.AwsKmsMrk != nil:
		cfg := keyring.AwsKmsMrk
		client, err := kmsClientForKey(ctx, cfg.KmsKeyID)
		if err != nil {
			return nil, err
		}
		return matProv.CreateAwsKmsMrkKeyring(ctx, mpltypes.CreateAwsKmsMrkKeyringInput{
			KmsClient:   client,
			KmsKeyId:    cfg.KmsKeyID,
			GrantTokens: cfg.GrantTokens,
		})
	case keyring.AwsKmsMultiKeyring != nil:
		cfg := keyring.AwsKmsMultiKeyring
		return matProv.CreateAwsKmsMultiKeyring(ctx, mpltypes.CreateAwsKmsMultiKeyringInput{
			Generator: cfg.Generator,
			KmsKeyIds: cfg.KmsKeyIDs,
		})
	case keyring.AwsKmsMrkMultiKeyring != nil:
		cfg := keyring.AwsKmsMrkMultiKeyring
		return matProv.CreateAwsKmsMrkMultiKeyring(ctx, mpltypes.CreateAwsKmsMrkMultiKeyringInput{
			Generator: cfg.Generator,
			KmsKeyIds: cfg.KmsKeyIDs,
		})
	case keyring.AwsKmsDiscovery != nil:
		cfg := keyring.AwsKmsDiscovery
		client, err := kmsClient(ctx, defaultRegion())
		if err != nil {
			return nil, err
		}
		return matProv.CreateAwsKmsDiscoveryKeyring(ctx, mpltypes.CreateAwsKmsDiscoveryKeyringInput{
			KmsClient:       client,
			DiscoveryFilter: discoveryFilter(cfg.DiscoveryFilter),
			GrantTokens:     cfg.GrantTokens,
		})
	case keyring.AwsKmsMrkDiscovery != nil:
		cfg := keyring.AwsKmsMrkDiscovery
		client, err := kmsClient(ctx, cfg.Region)
		if err != nil {
			return nil, err
		}
		return matProv.CreateAwsKmsMrkDiscoveryKeyring(ctx, mpltypes.CreateAwsKmsMrkDiscoveryKeyringInput{
			KmsClient:       client,
			Region:          cfg.Region,
			DiscoveryFilter: discoveryFilter(cfg.DiscoveryFilter),
			GrantTokens:     cfg.GrantTokens,
		})
	case keyring.AwsKmsRsa != nil:
		return buildAwsKmsRsaKeyring(ctx, matProv, keyring.AwsKmsRsa)
	case keyring.AwsKmsHierarchical != nil:
		return buildAwsKmsHierarchicalKeyring(ctx, matProv, keyring.AwsKmsHierarchical)
	default:
		return buildMultiKeyring(ctx, matProv, keyring.Multi)
	}
}

func buildAwsKmsRsaKeyring(ctx context.Context, matProv *mpl.Client, cfg *AwsKmsRsaKeyringConfig) (mpltypes.IKeyring, error) {
	client, err := kmsClientForKey(ctx, cfg.KmsKeyID)
	if err != nil {
		return nil, err
	}
	publicKey := cfg.PublicKey
	if publicKey == nil {
		// Fetch the RSA public key from KMS so the keyring can OnEncrypt. KMS
		// GetPublicKey returns DER (X.509 SubjectPublicKeyInfo); the MPL wants PEM.
		out, err := client.GetPublicKey(ctx, &kms.GetPublicKeyInput{KeyId: &cfg.KmsKeyID})
		if err != nil {
			return nil, err
		}
		publicKey = derToPublicKeyPEM(out.PublicKey)
	}
	input := mpltypes.CreateAwsKmsRsaKeyringInput{
		KmsClient:   client,
		KmsKeyId:    cfg.KmsKeyID,
		PublicKey:   publicKey,
		GrantTokens: cfg.GrantTokens,
	}
	if cfg.EncryptionAlgorithm != nil {
		input.EncryptionAlgorithm = *cfg.EncryptionAlgorithm
	}
	return matProv.CreateAwsKmsRsaKeyring(ctx, input)
}

func buildAwsKmsHierarchicalKeyring(ctx context.Context, matProv *mpl.Client, cfg *AwsKmsHierarchicalKeyringConfig) (mpltypes.IKeyring, error) {
	awsCfg, err := awsConfigForRegion(ctx, defaultRegion())
	if err != nil {
		return nil, err
	}
	keyStore, err := keystore.NewClient(keystoretypes.KeyStoreConfig{
		DdbTableName:        cfg.KeyStoreTableName,
		LogicalKeyStoreName: cfg.LogicalKeyStoreName,
		KmsConfiguration:    &keystoretypes.KMSConfigurationMemberkmsKeyArn{Value: cfg.KmsKeyArn},
		DdbClient:           dynamodb.NewFromConfig(awsCfg),
		KmsClient:           kms.NewFromConfig(awsCfg),
	})
	if err != nil {
		return nil, err
	}
	return matProv.CreateAwsKmsHierarchicalKeyring(ctx, mpltypes.CreateAwsKmsHierarchicalKeyringInput{
		KeyStore:    keyStore,
		BranchKeyId: &cfg.BranchKeyID,
		TtlSeconds:  cfg.TTLSeconds,
		Cache:       &mpltypes.CacheTypeMemberDefault{Value: mpltypes.DefaultCache{EntryCapacity: 100}},
	})
}

func buildMultiKeyring(ctx context.Context, matProv *mpl.Client, cfg *MultiKeyringConfig) (mpltypes.IKeyring, error) {
	children := make([]mpltypes.IKeyring, 0, len(cfg.ChildKeyrings))
	for _, child := range cfg.ChildKeyrings {
		built, err := buildKeyring(ctx, matProv, child)
		if err != nil {
			return nil, err
		}
		children = append(children, built)
	}
	var generator mpltypes.IKeyring
	if cfg.Generator != nil {
		built, err := buildKeyring(ctx, matProv, cfg.Generator)
		if err != nil {
			return nil, err
		}
		generator = built
	}
	return matProv.CreateMultiKeyring(ctx, mpltypes.CreateMultiKeyringInput{
		Generator:     generator,
		ChildKeyrings: children,
	})
}

func discoveryFilter(cfg *DiscoveryFilterConfig) *mpltypes.DiscoveryFilter {
	if cfg == nil {
		return nil
	}
	return &mpltypes.DiscoveryFilter{Partition: cfg.Partition, AccountIds: cfg.AccountIDs}
}

// defaultRegion resolves the AWS region for KMS and DynamoDB clients.
func defaultRegion() string {
	for _, key := range []string{"AWS_REGION", "AWS_DEFAULT_REGION", "ESDK_TESTSERVER_KMS_REGION"} {
		if value := os.Getenv(key); value != "" {
			return value
		}
	}
	return "us-west-2"
}

// awsConfigForRegion loads the default AWS configuration (credential chain,
// shared config) with the region overridden, so every KMS and DynamoDB
// request the server makes is signed.
func awsConfigForRegion(ctx context.Context, region string) (aws.Config, error) {
	return awsconfig.LoadDefaultConfig(ctx, awsconfig.WithRegion(region))
}

func kmsClient(ctx context.Context, region string) (*kms.Client, error) {
	cfg, err := awsConfigForRegion(ctx, region)
	if err != nil {
		return nil, err
	}
	return kms.NewFromConfig(cfg), nil
}

// kmsClientForKey builds a KMS client in the key's own region: KMS rejects an
// ARN whose region differs from the client's region. Falls back to the default
// region for a bare key id or alias that carries no region.
func kmsClientForKey(ctx context.Context, kmsKeyID string) (*kms.Client, error) {
	region := defaultRegion()
	if strings.HasPrefix(kmsKeyID, "arn:") {
		if parts := strings.Split(kmsKeyID, ":"); len(parts) > 3 && parts[3] != "" {
			region = parts[3]
		}
	}
	return kmsClient(ctx, region)
}

// derToPublicKeyPEM wraps DER (X.509 SubjectPublicKeyInfo) bytes as a PEM
// PUBLIC KEY block, passing bytes that already look like PEM through unchanged.
func derToPublicKeyPEM(der []byte) []byte {
	if bytes.HasPrefix(der, []byte("-----BEGIN ")) {
		return der
	}
	return pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der})
}
