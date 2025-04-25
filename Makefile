clippy:
	cargo clippy

fmt:
	cargo fmt

fix:
	cargo fix --allow-dirty --allow-staged

fix_n_lint: fmt fix clippy

test:
	cargo test

build:
	RUSTFLAGS='-C target-feature=+avx2' cargo build --release

bencher:
	BRANCH=$$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "main"); \
	HOST=$$(hostname 2>/dev/null || echo "unknown"); \
	TESTBED=$${BENCHER_TESTBED:-$$HOST}; \
	bencher run --adapter rust_criterion --branch "$$BRANCH" --testbed "$$TESTBED" --project="extended-float" "cargo bench"

.PHONY: clippy fmt fix fix_n_lint test build bencher
