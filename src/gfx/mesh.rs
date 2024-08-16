// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use glium::{glutin::surface::WindowSurface, index::{IndicesSource, NoIndices, PrimitiveType}, uniforms::Uniforms, Display, Frame, IndexBuffer, Program, Surface, VertexBuffer};

use crate::Vertex;

use crate::TexturedVertex;

pub struct Mesh {
    vbo: MeshVertexBuffer,
    ibo: MeshIndices,
}

impl Mesh {
    pub fn new_square(display: &Display<WindowSurface>) -> Self {
        let val = 0.5;
        let vertices = [
            Vertex { position: [ val,  val]},
            Vertex { position: [ val, -val]},
            Vertex { position: [-val, -val]},
            Vertex { position: [-val,  val]},
        ];
        let indices = &[
            0, 1, 2,
            0, 2, 3
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap();

        Self {
            vbo: MeshVertexBuffer::Normal(vertex_buffer),
            ibo: MeshIndices::Buffer(indices),
        }
    }

    pub fn new_textured_square(display: &Display<WindowSurface>) -> Self {
        let val = 0.5;
        let vertices = [
            TexturedVertex { position: [ val,  val], tex_coords: [ 1.0,  1.0] },
            TexturedVertex { position: [ val, -val], tex_coords: [ 1.0, -1.0] },
            TexturedVertex { position: [-val, -val], tex_coords: [-1.0, -1.0] },
            TexturedVertex { position: [-val,  val], tex_coords: [-1.0,  1.0] },
        ];
        let indices = &[
            0, 1, 2,
            0, 2, 3
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &vertices).unwrap();
        let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap();

        Self {
            vbo: MeshVertexBuffer::Textured(vertex_buffer),
            ibo: MeshIndices::Buffer(indices),
        }
    }

    pub fn draw<U>(&self, target: &mut Frame, program: &Program, uniforms: &U)
            where U: Uniforms {
        match &self.vbo {
            MeshVertexBuffer::Normal(vbo) => {
                target.draw(vbo, &self.ibo, &program, uniforms, &Default::default()).unwrap();
            }

            MeshVertexBuffer::Textured(vbo) => {
                target.draw(vbo, &self.ibo, &program, uniforms, &Default::default()).unwrap();
            }
        }
    }
}

pub enum MeshVertexBuffer {
    Normal(VertexBuffer<Vertex>),
    Textured(VertexBuffer<TexturedVertex>),
}

pub enum MeshIndices {
    Buffer(IndexBuffer<u16>),
    NoIndicies(NoIndices),
}

impl MeshIndices {
    pub fn none(ty: PrimitiveType) -> MeshIndices {
        Self::NoIndicies(NoIndices(ty))
    }
}

impl<'a> From<&'a MeshIndices> for IndicesSource<'a> {
    fn from(indices: &'a MeshIndices) -> Self {
        match indices {
            MeshIndices::Buffer(buf) => buf.into(),
            MeshIndices::NoIndicies(ibo) => ibo.into(),
        }
    }
}
