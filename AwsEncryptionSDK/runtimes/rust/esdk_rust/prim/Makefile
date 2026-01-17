clippy:
	cargo clippy --tests --examples
	cargo clippy --tests --examples --features fips
	cargo clippy --tests --examples --features track
	cargo clippy --tests --examples --features track,fips

test:
	cargo test
	cargo test --features track
	cargo test --features fips
