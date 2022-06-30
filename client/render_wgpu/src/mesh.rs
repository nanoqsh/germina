use crate::render::Connection;
use core_client::render::{AsBytes, MeshData, Vert};
use wgpu::{vertex_attr_array, Buffer, VertexAttribute, VertexBufferLayout};

pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    num_indices: u32,
}

impl Mesh {
    pub fn new(connection: &Connection, data: MeshData) -> Self {
        use wgpu::{
            util::{BufferInitDescriptor, DeviceExt},
            BufferUsages,
        };

        let vertex_buffer = connection.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: data.verts.as_bytes(),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = connection.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("index buffer"),
            contents: data.faces.as_bytes(),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: data.faces.len() as u32 * 3,
        }
    }

    pub fn layout() -> VertexBufferLayout<'static> {
        use std::mem::size_of;
        use wgpu::{BufferAddress, VertexStepMode};

        const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

        VertexBufferLayout {
            array_stride: size_of::<Vert>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }

    pub fn vertex_buffer(&self) -> &Buffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &Buffer {
        &self.index_buffer
    }

    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}
