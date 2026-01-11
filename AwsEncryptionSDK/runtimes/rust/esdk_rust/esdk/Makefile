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

