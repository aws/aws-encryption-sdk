#![allow(unused_variables)]

use crate::test_vectors::types::*;
use aws_mpl_legacy::implementation_from_dafny::_AwsCryptographyKeyStoreOperations_Compile::*;
use std::sync::Arc;
use aws_mpl_legacy::dafny::aes_gcm::AESEncryption::DynAny;
use aws_mpl_legacy::dafny::aes_gcm::AESEncryption::UpcastObject;
use aws_mpl_legacy::implementation_from_dafny::_AwsCryptographyKeyStoreOperations_Compile::Error;
use aws_mpl_legacy::implementation_from_dafny::AESEncryption::Result;

struct StaticKeyStore {
    key: Key,
}
impl UpcastObject<DynAny> for StaticKeyStore {
    aws_mpl_legacy::implementation_from_dafny::_AwsKmsDiscoveryKeyring_Compile::UpcastObjectFn!(DynAny);
}

impl aws_mpl_legacy::implementation_from_dafny::software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient for StaticKeyStore {
 fn GetKeyStoreInfo(
        &self,
    ) -> Arc<Result<Arc<GetKeyStoreInfoOutput>, Arc<Error>>>
    {todo!()}
    fn CreateKeyStore(
        &self,
        input: &Arc<CreateKeyStoreInput>,
    ) -> Arc<Result<Arc<CreateKeyStoreOutput>, Arc<Error>>>{todo!()}
    fn CreateKey(
        &self,
        input: &Arc<CreateKeyInput>,
    ) -> Arc<Result<Arc<CreateKeyOutput>, Arc<Error>>>{todo!()}
    fn VersionKey(
        &self,
        input: &Arc<VersionKeyInput>,
    ) -> Arc<Result<Arc<VersionKeyOutput>, Arc<Error>>>{todo!()}
    fn GetActiveBranchKey(
        &self,
        input: &Arc<GetActiveBranchKeyInput>,
    ) -> Arc<Result<Arc<GetActiveBranchKeyOutput>, Arc<Error>>>{todo!()}
    fn GetBranchKeyVersion(
        &self,
        input: &Arc<GetBranchKeyVersionInput>,
    ) -> Arc<Result<Arc<GetBranchKeyVersionOutput>, Arc<Error>>>{todo!()}
    fn GetBeaconKey(
        &self,
        input: &Arc<GetBeaconKeyInput>,
    ) -> Arc<Result<Arc<GetBeaconKeyOutput>, Arc<Error>>>{todo!()}

}
