clippy:
	cargo clippy --tests --examples # redundant with one of the below. Probably ddb.
	cargo clippy --tests --examples --no-default-features
	cargo clippy --tests --examples --no-default-features --features kms
	cargo clippy --tests --examples --no-default-features --features ddb
	cargo clippy --tests --examples --no-default-features --features legacy
	cargo clippy --tests --examples --no-default-features --features test_vectors
	cargo clippy --tests --examples --no-default-features --features kms,test_vectors
	cargo clippy --tests --examples --no-default-features --features ddb,test_vectors
	cargo clippy --tests --examples --no-default-features --features legacy,test_vectors

test:
	cargo test
	cargo test --no-default-features
	cargo test --no-default-features --features kms
	cargo test --no-default-features --features ddb
	cargo test --no-default-features --features legacy
	cargo test --no-default-features --features test_vectors
	cargo test --no-default-features --features kms,test_vectors
	cargo test --no-default-features --features ddb,test_vectors
	cargo test --no-default-features --features legacy,test_vectors

examples:
	cargo run --example main
	cargo run --example main --no-default-features
	cargo run --example main --no-default-features --features kms
	cargo run --example main --no-default-features --features ddb
	cargo run --example main --no-default-features --features legacy
	cargo run --example main --no-default-features --features test_vectors
	cargo run --example main --no-default-features --features kms,test_vectors
	cargo run --example main --no-default-features --features ddb,test_vectors
	cargo run --example main --no-default-features --features legacy,test_vectors

	
duvet_extract:
	rm -rf compliance
	$(foreach file, $(shell find aws-encryption-sdk-specification/framework -name '*.md'), duvet extract -o compliance -f MARKDOWN $(file);)
	# $(foreach file, $(shell find aws-encryption-sdk-specification/client-apis -name '*.md'), duvet extract -o compliance -f MARKDOWN $(file);)
	# $(foreach file, $(shell find aws-encryption-sdk-specification/data-format -name '*.md'), duvet extract -o compliance -f MARKDOWN $(file);)
