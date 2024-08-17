// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Rect;
use crate::Material;

pub trait PainterImplementation {
    fn paint_filled_rect(&mut self, rect: Rect<f32>, brush: Material);
}

pub struct Painter<'pi> {
    pub(super) inner: &'pi mut dyn PainterImplementation,
}

impl<'pi> Painter<'pi> {
    pub fn paint_filled_rect(&mut self, rect: Rect<f32>, brush: impl Into<Material>) {
        self.inner.paint_filled_rect(rect, brush.into())
    }
}
