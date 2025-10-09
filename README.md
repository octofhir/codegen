# OctoFHIR Codegen

Generate type-safe FHIR SDKs for multiple programming languages.

## Status

🚧 **Work in Progress** - Phase 3: CLI & Config (Tasks 017-018 ✅)

## Features (Planned)

- Multi-language support (TypeScript, Rust, Python, Go)
- Trait-based extensible architecture
- Deep integration with canonical-manager
- Superior type safety and developer experience
- CLI and Rust API

## Quick Start

```bash
# Install
cargo install octofhir-codegen

# Generate TypeScript SDK
octofhir-codegen generate --language typescript --output ./sdk
```

## CLI Usage

```bash
# Show help
just cli-help

# Show version
just cli-version

# List available generators
just cli-list-generators

# Validate configuration
just cli-validate

# Generate SDK
just cli-generate

# Run comprehensive demo
just cli-demo
```

See `just --list` for all available CLI commands.

## Development

```bash
# Build
just build

# Test
just test

# Run CLI commands
just cli-help

# Check everything
just check
```

See [justfile](justfile) for all available development commands.

## Documentation

- [Architecture](ADR-001-CODEGEN-ARCHITECTURE.md) - Complete architecture specification
- [Getting Started](GETTING_STARTED.md) - Developer onboarding guide
- [Task Roadmap](tasks/ROADMAP.md) - Implementation plan

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.