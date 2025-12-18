use crate::error::*;
use crate::types::*;
use async_trait::async_trait;

// Basic structures

#[async_trait]
pub trait ClientSupplier: Send + Sync + std::fmt::Debug {
    async fn get_client(&self, input: &GetClientInput) -> Result<aws_sdk_kms::Client, Error>;
}

pub type ClientSupplierReference = std::sync::Arc<dyn ClientSupplier>;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
/// Inputs for getting a AWS KMS Client.
pub struct GetClientInput {
  /// The region the client should be created in.
  pub region: Region,
}

#[derive(Debug, Clone, Default)]
struct DefaultClientSupplier {}
#[async_trait]
impl ClientSupplier for DefaultClientSupplier {
    async fn get_client(&self, _input: &GetClientInput) -> Result<aws_sdk_kms::Client, Error> {
        Err(mpl_err("foo"))
    }
}
pub fn create_default_client_supplier(_input: &CreateDefaultClientSupplierInput) -> Result<ClientSupplierReference, Error>
{
    Ok(std::sync::Arc::new(DefaultClientSupplier{}))

}

#[derive(Debug, Clone, Default, Copy)]
#[non_exhaustive]
pub struct CreateDefaultClientSupplierInput {}
