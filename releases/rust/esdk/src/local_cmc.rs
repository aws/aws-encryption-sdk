// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

#[allow(non_snake_case)]
pub mod internal_SynchronizedLocalCMC {
    use crate::*;
    use std::sync::Mutex;

    pub struct SynchronizedLocalCMC {
        wrapped: Mutex<::dafny_runtime::Object<_LocalCMC_Compile::LocalCMC>>,
    }

    impl SynchronizedLocalCMC {
        pub fn _allocate_object(
            cmc: ::dafny_runtime::Object<_LocalCMC_Compile::LocalCMC>,
        ) -> ::dafny_runtime::Object<Self> {
            ::dafny_runtime::Object::new(SynchronizedLocalCMC {
                wrapped: Mutex::new(cmc),
            })
        }
    }

    impl ::dafny_runtime::UpcastObject<dafny_runtime::DynAny> for SynchronizedLocalCMC {
        ::dafny_runtime::UpcastObjectFn!(dafny_runtime::DynAny);
    }

    impl ::dafny_runtime::UpcastObject<dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache>
    for SynchronizedLocalCMC {
    ::dafny_runtime::UpcastObjectFn!(dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache);
  }

    impl software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache for SynchronizedLocalCMC {
    fn r#_PutCacheEntry_k(&self, input: &dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput>) -> dafny_runtime::Rc<crate::_Wrappers_Compile::Result<(), dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      self.wrapped.lock().unwrap().as_mut()._PutCacheEntry_k(input)
    }

    fn r#_UpdateUsageMetadata_k(&self, input: &dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::UpdateUsageMetadataInput>) -> dafny_runtime::Rc<crate::_Wrappers_Compile::Result<(), dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      self.wrapped.lock().unwrap().as_mut()._UpdateUsageMetadata_k(input)
    }

    fn r#_GetCacheEntry_k(&self, input: &dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput>) -> dafny_runtime::Rc<crate::_Wrappers_Compile::Result<dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput>, dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      self.wrapped.lock().unwrap().as_mut()._GetCacheEntry_k(input)
    }

    fn r#_DeleteCacheEntry_k(&self, input: &dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput>) -> dafny_runtime::Rc<crate::_Wrappers_Compile::Result<(), dafny_runtime::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      self.wrapped.lock().unwrap().as_mut()._DeleteCacheEntry_k(input)
    }
  }
}
