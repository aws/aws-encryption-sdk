// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Native Go mirror of the ESDK TestServer Smithy model shapes and their
// rpcv2Cbor wire form: structure members are keyed by the Smithy member name,
// blobs travel as CBOR byte strings, and enums as the Smithy member NAME
// strings (the MPL enum VALUES are hex suite ids, so both directions map by
// name through the tables below).
//
// The polymorphic CryptographicMaterialsManager and Keyring shapes are tagged
// unions via optional members: every variant decodes as optional here and the
// exactly-one-set invariant is enforced at build time in keyring.go.

package main

import (
	"fmt"

	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
	kmstypes "github.com/aws/aws-sdk-go-v2/service/kms/types"
	"github.com/aws/smithy-go/encoding/cbor"
)

// Operation request / response shapes.

type CreateClientRequest struct {
	Config *ESDKClientConfig
}

type CreateClientResponse struct {
	ClientID string
}

func (r *CreateClientResponse) toCBOR() cbor.Value {
	return cbor.Map{"clientId": cbor.String(r.ClientID)}
}

type EncryptRequest struct {
	ClientID          string
	Plaintext         []byte
	EncryptionContext map[string]string
	AlgorithmSuiteID  *mpltypes.ESDKAlgorithmSuiteId
	FrameLength       *int64
}

type EncryptResponse struct {
	Ciphertext []byte
}

func (r *EncryptResponse) toCBOR() cbor.Value {
	return cbor.Map{"ciphertext": cbor.Slice(r.Ciphertext)}
}

type DecryptRequest struct {
	ClientID          string
	Ciphertext        []byte
	EncryptionContext map[string]string
}

type DecryptResponse struct {
	Plaintext         []byte
	EncryptionContext map[string]string
	AlgorithmSuiteID  mpltypes.ESDKAlgorithmSuiteId
}

func (r *DecryptResponse) toCBOR() cbor.Value {
	m := cbor.Map{"plaintext": cbor.Slice(r.Plaintext)}
	if len(r.EncryptionContext) > 0 {
		ec := make(cbor.Map, len(r.EncryptionContext))
		for k, v := range r.EncryptionContext {
			ec[k] = cbor.String(v)
		}
		m["encryptionContext"] = ec
	}
	if name, ok := algorithmSuiteWireNames[r.AlgorithmSuiteID]; ok {
		m["algorithmSuiteId"] = cbor.String(name)
	}
	return m
}

// Client configuration shapes.

type ESDKClientConfig struct {
	CommitmentPolicy     mpltypes.ESDKCommitmentPolicy
	MaxEncryptedDataKeys *int64
	CMM                  *CMMConfig
}

type CMMConfig struct {
	Default                   *DefaultCMMConfig
	RequiredEncryptionContext *RequiredEncryptionContextCMMConfig
	Caching                   *CachingCMMConfig
}

type DefaultCMMConfig struct {
	Keyring *KeyringConfig
}

type RequiredEncryptionContextCMMConfig struct {
	UnderlyingCMM                 *CMMConfig
	RequiredEncryptionContextKeys []string
}

// CachingCMMConfig marks the Caching variant as present; its members are not
// decoded because the server rejects the variant as unsupported.
type CachingCMMConfig struct{}

type KeyringConfig struct {
	AwsKms                *AwsKmsKeyringConfig
	AwsKmsMrk             *AwsKmsKeyringConfig
	AwsKmsMultiKeyring    *AwsKmsMultiKeyringConfig
	AwsKmsMrkMultiKeyring *AwsKmsMultiKeyringConfig
	AwsKmsDiscovery       *AwsKmsDiscoveryKeyringConfig
	AwsKmsMrkDiscovery    *AwsKmsMrkDiscoveryKeyringConfig
	AwsKmsRsa             *AwsKmsRsaKeyringConfig
	RawAes                *RawAesKeyringConfig
	RawRsa                *RawRsaKeyringConfig
	AwsKmsHierarchical    *AwsKmsHierarchicalKeyringConfig
	Multi                 *MultiKeyringConfig
}

// AwsKmsKeyringConfig carries the AwsKms and AwsKmsMrk variants (same shape).
type AwsKmsKeyringConfig struct {
	KmsKeyID    string
	GrantTokens []string
}

// AwsKmsMultiKeyringConfig carries the AwsKmsMultiKeyring and
// AwsKmsMrkMultiKeyring variants (same shape).
type AwsKmsMultiKeyringConfig struct {
	Generator *string
	KmsKeyIDs []string
}

type AwsKmsDiscoveryKeyringConfig struct {
	DiscoveryFilter *DiscoveryFilterConfig
	GrantTokens     []string
}

type AwsKmsMrkDiscoveryKeyringConfig struct {
	Region          string
	DiscoveryFilter *DiscoveryFilterConfig
	GrantTokens     []string
}

type AwsKmsRsaKeyringConfig struct {
	KmsKeyID            string
	PublicKey           []byte
	EncryptionAlgorithm *kmstypes.EncryptionAlgorithmSpec
	GrantTokens         []string
}

type DiscoveryFilterConfig struct {
	Partition  string
	AccountIDs []string
}

type RawAesKeyringConfig struct {
	KeyNamespace string
	KeyName      string
	WrappingKey  []byte
	WrappingAlg  mpltypes.AesWrappingAlg
}

type RawRsaKeyringConfig struct {
	KeyNamespace  string
	KeyName       string
	PaddingScheme mpltypes.PaddingScheme
	PublicKey     []byte
	PrivateKey    []byte
}

type AwsKmsHierarchicalKeyringConfig struct {
	BranchKeyID         string
	KeyStoreTableName   string
	LogicalKeyStoreName string
	KmsKeyArn           string
	TTLSeconds          int64
}

type MultiKeyringConfig struct {
	Generator     *KeyringConfig
	ChildKeyrings []*KeyringConfig
}

// Enum tables: wire member name -> library constant.

var commitmentPolicies = map[string]mpltypes.ESDKCommitmentPolicy{
	"FORBID_ENCRYPT_ALLOW_DECRYPT":    mpltypes.ESDKCommitmentPolicyForbidEncryptAllowDecrypt,
	"REQUIRE_ENCRYPT_ALLOW_DECRYPT":   mpltypes.ESDKCommitmentPolicyRequireEncryptAllowDecrypt,
	"REQUIRE_ENCRYPT_REQUIRE_DECRYPT": mpltypes.ESDKCommitmentPolicyRequireEncryptRequireDecrypt,
}

var algorithmSuites = map[string]mpltypes.ESDKAlgorithmSuiteId{
	"ALG_AES_128_GCM_IV12_TAG16_NO_KDF":                 mpltypes.ESDKAlgorithmSuiteIdAlgAes128GcmIv12Tag16NoKdf,
	"ALG_AES_192_GCM_IV12_TAG16_NO_KDF":                 mpltypes.ESDKAlgorithmSuiteIdAlgAes192GcmIv12Tag16NoKdf,
	"ALG_AES_256_GCM_IV12_TAG16_NO_KDF":                 mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmIv12Tag16NoKdf,
	"ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256":            mpltypes.ESDKAlgorithmSuiteIdAlgAes128GcmIv12Tag16HkdfSha256,
	"ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256":            mpltypes.ESDKAlgorithmSuiteIdAlgAes192GcmIv12Tag16HkdfSha256,
	"ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256":            mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmIv12Tag16HkdfSha256,
	"ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256": mpltypes.ESDKAlgorithmSuiteIdAlgAes128GcmIv12Tag16HkdfSha256EcdsaP256,
	"ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384": mpltypes.ESDKAlgorithmSuiteIdAlgAes192GcmIv12Tag16HkdfSha384EcdsaP384,
	"ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384": mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmIv12Tag16HkdfSha384EcdsaP384,
	"ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY":            mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKey,
	"ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384": mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
}

// algorithmSuiteWireNames inverts algorithmSuites so decrypt can report the
// suite it determined from the message header.
var algorithmSuiteWireNames = func() map[mpltypes.ESDKAlgorithmSuiteId]string {
	names := make(map[mpltypes.ESDKAlgorithmSuiteId]string, len(algorithmSuites))
	for name, id := range algorithmSuites {
		names[id] = name
	}
	return names
}()

var aesWrappingAlgs = map[string]mpltypes.AesWrappingAlg{
	"ALG_AES128_GCM_IV12_TAG16": mpltypes.AesWrappingAlgAlgAes128GcmIv12Tag16,
	"ALG_AES192_GCM_IV12_TAG16": mpltypes.AesWrappingAlgAlgAes192GcmIv12Tag16,
	"ALG_AES256_GCM_IV12_TAG16": mpltypes.AesWrappingAlgAlgAes256GcmIv12Tag16,
}

var paddingSchemes = map[string]mpltypes.PaddingScheme{
	"PKCS1":            mpltypes.PaddingSchemePkcs1,
	"OAEP_SHA1_MGF1":   mpltypes.PaddingSchemeOaepSha1Mgf1,
	"OAEP_SHA256_MGF1": mpltypes.PaddingSchemeOaepSha256Mgf1,
	"OAEP_SHA384_MGF1": mpltypes.PaddingSchemeOaepSha384Mgf1,
	"OAEP_SHA512_MGF1": mpltypes.PaddingSchemeOaepSha512Mgf1,
}

var kmsRsaEncryptionAlgorithms = map[string]kmstypes.EncryptionAlgorithmSpec{
	"RSAES_OAEP_SHA_1":   kmstypes.EncryptionAlgorithmSpecRsaesOaepSha1,
	"RSAES_OAEP_SHA_256": kmstypes.EncryptionAlgorithmSpecRsaesOaepSha256,
}

// CBOR member helpers.

// member returns the value for key, treating CBOR null/undefined as absent.
func member(m cbor.Map, key string) (cbor.Value, bool) {
	v, ok := m[key]
	if !ok {
		return nil, false
	}
	switch v.(type) {
	case *cbor.Nil, *cbor.Undefined:
		return nil, false
	}
	return v, true
}

func asMap(v cbor.Value, what string) (cbor.Map, error) {
	m, ok := v.(cbor.Map)
	if !ok {
		return nil, fmt.Errorf("%s: expected a CBOR map, got %T", what, v)
	}
	return m, nil
}

func requireMember(m cbor.Map, key string) (cbor.Value, error) {
	v, ok := member(m, key)
	if !ok {
		return nil, fmt.Errorf("missing required member %s", key)
	}
	return v, nil
}

func asString(v cbor.Value, key string) (string, error) {
	s, ok := v.(cbor.String)
	if !ok {
		return "", fmt.Errorf("member %s: expected a text string, got %T", key, v)
	}
	return string(s), nil
}

func requireString(m cbor.Map, key string) (string, error) {
	v, err := requireMember(m, key)
	if err != nil {
		return "", err
	}
	return asString(v, key)
}

func optionalString(m cbor.Map, key string) (*string, error) {
	v, ok := member(m, key)
	if !ok {
		return nil, nil
	}
	s, err := asString(v, key)
	if err != nil {
		return nil, err
	}
	return &s, nil
}

func requireBytes(m cbor.Map, key string) ([]byte, error) {
	v, err := requireMember(m, key)
	if err != nil {
		return nil, err
	}
	b, ok := v.(cbor.Slice)
	if !ok {
		return nil, fmt.Errorf("member %s: expected a byte string, got %T", key, v)
	}
	return []byte(b), nil
}

func optionalBytes(m cbor.Map, key string) ([]byte, error) {
	if _, ok := member(m, key); !ok {
		return nil, nil
	}
	return requireBytes(m, key)
}

func requireInt64(m cbor.Map, key string) (int64, error) {
	v, err := requireMember(m, key)
	if err != nil {
		return 0, err
	}
	n, err := cbor.AsInt64(v)
	if err != nil {
		return 0, fmt.Errorf("member %s: %v", key, err)
	}
	return n, nil
}

func optionalInt64(m cbor.Map, key string) (*int64, error) {
	if _, ok := member(m, key); !ok {
		return nil, nil
	}
	n, err := requireInt64(m, key)
	if err != nil {
		return nil, err
	}
	return &n, nil
}

func optionalStringMap(m cbor.Map, key string) (map[string]string, error) {
	v, ok := member(m, key)
	if !ok {
		return nil, nil
	}
	vm, err := asMap(v, key)
	if err != nil {
		return nil, err
	}
	out := make(map[string]string, len(vm))
	for k, entry := range vm {
		s, err := asString(entry, key+"."+k)
		if err != nil {
			return nil, err
		}
		out[k] = s
	}
	return out, nil
}

func asStringList(v cbor.Value, key string) ([]string, error) {
	list, ok := v.(cbor.List)
	if !ok {
		return nil, fmt.Errorf("member %s: expected a list, got %T", key, v)
	}
	out := make([]string, 0, len(list))
	for _, entry := range list {
		s, err := asString(entry, key)
		if err != nil {
			return nil, err
		}
		out = append(out, s)
	}
	return out, nil
}

func requireStringList(m cbor.Map, key string) ([]string, error) {
	v, err := requireMember(m, key)
	if err != nil {
		return nil, err
	}
	return asStringList(v, key)
}

func optionalStringList(m cbor.Map, key string) ([]string, error) {
	v, ok := member(m, key)
	if !ok {
		return nil, nil
	}
	return asStringList(v, key)
}

func requireEnum[T ~string](m cbor.Map, key string, values map[string]T) (T, error) {
	var zero T
	s, err := requireString(m, key)
	if err != nil {
		return zero, err
	}
	v, ok := values[s]
	if !ok {
		return zero, fmt.Errorf("member %s: unknown enum value %q", key, s)
	}
	return v, nil
}

func optionalEnum[T ~string](m cbor.Map, key string, values map[string]T) (*T, error) {
	if _, ok := member(m, key); !ok {
		return nil, nil
	}
	v, err := requireEnum(m, key, values)
	if err != nil {
		return nil, err
	}
	return &v, nil
}

// Operation request decoding.

func decodeBody(body []byte, what string) (cbor.Map, error) {
	v, err := cbor.Decode(body)
	if err != nil {
		return nil, err
	}
	return asMap(v, what)
}

func decodeCreateClientRequest(body []byte) (*CreateClientRequest, error) {
	m, err := decodeBody(body, "CreateClientRequest")
	if err != nil {
		return nil, err
	}
	v, err := requireMember(m, "config")
	if err != nil {
		return nil, err
	}
	config, err := decodeClientConfig(v)
	if err != nil {
		return nil, err
	}
	return &CreateClientRequest{Config: config}, nil
}

func decodeEncryptRequest(body []byte) (*EncryptRequest, error) {
	m, err := decodeBody(body, "EncryptRequest")
	if err != nil {
		return nil, err
	}
	req := &EncryptRequest{}
	if req.ClientID, err = requireString(m, "clientId"); err != nil {
		return nil, err
	}
	if req.Plaintext, err = requireBytes(m, "plaintext"); err != nil {
		return nil, err
	}
	if req.EncryptionContext, err = optionalStringMap(m, "encryptionContext"); err != nil {
		return nil, err
	}
	if req.AlgorithmSuiteID, err = optionalEnum(m, "algorithmSuiteId", algorithmSuites); err != nil {
		return nil, err
	}
	if req.FrameLength, err = optionalInt64(m, "frameLength"); err != nil {
		return nil, err
	}
	return req, nil
}

func decodeDecryptRequest(body []byte) (*DecryptRequest, error) {
	m, err := decodeBody(body, "DecryptRequest")
	if err != nil {
		return nil, err
	}
	req := &DecryptRequest{}
	if req.ClientID, err = requireString(m, "clientId"); err != nil {
		return nil, err
	}
	if req.Ciphertext, err = requireBytes(m, "ciphertext"); err != nil {
		return nil, err
	}
	if req.EncryptionContext, err = optionalStringMap(m, "encryptionContext"); err != nil {
		return nil, err
	}
	return req, nil
}

// Configuration decoding.

func decodeClientConfig(v cbor.Value) (*ESDKClientConfig, error) {
	m, err := asMap(v, "config")
	if err != nil {
		return nil, err
	}
	config := &ESDKClientConfig{}
	if config.CommitmentPolicy, err = requireEnum(m, "commitmentPolicy", commitmentPolicies); err != nil {
		return nil, err
	}
	if config.MaxEncryptedDataKeys, err = optionalInt64(m, "maxEncryptedDataKeys"); err != nil {
		return nil, err
	}
	cmmV, err := requireMember(m, "cmm")
	if err != nil {
		return nil, err
	}
	if config.CMM, err = decodeCMM(cmmV); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeCMM(v cbor.Value) (*CMMConfig, error) {
	m, err := asMap(v, "cmm")
	if err != nil {
		return nil, err
	}
	cmm := &CMMConfig{}
	if dv, ok := member(m, "Default"); ok {
		dm, err := asMap(dv, "Default")
		if err != nil {
			return nil, err
		}
		kv, err := requireMember(dm, "keyring")
		if err != nil {
			return nil, err
		}
		keyring, err := decodeKeyring(kv)
		if err != nil {
			return nil, err
		}
		cmm.Default = &DefaultCMMConfig{Keyring: keyring}
	}
	if rv, ok := member(m, "RequiredEncryptionContext"); ok {
		rm, err := asMap(rv, "RequiredEncryptionContext")
		if err != nil {
			return nil, err
		}
		uv, err := requireMember(rm, "underlyingCMM")
		if err != nil {
			return nil, err
		}
		underlying, err := decodeCMM(uv)
		if err != nil {
			return nil, err
		}
		keys, err := requireStringList(rm, "requiredEncryptionContextKeys")
		if err != nil {
			return nil, err
		}
		cmm.RequiredEncryptionContext = &RequiredEncryptionContextCMMConfig{
			UnderlyingCMM:                 underlying,
			RequiredEncryptionContextKeys: keys,
		}
	}
	if cv, ok := member(m, "Caching"); ok {
		if _, err := asMap(cv, "Caching"); err != nil {
			return nil, err
		}
		cmm.Caching = &CachingCMMConfig{}
	}
	return cmm, nil
}

func decodeKeyring(v cbor.Value) (*KeyringConfig, error) {
	m, err := asMap(v, "keyring")
	if err != nil {
		return nil, err
	}
	keyring := &KeyringConfig{}
	if kv, ok := member(m, "AwsKms"); ok {
		if keyring.AwsKms, err = decodeAwsKmsKeyring(kv, "AwsKms"); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsMrk"); ok {
		if keyring.AwsKmsMrk, err = decodeAwsKmsKeyring(kv, "AwsKmsMrk"); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsMultiKeyring"); ok {
		if keyring.AwsKmsMultiKeyring, err = decodeAwsKmsMultiKeyring(kv, "AwsKmsMultiKeyring"); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsMrkMultiKeyring"); ok {
		if keyring.AwsKmsMrkMultiKeyring, err = decodeAwsKmsMultiKeyring(kv, "AwsKmsMrkMultiKeyring"); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsDiscovery"); ok {
		if keyring.AwsKmsDiscovery, err = decodeAwsKmsDiscoveryKeyring(kv); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsMrkDiscovery"); ok {
		if keyring.AwsKmsMrkDiscovery, err = decodeAwsKmsMrkDiscoveryKeyring(kv); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsRsa"); ok {
		if keyring.AwsKmsRsa, err = decodeAwsKmsRsaKeyring(kv); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "RawAes"); ok {
		if keyring.RawAes, err = decodeRawAesKeyring(kv); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "RawRsa"); ok {
		if keyring.RawRsa, err = decodeRawRsaKeyring(kv); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "AwsKmsHierarchical"); ok {
		if keyring.AwsKmsHierarchical, err = decodeAwsKmsHierarchicalKeyring(kv); err != nil {
			return nil, err
		}
	}
	if kv, ok := member(m, "Multi"); ok {
		if keyring.Multi, err = decodeMultiKeyring(kv); err != nil {
			return nil, err
		}
	}
	return keyring, nil
}

func decodeAwsKmsKeyring(v cbor.Value, what string) (*AwsKmsKeyringConfig, error) {
	m, err := asMap(v, what)
	if err != nil {
		return nil, err
	}
	config := &AwsKmsKeyringConfig{}
	if config.KmsKeyID, err = requireString(m, "kmsKeyId"); err != nil {
		return nil, err
	}
	if config.GrantTokens, err = optionalStringList(m, "grantTokens"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeAwsKmsMultiKeyring(v cbor.Value, what string) (*AwsKmsMultiKeyringConfig, error) {
	m, err := asMap(v, what)
	if err != nil {
		return nil, err
	}
	config := &AwsKmsMultiKeyringConfig{}
	if config.Generator, err = optionalString(m, "generator"); err != nil {
		return nil, err
	}
	if config.KmsKeyIDs, err = optionalStringList(m, "kmsKeyIds"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeAwsKmsDiscoveryKeyring(v cbor.Value) (*AwsKmsDiscoveryKeyringConfig, error) {
	m, err := asMap(v, "AwsKmsDiscovery")
	if err != nil {
		return nil, err
	}
	config := &AwsKmsDiscoveryKeyringConfig{}
	if config.DiscoveryFilter, err = decodeDiscoveryFilter(m); err != nil {
		return nil, err
	}
	if config.GrantTokens, err = optionalStringList(m, "grantTokens"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeAwsKmsMrkDiscoveryKeyring(v cbor.Value) (*AwsKmsMrkDiscoveryKeyringConfig, error) {
	m, err := asMap(v, "AwsKmsMrkDiscovery")
	if err != nil {
		return nil, err
	}
	config := &AwsKmsMrkDiscoveryKeyringConfig{}
	if config.Region, err = requireString(m, "region"); err != nil {
		return nil, err
	}
	if config.DiscoveryFilter, err = decodeDiscoveryFilter(m); err != nil {
		return nil, err
	}
	if config.GrantTokens, err = optionalStringList(m, "grantTokens"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeDiscoveryFilter(m cbor.Map) (*DiscoveryFilterConfig, error) {
	v, ok := member(m, "discoveryFilter")
	if !ok {
		return nil, nil
	}
	fm, err := asMap(v, "discoveryFilter")
	if err != nil {
		return nil, err
	}
	filter := &DiscoveryFilterConfig{}
	if filter.Partition, err = requireString(fm, "partition"); err != nil {
		return nil, err
	}
	if filter.AccountIDs, err = requireStringList(fm, "accountIds"); err != nil {
		return nil, err
	}
	return filter, nil
}

func decodeAwsKmsRsaKeyring(v cbor.Value) (*AwsKmsRsaKeyringConfig, error) {
	m, err := asMap(v, "AwsKmsRsa")
	if err != nil {
		return nil, err
	}
	config := &AwsKmsRsaKeyringConfig{}
	if config.KmsKeyID, err = requireString(m, "kmsKeyId"); err != nil {
		return nil, err
	}
	if config.PublicKey, err = optionalBytes(m, "publicKey"); err != nil {
		return nil, err
	}
	if config.EncryptionAlgorithm, err = optionalEnum(m, "encryptionAlgorithm", kmsRsaEncryptionAlgorithms); err != nil {
		return nil, err
	}
	if config.GrantTokens, err = optionalStringList(m, "grantTokens"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeRawAesKeyring(v cbor.Value) (*RawAesKeyringConfig, error) {
	m, err := asMap(v, "RawAes")
	if err != nil {
		return nil, err
	}
	config := &RawAesKeyringConfig{}
	if config.KeyNamespace, err = requireString(m, "keyNamespace"); err != nil {
		return nil, err
	}
	if config.KeyName, err = requireString(m, "keyName"); err != nil {
		return nil, err
	}
	if config.WrappingKey, err = requireBytes(m, "wrappingKey"); err != nil {
		return nil, err
	}
	if config.WrappingAlg, err = requireEnum(m, "wrappingAlg", aesWrappingAlgs); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeRawRsaKeyring(v cbor.Value) (*RawRsaKeyringConfig, error) {
	m, err := asMap(v, "RawRsa")
	if err != nil {
		return nil, err
	}
	config := &RawRsaKeyringConfig{}
	if config.KeyNamespace, err = requireString(m, "keyNamespace"); err != nil {
		return nil, err
	}
	if config.KeyName, err = requireString(m, "keyName"); err != nil {
		return nil, err
	}
	if config.PaddingScheme, err = requireEnum(m, "paddingScheme", paddingSchemes); err != nil {
		return nil, err
	}
	if config.PublicKey, err = optionalBytes(m, "publicKey"); err != nil {
		return nil, err
	}
	if config.PrivateKey, err = optionalBytes(m, "privateKey"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeAwsKmsHierarchicalKeyring(v cbor.Value) (*AwsKmsHierarchicalKeyringConfig, error) {
	m, err := asMap(v, "AwsKmsHierarchical")
	if err != nil {
		return nil, err
	}
	config := &AwsKmsHierarchicalKeyringConfig{}
	if config.BranchKeyID, err = requireString(m, "branchKeyId"); err != nil {
		return nil, err
	}
	if config.KeyStoreTableName, err = requireString(m, "keyStoreTableName"); err != nil {
		return nil, err
	}
	if config.LogicalKeyStoreName, err = requireString(m, "logicalKeyStoreName"); err != nil {
		return nil, err
	}
	if config.KmsKeyArn, err = requireString(m, "kmsKeyArn"); err != nil {
		return nil, err
	}
	if config.TTLSeconds, err = requireInt64(m, "ttlSeconds"); err != nil {
		return nil, err
	}
	return config, nil
}

func decodeMultiKeyring(v cbor.Value) (*MultiKeyringConfig, error) {
	m, err := asMap(v, "Multi")
	if err != nil {
		return nil, err
	}
	config := &MultiKeyringConfig{}
	if gv, ok := member(m, "generator"); ok {
		if config.Generator, err = decodeKeyring(gv); err != nil {
			return nil, err
		}
	}
	cv, err := requireMember(m, "childKeyrings")
	if err != nil {
		return nil, err
	}
	list, ok := cv.(cbor.List)
	if !ok {
		return nil, fmt.Errorf("member childKeyrings: expected a list, got %T", cv)
	}
	config.ChildKeyrings = make([]*KeyringConfig, 0, len(list))
	for _, entry := range list {
		child, err := decodeKeyring(entry)
		if err != nil {
			return nil, err
		}
		config.ChildKeyrings = append(config.ChildKeyrings, child)
	}
	return config, nil
}
