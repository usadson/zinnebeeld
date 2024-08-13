// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use glium::{glutin::surface::WindowSurface, Display, Program};

const SOLID_COLOR_VERTEX_SHADER: &str = include_str!("../../res/solid_color_vertex.glsl");
const SOLID_COLOR_FRAGMENT_SHADER: &str = include_str!("../../res/solid_color_fragment.glsl");

pub struct ShaderPrograms;

impl ShaderPrograms {
    pub fn create_solid_color(display: &Display<WindowSurface>) -> Program {
        Program::from_source(display, SOLID_COLOR_VERTEX_SHADER, SOLID_COLOR_FRAGMENT_SHADER, None).unwrap()
    }
}
