use bytemuck::{Pod, Zeroable};
use wgpu::{vertex_attr_array, Buffer, Device, VertexAttribute, VertexBufferLayout};

pub struct InternalMesh {
    pub(crate) vertex_buffer: Buffer,
    pub(crate) num_vertices: u32,
}

impl InternalMesh {
    pub fn new(device: &Device, data: &Data) -> Self {
        use wgpu::{
            util::{BufferInitDescriptor, DeviceExt},
            BufferUsages,
        };

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(data.verts),
            usage: BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            num_vertices: data.verts.len() as u32,
        }
    }
}

pub struct Data<'a> {
    pub verts: &'a [Vert],
    pub faces: &'a [Face],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vert {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}

impl Vert {
    const ATTRIBS: [VertexAttribute; 2] = vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn layout() -> VertexBufferLayout<'static> {
        use wgpu::{BufferAddress, VertexStepMode};

        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub type Face = [u16; 3];
