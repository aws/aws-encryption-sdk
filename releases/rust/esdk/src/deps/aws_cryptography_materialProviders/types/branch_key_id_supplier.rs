// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

#[allow(missing_docs)]
pub trait BranchKeyIdSupplier: Send + Sync {
    fn get_branch_key_id(
    &self,
    input: crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdInput,
  ) -> Result<
    crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput,
    crate::deps::aws_cryptography_materialProviders::types::error::Error,
  >;
}

#[derive(::std::clone::Clone)]
/// A reference to a BranchKeyIdSupplier
pub struct BranchKeyIdSupplierRef {
    pub inner: ::dafny_runtime::Rc<::dafny_runtime::RefCell<dyn BranchKeyIdSupplier>>,
}

impl<T: BranchKeyIdSupplier + 'static> From<T> for BranchKeyIdSupplierRef {
    fn from(value: T) -> Self {
        Self {
            inner: dafny_runtime::Rc::new(::dafny_runtime::RefCell::new(value)),
        }
    }
}

impl ::std::cmp::PartialEq for BranchKeyIdSupplierRef {
    fn eq(&self, other: &BranchKeyIdSupplierRef) -> bool {
        ::dafny_runtime::Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl ::std::fmt::Debug for BranchKeyIdSupplierRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<BranchKeyIdSupplierRef>")
    }
}

mod get_branch_key_id;
