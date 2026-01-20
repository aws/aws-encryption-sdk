use crate::MplPrivate;
use crate::error::*;
use crate::kms_keyring::Region;
use async_trait::async_trait;

// Basic structures

#[async_trait]
#[allow(private_bounds)]
pub trait ClientSupplier: Send + Sync + std::fmt::Debug + MplPrivate {
    async fn get_client(&self, input: &GetClientInput) -> Result<aws_sdk_kms::Client, Error>;
}

pub type ClientSupplierRef = std::sync::Arc<dyn ClientSupplier>;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Inputs for getting a AWS KMS Client.
pub struct GetClientInput {
    /// The region the client should be created in.
    pub region: Region,
}

pub fn create_default_client_supplier(
    _input: &CreateDefaultClientSupplierInput,
) -> Result<ClientSupplierRef, Error> {
    not_implemented("create_multi_keyring")
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct CreateDefaultClientSupplierInput {}
