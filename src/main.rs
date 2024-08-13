// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;
mod gfx;

pub use self::{
    app::*,
    gfx::*,
};

use glium::{backend::glutin::SimpleWindowBuilder, winit::event_loop::EventLoop};

fn main() {
    let event_loop = EventLoop::builder()
        .build()
        .expect("event loop building");

    let (window, display) = SimpleWindowBuilder::new()
        .with_inner_size(1600, 1200)
        .with_title("Zinnebeeld")
        .build(&event_loop);

    let mut app = App {
        window,
        display,
    };

    let _ = event_loop.run_app(&mut app);
}
