package restrictalgorithmsuite

import (
	"context"
	"fmt"

	mpl "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygenerated"
	mpltypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/awscryptographymaterialproviderssmithygeneratedtypes"
)

type SigningSuiteOnlyCMM struct {
	approvedAlgos map[mpltypes.ESDKAlgorithmSuiteId]bool
	cmm           mpltypes.ICryptographicMaterialsManager
}

// NewSigningSuiteOnlyCMM creates a new SigningSuiteOnlyCMM
func NewSigningSuiteOnlyCMM(keyring mpltypes.IKeyring) (*SigningSuiteOnlyCMM, error) {
	// Initialize the MPL client
	materialProviders, err := mpl.NewClient(mpltypes.MaterialProvidersConfig{})
	if err != nil {
		panic(err)
	}
	// Create a DefaultCryptographicMaterialsManager
	cmmInput := mpltypes.CreateDefaultCryptographicMaterialsManagerInput{
		Keyring: keyring,
	}
	cmm, err := materialProviders.CreateDefaultCryptographicMaterialsManager(context.Background(), cmmInput)
	if err != nil {
		return nil, err
	}
	// Create list of approved algorithm
	var approvedAlgos = map[mpltypes.ESDKAlgorithmSuiteId]bool{
		mpltypes.ESDKAlgorithmSuiteIdAlgAes128GcmIv12Tag16HkdfSha256EcdsaP256: true,
		mpltypes.ESDKAlgorithmSuiteIdAlgAes192GcmIv12Tag16HkdfSha384EcdsaP384: true,
		mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmIv12Tag16HkdfSha384EcdsaP384: true,
		mpltypes.ESDKAlgorithmSuiteIdAlgAes256GcmHkdfSha512CommitKeyEcdsaP384: true,
	}
	return &SigningSuiteOnlyCMM{
		approvedAlgos: approvedAlgos,
		cmm:           cmm,
	}, nil
}

func (signingSuiteOnlyCMM *SigningSuiteOnlyCMM) GetEncryptionMaterials(input mpltypes.GetEncryptionMaterialsInput) (*mpltypes.GetEncryptionMaterialsOutput, error) {
	// Get the algorithm suite from the input
	esdkAlgorithmSuite, err := getESDKAlgorithmSuite(input.AlgorithmSuiteId)
	if err != nil {
		return nil, err
	}
	// Check if the algorithm is approved
	if !signingSuiteOnlyCMM.approvedAlgos[esdkAlgorithmSuite.Value] {
		return nil, mpltypes.AwsCryptographicMaterialProvidersException{Message: "Algorithm Suite must use Signing"}
	}
	// Delegate to the underlying CMM
	return signingSuiteOnlyCMM.cmm.GetEncryptionMaterials(input)
}

func getESDKAlgorithmSuite(algSuite mpltypes.AlgorithmSuiteId) (*mpltypes.AlgorithmSuiteIdMemberESDK, error) {
	if esdk, ok := algSuite.(*mpltypes.AlgorithmSuiteIdMemberESDK); ok {
		return esdk, nil
	}
	return nil, fmt.Errorf("algorithm suite is not ESDK type")
}

func (signingSuiteOnlyCMM *SigningSuiteOnlyCMM) DecryptMaterials(input mpltypes.DecryptMaterialsInput) (*mpltypes.DecryptMaterialsOutput, error) {
	// Get the algorithm suite from the input
	esdkAlgorithmSuite, err := getESDKAlgorithmSuite(input.AlgorithmSuiteId)
	if err != nil {
		return nil, err
	}
	// Check if the algorithm is approved
	if !signingSuiteOnlyCMM.approvedAlgos[esdkAlgorithmSuite.Value] {
		return nil, mpltypes.AwsCryptographicMaterialProvidersException{Message: "Algorithm Suite must use Signing"}
	}
	// Delegate to the underlying CMM
	return signingSuiteOnlyCMM.cmm.DecryptMaterials(input)
}
