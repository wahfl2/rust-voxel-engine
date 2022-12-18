use guillotiere::{AtlasAllocator, size2, euclid::{Size2D, UnknownUnit}, Allocation};
use image::{DynamicImage, Rgba32FImage, ImageBuffer, Rgba, RgbaImage};
use wgpu::{Device, Extent3d, Texture, Queue};

pub struct TextureAtlas {
    allocator: AtlasAllocator,
    textures: Vec<AllocatedTexture>,
    device_texture: Option<wgpu::Texture>
}

impl TextureAtlas {
    pub fn new() -> Self {
        Self {
            allocator: AtlasAllocator::new(size2(1024, 1024)),
            textures: Vec::new(),
            device_texture: None,
        }
    }

    pub fn add_texture(&mut self, texture: DynamicImage) {
        let tex_size = size2(texture.width() as i32, texture.height() as i32);
        let mut allocation = self.allocator.allocate(tex_size);
        while allocation.is_none() {
            self.allocator.grow(self.allocator.size() * 2);
            allocation = self.allocator.allocate(tex_size);
        }
        self.textures.push(AllocatedTexture::new(allocation.unwrap(), texture));
    }

    pub fn build_atlas(&self) -> RgbaImage {
        let mut img = RgbaImage::new(self.allocator.size().width as u32, self.allocator.size().height as u32);
        for texture in &self.textures {
            texture.write_to_image(&mut img);
        }
        img
    }

    /// Should be called AFTER `get_bind_group_and_layout`
    pub fn write_buffer(&self, queue: &Queue) {
        if self.device_texture.is_none() { return }

        let img = self.build_atlas();
        let size = Extent3d { width: img.width(), height: img.height(), depth_or_array_layers: 1 };

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: self.device_texture.as_ref().unwrap(),
                mip_level: 0,
                origin: wgpu::Origin3d::default(),
                aspect: wgpu::TextureAspect::All,
            }, 
            &img, 
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * size.width),
                rows_per_image: std::num::NonZeroU32::new(size.height),
            }, 
            size
        );
    }

    pub fn get_atlas_pointers(&self) -> Vec<RawAtlasPointer> {
        let mut ret = Vec::new();
        for texture in &self.textures {
            ret.push(texture.alloc.into());
        }
        ret
    }

    pub fn get_bind_group_layout(device: &Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }

    /// Creates a bind group for the texture and returns both the layout and the bind group itself.
    pub fn get_bind_group_and_layout(&mut self, device: &Device) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = Self::get_bind_group_layout(device);

        self.device_texture = Some(device.create_texture(&wgpu::TextureDescriptor {
            size: Extent3d { 
                width: self.allocator.size().width as u32, 
                height: self.allocator.size().height as u32, 
                depth_or_array_layers: 1 
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("texture_atlas"),
        }));

        let view = self.device_texture.as_ref().unwrap().create_view(
            &wgpu::TextureViewDescriptor {
                label: Some("Texture Atlas View"),
                format: Some(wgpu::TextureFormat::Rgba8UnormSrgb),
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                aspect: wgpu::TextureAspect::All,
                base_mip_level: 0,
                mip_level_count: None,
                base_array_layer: 0,
                array_layer_count: None,
            }
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("texture_bind_group"),
            }
        );

        (layout, bind_group)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RawAtlasPointer {
    pub min: [i32; 2],
    pub max: [i32; 2],
}

impl From<Allocation> for RawAtlasPointer {
    fn from(alloc: Allocation) -> Self {
        RawAtlasPointer { 
            min: [alloc.rectangle.min.x, alloc.rectangle.min.y],
            max: [alloc.rectangle.max.x, alloc.rectangle.max.y], 
        }
    }
}

#[derive(Debug, Clone)]
pub struct AllocatedTexture {
    pub alloc: Allocation,
    pub texture: DynamicImage,
}

impl AllocatedTexture {
    pub fn new(alloc: Allocation, texture: DynamicImage) -> Self {
        Self {
            alloc,
            texture,
        }
    }

    fn write_to_image(&self, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let rect = self.alloc.rectangle;

        let x_range = 0u32..rect.width() as u32;
        let y_range = 0u32..rect.height() as u32;

        let x_off = rect.min.x as u32;
        let y_off = rect.min.y as u32;

        let tex = self.texture.to_rgba8();

        for x in x_range {
            for y in y_range.clone() {
                let pixel = tex.get_pixel(x, y).clone();
                img.put_pixel(x_off + x, y_off + y, pixel);
            }
        }
    }
}