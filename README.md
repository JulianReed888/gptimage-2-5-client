# gptimage-2-5-client

A lightweight Rust helper toolkit for building [GPT Image 2.5](https://gptimage-2-5.com/) image generation workflows.

GPT Image 2.5 is an AI image generation platform focused on exact text rendering, high-resolution output, reference-guided editing, and clean multi-turn revisions.

This crate provides small Rust types and helpers for representing prompt requests, generation settings, reference-image metadata, and the limits those requests have to respect.

## Features

- Prompt request structures with a builder API
- Generation settings (aspect ratio, resolution, seed, negative prompt)
- Reference image metadata with roles for character, product, style, and composition
- Validation against prompt-length and reference-count limits
- Optional Serde support for serialization

## Example

```rust
use gptimage_2_5_client::{AspectRatio, ImageRequest, ReferenceImage, ReferenceRole, Resolution};

let request = ImageRequest::new("A packaging shot with the brand name printed on curved glass")
    .aspect_ratio(AspectRatio::Widescreen)
    .resolution(Resolution::FourK)
    .reference(ReferenceImage::new("s3://refs/bottle.png", ReferenceRole::Product));

assert!(request.is_image_to_image());
assert!(request.validate().is_ok());
```

## Limits

| Limit | Value |
| --- | --- |
| Reference images per edit | 16 |
| Prompt length | 5000 characters |
| Resolution tiers | 1K, 2K, 4K |
| Aspect ratios | 1:1, 16:9, 9:16, 3:2, 2:3, auto, custom |

## Why this crate?

AI image workflows often need structured prompt data, generation parameters, and consistent reference metadata. This crate keeps those pieces simple and portable for Rust-based tools, CLIs, and backend services, so the same request can be built once and sent to whichever backend you use.

Learn more about [GPT Image 2.5](https://gptimage-2-5.com/).
