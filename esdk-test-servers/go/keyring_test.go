// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

package main

import (
	"context"
	"testing"
)

// The AWS config every KMS/DynamoDB client is built from must resolve
// credentials from the default chain (here: environment variables) and carry
// the requested region, so the server's AWS requests are signed.
func TestAwsConfigForRegionResolvesCredentialsAndRegion(t *testing.T) {
	t.Setenv("AWS_ACCESS_KEY_ID", "AKIDEXAMPLE")
	t.Setenv("AWS_SECRET_ACCESS_KEY", "example-secret")
	t.Setenv("AWS_SESSION_TOKEN", "")

	cfg, err := awsConfigForRegion(context.Background(), "eu-west-1")
	if err != nil {
		t.Fatalf("awsConfigForRegion: %v", err)
	}
	if cfg.Region != "eu-west-1" {
		t.Fatalf("region = %q, want eu-west-1", cfg.Region)
	}
	creds, err := cfg.Credentials.Retrieve(context.Background())
	if err != nil {
		t.Fatalf("credentials must resolve from the default chain: %v", err)
	}
	if creds.AccessKeyID != "AKIDEXAMPLE" {
		t.Fatalf("AccessKeyID = %q, want the environment credential", creds.AccessKeyID)
	}
}
