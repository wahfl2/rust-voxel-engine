use wgpu::{include_wgsl, util::DeviceExt, Extent3d};
use winit::{window::Window, event::WindowEvent};

use super::{util::{vertex::*, cube_model::{CubeModel, self}, texture::TextureArray, texture_atlas::TextureAtlas}, camera::{Camera, CameraUniform}, face_lighting::{FaceLightingUniform, FaceLighting}};

pub struct RenderState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    texture_atlas: TextureAtlas,
    pub camera: Camera,
    camera_bind_group: wgpu::BindGroup,
    pub face_lighting: FaceLighting,
    face_lighting_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    // instance_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pub num_indices: u32,
}

impl RenderState {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::TEXTURE_BINDING_ARRAY,
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(include_wgsl!("../shader/shader.wgsl"));

        let model = CubeModel::default();
        let mut model_verts = Vec::new();

        for quad in cube_model::DEFAULT_CUBE_MODEL_QUADS.iter() {
            for v in quad.get_vertices() {
                model_verts.push(VertexRaw::from(v));
            }
        }

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(model_verts.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let mut texture_atlas = TextureAtlas::new();

        for dyn_tex in model.textures {
            texture_atlas.add_texture(dyn_tex);
        }

        let (texture_bind_group_layout, texture_bind_group) = 
            texture_atlas.get_bind_group_and_layout(&device);

        texture_atlas.write_buffer(&queue);

        let mut camera = Camera::default();
        camera.aspect = config.width as f32 / config.height as f32;

        let (camera_bind_group_layout, camera_bind_group) = 
            camera.get_bind_group_and_layout(&device);

        let mut face_lighting = FaceLighting::new(1.0, 0.25, 0.5, 0.75);
        let (face_lighting_bind_group_layout, face_lighting_bind_group) =
            face_lighting.get_bind_group_and_layout(&device);

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera_bind_group_layout,
                    &face_lighting_bind_group_layout,
                ],
                push_constant_ranges: &[],
            }
        );

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    VertexRaw::desc(),
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            texture_atlas,
            camera,
            camera_bind_group,
            face_lighting,
            face_lighting_bind_group,
            render_pipeline,
            vertex_buffer,
            bind_group: texture_bind_group,
            num_indices: model_verts.len() as u32,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32
        }
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        self.queue.write_buffer(
            self.camera.buffer.as_ref().unwrap(), 
            0, 
            bytemuck::cast_slice(&[CameraUniform::from(&self.camera)]),
        );

        if self.face_lighting.changed {
            self.queue.write_buffer(
                self.face_lighting.buffer.as_ref().unwrap(),
                0,
                bytemuck::cast_slice(&[FaceLightingUniform::from(&self.face_lighting)]),
            );

            self.face_lighting.changed = false;
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.1,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        }
                    })
                ],
                depth_stencil_attachment: None,
            });
        
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.face_lighting_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.num_indices, 0..1);
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}