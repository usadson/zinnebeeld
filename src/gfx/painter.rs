// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::Rect;
use crate::Material;

pub trait PainterImplementation {
    fn paint_filled_rect(&mut self, rect: Rect<f32>, brush: Material);
    fn finish(&mut self);
}

pub struct Painter {
    inner: Box<dyn PainterImplementation>,
}

impl Painter {
    pub(super) fn new<PI>(painter: PI) -> Self
            where PI: PainterImplementation + 'static {
        Self {
            inner: Box::new(painter),
        }
    }

    pub fn paint_filled_rect(&mut self, rect: Rect<f32>, brush: impl Into<Material>) {
        self.inner.paint_filled_rect(rect, brush.into())
    }

    pub(super) fn finish(mut self) {
        self.inner.finish();
    }
}
