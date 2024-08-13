// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use glium::uniforms::{AsUniformValue, UniformValue};

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
}

#[derive(Debug)]
pub enum Material {
    Color(Color),
}

impl From<Color> for Material {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl AsUniformValue for Color {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Vec4(self.to_f32_rgba())
    }
}
