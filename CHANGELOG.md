# Changelog

All notable changes to this project will be documented in this file.  
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-02-22

[0.2.0]: https://github.com/sunsided/docker-image-rs/releases/tag/v0.2.0

### Added

- Added support for `serde` serialization and deserialization via the `serde`, `serde-serialize` and `serde-deserialize` crate features.

### Internal

- Added fuzz testing.

## [0.1.0] - 2024-11-17

[0.1.0]: https://github.com/sunsided/docker-image-rs/releases/tag/v0.1.0

### Added

- Initial release of the `docker-image` crate.
- Support for parsing Docker image references into structured components:
    - **Registry**: Parses optional registries like `docker.io`, `ghcr.io`, or custom ones like `my-registry.local:5000`.
    - **Name**: Extracts mandatory image names, including namespaced ones like `library/nginx`.
    - **Tag**: Supports optional tags like `:latest`, `:v1.0.0`.
    - **Digest**: Parses optional content digests in the form `@sha256:<64-character-digest>`.
- Validation of Docker image references against Docker's official naming rules.
- Support for the `Display` trait to reconstruct valid Docker image references from parsed components.
- Designed for safety:
    - `#![no_std]` compatible for embedded and minimal environments (with `alloc` as a requirement).
    - `#![forbid(unsafe_code)]` ensures memory safety.

### Internal

- Set Minimum Supported Rust Version (MSRV) to `1.81.0` and Rust Edition to `2021`.
- Added CI pipeline for testing against `1.81.0`, `stable`, and `nightly` Rust versions.
- Comprehensive unit tests for valid and invalid Docker image references, ensuring strict compliance with Docker naming conventions.
- Provided examples and documentation in `README.md`.

