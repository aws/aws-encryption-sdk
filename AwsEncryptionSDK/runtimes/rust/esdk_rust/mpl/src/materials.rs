use crate::error::*;
pub use crate::types::{EncryptionMaterials, DecryptionMaterials};
pub fn valid_encryption_materials_transition(_input: ValidEncryptionMaterialsTransitionInput) -> Result<(), Error>
{
//   errors: [InvalidEncryptionMaterialsTransition]
    Ok(())
}

pub fn valid_decryption_materials_transition(_input: ValidDecryptionMaterialsTransitionInput) -> Result<(), Error>
{
//   errors: [InvalidDecryptionMaterialsTransition]
    Ok(())
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ValidEncryptionMaterialsTransitionInput {
  pub start: EncryptionMaterials,
  pub stop: EncryptionMaterials,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct ValidDecryptionMaterialsTransitionInput {
  pub start: DecryptionMaterials,
  pub stop: DecryptionMaterials,
}

pub const fn encryption_materials_has_plaintext_data_key(_input : &EncryptionMaterials) -> Result<(), Error> {
    Ok(())
    //   errors: [InvalidEncryptionMaterials],
}

pub const fn decryption_materials_with_plaintext_data_key(_input : &DecryptionMaterials) -> Result<(), Error> {
    Ok(())
    //   errors: [InvalidDecryptionMaterials],
}
