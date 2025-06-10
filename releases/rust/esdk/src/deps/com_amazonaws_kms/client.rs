// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
use crate::deps::com_amazonaws_kms::conversions;
use std::sync::LazyLock;

#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    pub inner: aws_sdk_kms::Client,
}

impl ::std::cmp::PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl ::std::convert::Into<Client> for aws_sdk_kms::Client {
    fn into(self) -> Client {
        Client { inner: self }
    }
}

/// A runtime for executing operations on the asynchronous client in a blocking manner.
/// Necessary because Dafny only generates synchronous code.
static dafny_tokio_runtime: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

impl dafny_runtime::UpcastObject<::dafny_runtime::DynAny> for Client {
    ::dafny_runtime::UpcastObjectFn!(::dafny_runtime::DynAny);
}

impl dafny_runtime::UpcastObject<dyn crate::r#software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient> for Client {
  ::dafny_runtime::UpcastObjectFn!(dyn crate::r#software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient);
}

impl crate::r#software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient
    for Client
{
    fn CancelKeyDeletion(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CancelKeyDeletionRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CancelKeyDeletionResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::cancel_key_deletion::_cancel_key_deletion_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .cancel_key_deletion()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::cancel_key_deletion::_cancel_key_deletion_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::cancel_key_deletion::to_dafny_error)
    }
 fn ConnectCustomKeyStore(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectCustomKeyStoreRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ConnectCustomKeyStoreResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::connect_custom_key_store::_connect_custom_key_store_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .connect_custom_key_store()
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::connect_custom_key_store::_connect_custom_key_store_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::connect_custom_key_store::to_dafny_error)
    }
    fn CreateAlias(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateAliasRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::create_alias::_create_alias_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .create_alias()
                    .set_alias_name(inner_input.r#alias_name)
                    .set_target_key_id(inner_input.r#target_key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::create_alias::to_dafny_error,
        )
    }
 fn CreateCustomKeyStore(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateCustomKeyStoreResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::create_custom_key_store::_create_custom_key_store_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .create_custom_key_store()
                    .set_custom_key_store_name(inner_input.r#custom_key_store_name)
                    .set_cloud_hsm_cluster_id(inner_input.r#cloud_hsm_cluster_id)
                    .set_trust_anchor_certificate(inner_input.r#trust_anchor_certificate)
                    .set_key_store_password(inner_input.r#key_store_password)
                    .set_custom_key_store_type(inner_input.r#custom_key_store_type)
                    .set_xks_proxy_uri_endpoint(inner_input.r#xks_proxy_uri_endpoint)
                    .set_xks_proxy_uri_path(inner_input.r#xks_proxy_uri_path)
                    .set_xks_proxy_vpc_endpoint_service_name(
                        inner_input.r#xks_proxy_vpc_endpoint_service_name,
                    )
                    .set_xks_proxy_authentication_credential(
                        inner_input.r#xks_proxy_authentication_credential,
                    )
                    .set_xks_proxy_connectivity(inner_input.r#xks_proxy_connectivity)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::create_custom_key_store::_create_custom_key_store_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::create_custom_key_store::to_dafny_error)
    }
 fn CreateGrant(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateGrantResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::create_grant::_create_grant_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .create_grant()
                    .set_key_id(inner_input.r#key_id)
                    .set_grantee_principal(inner_input.r#grantee_principal)
                    .set_retiring_principal(inner_input.r#retiring_principal)
                    .set_operations(inner_input.r#operations)
                    .set_constraints(inner_input.r#constraints)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_name(inner_input.r#name)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::create_grant::_create_grant_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::create_grant::to_dafny_error)
    }
 fn CreateKey(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateKeyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::CreateKeyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::create_key::_create_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .create_key()
                    .set_policy(inner_input.r#policy)
                    .set_description(inner_input.r#description)
                    .set_key_usage(inner_input.r#key_usage)
                    .set_customer_master_key_spec(inner_input.r#customer_master_key_spec)
                    .set_key_spec(inner_input.r#key_spec)
                    .set_origin(inner_input.r#origin)
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .set_bypass_policy_lockout_safety_check(
                        inner_input.r#bypass_policy_lockout_safety_check,
                    )
                    .set_tags(inner_input.r#tags)
                    .set_multi_region(inner_input.r#multi_region)
                    .set_xks_key_id(inner_input.r#xks_key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::create_key::_create_key_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::create_key::to_dafny_error,
        )
    }
 fn Decrypt(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DecryptResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input =
            crate::deps::com_amazonaws_kms::conversions::decrypt::_decrypt_request::from_dafny(
                input.clone(),
            );
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .decrypt()
                    .set_ciphertext_blob(inner_input.r#ciphertext_blob)
                    .set_encryption_context(inner_input.r#encryption_context)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_key_id(inner_input.r#key_id)
                    .set_encryption_algorithm(inner_input.r#encryption_algorithm)
                    .set_recipient(inner_input.r#recipient)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::decrypt::_decrypt_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::decrypt::to_dafny_error,
        )
    }
    fn DeleteAlias(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeleteAliasRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::delete_alias::_delete_alias_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .delete_alias()
                    .set_alias_name(inner_input.r#alias_name)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::delete_alias::to_dafny_error,
        )
    }
 fn DeleteCustomKeyStore(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeleteCustomKeyStoreRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeleteCustomKeyStoreResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::delete_custom_key_store::_delete_custom_key_store_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .delete_custom_key_store()
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::delete_custom_key_store::_delete_custom_key_store_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::delete_custom_key_store::to_dafny_error)
    }
    fn DeleteImportedKeyMaterial(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeleteImportedKeyMaterialRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::delete_imported_key_material::_delete_imported_key_material_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .delete_imported_key_material()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    |x| (),
    crate::deps::com_amazonaws_kms::conversions::delete_imported_key_material::to_dafny_error)
    }
 fn DeriveSharedSecret(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DeriveSharedSecretResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::derive_shared_secret::_derive_shared_secret_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .derive_shared_secret()
                    .set_key_id(inner_input.r#key_id)
                    .set_key_agreement_algorithm(inner_input.r#key_agreement_algorithm)
                    .set_public_key(inner_input.r#public_key)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .set_recipient(inner_input.r#recipient)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::derive_shared_secret::_derive_shared_secret_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::derive_shared_secret::to_dafny_error)
    }
 fn DescribeCustomKeyStores(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeCustomKeyStoresResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::describe_custom_key_stores::_describe_custom_key_stores_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .describe_custom_key_stores()
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .set_custom_key_store_name(inner_input.r#custom_key_store_name)
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::describe_custom_key_stores::_describe_custom_key_stores_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::describe_custom_key_stores::to_dafny_error)
    }
 fn DescribeKey(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeKeyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DescribeKeyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::describe_key::_describe_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .describe_key()
                    .set_key_id(inner_input.r#key_id)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::describe_key::_describe_key_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::describe_key::to_dafny_error)
    }
    fn DisableKey(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisableKeyRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::disable_key::_disable_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .disable_key()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::disable_key::to_dafny_error,
        )
    }
    fn DisableKeyRotation(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisableKeyRotationRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::disable_key_rotation::_disable_key_rotation_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .disable_key_rotation()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::disable_key_rotation::to_dafny_error,
        )
    }
 fn DisconnectCustomKeyStore(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisconnectCustomKeyStoreRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::DisconnectCustomKeyStoreResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::disconnect_custom_key_store::_disconnect_custom_key_store_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .disconnect_custom_key_store()
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::disconnect_custom_key_store::_disconnect_custom_key_store_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::disconnect_custom_key_store::to_dafny_error)
    }
    fn EnableKey(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EnableKeyRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::enable_key::_enable_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .enable_key()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::enable_key::to_dafny_error,
        )
    }
    fn EnableKeyRotation(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EnableKeyRotationRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::enable_key_rotation::_enable_key_rotation_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .enable_key_rotation()
                    .set_key_id(inner_input.r#key_id)
                    .set_rotation_period_in_days(inner_input.r#rotation_period_in_days)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::enable_key_rotation::to_dafny_error,
        )
    }
 fn Encrypt(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EncryptRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::EncryptResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input =
            crate::deps::com_amazonaws_kms::conversions::encrypt::_encrypt_request::from_dafny(
                input.clone(),
            );
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .encrypt()
                    .set_key_id(inner_input.r#key_id)
                    .set_plaintext(inner_input.r#plaintext)
                    .set_encryption_context(inner_input.r#encryption_context)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_encryption_algorithm(inner_input.r#encryption_algorithm)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::encrypt::_encrypt_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::encrypt::to_dafny_error,
        )
    }
 fn GenerateDataKey(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::generate_data_key::_generate_data_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .generate_data_key()
                    .set_key_id(inner_input.r#key_id)
                    .set_encryption_context(inner_input.r#encryption_context)
                    .set_number_of_bytes(inner_input.r#number_of_bytes)
                    .set_key_spec(inner_input.r#key_spec)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_recipient(inner_input.r#recipient)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key::_generate_data_key_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key::to_dafny_error)
    }
 fn GenerateDataKeyPair(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::generate_data_key_pair::_generate_data_key_pair_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .generate_data_key_pair()
                    .set_encryption_context(inner_input.r#encryption_context)
                    .set_key_id(inner_input.r#key_id)
                    .set_key_pair_spec(inner_input.r#key_pair_spec)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_recipient(inner_input.r#recipient)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key_pair::_generate_data_key_pair_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key_pair::to_dafny_error)
    }
 fn GenerateDataKeyPairWithoutPlaintext(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairWithoutPlaintextRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyPairWithoutPlaintextResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::generate_data_key_pair_without_plaintext::_generate_data_key_pair_without_plaintext_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .generate_data_key_pair_without_plaintext()
                    .set_encryption_context(inner_input.r#encryption_context)
                    .set_key_id(inner_input.r#key_id)
                    .set_key_pair_spec(inner_input.r#key_pair_spec)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key_pair_without_plaintext::_generate_data_key_pair_without_plaintext_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key_pair_without_plaintext::to_dafny_error)
    }
 fn GenerateDataKeyWithoutPlaintext(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyWithoutPlaintextRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateDataKeyWithoutPlaintextResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::generate_data_key_without_plaintext::_generate_data_key_without_plaintext_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .generate_data_key_without_plaintext()
                    .set_key_id(inner_input.r#key_id)
                    .set_encryption_context(inner_input.r#encryption_context)
                    .set_key_spec(inner_input.r#key_spec)
                    .set_number_of_bytes(inner_input.r#number_of_bytes)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key_without_plaintext::_generate_data_key_without_plaintext_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::generate_data_key_without_plaintext::to_dafny_error)
    }
 fn GenerateMac(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateMacRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateMacResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::generate_mac::_generate_mac_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .generate_mac()
                    .set_message(inner_input.r#message)
                    .set_key_id(inner_input.r#key_id)
                    .set_mac_algorithm(inner_input.r#mac_algorithm)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::generate_mac::_generate_mac_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::generate_mac::to_dafny_error)
    }
 fn GenerateRandom(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateRandomResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::generate_random::_generate_random_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .generate_random()
                    .set_number_of_bytes(inner_input.r#number_of_bytes)
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .set_recipient(inner_input.r#recipient)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::generate_random::_generate_random_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::generate_random::to_dafny_error)
    }
 fn GetKeyPolicy(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyPolicyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::get_key_policy::_get_key_policy_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .get_key_policy()
                    .set_key_id(inner_input.r#key_id)
                    .set_policy_name(inner_input.r#policy_name)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::get_key_policy::_get_key_policy_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::get_key_policy::to_dafny_error)
    }
 fn GetKeyRotationStatus(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyRotationStatusRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetKeyRotationStatusResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::get_key_rotation_status::_get_key_rotation_status_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .get_key_rotation_status()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::get_key_rotation_status::_get_key_rotation_status_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::get_key_rotation_status::to_dafny_error)
    }
 fn GetParametersForImport(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetParametersForImportResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::get_parameters_for_import::_get_parameters_for_import_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .get_parameters_for_import()
                    .set_key_id(inner_input.r#key_id)
                    .set_wrapping_algorithm(inner_input.r#wrapping_algorithm)
                    .set_wrapping_key_spec(inner_input.r#wrapping_key_spec)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::get_parameters_for_import::_get_parameters_for_import_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::get_parameters_for_import::to_dafny_error)
    }
 fn GetPublicKey(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GetPublicKeyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::get_public_key::_get_public_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .get_public_key()
                    .set_key_id(inner_input.r#key_id)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::get_public_key::_get_public_key_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::get_public_key::to_dafny_error)
    }
 fn ImportKeyMaterial(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ImportKeyMaterialRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ImportKeyMaterialResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::import_key_material::_import_key_material_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .import_key_material()
                    .set_key_id(inner_input.r#key_id)
                    .set_import_token(inner_input.r#import_token)
                    .set_encrypted_key_material(inner_input.r#encrypted_key_material)
                    .set_valid_to(inner_input.r#valid_to)
                    .set_expiration_model(inner_input.r#expiration_model)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::import_key_material::_import_key_material_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::import_key_material::to_dafny_error)
    }
 fn ListAliases(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListAliasesRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListAliasesResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::list_aliases::_list_aliases_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .list_aliases()
                    .set_key_id(inner_input.r#key_id)
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::list_aliases::_list_aliases_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::list_aliases::to_dafny_error)
    }
 fn ListGrants(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListGrantsRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListGrantsResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::list_grants::_list_grants_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .list_grants()
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .set_key_id(inner_input.r#key_id)
                    .set_grant_id(inner_input.r#grant_id)
                    .set_grantee_principal(inner_input.r#grantee_principal)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::list_grants::_list_grants_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::list_grants::to_dafny_error)
    }
 fn ListKeyPolicies(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeyPoliciesRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeyPoliciesResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::list_key_policies::_list_key_policies_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .list_key_policies()
                    .set_key_id(inner_input.r#key_id)
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::list_key_policies::_list_key_policies_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::list_key_policies::to_dafny_error)
    }
 fn ListKeyRotations(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeyRotationsRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeyRotationsResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::list_key_rotations::_list_key_rotations_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .list_key_rotations()
                    .set_key_id(inner_input.r#key_id)
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::list_key_rotations::_list_key_rotations_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::list_key_rotations::to_dafny_error)
    }
 fn ListKeys(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeysRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListKeysResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input =
            crate::deps::com_amazonaws_kms::conversions::list_keys::_list_keys_request::from_dafny(
                input.clone(),
            );
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .list_keys()
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::list_keys::_list_keys_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::list_keys::to_dafny_error,
        )
    }
 fn ListResourceTags(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListResourceTagsRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ListResourceTagsResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::list_resource_tags::_list_resource_tags_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .list_resource_tags()
                    .set_key_id(inner_input.r#key_id)
                    .set_limit(inner_input.r#limit)
                    .set_marker(inner_input.r#marker)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::list_resource_tags::_list_resource_tags_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::list_resource_tags::to_dafny_error)
    }
    fn PutKeyPolicy(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::PutKeyPolicyRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::put_key_policy::_put_key_policy_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .put_key_policy()
                    .set_key_id(inner_input.r#key_id)
                    .set_policy_name(inner_input.r#policy_name)
                    .set_policy(inner_input.r#policy)
                    .set_bypass_policy_lockout_safety_check(
                        inner_input.r#bypass_policy_lockout_safety_check,
                    )
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::put_key_policy::to_dafny_error,
        )
    }
 fn ReEncrypt(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReEncryptResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::re_encrypt::_re_encrypt_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .re_encrypt()
                    .set_ciphertext_blob(inner_input.r#ciphertext_blob)
                    .set_source_encryption_context(inner_input.r#source_encryption_context)
                    .set_source_key_id(inner_input.r#source_key_id)
                    .set_destination_key_id(inner_input.r#destination_key_id)
                    .set_destination_encryption_context(
                        inner_input.r#destination_encryption_context,
                    )
                    .set_source_encryption_algorithm(inner_input.r#source_encryption_algorithm)
                    .set_destination_encryption_algorithm(
                        inner_input.r#destination_encryption_algorithm,
                    )
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::re_encrypt::_re_encrypt_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::re_encrypt::to_dafny_error,
        )
    }
 fn ReplicateKey(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReplicateKeyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ReplicateKeyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::replicate_key::_replicate_key_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .replicate_key()
                    .set_key_id(inner_input.r#key_id)
                    .set_replica_region(inner_input.r#replica_region)
                    .set_policy(inner_input.r#policy)
                    .set_bypass_policy_lockout_safety_check(
                        inner_input.r#bypass_policy_lockout_safety_check,
                    )
                    .set_description(inner_input.r#description)
                    .set_tags(inner_input.r#tags)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::replicate_key::_replicate_key_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::replicate_key::to_dafny_error)
    }
    fn RetireGrant(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RetireGrantRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::retire_grant::_retire_grant_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .retire_grant()
                    .set_grant_token(inner_input.r#grant_token)
                    .set_key_id(inner_input.r#key_id)
                    .set_grant_id(inner_input.r#grant_id)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::retire_grant::to_dafny_error,
        )
    }
    fn RevokeGrant(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RevokeGrantRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::revoke_grant::_revoke_grant_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .revoke_grant()
                    .set_key_id(inner_input.r#key_id)
                    .set_grant_id(inner_input.r#grant_id)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::revoke_grant::to_dafny_error,
        )
    }
 fn RotateKeyOnDemand(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotateKeyOnDemandRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::RotateKeyOnDemandResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::rotate_key_on_demand::_rotate_key_on_demand_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .rotate_key_on_demand()
                    .set_key_id(inner_input.r#key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::rotate_key_on_demand::_rotate_key_on_demand_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::rotate_key_on_demand::to_dafny_error)
    }
 fn ScheduleKeyDeletion(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ScheduleKeyDeletionRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::ScheduleKeyDeletionResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::schedule_key_deletion::_schedule_key_deletion_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .schedule_key_deletion()
                    .set_key_id(inner_input.r#key_id)
                    .set_pending_window_in_days(inner_input.r#pending_window_in_days)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::schedule_key_deletion::_schedule_key_deletion_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::schedule_key_deletion::to_dafny_error)
    }
 fn Sign(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::SignRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::SignResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input =
            crate::deps::com_amazonaws_kms::conversions::sign::_sign_request::from_dafny(
                input.clone(),
            );
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .sign()
                    .set_key_id(inner_input.r#key_id)
                    .set_message(inner_input.r#message)
                    .set_message_type(inner_input.r#message_type)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_signing_algorithm(inner_input.r#signing_algorithm)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::sign::_sign_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::sign::to_dafny_error,
        )
    }
    fn TagResource(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::TagResourceRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::tag_resource::_tag_resource_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .tag_resource()
                    .set_key_id(inner_input.r#key_id)
                    .set_tags(inner_input.r#tags)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::tag_resource::to_dafny_error,
        )
    }
    fn UntagResource(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UntagResourceRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::untag_resource::_untag_resource_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .untag_resource()
                    .set_key_id(inner_input.r#key_id)
                    .set_tag_keys(inner_input.r#tag_keys)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::untag_resource::to_dafny_error,
        )
    }
    fn UpdateAlias(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateAliasRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::update_alias::_update_alias_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .update_alias()
                    .set_alias_name(inner_input.r#alias_name)
                    .set_target_key_id(inner_input.r#target_key_id)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::update_alias::to_dafny_error,
        )
    }
 fn UpdateCustomKeyStore(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateCustomKeyStoreResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::update_custom_key_store::_update_custom_key_store_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .update_custom_key_store()
                    .set_custom_key_store_id(inner_input.r#custom_key_store_id)
                    .set_new_custom_key_store_name(inner_input.r#new_custom_key_store_name)
                    .set_key_store_password(inner_input.r#key_store_password)
                    .set_cloud_hsm_cluster_id(inner_input.r#cloud_hsm_cluster_id)
                    .set_xks_proxy_uri_endpoint(inner_input.r#xks_proxy_uri_endpoint)
                    .set_xks_proxy_uri_path(inner_input.r#xks_proxy_uri_path)
                    .set_xks_proxy_vpc_endpoint_service_name(
                        inner_input.r#xks_proxy_vpc_endpoint_service_name,
                    )
                    .set_xks_proxy_authentication_credential(
                        inner_input.r#xks_proxy_authentication_credential,
                    )
                    .set_xks_proxy_connectivity(inner_input.r#xks_proxy_connectivity)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(&native_result,
    crate::deps::com_amazonaws_kms::conversions::update_custom_key_store::_update_custom_key_store_response::to_dafny,
    crate::deps::com_amazonaws_kms::conversions::update_custom_key_store::to_dafny_error)
    }
    fn UpdateKeyDescription(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdateKeyDescriptionRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::update_key_description::_update_key_description_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .update_key_description()
                    .set_key_id(inner_input.r#key_id)
                    .set_description(inner_input.r#description)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::update_key_description::to_dafny_error,
        )
    }
    fn UpdatePrimaryRegion(
        &self,
        input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::UpdatePrimaryRegionRequest>,
    ) -> dafny_runtime::Rc<
        crate::r#_Wrappers_Compile::Result<
            (),
            dafny_runtime::Rc<
                crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error,
            >,
        >,
    > {
        let inner_input = crate::deps::com_amazonaws_kms::conversions::update_primary_region::_update_primary_region_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .update_primary_region()
                    .set_key_id(inner_input.r#key_id)
                    .set_primary_region(inner_input.r#primary_region)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            |x| (),
            crate::deps::com_amazonaws_kms::conversions::update_primary_region::to_dafny_error,
        )
    }
 fn Verify(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input =
            crate::deps::com_amazonaws_kms::conversions::verify::_verify_request::from_dafny(
                input.clone(),
            );
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .verify()
                    .set_key_id(inner_input.r#key_id)
                    .set_message(inner_input.r#message)
                    .set_message_type(inner_input.r#message_type)
                    .set_signature(inner_input.r#signature)
                    .set_signing_algorithm(inner_input.r#signing_algorithm)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::verify::_verify_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::verify::to_dafny_error,
        )
    }
 fn VerifyMac(&self, input: &dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyMacRequest>)
  -> dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::VerifyMacResponse>,
    dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::kms::internaldafny::types::Error>
  >
    >{
        let inner_input = crate::deps::com_amazonaws_kms::conversions::verify_mac::_verify_mac_request::from_dafny(input.clone());
        let native_result = tokio::task::block_in_place(|| {
            dafny_tokio_runtime.block_on(async {
                self.inner
                    .verify_mac()
                    .set_message(inner_input.r#message)
                    .set_key_id(inner_input.r#key_id)
                    .set_mac_algorithm(inner_input.r#mac_algorithm)
                    .set_mac(inner_input.r#mac)
                    .set_grant_tokens(inner_input.r#grant_tokens)
                    .set_dry_run(inner_input.r#dry_run)
                    .send()
                    .await
            })
        });
        crate::standard_library_conversions::result_to_dafny(
            &native_result,
            crate::deps::com_amazonaws_kms::conversions::verify_mac::_verify_mac_response::to_dafny,
            crate::deps::com_amazonaws_kms::conversions::verify_mac::to_dafny_error,
        )
    }
}
