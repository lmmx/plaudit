build:
    cargo build --workspace --locked

build-release:
    cargo build --workspace --release --locked

check:
    cargo check --workspace

clippy:
    cargo clippy --workspace --all-targets --all-features --target-dir target/clippy-all-features -- -D warnings

[working-directory: 'crates/']
test *args:
    cargo nextest run {{args}}

test-ci *args:
    #!/usr/bin/env -S echo-comment --color bright-green
    # 🏃 Running Rust tests...
    cargo test {{args}}

    # 📚 Running documentation tests...
    cargo test --doc {{args}}

# Install the `plaudit` binary into ~/.cargo/bin
install:
    cargo install --release --path {{bin}} --locked
