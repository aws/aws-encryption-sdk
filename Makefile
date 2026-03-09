# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

PROJECTS = ./AwsEncryptionSDK ./TestVectors

verify:
	$(MAKE) -C AwsEncryptionSDK verify CORES=4

dafny-reportgenerator:
	$(MAKE) -C AwsEncryptionSDK dafny-reportgenerator

duvet: | duvet_extract duvet_report

duvet_extract:
	rm -rf compliance
	$(foreach file, $(shell find aws-encryption-sdk-specification/framework -name '*.md'), duvet extract -o compliance -f MARKDOWN $(file);)
	$(foreach file, $(shell find aws-encryption-sdk-specification/client-apis -name '*.md'), duvet extract -o compliance -f MARKDOWN $(file);)
	$(foreach file, $(shell find aws-encryption-sdk-specification/data-format -name '*.md'), duvet extract -o compliance -f MARKDOWN $(file);)

# TODO add these arguments to duvet_report as the work completes
#		--ci \
#		--require-citations true \
#		--require-tests true \

duvet_report:
	duvet \
		report \
		--spec-pattern "compliance/**/*.toml" \
		--source-pattern "AwsCryptographicMaterialProviders/dafny/**/src/**/*.dfy" \
		--source-pattern "AwsCryptographicMaterialProviders/dafny/**/Model/**/*.smithy" \
		--source-pattern "AwsCryptographicMaterialProviders/compliance_exceptions/**/*.txt" \
		--source-pattern "(# //=,# //#).github/workflows/duvet.yaml" \
		--source-pattern "AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/**/*.rs" \
		--source-pattern "AwsEncryptionSDK/runtimes/rust/esdk_rust/aws-esdk-cxx/**/*.rs" \
		--source-pattern "AwsEncryptionSDK/runtimes/rust/esdk_rust/prim/**/*.rs" \
		--source-pattern "compliance_exceptions/**/*.txt" \
		--html specification_compliance_report.html

setup_semantic_release:
	npm i --no-save semantic-release @semantic-release/changelog semantic-release-replace-plugin conventional-changelog-conventionalcommits @semantic-release/git

run_semantic_release:
	npx semantic-release --no-ci

dry_run_semantic_release:
	npx semantic-release --dry-run

format_dafny:
	$(foreach PROJECT, $(PROJECTS), \
		$(MAKE) -C $(PROJECT) format_dafny && \
	) true

format_dafny-check:
	$(foreach PROJECT, $(PROJECTS), \
		$(MAKE) -C $(PROJECT) format_dafny-check && \
	) true

format_net:
	$(foreach PROJECT, $(PROJECTS), \
		$(MAKE) -C $(PROJECT) format_net && \
	) true

format_net-check:
	$(foreach PROJECT, $(PROJECTS), \
		$(MAKE) -C $(PROJECT) format_net-check && \
	) true

format_java_misc: setup_prettier
	npx prettier --plugin=prettier-plugin-java . --write

format_java_misc-check: setup_prettier
	npx prettier --plugin=prettier-plugin-java . --check

setup_prettier:
	npm i --no-save prettier@3.5.3 prettier-plugin-java@2.5
