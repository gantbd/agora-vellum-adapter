.PHONY: check

check:
	cargo fmt --check
	cargo clippy --all-targets --locked --offline -- -D warnings
	cargo test --all-targets --locked --offline
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --locked --offline
	git diff --check
