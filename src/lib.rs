//! Lightweight helpers for GPT Image 2.5-style AI image generation workflows.
//!
//! This crate provides request, aspect-ratio, resolution, and reference-image
//! types for Rust applications that need to organize image generation prompts
//! before handing them to a backend or HTTP client.
//!
//! ```
//! use gptimage_2_5_client::{AspectRatio, ImageRequest, Resolution};
//!
//! let request = ImageRequest::new("A product poster with the headline SPRING SALE")
//!     .aspect_ratio(AspectRatio::Widescreen)
//!     .resolution(Resolution::TwoK);
//!
//! assert_eq!(request.resolution, Resolution::TwoK);
//! assert!(request.validate().is_ok());
//! ```

use std::error::Error;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Maximum number of reference images accepted by an image-to-image edit.
pub const MAX_REFERENCE_IMAGES: usize = 16;

/// Maximum prompt length in characters.
pub const MAX_PROMPT_CHARS: usize = 5000;

/// Common image aspect ratios.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AspectRatio {
    /// 1:1 square image.
    Square,
    /// 16:9 widescreen image.
    Widescreen,
    /// 9:16 vertical image for short-form platforms.
    Vertical,
    /// 3:2 landscape image.
    Landscape,
    /// 2:3 portrait image.
    Portrait,
    /// Let the model choose the frame that fits the prompt.
    Auto,
    /// Custom aspect ratio, such as "4:3" or "21:9".
    Custom(String),
}

impl AspectRatio {
    /// Returns the ratio in `width:height` form, or `"auto"` for [`AspectRatio::Auto`].
    pub fn as_str(&self) -> &str {
        match self {
            Self::Square => "1:1",
            Self::Widescreen => "16:9",
            Self::Vertical => "9:16",
            Self::Landscape => "3:2",
            Self::Portrait => "2:3",
            Self::Auto => "auto",
            Self::Custom(value) => value,
        }
    }
}

impl fmt::Display for AspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Output resolution tiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Resolution {
    /// 1K output, the cheapest tier.
    OneK,
    /// 2K output.
    TwoK,
    /// 4K output, the most detailed tier.
    FourK,
}

impl Resolution {
    /// Returns the short label used in most APIs, such as `"2K"`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneK => "1K",
            Self::TwoK => "2K",
            Self::FourK => "4K",
        }
    }

    /// Returns the long edge in pixels for this tier.
    pub fn long_edge_pixels(&self) -> u32 {
        match self {
            Self::OneK => 1024,
            Self::TwoK => 2048,
            Self::FourK => 4096,
        }
    }
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How a reference image should steer the generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReferenceRole {
    /// Keep a character or person consistent.
    Character,
    /// Keep a product or object consistent.
    Product,
    /// Carry over an art style.
    Style,
    /// Carry over a layout or composition.
    Composition,
}

/// A single reference image supplied to an image-to-image edit.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReferenceImage {
    /// Source URL or storage identifier.
    pub source: String,
    /// What the reference is meant to steer.
    pub role: ReferenceRole,
    /// Optional human-readable label.
    pub label: Option<String>,
}

impl ReferenceImage {
    /// Creates a reference image with the given source and role.
    pub fn new(source: impl Into<String>, role: ReferenceRole) -> Self {
        Self {
            source: source.into(),
            role,
            label: None,
        }
    }

    /// Attaches a human-readable label.
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Errors produced when a request is not valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestError {
    /// The prompt was empty or only whitespace.
    EmptyPrompt,
    /// The prompt exceeded [`MAX_PROMPT_CHARS`].
    PromptTooLong {
        /// Actual prompt length in characters.
        chars: usize,
    },
    /// More than [`MAX_REFERENCE_IMAGES`] references were supplied.
    TooManyReferences {
        /// Actual number of references supplied.
        count: usize,
    },
    /// A reference image had an empty source.
    EmptyReferenceSource,
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPrompt => write!(f, "prompt must not be empty"),
            Self::PromptTooLong { chars } => {
                write!(f, "prompt is {chars} characters, limit is {MAX_PROMPT_CHARS}")
            }
            Self::TooManyReferences { count } => {
                write!(
                    f,
                    "{count} reference images supplied, limit is {MAX_REFERENCE_IMAGES}"
                )
            }
            Self::EmptyReferenceSource => write!(f, "reference image source must not be empty"),
        }
    }
}

impl Error for RequestError {}

/// A structured request for an AI image generation workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImageRequest {
    /// Main text prompt.
    pub prompt: String,
    /// Desired aspect ratio.
    pub aspect_ratio: AspectRatio,
    /// Desired output resolution.
    pub resolution: Resolution,
    /// Optional negative prompt.
    pub negative_prompt: Option<String>,
    /// Optional model name or provider identifier.
    pub model: Option<String>,
    /// Optional seed for reproducible output.
    pub seed: Option<u64>,
    /// Reference images for image-to-image editing.
    pub references: Vec<ReferenceImage>,
}

impl ImageRequest {
    /// Creates a new image request with sensible defaults.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            aspect_ratio: AspectRatio::Square,
            resolution: Resolution::OneK,
            negative_prompt: None,
            model: None,
            seed: None,
            references: Vec::new(),
        }
    }

    /// Sets the aspect ratio.
    pub fn aspect_ratio(mut self, aspect_ratio: AspectRatio) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Sets the output resolution.
    pub fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    /// Sets a negative prompt.
    pub fn negative_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.negative_prompt = Some(prompt.into());
        self
    }

    /// Sets a model identifier.
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    /// Sets a seed for reproducible output.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Adds a reference image, ignoring the limit until [`ImageRequest::validate`] runs.
    pub fn reference(mut self, reference: ReferenceImage) -> Self {
        self.references.push(reference);
        self
    }

    /// Adds a reference image, failing if the request already holds the maximum.
    pub fn try_reference(&mut self, reference: ReferenceImage) -> Result<(), RequestError> {
        if self.references.len() >= MAX_REFERENCE_IMAGES {
            return Err(RequestError::TooManyReferences {
                count: self.references.len() + 1,
            });
        }
        self.references.push(reference);
        Ok(())
    }

    /// Reports whether this request runs as an image-to-image edit.
    pub fn is_image_to_image(&self) -> bool {
        !self.references.is_empty()
    }

    /// Checks the prompt and reference images against the documented limits.
    pub fn validate(&self) -> Result<(), RequestError> {
        if self.prompt.trim().is_empty() {
            return Err(RequestError::EmptyPrompt);
        }

        let chars = self.prompt.chars().count();
        if chars > MAX_PROMPT_CHARS {
            return Err(RequestError::PromptTooLong { chars });
        }

        if self.references.len() > MAX_REFERENCE_IMAGES {
            return Err(RequestError::TooManyReferences {
                count: self.references.len(),
            });
        }

        if self.references.iter().any(|r| r.source.trim().is_empty()) {
            return Err(RequestError::EmptyReferenceSource);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_default_image_request() {
        let request = ImageRequest::new("A neon lit street at night");

        assert_eq!(request.aspect_ratio, AspectRatio::Square);
        assert_eq!(request.resolution, Resolution::OneK);
        assert!(!request.is_image_to_image());
        assert!(request.validate().is_ok());
    }

    #[test]
    fn updates_image_request_fields() {
        let request = ImageRequest::new("A packaging shot with readable labels")
            .aspect_ratio(AspectRatio::Widescreen)
            .resolution(Resolution::FourK)
            .negative_prompt("blurry text")
            .model("gpt-image-2.5")
            .seed(42);

        assert_eq!(request.aspect_ratio.as_str(), "16:9");
        assert_eq!(request.resolution.long_edge_pixels(), 4096);
        assert_eq!(request.negative_prompt.as_deref(), Some("blurry text"));
        assert_eq!(request.model.as_deref(), Some("gpt-image-2.5"));
        assert_eq!(request.seed, Some(42));
    }

    #[test]
    fn tracks_reference_images() {
        let request = ImageRequest::new("Keep this character consistent")
            .reference(ReferenceImage::new("s3://refs/face.png", ReferenceRole::Character).label("hero"))
            .reference(ReferenceImage::new("s3://refs/jacket.png", ReferenceRole::Product));

        assert!(request.is_image_to_image());
        assert_eq!(request.references.len(), 2);
        assert_eq!(request.references[0].label.as_deref(), Some("hero"));
        assert!(request.validate().is_ok());
    }

    #[test]
    fn rejects_empty_prompt() {
        let request = ImageRequest::new("   ");

        assert_eq!(request.validate(), Err(RequestError::EmptyPrompt));
    }

    #[test]
    fn rejects_overlong_prompt() {
        let request = ImageRequest::new("a".repeat(MAX_PROMPT_CHARS + 1));

        assert_eq!(
            request.validate(),
            Err(RequestError::PromptTooLong {
                chars: MAX_PROMPT_CHARS + 1
            })
        );
    }

    #[test]
    fn rejects_too_many_references() {
        let mut request = ImageRequest::new("A coordinated campaign set");
        for index in 0..MAX_REFERENCE_IMAGES {
            request
                .try_reference(ReferenceImage::new(
                    format!("s3://refs/{index}.png"),
                    ReferenceRole::Style,
                ))
                .expect("within limit");
        }

        let overflow = request.try_reference(ReferenceImage::new(
            "s3://refs/extra.png",
            ReferenceRole::Style,
        ));

        assert_eq!(
            overflow,
            Err(RequestError::TooManyReferences {
                count: MAX_REFERENCE_IMAGES + 1
            })
        );
        assert_eq!(request.references.len(), MAX_REFERENCE_IMAGES);
    }

    #[test]
    fn rejects_empty_reference_source() {
        let request = ImageRequest::new("A hero image")
            .reference(ReferenceImage::new("  ", ReferenceRole::Composition));

        assert_eq!(request.validate(), Err(RequestError::EmptyReferenceSource));
    }

    #[test]
    fn formats_aspect_ratio_and_resolution() {
        assert_eq!(AspectRatio::Auto.to_string(), "auto");
        assert_eq!(AspectRatio::Custom("21:9".into()).to_string(), "21:9");
        assert_eq!(Resolution::TwoK.to_string(), "2K");
    }
}
