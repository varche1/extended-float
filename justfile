clippy:
    cargo clippy

fmt:
    cargo fmt

fix:
    cargo fix --allow-dirty --allow-staged

fix_n_lint:
    cargo fmt
    cargo fix --allow-dirty --allow-staged
    cargo clippy

test:
    cargo test

build:
    RUSTFLAGS='-C target-feature=+avx2' cargo build --release

bench *args:
    sudo nice -n -20 cargo bench --manifest-path benchmark/Cargo.toml {{args}}

bench-all *args:
    sudo nice -n -20 cargo bench --manifest-path benchmark/Cargo.toml --features comparison {{args}}

bencher *args:
    BRANCH="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo main)"; \
    HOST="$(hostname 2>/dev/null || echo unknown)"; \
    TESTBED="${BENCHER_TESTBED:-$HOST}"; \
    bencher run --adapter rust_criterion \
               --branch "$$BRANCH" \
               --testbed "$$TESTBED" \
               --project extended-float \
               "sudo nice -n -20 cargo bench --manifest-path benchmark/Cargo.toml {{args}}"

bench-cpu *args:
    sudo nice -n -20 cargo bench --manifest-path benchmark_iai_callgrind/Cargo.toml {{args}}

sync host:
    @if [ -z "{{host}}" ]; then \
      echo "Usage: just sync <host>"; exit 1; \
    fi
    rsync -avz --delete --exclude 'target' --exclude '.git' . {{host}}:~/extended_float
