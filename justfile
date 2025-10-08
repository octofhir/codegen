# OctoFHIR Codegen - Development Commands

# Default recipe (list all commands)
default:
    @just --list

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Build the project
build:
    cargo build

# Build release
build-release:
    cargo build --release

# Run tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run doc tests
test-doc:
    cargo test --doc

# Review snapshot tests
insta-review:
    cargo insta review

# Accept all snapshot changes
insta-accept:
    cargo insta accept

# Run benchmarks
bench:
    cargo bench

# Generate documentation
doc:
    cargo doc --no-deps --open

# Build and open documentation
docs: doc

# Check everything (formatting, clippy, build, test)
check: fmt-check clippy build test

# Quick check (format, clippy, build)
quick: fmt clippy build

# Clean build artifacts
clean:
    cargo clean

# Run the CLI
run *ARGS:
    cargo run -- {{ARGS}}

# Run in release mode
run-release *ARGS:
    cargo run --release -- {{ARGS}}

# Watch and rebuild on changes (requires cargo-watch)
watch:
    cargo watch -x build -x test

# Install cargo tools needed for development
install-tools:
    cargo install cargo-watch
    cargo install cargo-insta
    cargo install cargo-tarpaulin

# Run coverage report (requires cargo-tarpaulin)
coverage:
    cargo tarpaulin --out Html --out Json --output-dir target/coverage

# Update dependencies
update:
    cargo update

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Full quality check (everything before commit)
pre-commit: fmt clippy test

# CI simulation (what runs on GitHub Actions)
ci: fmt-check clippy build test test-doc

# Task workflow - start new task
task-start NUMBER:
    @echo "Starting TASK-{{NUMBER}}"
    @echo "See: tasks/phase*/TASK-{{NUMBER}}-*.md"

# Task workflow - complete task
task-complete: check
    @echo "✅ Task complete! All checks passed."
    @echo "Run: git add . && git commit -m 'feat: complete TASK-XXX'"

# Quick development cycle
dev: fmt build test
