use crate::{mesh::Mesh, texture::Texture};
use client_core::render::{Render as ClientRender, Size};
use raw_window_handle::HasRawWindowHandle;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceError};

pub struct Render {
    surface: Surface,
    surface_config: SurfaceConfiguration,
    device: Device,
    queue: Queue,
}

impl Render {
    pub async fn new<W>(window: &'static W) -> Self
    where
        W: HasRawWindowHandle,
    {
        use wgpu::{
            Backends, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode,
            RequestAdapterOptions, TextureUsages,
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

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: 0,
            height: 0,
            present_mode: PresentMode::Fifo,
        };

        Self {
            surface,
            surface_config,
            device,
            queue,
        }
    }

    fn resize(&mut self, size: Option<Size>) {
        if let Some((width, height)) = size {
            self.surface_config.width = width.get();
            self.surface_config.height = height.get();
        }

        self.surface.configure(&self.device, &self.surface_config);
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        use wgpu::{
            Color, CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment,
            RenderPassDescriptor, TextureViewDescriptor,
        };

        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("render encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render pass"),
                color_attachments: &[RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit([encoder.finish()]);
        output.present();

        Ok(())
    }
}

impl ClientRender for Render {
    type Mesh = Mesh;
    type Texture = Texture;

    fn resize(&mut self, size: Size) {
        self.resize(Some(size));
    }

    fn start_frame(&mut self) {
        match self.render() {
            Ok(()) => {}
            Err(SurfaceError::Lost) => self.resize(None),
            Err(SurfaceError::OutOfMemory) => panic!("out of memory"),
            Err(err) => log::error!("{err:?}"),
        }
    }

    fn end_frame(&mut self) {}

    fn make_mesh(&mut self) -> Self::Mesh {
        todo!()
    }

    fn draw_mesh(&mut self, _: &Self::Mesh) {
        todo!()
    }

    fn make_texture(&mut self) -> Self::Texture {
        todo!()
    }

    fn bind_texture(&mut self, _: &Self::Texture) {
        todo!()
    }
}
