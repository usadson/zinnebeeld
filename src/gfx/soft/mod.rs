// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod painter;

use std::{cell::RefCell, num::NonZero, path::Path, rc::Rc};

use euclid::default::Size2D;
use glium::winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};
use image::RgbaImage;
use painter::SoftwarePainter;
use softbuffer::Surface;

use crate::{EventTy, ImageLoadError, ResourceManager, ResourceNamespace};

use super::{painter::PainterImplementation, ContextImplementation, Image};

type SoftwareSurface = Rc<RefCell<Surface<Rc<Window>, Rc<Window>>>>;

/// A software renderer [`Context`].
pub struct SoftwareContext {
    window: Rc<Window>,
    surface: SoftwareSurface,
    resources: Rc<SoftwareResources>,
}

impl SoftwareContext {
    pub fn new(event_loop: &EventLoop<EventTy>) -> (Box<dyn ContextImplementation>, Rc<Window>) {
        let attr = Window::default_attributes()
            .with_inner_size(PhysicalSize::new(1600, 1200))
            .with_title("Zinnebeeld");

        #[allow(deprecated)]
        let window = event_loop.create_window(attr).expect("Failed to create window");
        let window = Rc::new(window);

        let context = softbuffer::Context::new(window.clone()).unwrap();

        let surface = Surface::new(&context, window.clone()).unwrap();
        let surface = Rc::new(RefCell::new(surface));

        let this = Self {
            window: Rc::clone(&window),
            surface,
            resources: SoftwareResources::new(),
        };

        (Box::new(this), window)
    }

    fn set_size(&self, size: Size2D<u32>) {
        let Some(width) = NonZero::new(size.width) else {
            return;
        };

        let Some(height) = NonZero::new(size.height) else {
            return;
        };

        self.surface.borrow_mut().resize(width, height).unwrap();
    }

    fn get_size_from_window(&self) -> Size2D<u32> {
        let size = self.window.inner_size(); //.to_logical(self.window.scale_factor());
        Size2D::new(size.width, size.height)
    }
}

impl ContextImplementation for SoftwareContext {
    fn resize(&mut self, size: Size2D<u32>) {
        self.set_size(size);
    }

    fn load_image(&mut self, path: &Path) -> Result<Image, ImageLoadError> {
        let (img, size) = Image::load(path)?;
        let id = self.resources.images.add(img);

        Ok(Image {
            id,
            size,
        })
    }

    fn paint_frame(&self, f: &mut dyn FnMut(&mut dyn PainterImplementation)) {
        let size = self.get_size_from_window();
        self.set_size(size);

        let mut surface = self.surface.borrow_mut();
        let buffer = surface.buffer_mut().unwrap();

        let mut painter = SoftwarePainter::new(size, self.window.scale_factor(), buffer, Rc::clone(&self.resources));

        f(&mut painter);

        painter.finish();
    }

}

struct SoftwareResources {
    images: ResourceManager<RgbaImage>,
}

impl SoftwareResources {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            images: ResourceManager::new(ResourceNamespace::Image),
        })
    }
}
