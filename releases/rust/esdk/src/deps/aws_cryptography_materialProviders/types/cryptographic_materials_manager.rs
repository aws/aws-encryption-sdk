// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

#[allow(missing_docs)]
pub trait CryptographicMaterialsManager: Send + Sync {
    fn get_encryption_materials(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsInput,
  ) -> Result<
    crate::deps::aws_cryptography_materialProviders::operation::get_encryption_materials::GetEncryptionMaterialsOutput,
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;

    fn decrypt_materials(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsInput,
  ) -> Result<
    crate::deps::aws_cryptography_materialProviders::operation::decrypt_materials::DecryptMaterialsOutput,
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
/// A reference to a CryptographicMaterialsManager
pub struct CryptographicMaterialsManagerRef {
    pub inner: ::dafny_runtime::Rc<::dafny_runtime::RefCell<dyn CryptographicMaterialsManager>>,
}

impl<T: CryptographicMaterialsManager + 'static> From<T> for CryptographicMaterialsManagerRef {
    fn from(value: T) -> Self {
        Self {
            inner: dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(value)),
        }
    }
}

impl ::std::cmp::PartialEq for CryptographicMaterialsManagerRef {
    fn eq(&self, other: &CryptographicMaterialsManagerRef) -> bool {
        ::dafny_runtime::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for CryptographicMaterialsManagerRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<CryptographicMaterialsManagerRef>")
    }
}

mod get_encryption_materials;

mod decrypt_materials;
