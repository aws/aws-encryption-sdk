// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package benchmark

import (
	"context"
	"crypto/rand"
	"fmt"
	"log"
	"runtime"

	mplsmithygenerated "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	esdksmithygenerated "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygenerated"
	esdktypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/awscryptographyencryptionsdksmithygeneratedtypes"
	"github.com/shirou/gopsutil/v3/mem"
)

// Constants for memory testing
const (
	MemoryTestIterations = 5
	SamplingIntervalMs   = 1
	GCSettleTimeMs       = 5
	FinalSampleWaitMs    = 2
)

// ESDKBenchmark is the main benchmark struct
type ESDKBenchmark struct {
	Config        TestConfig
	EsdkClient    *esdksmithygenerated.Client
	Keyring       mpltypes.IKeyring
	Results       []BenchmarkResult
	CPUCount      int
	TotalMemoryGB float64
}

// New creates a new benchmark instance
func New(configPath string) (*ESDKBenchmark, error) {
	benchmark := &ESDKBenchmark{
		CPUCount: runtime.NumCPU(),
	}

	// Get system memory
	if vmStat, err := mem.VirtualMemory(); err == nil {
		benchmark.TotalMemoryGB = float64(vmStat.Total) / (1024 * 1024 * 1024)
	}

	// Load configuration
	config, err := LoadConfig(configPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load config: %w", err)
	}
	benchmark.Config = config

	// Setup ESDK
	if err := benchmark.setupESDK(); err != nil {
		return nil, fmt.Errorf("failed to setup ESDK: %w", err)
	}

	log.Printf("Initialized ESDK Benchmark - CPU cores: %d, Memory: %.1fGB",
		benchmark.CPUCount, benchmark.TotalMemoryGB)

	return benchmark, nil
}

// setupESDK initializes the ESDK client and creates a default keyring
func (b *ESDKBenchmark) setupESDK() error {
	// Initialize the material providers client
	matProvConfig := mpltypes.MaterialProvidersConfig{}
	matProv, err := mplsmithygenerated.NewClient(matProvConfig)
	if err != nil {
		return fmt.Errorf("failed to create material providers client: %w", err)
	}

	// Create default AES-256 keyring
	key := make([]byte, 32) // 256-bit key
	if _, err := rand.Read(key); err != nil {
		return fmt.Errorf("failed to generate AES-256 key: %w", err)
	}

	keyringInput := mpltypes.CreateRawAesKeyringInput{
		KeyName:      "test-aes-256-key",
		KeyNamespace: "esdk-performance-test",
		WrappingKey:  key,
		WrappingAlg:  mpltypes.AesWrappingAlgAlgAes256GcmIv12Tag16,
	}

	keyring, err := matProv.CreateRawAesKeyring(context.Background(), keyringInput)
	if err != nil {
		return fmt.Errorf("failed to create keyring: %w", err)
	}
	b.Keyring = keyring

	// Create ESDK client with proper commitment policy
	commitmentPolicy := mpltypes.ESDKCommitmentPolicyRequireEncryptRequireDecrypt
	esdkConfig := esdktypes.AwsEncryptionSdkConfig{
		CommitmentPolicy: &commitmentPolicy,
	}

	esdkClient, err := esdksmithygenerated.NewClient(esdkConfig)
	if err != nil {
		return fmt.Errorf("failed to create ESDK client: %w", err)
	}
	b.EsdkClient = esdkClient

	log.Println("ESDK client initialized successfully")
	return nil
}

// GenerateTestData creates test data of specified size
func (b *ESDKBenchmark) GenerateTestData(size int) []byte {
	data := make([]byte, size)
	rand.Read(data)
	return data
}
