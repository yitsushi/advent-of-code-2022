ROOT_DIR := $(shell pwd)

build:
	cargo build --release

clippy:
	cargo clippy --workspace -- -Dwarnings

test:
	cargo test --workspace

generate_day%:
	@mkdir lib/solution/src/day$*
	@cp template/day_mod.rs lib/solution/src/day$*/mod.rs

readme_day%:
	python $(ROOT_DIR)/scripts/gen_readme.py $*

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
		cargo test --workspace

	grcov . \
		-s $(ROOT_DIR)/ \
		--binary-path ./target/debug/ \
		-t html \
		--branch \
		--ignore-not-existing \
		-o $(ROOT_DIR)/target/debug/coverage/

	xdg-open $(ROOT_DIR)/target/debug/coverage/index.html
