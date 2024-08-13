// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use glium::{glutin::surface::WindowSurface, index::{IndicesSource, NoIndices, PrimitiveType}, Display, IndexBuffer, VertexBuffer};

use crate::Vertex;

pub struct Mesh {
    vbo: VertexBuffer<Vertex>,
    ibo: MeshIndices,
}

impl Mesh {
    pub fn new_square(display: &Display<WindowSurface>) -> Self {
        let val = 0.5;
        let shape = [
            Vertex { position: [ val,  val]},
            Vertex { position: [ val, -val]},
            Vertex { position: [-val, -val]},
            Vertex { position: [-val,  val]},
        ];
        Self::new(display, &shape, &[
            0, 1, 2,
            0, 2, 3
        ])
    }

    pub fn new(display: &Display<WindowSurface>, vertices: &[Vertex], indices: &[u16]) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(display, vertices).unwrap();
        let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, indices).unwrap();

        Self {
            vbo: vertex_buffer,
            ibo: MeshIndices::Buffer(indices),
        }
    }

    #[must_use]
    pub fn vbo(&self) -> &VertexBuffer<Vertex> {
        &self.vbo
    }

    #[must_use]
    pub fn ibo(&self) -> &MeshIndices {
        &self.ibo
    }
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
