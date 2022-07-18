use crate::{mesh::Mesh as InternalMesh, storage::Storage, texture::Texture as InternalTexture};
use core_client::render::{MeshData, Size, TextureData};
use raw_window_handle::HasRawWindowHandle;
use wgpu::{
    BindGroupLayout, Device, Queue, RenderPass, RenderPipeline, Surface, SurfaceConfiguration,
    SurfaceError,
};

pub struct Render {
    surface: Surface,
    surface_config: SurfaceConfiguration,
    connection: Connection,
    pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    resources: Resources,
}

impl Render {
    pub async fn new<W>(window: &'static W) -> Self
    where
        W: HasRawWindowHandle,
    {
        use wgpu::{
            Backends, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState,
            ColorTargetState, ColorWrites, DeviceDescriptor, Face, Features, FragmentState,
            FrontFace, Instance, Limits, MultisampleState, PipelineLayoutDescriptor, PolygonMode,
            PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology,
            RenderPipelineDescriptor, RequestAdapterOptions, SamplerBindingType,
            ShaderModuleDescriptor, ShaderSource, ShaderStages, TextureSampleType, TextureUsages,
            TextureViewDimension, VertexState,
        };

        let instance = Instance::new(Backends::all());
        let surface = unsafe {
            // # Safety
            // - The window handle lifes for the 'static time
            //   and hence long enough to use a surface.
            instance.create_surface(window)
        };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("request adapter");
        log::debug!("adapter: {adapter:?}");

        let connection = {
            let (device, queue) = adapter
                .request_device(
                    &DeviceDescriptor {
                        features: Features::empty(),
                        limits: Limits::default(),
                        label: None,
                    },
                    None,
                )
                .await
                .expect("request device");

            Connection { device, queue }
        };

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: 0,
            height: 0,
            present_mode: PresentMode::Fifo,
        };

        let shader = connection
            .device
            .create_shader_module(ShaderModuleDescriptor {
                label: Some("shader"),
                source: ShaderSource::Wgsl(include_str!("../shaders/def.wgsl").into()),
            });

        let bind_group_layout =
            connection
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                multisampled: false,
                                view_dimension: TextureViewDimension::D2,
                                sample_type: TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Sampler(SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                    label: Some("bind group layout"),
                });

        let pipeline_layout = connection
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("pipeline layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = connection
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[InternalMesh::layout()],
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(ColorTargetState {
                        format: surface_config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        Self {
            surface,
            surface_config,
            connection,
            pipeline,
            bind_group_layout,
            resources: Resources::default(),
        }
    }

    fn resize(&mut self, size: Option<Size>) {
        if let Some((width, height)) = size {
            self.surface_config.width = width.get();
            self.surface_config.height = height.get();
        }

        self.surface
            .configure(&self.connection.device, &self.surface_config);
    }

    fn draw_frame<D>(&mut self, draw_fn: D)
    where
        D: FnOnce(&mut Frame),
    {
        use wgpu::{
            Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
            RenderPassDescriptor, TextureViewDescriptor,
        };

        let output = loop {
            match self.surface.get_current_texture() {
                Ok(output) => break output,
                Err(SurfaceError::Lost) => self.resize(None),
                Err(SurfaceError::OutOfMemory) => panic!("out of memory"),
                Err(err) => panic!("{err:?}"),
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder =
            self.connection
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("render encoder"),
                });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            pass.set_pipeline(&self.pipeline);

            let mut frame = Frame {
                pass,
                resources: &self.resources,
            };
            draw_fn(&mut frame);
        }

        self.connection.queue.submit([encoder.finish()]);
        output.present();
    }
}

pub struct Connection {
    pub device: Device,
    pub queue: Queue,
}

/// The struct represented a current frame
/// and exists during a frame render.
///
/// It has an drawing functions which calls by the engine.
pub struct Frame<'d> {
    pass: RenderPass<'d>,
    resources: &'d Resources,
}

impl<'d> Frame<'d> {
    pub fn bind_texture(&mut self, texture: Texture) {
        let texture = self.resources.textures.get(texture.0);
        self.pass.set_bind_group(0, texture.bind_group(), &[]);
    }

    pub fn draw_mesh(&mut self, mesh: Mesh) {
        use wgpu::IndexFormat;

        let mesh = self.resources.meshes.get(mesh.0);
        self.pass
            .set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        self.pass
            .set_index_buffer(mesh.index_buffer().slice(..), IndexFormat::Uint16);
        self.pass.draw_indexed(0..mesh.num_indices(), 0, 0..1);
    }
}

// A temporary struct wrapper arount the Render.
//
// TODO: Remove.
pub struct ClientRender {
    render: Render,
}

impl ClientRender {
    pub fn new(render: Render) -> Self {
        Self { render }
    }

    pub fn make_mesh(&mut self, data: MeshData) -> Mesh {
        let mesh = InternalMesh::new(&self.render.connection, data);
        let id = self.render.resources.meshes.insert(mesh);
        Mesh(id)
    }

    pub fn delete_mesh(&mut self, mesh: Mesh) {
        self.render.resources.meshes.remove(mesh.0);
    }

    pub fn make_texture(&mut self, data: TextureData) -> Texture {
        let texture = InternalTexture::new(
            &self.render.connection,
            &self.render.bind_group_layout,
            data,
        );
        let id = self.render.resources.textures.insert(texture);
        Texture(id)
    }

    pub fn delete_texture(&mut self, texture: Texture) {
        self.render.resources.textures.remove(texture.0);
    }

    pub fn draw_frame<D>(&mut self, draw_fn: D)
    where
        D: FnOnce(&mut Frame),
    {
        self.render.draw_frame(draw_fn);
    }

    pub fn resize(&mut self, size: Size) {
        self.render.resize(Some(size));
    }
}

#[derive(Clone, Copy)]
pub struct Mesh(u32);

#[derive(Clone, Copy)]
pub struct Texture(u32);

#[derive(Default)]
pub struct Resources {
    pub meshes: Storage<InternalMesh>,
    pub textures: Storage<InternalTexture>,
}
