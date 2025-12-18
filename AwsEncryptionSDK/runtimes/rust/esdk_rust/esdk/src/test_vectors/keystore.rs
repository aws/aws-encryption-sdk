
use aws_mpl_legacy::deps::aws_cryptography_keyStore::types::*;
use aws_mpl_legacy::types::error::Error;

trait IKeyStoreClient {
    fn get_key_store_info(&self) -> Result<GetKeyStoreInfoOutput, Error>;
    fn create_key_store(&self, input: &CreateKeyStoreInput) -> Result<CreateKeyStoreOutput, Error>;
    fn create_key(&self, input: &CreateKeyInput) -> Result<CreateKeyOutput, Error>;
    fn version_key(&self, input: &VersionKeyInput) -> Result<VersionKeyOutput, Error>;
    fn get_active_branch_key(&self, input: &GetActiveBranchKeyInput) -> Result<GetActiveBranchKeyOutput, Error>;
    fn get_branch_key_version(&self, input: &GetBranchKeyVersionInput) -> Result<GetBranchKeyVersionOutput, Error>;
    fn get_beacon_key(&self, input: &GetBeaconKeyInput) -> Result<GetBeaconKeyOutput, Error>;
}
