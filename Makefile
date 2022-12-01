build:
	cargo build --release

clippy:
	cargo clippy --workspace -- -Dwarnings

test:
	cargo test --workspace
