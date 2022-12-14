ROOT_DIR := $(shell pwd)

build:
	cargo build --release

clippy:
	cargo clippy --workspace -- -Dwarnings

test:
	cargo test --workspace

generate_day%:
	python $(ROOT_DIR)/scripts/gen_day.py $*

readme_day%: build
	python $(ROOT_DIR)/scripts/gen_readme.py $*

fs-check:
	python $(ROOT_DIR)/scripts/fs-check.py

quality-check: test clippy fs-check

docs:
	cargo clean --doc
	cargo doc --no-deps --workspace

open_docs:
	cargo clean --doc
	cargo doc --no-deps --workspace --open

codecov:
	@rm -rf $(ROOT_DIR)/target/cov/
	@mkdir -p $(ROOT_DIR)/target/cov/
	CARGO_INCREMENTAL=0 \
		RUSTFLAGS='-Cinstrument-coverage' \
		LLVM_PROFILE_FILE="$(ROOT_DIR)/target/cov/cargo-test-%p-%m.profraw" \
		cargo test --workspace --profile=codecov

	grcov . \
		-s $(ROOT_DIR)/ \
		--binary-path ./target/codecov/ \
		-t html \
		--branch \
		--ignore-not-existing \
		-o $(ROOT_DIR)/target/codecov/coverage/

	xdg-open $(ROOT_DIR)/target/codecov/coverage/index.html
