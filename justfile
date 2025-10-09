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
    cargo run --bin octofhir-codegen --features cli -- {{ARGS}}

# Run in release mode
run-release *ARGS:
    cargo run --release --bin octofhir-codegen --features cli -- {{ARGS}}

# === CLI Command Examples ===

# Show CLI help
cli-help:
    cargo run --bin octofhir-codegen --features cli -- --help

# Show version information
cli-version:
    cargo run --bin octofhir-codegen --features cli -- --version

# Show detailed version information
cli-version-detailed:
    cargo run --bin octofhir-codegen --features cli -- version --detailed

# List available generators
cli-list-generators:
    cargo run --bin octofhir-codegen --features cli -- list-generators

# List generators with details
cli-list-generators-detailed:
    cargo run --bin octofhir-codegen --features cli -- list-generators --detailed

# Validate example configuration
cli-validate:
    cargo run --bin octofhir-codegen --features cli -- validate --config examples/codegen.toml

# Validate with detailed output
cli-validate-detailed:
    cargo run --bin octofhir-codegen --features cli -- validate --config examples/codegen.toml --detailed

# Show generate command help
cli-generate-help:
    cargo run --bin octofhir-codegen --features cli -- generate --help

# Show init command help
cli-init-help:
    cargo run --bin octofhir-codegen --features cli -- init --help

# Run init command (example)
cli-init:
    cargo run --bin octofhir-codegen --features cli -- init --template typescript

# Run generate command (example)
cli-generate:
    cargo run --bin octofhir-codegen --features cli -- generate --config examples/codegen.toml

# Describe TypeScript generator
cli-describe-typescript:
    cargo run --bin octofhir-codegen --features cli -- describe typescript

# Describe TypeScript generator with examples
cli-describe-typescript-examples:
    cargo run --bin octofhir-codegen --features cli -- describe typescript --examples

# Analyze FHIR package (example)
cli-analyze:
    cargo run --bin octofhir-codegen --features cli -- analyze --package hl7.fhir.r4.core@4.0.1 --show-resources

# Run CLI with verbose logging
cli-verbose:
    cargo run --bin octofhir-codegen --features cli -- -vvv list-generators

# Run CLI with JSON output format
cli-json:
    cargo run --bin octofhir-codegen --features cli -- --format json list-generators

# Run CLI with no color
cli-no-color:
    cargo run --bin octofhir-codegen --features cli -- --no-color list-generators

# Show all CLI commands (comprehensive demo)
cli-demo:
    @echo "=== OctoFHIR Codegen CLI Demo ==="
    @echo ""
    @echo "1. Version:"
    @just cli-version
    @echo ""
    @echo "2. List Generators:"
    @just cli-list-generators
    @echo ""
    @echo "3. Validate Config:"
    @just cli-validate
    @echo ""
    @echo "Demo complete!"

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
