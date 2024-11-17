//! # docker-image
//!
//! A library for parsing and handling Docker image references in a structured way.
//!
//! Docker image references can include components like a registry, name, tag, and digest.
//! This library parses valid Docker image strings into their respective components, with proper validation.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
use alloc::string::{String, ToString};

use core::fmt;
use core::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

/// Represents a parsed Docker image reference.
///
/// A Docker image can have the following components:
/// - `registry`: The optional registry URL (e.g., `docker.io`, `ghcr.io`, or a custom registry like `my-registry.local:5000`).
/// - `name`: The mandatory name of the image, which may include namespaces (e.g., `library/nginx`).
/// - `tag`: An optional version tag for the image (e.g., `latest`, `v1.0.0`).
/// - `digest`: An optional digest for the image content (e.g., `sha256:<64-hex-digest>`).
///
/// # Examples
/// ```
/// use docker_image::DockerImage;
///
/// let image = DockerImage::parse("docker.io/library/nginx:latest").unwrap();
/// assert_eq!(image.registry, Some("docker.io".to_string()));
/// assert_eq!(image.name, "library/nginx".to_string());
/// assert_eq!(image.tag, Some("latest".to_string()));
/// assert_eq!(image.digest, None);
/// ```
#[derive(Debug, PartialEq)]
pub struct DockerImage {
    /// The optional registry URL.
    pub registry: Option<String>,
    /// The name of the image, including namespaces if present.
    pub name: String,
    /// The optional version tag.
    pub tag: Option<String>,
    /// The optional content digest (e.g., `sha256:<64-hex-digest>`).
    pub digest: Option<String>,
}

impl fmt::Display for DockerImage {
    /// Formats the `DockerImage` as a valid Docker image reference string.
    ///
    /// The format includes:
    /// - `[registry/]name[:tag][@digest]`
    ///
    /// Examples:
    /// - `nginx`
    /// - `nginx:latest`
    /// - `docker.io/library/nginx:latest`
    /// - `ubuntu@sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234`
    /// - `my-registry.local:5000/library/image-name:v1.0.0@sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(registry) = &self.registry {
            write!(f, "{}/", registry)?;
        }
        write!(f, "{}", self.name)?;
        if let Some(tag) = &self.tag {
            write!(f, ":{}", tag)?;
        }
        if let Some(digest) = &self.digest {
            write!(f, "@{}", digest)?;
        }
        Ok(())
    }
}

/// Errors that can occur while parsing Docker image references.
#[derive(Debug, PartialEq)]
pub enum DockerImageError {
    /// Indicates that the Docker image string has an invalid format.
    InvalidFormat,
}

impl fmt::Display for DockerImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DockerImageError::InvalidFormat => write!(f, "Invalid Docker image format"),
        }
    }
}

impl core::error::Error for DockerImageError {}

impl FromStr for DockerImage {
    type Err = DockerImageError;

    /// Parses a Docker image string into its structured components.
    ///
    /// This function supports the following Docker image formats:
    /// - `nginx`
    /// - `nginx:latest`
    /// - `docker.io/library/nginx`
    /// - `docker.io/library/nginx:latest`
    /// - `docker.io/library/nginx@sha256:<digest>`
    /// - `docker.io/library/nginx:latest@sha256:<digest>`
    ///
    /// # Examples
    /// ```
    /// use docker_image::DockerImage;
    ///
    /// let image: DockerImage = "nginx:latest".parse().unwrap();
    /// assert_eq!(image.name, "nginx");
    /// assert_eq!(image.tag, Some("latest".to_string()));
    /// assert_eq!(image.digest, None);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref DOCKER_IMAGE_REGEX: Regex = Regex::new(
                r"^(?:(?P<registry>[a-z0-9]+(?:[._-][a-z0-9]+)*\.[a-z]{2,}(?::\d+)?)/)?(?P<name>[a-z0-9]+(?:[._-][a-z0-9]+)*(?:/[a-z0-9]+(?:[._-][a-z0-9]+)*)*)(?::(?P<tag>[a-zA-Z0-9._-]+))?(?:@(?P<digest>[a-z0-9]+:[a-fA-F0-9]{64}))?$"
            )
            .unwrap();
        }

        if let Some(captures) = DOCKER_IMAGE_REGEX.captures(s) {
            Ok(DockerImage {
                registry: captures.name("registry").map(|m| m.as_str().to_string()),
                name: captures
                    .name("name")
                    .ok_or(DockerImageError::InvalidFormat)?
                    .as_str()
                    .to_string(),
                tag: captures.name("tag").map(|m| m.as_str().to_string()),
                digest: captures.name("digest").map(|m| m.as_str().to_string()),
            })
        } else {
            Err(DockerImageError::InvalidFormat)
        }
    }
}

impl DockerImage {
    /// Parses a Docker image string into its structured components.
    ///
    /// This is a convenience function for [`DockerImage::from_str`].
    ///
    /// # Examples
    /// ```
    /// use docker_image::DockerImage;
    ///
    /// let image = DockerImage::parse("ubuntu@sha256:45b23dee08af5e43a7fea6c4cf9c25ccf269ee113168c19722f87876677c5cb2").unwrap();
    /// assert_eq!(image.name, "ubuntu");
    /// assert_eq!(image.digest, Some("sha256:45b23dee08af5e43a7fea6c4cf9c25ccf269ee113168c19722f87876677c5cb2".to_string()));
    /// ```
    pub fn parse(image_str: &str) -> Result<Self, DockerImageError> {
        Self::from_str(image_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_format::assert_display_fmt;

    #[test]
    fn test_trivial_name() {
        let result = DockerImage::parse("nginx");
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: None,
                name: "nginx".to_string(),
                tag: None,
                digest: None,
            })
        );
    }

    #[test]
    fn test_name_with_tag() {
        let result = DockerImage::parse("nginx:latest");
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: None,
                name: "nginx".to_string(),
                tag: Some("latest".to_string()),
                digest: None,
            })
        );
    }

    #[test]
    fn test_name_with_complex_tag() {
        let result = DockerImage::parse("nginx:stable-alpine3.20-perl");
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: None,
                name: "nginx".to_string(),
                tag: Some("stable-alpine3.20-perl".to_string()),
                digest: None,
            })
        );
    }

    #[test]
    fn test_registry_and_name() {
        let result = DockerImage::parse("docker.io/nginx");
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: Some("docker.io".to_string()),
                name: "nginx".to_string(),
                tag: None,
                digest: None,
            })
        );
    }

    #[test]
    fn test_registry_with_namespace() {
        let result = DockerImage::parse("ghcr.io/nginx/nginx");
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: Some("ghcr.io".to_string()),
                name: "nginx/nginx".to_string(),
                tag: None,
                digest: None,
            })
        );
    }

    #[test]
    fn test_name_with_digest() {
        let result = DockerImage::parse(
            "ubuntu@sha256:45b23dee08af5e43a7fea6c4cf9c25ccf269ee113168c19722f87876677c5cb2",
        );
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: None,
                name: "ubuntu".to_string(),
                tag: None,
                digest: Some(
                    "sha256:45b23dee08af5e43a7fea6c4cf9c25ccf269ee113168c19722f87876677c5cb2"
                        .to_string()
                ),
            })
        );
    }

    #[test]
    fn test_name_with_tag_and_digest() {
        let result = DockerImage::parse(
            "ubuntu:latest@sha256:45b23dee08af5e43a7fea6c4cf9c25ccf269ee113168c19722f87876677c5cb2",
        );
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: None,
                name: "ubuntu".to_string(),
                tag: Some("latest".to_string()),
                digest: Some(
                    "sha256:45b23dee08af5e43a7fea6c4cf9c25ccf269ee113168c19722f87876677c5cb2"
                        .to_string()
                ),
            })
        );
    }

    #[test]
    fn test_registry_name_tag() {
        let result = DockerImage::parse("registry.example.com/library/my-image:1.0.0");
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: Some("registry.example.com".to_string()),
                name: "library/my-image".to_string(),
                tag: Some("1.0.0".to_string()),
                digest: None,
            })
        );
    }

    #[test]
    fn test_registry_name_digest() {
        let result = DockerImage::parse(
            "my-registry.local:5000/library/image-name@sha256:deadbeefcafe1234567890abcdef1234567890abcdef1234567890abcdef1234",
        );
        assert_eq!(
            result,
            Ok(DockerImage {
                registry: Some("my-registry.local:5000".to_string()),
                name: "library/image-name".to_string(),
                tag: None,
                digest: Some(
                    "sha256:deadbeefcafe1234567890abcdef1234567890abcdef1234567890abcdef1234"
                        .to_string()
                ),
            })
        );
    }

    #[test]
    fn test_invalid_format() {
        let result = DockerImage::parse("invalid@@sha256:wrong");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_invalid_characters_in_tag() {
        let result = DockerImage::parse("nginx:lat@est");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_invalid_digest_format() {
        let result = DockerImage::parse("ubuntu@sha256:not-a-hex-string");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_invalid_registry_format() {
        let result = DockerImage::parse("http://registry.example.com/image-name");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_invalid_double_colons_in_tag() {
        let result = DockerImage::parse("nginx::latest");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_missing_image_name_with_tag() {
        let result = DockerImage::parse(":latest");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_missing_image_name_with_digest() {
        let result = DockerImage::parse(
            "@sha256:deadbeefcafe1234567890abcdef1234567890abcdef1234567890abcdef1234",
        );
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_extra_tag_components() {
        let result = DockerImage::parse("my-image:1.0.0:latest");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_unicode_in_name() {
        let result = DockerImage::parse("nginx🚀");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_unicode_in_registry() {
        let result = DockerImage::parse("docker🚀.io/library/nginx");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_unicode_in_tag() {
        let result = DockerImage::parse("nginx:lat🚀est");
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_unicode_in_digest() {
        let result = DockerImage::parse(
            "nginx@sha256:deadbeef🚀1234567890abcdef1234567890abcdef1234567890abcdef1234",
        );
        assert_eq!(result, Err(DockerImageError::InvalidFormat));
    }

    #[test]
    fn test_display_trivial_name() {
        let image = DockerImage {
            registry: None,
            name: "nginx".to_string(),
            tag: None,
            digest: None,
        };

        assert_display_fmt!(image, "nginx");
    }

    #[test]
    fn test_display_name_with_tag() {
        let image = DockerImage {
            registry: None,
            name: "nginx".to_string(),
            tag: Some("latest".to_string()),
            digest: None,
        };

        assert_display_fmt!(image, "nginx:latest");
    }

    #[test]
    fn test_display_name_with_digest() {
        let image = DockerImage {
            registry: None,
            name: "ubuntu".to_string(),
            tag: None,
            digest: Some(
                "sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234".to_string(),
            ),
        };

        assert_display_fmt!(
            image,
            "ubuntu@sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234"
        );
    }

    #[test]
    fn test_display_name_with_tag_and_digest() {
        let image = DockerImage {
            registry: None,
            name: "ubuntu".to_string(),
            tag: Some("latest".to_string()),
            digest: Some(
                "sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234".to_string(),
            ),
        };

        assert_display_fmt!(
            image,
            "ubuntu:latest@sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234"
        );
    }

    #[test]
    fn test_display_registry_and_name() {
        let image = DockerImage {
            registry: Some("docker.io".to_string()),
            name: "library/nginx".to_string(),
            tag: None,
            digest: None,
        };

        assert_display_fmt!(image, "docker.io/library/nginx");
    }

    #[test]
    fn test_display_registry_name_with_tag() {
        let image = DockerImage {
            registry: Some("docker.io".to_string()),
            name: "library/nginx".to_string(),
            tag: Some("latest".to_string()),
            digest: None,
        };

        assert_display_fmt!(image, "docker.io/library/nginx:latest");
    }

    #[test]
    fn test_display_full_reference() {
        let image = DockerImage {
            registry: Some("my-registry.local:5000".to_string()),
            name: "library/image-name".to_string(),
            tag: Some("v1.0.0".to_string()),
            digest: Some(
                "sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234".to_string(),
            ),
        };

        assert_display_fmt!(
            image,
            "my-registry.local:5000/library/image-name:v1.0.0@sha256:deadbeef1234567890abcdef1234567890abcdef1234567890abcdef1234"
        );
    }
}
