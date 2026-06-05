# plaudit — Plaud recording export CLI

bin := "plaudit-cli"

# Show available recipes
default:
    @just --list

# --- dev ---

# Fast type-check
check:
    cargo check --workspace

# Build release binary
build:
    cargo build --release --locked

# Format, lint, test — run before committing
ci: fmt-check clippy test

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all --check

clippy:
    cargo clippy --all-targets --workspace -- -D warnings

test:
    cargo test --workspace

# Install the `plaudit` binary into ~/.cargo/bin
install:
    cargo install --path plaudit-cli --locked

# --- run (dev, debug build) ---

login:
    cargo run -p {{bin}} -- login

logout:
    cargo run -p {{bin}} -- logout

me:
    cargo run -p {{bin}} -- me

files *args:
    cargo run -p {{bin}} -- files {{args}}

file id:
    cargo run -p {{bin}} -- file {{id}}

transcript id *args:
    cargo run -p {{bin}} -- transcript {{id}} {{args}}

summary id *args:
    cargo run -p {{bin}} -- summary {{id}} {{args}}

audio id:
    cargo run -p {{bin}} -- audio {{id}}

sync folder:
    cargo run -p {{bin}} -- sync {{folder}}
