// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::rc::Rc;

use euclid::default::{Point2D, Rect, Size2D};
use glium::winit::window::Window;
use image::Rgba;
use softbuffer::Buffer;

use crate::{gfx::painter::PainterImplementation, Color, Image, Material};

use super::SoftwareResources;

pub(super) struct SoftwarePainter<'ctx> {
    size: Size2D<u32>,
    scale_factor: f64,
    buffer: Buffer<'ctx, Rc<Window>, Rc<Window>>,
    resources: Rc<SoftwareResources>,
}

impl<'ctx> SoftwarePainter<'ctx> {
    pub fn new(
        size: Size2D<u32>,
        scale_factor: f64,
        buffer: Buffer<'ctx, Rc<Window>, Rc<Window>>,
        resources: Rc<SoftwareResources>,
    ) -> Self {
        let mut this = Self {
            size,
            scale_factor: 1.0 / scale_factor,
            buffer,
            resources,
        };

        this.paint_filled_rect(Rect::new(Point2D::zero(), size.cast()), Color::BLACK.into());

        this
    }

    pub fn finish(self) {
        self.buffer.present().unwrap();
    }

    fn paint_rect_with_color(&mut self, rect: Rect<usize>, color: Color) {
        let color = color.as_bgra();

        for y in rect.y_range() {
            let offset = y * self.size.width as usize;

            let start = offset + rect.min_x();
            let end = offset + rect.max_x();

            self.buffer[start..end].fill(color);
        }
    }

    fn paint_rect_with_image(&mut self, rect: Rect<usize>, image: Image) {
        self.resources.images.with(image.id, |image| {
            for y in rect.y_range() {
                let line_offset = y * self.size.width as usize;

                for x in rect.x_range() {
                    let offset = line_offset + x;

                    let u = (x - rect.min_x()) as f64 / rect.width() as f64;
                    let v = (y - rect.min_y()) as f64 / rect.width() as f64;
                    let u = (u * image.width() as f64) as u32;
                    let v = (v * image.height() as f64) as u32;
                    let pixel: Color = image.get_pixel(u, v).into();

                    self.buffer[offset] = pixel.as_bgra();
                }
            }
        });
    }
}

impl<'ctx> PainterImplementation for SoftwarePainter<'ctx> {
    fn paint_filled_rect(&mut self, rect: Rect<f32>, brush: Material) {
        let rect = rect.cast::<f64>().scale(self.scale_factor, self.scale_factor);

        let rect = Rect::new(
            Point2D::new(
                (rect.min_x().round() as usize).max(0),
                (rect.min_y().round() as usize).max(0),
            ),
            Size2D::new(
                (rect.width().round() as usize).min(self.size.width as usize - 1),
                (rect.height().round() as usize).min(self.size.height as usize - 1),
            ),
        );

        match brush {
            Material::Color(color) => self.paint_rect_with_color(rect, color),
            Material::Image(image) => self.paint_rect_with_image(rect, image),
        }
    }
}

impl From<&Rgba<u8>> for Color {
    fn from(value: &Rgba<u8>) -> Self {
        Self::new(value.0[0], value.0[1], value.0[2], value.0[3])
    }
}
