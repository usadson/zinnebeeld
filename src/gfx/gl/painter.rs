// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::rc::Rc;

use euclid::default::{Rect, Size2D, Transform3D, Vector3D};
use glium::{glutin::surface::WindowSurface, uniform, Display, Frame, Surface};

use crate::{gfx::painter::PainterImplementation, Material, Mesh, ShaderPrograms};

use super::GLResources;

pub struct GLPainter {
    target: Frame,
    target_size: Size2D<f32>,
    display: Display<WindowSurface>,
    resources: Rc<GLResources>,
}

impl GLPainter {
    pub fn new(display: Display<WindowSurface>, resources: Rc<GLResources>) -> Self {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let (width, height) = display.get_framebuffer_dimensions();
        Self {
            target,
            target_size: Size2D::new(width as _, height as _),
            display,
            resources,
        }
    }

    pub fn finish(self) {
        self.target.finish().unwrap();
    }
}

impl PainterImplementation for GLPainter {
    fn paint_filled_rect(&mut self, rect: Rect<f32>, brush: Material) {
        let x_scale = rect.width() / self.target_size.width;
        let y_scale = rect.height() / self.target_size.height;

        let matrix = Transform3D::identity()
            .then_translate(Vector3D::new(rect.min_x() / rect.width(), -rect.min_y() / rect.height(), 0.0))
            .then_scale(x_scale, y_scale, 1.0)
            .then_translate(Vector3D::new(x_scale / 2.0 - 1.0, 1.0 - y_scale / 2.0, 0.0))
            .to_arrays();

        match brush {
            Material::Color(color) => {
                let mesh = Mesh::new_square(&self.display);
                let program = ShaderPrograms::create_solid_color(&self.display);

                let uniforms = uniform! {
                    matrix: matrix,
                    color: color,
                };
                mesh.draw(&mut self.target, &program, &uniforms);
            }
            Material::Image(image) => {
                let mesh = Mesh::new_textured_square(&self.display);
                let program = ShaderPrograms::create_textured(&self.display);

                self.resources.images.with(image.id, |tex| {
                    let uniforms = uniform! {
                        matrix: matrix,
                        tex: tex,
                    };

                    mesh.draw(&mut self.target, &program, &uniforms);
                });
            }
        };
    }
}
