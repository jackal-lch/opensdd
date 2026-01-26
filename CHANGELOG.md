# Changelog

All notable changes to OpenSDD will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-26

### Added

- **Skills**
  - `/create-blueprint` - 8-phase guided product definition (vision, users, features, flows, data, integrations, constraints, assembly)
  - `/create-spec` - 4-phase technical specification generator (foundation, types, contracts, integrations)
  - `/build-loop` - 5-phase implementation loop with automatic drift detection and fixing

- **Agents**
  - `verify-compare` - Semantic comparison agent for drift detection with language idiom translation

- **Tools**
  - `spec-extract` - Rust CLI for extracting code signatures using tree-sitter
    - Supports TypeScript, Python, Rust, Go
    - Outputs YAML/JSON format
    - Generates index files for project-wide extraction

- **Commands**
  - `/cov` - Chain of Verification prompt

- **Documentation**
  - Comprehensive README with installation and usage instructions
  - Blueprint philosophy document explaining the OpenSDD approach
  - Spec schema reference

### Technical Details

- State management via Python scripts for phase tracking
- Cross-platform install scripts for spec-extract (macOS, Linux, Windows)
- GitHub Actions workflow for automated releases
