// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdInput;
use aws_esdk::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput;
use aws_esdk::aws_cryptography_materialProviders::types::error::Error;
use aws_esdk::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplier;
use std::collections::HashMap;

/*
 Demonstrates how to create a BranchKeyIdSupplier.

 The BranchKeyIdSupplier determines which Branch Key is used
 to protect or access data.
 It is an important component in a Multi-tenant solution,
 where each tenant is cryptographically isolated.
 The Branch Key ID Supplier uses the Encryption Context
 provided at Encrypt or Decrypt
 to determine what "shared secret" (Branch Key)
 is used.
*/

pub struct ExampleBranchKeyIdSupplier {
    branch_key_id_for_tenant_a: String,
    branch_key_id_for_tenant_b: String,
}

impl ExampleBranchKeyIdSupplier {
    pub fn new(tenant_a_id: &str, tenant_b_id: &str) -> Self {
        Self {
            branch_key_id_for_tenant_a: tenant_a_id.to_string(),
            branch_key_id_for_tenant_b: tenant_b_id.to_string(),
        }
    }
}

// We MUST use the encryption context to determine
// the Branch Key ID.
impl BranchKeyIdSupplier for ExampleBranchKeyIdSupplier {
    fn get_branch_key_id(
        &self,
        input: GetBranchKeyIdInput,
    ) -> Result<GetBranchKeyIdOutput, Error> {
        let encryption_context: HashMap<String, String> = input.encryption_context.unwrap();

        if !encryption_context.contains_key("tenant") {
            return Err( Error::AwsCryptographicMaterialProvidersException {
                message:
                    "EncryptionContext invalid, does not contain expected tenant key value pair."
                    .to_string()
                }
            );
        }
        let tenant_key_id: &str = encryption_context["tenant"].as_str();

        if tenant_key_id == "TenantA" {
            Ok(GetBranchKeyIdOutput::builder()
                .branch_key_id(self.branch_key_id_for_tenant_a.clone())
                .build()
                .unwrap())
        } else if tenant_key_id == "TenantB" {
            Ok(GetBranchKeyIdOutput::builder()
                .branch_key_id(self.branch_key_id_for_tenant_b.clone())
                .build()
                .unwrap())
        } else {
            Err( Error::AwsCryptographicMaterialProvidersException {
                message:
                    "Item does not contain valid tenant ID.".to_string()
                }
            )
        }
    }
}
