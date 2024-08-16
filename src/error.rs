// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use glium::texture::TextureCreationError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageLoadError {
    #[error("decode error: failed to decode")]
    DecodeError(image::ImageError),

    #[error("invalid path: the operating system cannot understand this path")]
    InvalidPath,

    #[error("I/O error: {0}")]
    Io(std::io::Error),

    #[error("texture error: {0}")]
    TextureError(TextureCreationError),
}

impl From<image::ImageError> for ImageLoadError {
    fn from(value: image::ImageError) -> Self {
        Self::DecodeError(value)
    }
}

impl From<std::io::Error> for ImageLoadError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<TextureCreationError> for ImageLoadError {
    fn from(value: TextureCreationError) -> Self {
        Self::TextureError(value)
    }
}
