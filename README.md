# 🐋 docker-image

> A utility crate for parsing Docker image references.

[![Crates.io][crates-image]][crates-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Safety Dance][safety-image]][safety-link]
[![codecov][codecov-image]][codecov-link]
![MSRV][msrv-image]
[![EUPL 1.2 licensed][license-eupl-image]][license-eupl-link]

## Features

- Parse Docker image references into structured components:
    - Registry (e.g., `docker.io`, `ghcr.io`)
    - Name (e.g., `nginx`, `library/nginx`)
    - Tag (e.g., `latest`, `v1.0.0`)
    - Digest (e.g., `sha256:<64-character-digest>`)
- Validate Docker image references against Docker's official naming rules.
- Designed for safety:
    - `#![no_std]` compatible for embedded and minimal environments (however `alloc` remains a requirement).
    - `#![forbid(unsafe_code)]` ensures memory safety.
- Minimum Supported Rust Version (MSRV): Rust 1.65.0
  - This is the earliest version of Rust required to use this library.
  - Note: The MSRV may increase in future releases due to updates in dependencies or library features.

**Supported Formats:**

- `nginx`
- `nginx:latest`
- `docker.io/library/nginx`
- `docker.io/library/nginx:latest`
- `docker.io/library/nginx@sha256:<digest>`
- `docker.io/library/nginx:latest@sha256:<digest>`

**Not Supported:**

- Unicode characters in names, tags, or digests.
- Uppercase letters in names (Docker requires lowercase).
- Invalid or malformed registries, tags, or digests.

## Usage Examples

### Parsing a Docker Image Reference

```rust
use docker_image::DockerImage;

fn it_works() {
    let image = DockerImage::parse("docker.io/library/nginx:latest").unwrap();
    assert_eq!(image.registry, Some("docker.io".to_string()));
    assert_eq!(image.name, "library/nginx".to_string());
    assert_eq!(image.tag, Some("latest".to_string()));
    assert_eq!(image.digest, None);
}
```

### Invalid Docker Image Reference

```rust
use docker_image::DockerImage;

fn it_works() {
    let result = DockerImage::parse("nginx🚀");
    assert_eq!(result, Err(DockerImageError::InvalidFormat));
}
```

### Display Implementation

```rust
use docker_image::DockerImage;

fn it_works() {
    let image = DockerImage {
        registry: Some("docker.io".to_string()),
        name: "library/nginx".to_string(),
        tag: Some("latest".to_string()),
        digest: None,
    };

    assert_eq!(format!("{}", image), "docker.io/library/nginx:latest");
}
```

[crates-image]: https://img.shields.io/crates/v/docker-image

[crates-link]: https://crates.io/crates/docker-image

[docs-image]: https://docs.rs/docker-image/badge.svg

[docs-link]: https://docs.rs/docker-image/

[build-image]: https://github.com/sunsided/docker-image-rs/workflows/Rust/badge.svg

[build-link]: https://github.com/sunsided/docker-image-rs/actions

[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg

[safety-link]: https://github.com/rust-secure-code/safety-dance/

[msrv-image]: https://img.shields.io/badge/rustc-1.65+-blue.svg

[license-eupl-image]: https://img.shields.io/badge/license-EUPL_1.2-blue.svg

[license-eupl-link]: https://github.com/sunsided/docker-image-rs/blob/main/LICENSE.md

[codecov-image]: https://codecov.io/gh/sunsided/docker-image-rs/graph/badge.svg?token=nJPELlY1YV

[codecov-link]: https://codecov.io/gh/sunsided/docker-image-rs

[cc]: https://contributor-covenant.org
