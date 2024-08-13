// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use euclid::default::{Point2D, Rect, Size2D};
use glium::{
    glutin::surface::WindowSurface,
    winit::{
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::ActiveEventLoop,
        window::{Window, WindowId},
    },
    Display,
};

use crate::{Color, Painter};

pub struct App {
    pub window: Window,
    pub display: Display<WindowSurface>,
}
impl App {
    fn draw(&self) {
        let mut painter = Painter::new(self.display.clone());
        painter.paint_filled_rect(
            Rect::new(
                Point2D::new(200.0, 200.0),
                Size2D::new(400.0, 200.0),
            ),
            Color::YELLOW,
        );
        painter.finish();
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
                self.display.resize((size.width, size.height));
            }

            _ => (),
        }
    }

}
