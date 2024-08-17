// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

mod app;
mod error;
mod gfx;
mod resource;

pub type EventTy = ();

pub use self::{
    app::*,
    error::*,
    gfx::*,
    resource::*,
};

use glium::winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::builder()
        .build()
        .expect("event loop building");

    let (context, window) = Context::new(&event_loop);

    let mut app = App {
        window,
        context,
    };

    let _ = event_loop.run_app(&mut app);
}
