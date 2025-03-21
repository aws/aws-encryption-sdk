// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

#[allow(missing_docs)]
pub trait Keyring: Send + Sync {
    fn on_encrypt(
        &self,
        input: crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptInput,
    ) -> Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_encrypt::OnEncryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >;

    fn on_decrypt(
        &self,
        input: crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptInput,
    ) -> Result<
        crate::deps::aws_cryptography_materialProviders::operation::on_decrypt::OnDecryptOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >;
}

#[derive(::std::clone::Clone)]
/// A reference to a Keyring
pub struct KeyringRef {
    pub inner: ::dafny_runtime::Rc<::dafny_runtime::RefCell<dyn Keyring>>,
}

impl<T: Keyring + 'static> From<T> for KeyringRef {
    fn from(value: T) -> Self {
        Self {
            inner: dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(value)),
        }
    }
}

impl ::std::cmp::PartialEq for KeyringRef {
    fn eq(&self, other: &KeyringRef) -> bool {
        ::dafny_runtime::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for KeyringRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<KeyringRef>")
    }
}

mod on_encrypt;

mod on_decrypt;
