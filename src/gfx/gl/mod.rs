// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{path::Path, rc::Rc};

use euclid::default::Size2D;
use glium::{backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, texture::RawImage2d, uniforms::{AsUniformValue, UniformValue}, winit::{event_loop::EventLoop, window::Window}, Display, Texture2d};
use painter::GLPainter;

use crate::{
    Color,
    ContextImplementation,
    EventTy,
    Image,
    ImageLoadError,
    ResourceManager,
    ResourceNamespace,
};

use super::painter::PainterImplementation;

mod painter;

pub struct GLContext {
    display: Display<WindowSurface>,
    resources: Rc<GLResources>,
}

impl GLContext {
    pub fn new(event_loop: &EventLoop<EventTy>) -> (Box<dyn ContextImplementation>, Rc<Window>) {
        let (window, display) = SimpleWindowBuilder::new()
            .with_inner_size(1600, 1200)
            .with_title("Zinnebeeld")
            .build(&event_loop);

        let this = Self {
            display,
            resources: Rc::new(GLResources::new()),
        };

        (Box::new(this), Rc::new(window))
    }
}

impl ContextImplementation for GLContext {
    fn resize(&mut self, size: Size2D<u32>) {
        self.display.resize((size.width, size.height));
    }

    fn load_image(&mut self, path: &Path) -> Result<Image, ImageLoadError> {
        let (img, size) = Image::load(path)?;
        let dimensions = img.dimensions();

        let img = RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dimensions);
        let texture = glium::texture::Texture2d::new(&self.display, img)?;
        let id = self.resources.images.add(texture);
        Ok(Image {
            id,
            size,
        })
    }

    fn paint_frame(&self, f: &mut dyn FnMut(&mut dyn PainterImplementation)) {
        let mut painter = GLPainter::new(self.display.clone(), Rc::clone(&self.resources));

        f(&mut painter);

        painter.finish();
    }
}

struct GLResources {
    images: ResourceManager<Texture2d>,
}

impl GLResources {
    pub fn new() -> Self {
        Self {
            images: ResourceManager::new(ResourceNamespace::Image),
        }
    }
}

impl AsUniformValue for Color {
    fn as_uniform_value(&self) -> UniformValue<'_> {
        UniformValue::Vec4(self.to_f32_rgba())
    }
}
