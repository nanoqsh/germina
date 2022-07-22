use crate::render::Connection;
use base::geometry::TextureData;
use wgpu::{BindGroup, BindGroupLayout};

pub struct Texture {
    bind_group: BindGroup,
}

impl Texture {
    pub fn new(connection: &Connection, layout: &BindGroupLayout, data: TextureData) -> Self {
        use std::num::NonZeroU32;
        use wgpu::{
            AddressMode, BindGroupDescriptor, BindGroupEntry, BindingResource, Extent3d,
            FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, SamplerDescriptor,
            TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
            TextureViewDescriptor,
        };

        let (width, height) = data.size;
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = connection.device.create_texture(&TextureDescriptor {
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("texture"),
        });

        connection.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            data.bytes,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(width * 4),
                rows_per_image: NonZeroU32::new(height),
            },
            size,
        );

        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = connection.device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = connection.device.create_bind_group(&BindGroupDescriptor {
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("bind group"),
        });

        Self { bind_group }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
