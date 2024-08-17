// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{path::Path, rc::Rc};

use euclid::default::Size2D;
use glium::winit::{event_loop::EventLoop, window::Window};

use crate::{EventTy, GLContext, Image, ImageLoadError, Painter};

pub trait ContextImplementation {
    fn resize(&mut self, size: Size2D<u32>);

    fn load_image(&mut self, path: &Path) -> Result<Image, ImageLoadError>;

    fn paint_frame(&self) -> Painter;
}

pub struct Context {
    inner: Box<dyn ContextImplementation>,
}

impl Context {
    pub fn new(event_loop: &EventLoop<EventTy>) -> (Self, Rc<Window>) {
        let (inner, window) = GLContext::new(event_loop);

        let this = Self {
            inner: Box::new(inner),
        };

        (this, window)
    }

    pub fn load_image(&mut self, path: &Path) -> Result<Image, ImageLoadError> {
        self.inner.load_image(path)
    }

    pub fn paint<F: FnOnce(&mut Painter)>(&self, f: F) {
        let mut painter = self.inner.paint_frame();

        f(&mut painter);

        painter.finish();
    }

    pub fn resize(&mut self, size: Size2D<u32>) {
        self.inner.resize(size);
    }
}
