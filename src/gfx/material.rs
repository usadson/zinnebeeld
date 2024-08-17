// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{fs::File, io::BufReader, path::Path};

use euclid::default::Size2D;
use image::{ImageReader, RgbaImage};

use crate::{ImageLoadError, ResourceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Self = Self::from_rgb(0x00, 0x00, 0x00);
    pub const WHITE: Self = Self::from_rgb(0xFF, 0xFF, 0xFF);
    pub const RED: Self = Self::from_rgb(0xFF, 0x00, 0x00);
    pub const GREEN: Self = Self::from_rgb(0x00, 0xFF, 0x00);
    pub const BLUE: Self = Self::from_rgb(0x00, 0x00, 0xFF);
    pub const TEAL: Self = Self::from_rgb(0x00, 0xFF, 0xFF);
    pub const MAGENTA: Self = Self::from_rgb(0xFF, 0x00, 0xFF);
    pub const YELLOW: Self = Self::from_rgb(0xFF, 0xFF, 0x00);

    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    #[must_use]
    pub const fn red(&self) -> u8 {
        self.r
    }

    #[must_use]
    pub const fn green(&self) -> u8 {
        self.g
    }

    #[must_use]
    pub const fn blue(&self) -> u8 {
        self.b
    }

    #[must_use]
    pub const fn alpha(&self) -> u8 {
        self.a
    }

    pub const fn to_f32_rgba(&self) -> [f32; 4] {
        [self.r as _, self.g as _, self.b as _, self.a as _]
    }

    #[inline]
    #[must_use]
    pub const fn as_bgra(&self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let b = self.b as u32;
        let a = self.a as u32;

        a << 24
            | b << 16
            | g << 8
            | r
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Image {
    pub(super) size: Size2D<u32>,
    pub(super) id: ResourceId,
}

impl Image {
    #[must_use]
    pub const fn size(&self) -> Size2D<u32> {
        self.size
    }

    pub(super) fn load(path: &Path) -> Result<(RgbaImage, Size2D<u32>), ImageLoadError> {
        let reader = BufReader::new(File::open(path)?);
        let img = ImageReader::new(reader)
                .with_guessed_format()?
                .decode()?
                .to_rgba8();
        let dimensions = img.dimensions();
        let size = Size2D::from(dimensions);
        Ok((img, size))
    }
}

#[derive(Debug, Clone)]
pub enum Material {
    Color(Color),
    Image(Image),
}

impl From<Color> for Material {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl From<Image> for Material {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}
