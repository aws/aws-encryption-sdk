// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

#[allow(missing_docs)]
pub trait ClientSupplier: Send + Sync {
    fn get_client(
        &self,
        input: crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientInput,
    ) -> Result<
        crate::deps::com_amazonaws_kms::client::Client,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >;
}

#[derive(::std::clone::Clone)]
/// A reference to a ClientSupplier
pub struct ClientSupplierRef {
    pub inner: ::dafny_runtime::Rc<::dafny_runtime::RefCell<dyn ClientSupplier>>,
}

impl<T: ClientSupplier + 'static> From<T> for ClientSupplierRef {
    fn from(value: T) -> Self {
        Self {
            inner: dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(value)),
        }
    }
}

impl ::std::cmp::PartialEq for ClientSupplierRef {
    fn eq(&self, other: &ClientSupplierRef) -> bool {
        ::dafny_runtime::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for ClientSupplierRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<ClientSupplierRef>")
    }
}

mod get_client;
