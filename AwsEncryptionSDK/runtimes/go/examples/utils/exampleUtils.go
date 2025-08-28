// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package utils

import (
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/x509"
	"encoding/pem"
	"errors"
	"fmt"
	"os"

	"github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/awscryptographyprimitivessmithygeneratedtypes"
	"github.com/aws/aws-sdk-go-v2/service/kms"
	"github.com/google/uuid"
)

const (
	testKmsRsaPublicKey = `-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA27Uc/fBaMVhxCE/SpCMQ
oSBRSzQJw+o2hBaA+FiPGtiJ/aPy7sn18aCkelaSj4kwoC79b/arNHlkjc7OJFsN
/GoFKgNvaiY4lOeJqEiWQGSSgHtsJLdbO2u4OOSxh8qIRAMKbMgQDVX4FR/PLKeK
fc2aCDvcNSpAM++8NlNmv7+xQBJydr5ce91eISbHkFRkK3/bAM+1iddupoRw4Wo2
r3avzrg5xBHmzR7u1FTab22Op3Hgb2dBLZH43wNKAceVwKqKA8UNAxashFON7xK9
yy4kfOL0Z/nhxRKe4jRZ/5v508qIzgzCksYy7Y3QbMejAtiYnr7s5/d5KWw0swou
twIDAQAB
-----END PUBLIC KEY-----`
	testKmsRsaKeyID                    = "arn:aws:kms:us-west-2:370957321024:key/mrk-63d386cb70614ea59b32ad65c9315297"
	testDefaultKMSKeyId                = "arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f"
	defaultKmsKeyRegion                = "us-west-2"
	testAlternateRegionKMSKeyId        = "arn:aws:kms:eu-central-1:658956600833:key/75414c93-5285-4b57-99c9-30c1cf0a22c2"
	testAlternateRegionKMSKeyRegion    = "eu-central-1"
	testDefaultMRKKeyId                = "arn:aws:kms:us-east-1:658956600833:key/mrk-80bd8ecdcd4342aebd84b7dc9da498a7"
	defaultMRKKeyRegion                = "us-east-1"
	testAlternateRegionMrkKeyId        = "arn:aws:kms:eu-west-1:658956600833:key/mrk-80bd8ecdcd4342aebd84b7dc9da498a7"
	alternateRegionMrkKeyRegion        = "eu-west-1"
	testKeyStoreKMSKeyRegion           = "us-west-2"
	testKeyStoreKMSKeyID               = "arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126"
	testLogicalKeyStoreName            = "KeyStoreDdbTable"
	testKeyStoreName                   = "KeyStoreDdbTable"
	testKeyStoreRegion                 = "us-west-2"
	defaultKMSKeyAccountID             = "658956600833"
	eccPrivateKeyFileNameSender        = "sender_private.pem"
	eccPrivateKeyFileNameRecipient     = "recipient_private.pem"
	eccPublicKeyFileNameRecipient      = "recipient_public.pem"
	kmsEccPublicKeyFileNameRecipient   = "KmsEccKeyringExamplePublicKeyRecipient.pem"
	kmsEccPublicKeyFileNameSender      = "KmsEccKeyringExamplePublicKeySender.pem"
	testKmsEcdhKeyIdP256SenderKeyId    = "arn:aws:kms:us-west-2:370957321024:key/eabdf483-6be2-4d2d-8ee4-8c2583d416e9"
	testKmsEcdhKeyIdP256RecipientKeyId = "arn:aws:kms:us-west-2:370957321024:key/0265c8e9-5b6a-4055-8f70-63719e09fda5"
)

var filesCreatedByExamples = []string{
	eccPrivateKeyFileNameSender,
	eccPrivateKeyFileNameRecipient,
	eccPublicKeyFileNameRecipient,
	kmsEccPublicKeyFileNameRecipient,
	kmsEccPublicKeyFileNameSender,
}

// Getter functions

func KmsEcdhKeyIdP256SenderKeyId() string {
	return testKmsEcdhKeyIdP256SenderKeyId
}

func KmsEcdhKeyIdP256RecipientKeyId() string {
	return testKmsEcdhKeyIdP256RecipientKeyId
}

func KmsEccPublicKeyFileNameRecipient() string {
	return kmsEccPublicKeyFileNameRecipient
}

func KmsEccPublicKeyFileNameSender() string {
	return kmsEccPublicKeyFileNameSender
}

func EccPrivateKeyFileNameSender() string {
	return eccPrivateKeyFileNameSender
}

func EccPrivateKeyFileNameRecipient() string {
	return eccPrivateKeyFileNameRecipient
}

func EccPublicKeyFileNameRecipient() string {
	return eccPublicKeyFileNameRecipient
}

func RegionsOfMRKKeys() []string {
	return []string{defaultMRKKeyRegion, alternateRegionMrkKeyRegion}
}

func Regions() []string {
	return []string{defaultKmsKeyRegion, testAlternateRegionKMSKeyRegion}
}

func DefaultKmsKeyRegion() string {
	return defaultKmsKeyRegion
}

func DefaultMRKKeyRegion() string {
	return defaultMRKKeyRegion
}

func AlternateRegionMrkKeyRegion() string {
	return alternateRegionMrkKeyRegion
}

func AlternateRegionMrkKeyArn() string {
	return testAlternateRegionMrkKeyId
}

func DefaultRegionMrkKeyArn() string {
	return testDefaultMRKKeyId
}

func AlternateRegionKMSKeyRegion() string {
	return testAlternateRegionKMSKeyRegion
}

func AlternateRegionKMSKeyId() string {
	return testAlternateRegionKMSKeyId
}

func DefaultKMSKeyAccountID() string {
	return defaultKMSKeyAccountID
}

func DefaultKMSKeyId() string {
	return testDefaultKMSKeyId
}

func TestKmsRsaKeyID() string {
	return testKmsRsaKeyID
}

func KmsRSAPublicKey() []byte {
	return []byte(testKmsRsaPublicKey)
}

func KeyStoreRegion() string {
	return testKeyStoreRegion
}

func KeyStoreKMSKeyRegion() string {
	return testKeyStoreKMSKeyRegion
}

func KeyStoreKMSKeyID() string {
	return testKeyStoreKMSKeyID
}

func LogicalKeyStoreName() string {
	return testLogicalKeyStoreName
}

func KeyStoreName() string {
	return testKeyStoreName
}

// Utility functions

func CleanUpFiles() {
	for _, file := range filesCreatedByExamples {
		os.Remove(file)
	}
}

func WriteRawEcdhEccKeys(ecdhCurveSpec awscryptographyprimitivessmithygeneratedtypes.ECDHCurveSpec) error {
	// Safety check: Validate neither file is present
	if FileExists(eccPrivateKeyFileNameSender) ||
		FileExists(eccPrivateKeyFileNameRecipient) ||
		FileExists(eccPublicKeyFileNameRecipient) {
		return errors.New("WriteRawEcdhEccKeys will not overwrite existing PEM files")
	}

	// Generate key pairs
	_, privateKeySender, err := generateRawEccKeyPair(ecdhCurveSpec)
	if err != nil {
		return err
	}

	publicKeyRecipient, privateKeyRecipient, err := generateRawEccKeyPair(ecdhCurveSpec)
	if err != nil {
		return err
	}

	// Create PEM blocks
	privateKeySenderPEM := &pem.Block{
		Type:  "PRIVATE KEY",
		Bytes: privateKeySender,
	}

	privateKeyRecipientPEM := &pem.Block{
		Type:  "PRIVATE KEY",
		Bytes: privateKeyRecipient,
	}

	publicKeyRecipientPEM := &pem.Block{
		Type:  "PUBLIC KEY",
		Bytes: publicKeyRecipient,
	}

	// Write private key for sender in PEM format
	err = os.WriteFile(
		eccPrivateKeyFileNameSender,
		pem.EncodeToMemory(privateKeySenderPEM),
		0600,
	)
	if err != nil {
		return fmt.Errorf("failed to write sender's private key: %w", err)
	}

	// Write private key for recipient in PEM format
	err = os.WriteFile(
		eccPrivateKeyFileNameRecipient,
		pem.EncodeToMemory(privateKeyRecipientPEM),
		0600,
	)
	if err != nil {
		return fmt.Errorf("failed to write recipient's private key: %w", err)
	}

	// Write public key for recipient in PEM format
	err = os.WriteFile(
		eccPublicKeyFileNameRecipient,
		pem.EncodeToMemory(publicKeyRecipientPEM),
		0600,
	)
	if err != nil {
		return fmt.Errorf("failed to write recipient's public key: %w", err)
	}

	return nil
}

func LoadPublicKeyFromPEM(filename string) ([]byte, error) {
	// Read the PEM file content as string
	pemContent, err := os.ReadFile(filename)
	if err != nil {
		return nil, fmt.Errorf("failed to read PEM file: %w", err)
	}
	// Parse PEM block
	block, _ := pem.Decode(pemContent)

	if block == nil {
		return nil, fmt.Errorf("failed to decode PEM block")
	}

	// The block.Bytes contains the DER encoded key
	return block.Bytes, nil
}

func FileExists(filename string) bool {
	_, err := os.Stat(filename)
	return !os.IsNotExist(err)
}

func generateRawEccKeyPair(curveSpec awscryptographyprimitivessmithygeneratedtypes.ECDHCurveSpec) ([]byte, []byte, error) {
	// Select the appropriate elliptic curve based on the specification
	var curve elliptic.Curve
	switch curveSpec {
	case awscryptographyprimitivessmithygeneratedtypes.ECDHCurveSpecEccNistP256:
		curve = elliptic.P256()
	case awscryptographyprimitivessmithygeneratedtypes.ECDHCurveSpecEccNistP384:
		curve = elliptic.P384()
	case awscryptographyprimitivessmithygeneratedtypes.ECDHCurveSpecEccNistP521:
		curve = elliptic.P521()
	default:
		return nil, nil, fmt.Errorf("unsupported curve specification: %s", curveSpec)
	}
	// Generate the private key
	privateKey, err := ecdsa.GenerateKey(curve, rand.Reader)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to generate private key: %w", err)
	}
	// Extract the public key
	publicKey := &privateKey.PublicKey
	// Marshal the private key to bytes (X.509 PKCS#8 format)
	privateKeyBytes, err := x509.MarshalPKCS8PrivateKey(privateKey)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal private key: %w", err)
	}
	// Marshal the public key to bytes (X.509 SPKI format)
	publicKeyBytes, err := x509.MarshalPKIXPublicKey(publicKey)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to marshal public key: %w", err)
	}
	return publicKeyBytes, privateKeyBytes, nil
}

func WriteKmsEcdhEccPublicKey(eccKeyArn, publicKeyFileName string, kmsClient *kms.Client) error {
	// Safety check: Validate neither file is present
	if FileExists(publicKeyFileName) {
		return errors.New("WriteKmsEcdhEccPublicKey will not overwrite existing PEM files")
	}
	// Generate public key
	publicKey, err := GenerateKmsEccPublicKey(eccKeyArn, kmsClient)
	if err != nil {
		return fmt.Errorf("failed to generate public key: %w", err)
	}
	// Create PEM block
	pemBlock := &pem.Block{
		Type:  "PUBLIC KEY",
		Bytes: publicKey,
	}
	// Encode PEM
	pemData := pem.EncodeToMemory(pemBlock)
	if pemData == nil {
		return errors.New("failed to encode PEM data")
	}
	// Write file with proper permissions
	err = os.WriteFile(publicKeyFileName, pemData, 0600)
	if err != nil {
		return fmt.Errorf("failed to write public key file: %w", err)
	}
	return nil
}

func GenerateKmsEccPublicKey(eccKeyArn string, kmsClient *kms.Client) ([]byte, error) {
	ctx := context.Background()
	// Get public key from KMS
	response, err := kmsClient.GetPublicKey(ctx, &kms.GetPublicKeyInput{
		KeyId: &eccKeyArn,
	})
	if err != nil {
		return nil, fmt.Errorf("failed to get public key from KMS: %w", err)
	}
	// Check if public key is present
	if response.PublicKey == nil {
		return nil, errors.New("no public key in KMS response")
	}
	return response.PublicKey, nil
}

// GenerateUUIDTestData creates an array of random UUID strings
func GenerateUUIDTestData(count int) []string {
	testData := make([]string, count)
	for i := 0; i < count; i++ {
		// Generate a random UUID
		uuid := uuid.New()
		testData[i] = uuid.String()
	}
	return testData
}
