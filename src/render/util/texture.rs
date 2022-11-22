use std::{num::NonZeroU32, fs::File, io::Read};

use image::DynamicImage;
use wgpu::{Extent3d, Device};

pub struct TextureArray {
    pub texture: wgpu::Texture,
    pub extent: Extent3d,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub num_layers: u32,
}

impl TextureArray {
    pub fn new(device: &Device, texture_size: Extent3d) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture_array"),
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: Some("Texture Array View"),
            format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: NonZeroU32::new(texture_size.depth_or_array_layers),
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            texture,
            extent: texture_size,
            view,
            sampler,
            num_layers: 0,
        }
    }

    pub fn push_image(&mut self,
        queue: &wgpu::Queue,
        image: DynamicImage,
    ) {
        let rgba8 = image.to_rgba8();

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: 0, y: 0, z: self.num_layers },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba8,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.extent.width),
                rows_per_image: std::num::NonZeroU32::new(self.extent.height),
            },
            self.extent,
        );

        self.num_layers += 1;
    }

    pub fn push_image_path(&mut self,
        queue: &wgpu::Queue,
        path: &str,
    ) {
        let path = format!("assets/{}", path);
        let file = File::open(path).unwrap();
        let image = image::load_from_memory(
            file.bytes().map(|b| { b.unwrap() })
            .collect::<Vec<_>>().as_slice()
        ).unwrap();

        self.push_image(queue, image);
    }
}