// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use aws_esdk::EncryptionContext;

/*
 Class containing config for the RegionalRoleClientSupplier.
 In your own code, this might be hardcoded, or reference
 an external source, e.g. environment variables or AWS AppConfig.
*/

const US_EAST_1_IAM_ROLE: &str =
    "arn:aws:iam::370957321024:role/GitHub-CI-Public-ESDK-Dafny-Role-only-us-east-1-KMS-keys";

const EU_WEST_1_IAM_ROLE: &str =
    "arn:aws:iam::370957321024:role/GitHub-CI-Public-ESDK-Dafny-Role-only-eu-west-1-KMS-keys";

pub fn region_iam_role_map() -> EncryptionContext {
    EncryptionContext::from([
        ("us-east-1".to_string(), US_EAST_1_IAM_ROLE.to_string()),
        ("eu-west-1".to_string(), EU_WEST_1_IAM_ROLE.to_string()),
    ])
}
