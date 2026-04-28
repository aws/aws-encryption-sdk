.PHONY: clippy test examples

clippy:
	cargo clippy --tests --examples # redundant with one of the below. Probably ddb.
	cargo clippy --tests --examples --no-default-features
	cargo clippy --tests --examples --no-default-features --features kms
	cargo clippy --tests --examples --no-default-features --features ddb
# 	cargo clippy --tests --examples --no-default-features --features test_vectors
# 	cargo clippy --tests --examples --no-default-features --features kms,test_vectors
# 	cargo clippy --tests --examples --no-default-features --features ddb,test_vectors

test:
	cargo test
	cargo test --no-default-features
	cargo test --no-default-features --features kms
	cargo test --no-default-features --features ddb

examples:
	echo no mpl examples yet
