use core_client::render::{AsBytes, MeshData, Vert};
use wgpu::{vertex_attr_array, Buffer, Device, VertexAttribute, VertexBufferLayout};

pub struct InternalMesh {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) num_vertices: u32,
}

impl InternalMesh {
    pub fn new(device: &Device, data: &MeshData) -> Self {
        use wgpu::{
            util::{BufferInitDescriptor, DeviceExt},
            BufferUsages,
        };

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: data.verts.as_bytes(),
            usage: BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            num_vertices: data.verts.len() as u32,
        }
    }

    pub fn vert_layout() -> VertexBufferLayout<'static> {
        use std::mem::size_of;
        use wgpu::{BufferAddress, VertexStepMode};

        const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

        VertexBufferLayout {
            array_stride: size_of::<Vert>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}
