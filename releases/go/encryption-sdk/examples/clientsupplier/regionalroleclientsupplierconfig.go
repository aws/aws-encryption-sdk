// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package clientsupplier

/*
 File containing config for the RegionalRoleClientSupplier.
 In your own code, this might be hardcoded, or reference
 an external source, e.g. environment variables or AWS AppConfig.
*/

const (
	usEast1IamRole = "arn:aws:iam::370957321024:role/GitHub-CI-Public-ESDK-Dafny-Role-only-us-east-1-KMS-keys"
	euWest1IamRole = "arn:aws:iam::370957321024:role/GitHub-CI-Public-ESDK-Dafny-Role-only-eu-west-1-KMS-keys"
)

func RegionIamRoleMap() map[string]string {
	return map[string]string{
		"us-east-1": usEast1IamRole,
		"eu-west-1": euWest1IamRole,
	}
}
