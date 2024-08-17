// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{env::var, path::Path, rc::Rc};

use euclid::default::Size2D;
use glium::winit::{event_loop::EventLoop, window::Window};

use crate::{EventTy, GLContext, Image, ImageLoadError, Painter};

use super::{painter::PainterImplementation, soft::SoftwareContext};

pub trait ContextImplementation {
    fn resize(&mut self, size: Size2D<u32>);

    fn load_image(&mut self, path: &Path) -> Result<Image, ImageLoadError>;

    fn paint_frame(&self, f: &mut dyn FnMut(&mut dyn PainterImplementation));
}

pub struct Context {
    inner: Box<dyn ContextImplementation>,
}

impl Context {
    pub fn new(event_loop: &EventLoop<EventTy>) -> (Self, Rc<Window>) {
        let ctx = var("ZINNEBEELD_CTX").ok().unwrap_or_default();
        let (inner, window) = match ctx.as_str() {
            "software" => SoftwareContext::new(event_loop),
            _ => GLContext::new(event_loop),
        };

        let this = Self {
            inner,
        };

        (this, window)
    }

    pub fn load_image(&mut self, path: &Path) -> Result<Image, ImageLoadError> {
        self.inner.load_image(path)
    }

    pub fn paint<F: FnMut(&mut Painter)>(&self, mut f: F) {
        self.inner.paint_frame(&mut |painter| {
            let mut painter = Painter {
                inner: painter,
            };

            f(&mut painter);
        });
    }

    pub fn resize(&mut self, size: Size2D<u32>) {
        self.inner.resize(size);
    }
}
