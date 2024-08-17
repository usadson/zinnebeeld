// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::{path::Path, rc::Rc};

use euclid::default::{Point2D, Rect, Size2D};
use glium::winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::Context;

pub struct App {
    pub window: Rc<Window>,
    pub context: Context,
}

impl App {
    fn draw(&mut self) {
        let image = self.context.load_image(Path::new("res/test-image.png")).unwrap();

        self.context.paint(|painter| {

            painter.paint_filled_rect(
                Rect::new(
                    Point2D::new(200.0, 200.0),
                    Size2D::new(400.0, 200.0),
                ),
                // crate::Color::YELLOW,
                image,
            );
        });
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        _ = event_loop;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        _ = window_id;

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                self.draw();
            }

            WindowEvent::Resized(size) => {
                let size = size.to_logical(self.window.scale_factor());
                self.context.resize(Size2D::new(size.width, size.height));
            }

            _ => (),
        }
    }

}
