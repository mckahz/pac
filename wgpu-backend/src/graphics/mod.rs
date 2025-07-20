use bytemuck;
use image::GenericImageView;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wgpu::util::DeviceExt;
use winit::{dpi::PhysicalSize, event::*};

use crate::{math, Error, Result};

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    size: PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    instance_buffer: wgpu::Buffer,

    screen_buffer: wgpu::Buffer,
    screen_bind_group: wgpu::BindGroup,

    virtual_screen: Size,
    virtual_screen_buffer: wgpu::Buffer,
    virtual_screen_bind_group: wgpu::BindGroup,

    // TODO: decouple the rest from State
    camera: Camera,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

pub struct Sprite {
    pub image: Image,
    pub origin: Origin,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Image {
    rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    width: u32,
    height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// HACK: f32 because wgpu doesn't seem to like unsigned integers, especially not u16's
struct Size {
    width: f32,
    height: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub position: [f32; 2],
    pub zoom: f32,
    pub rotation: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureAtlas {
    pub tile_width: u32,
    pub tile_height: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub spacing_x: u32,
    pub spacing_y: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Origin {
    x: f32,
    y: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub uv: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub position: [f32; 2],
    pub origin: [f32; 2],
    pub depth: f32,
    pub rotation: f32,
    pub scale: f32,
    pub frame: u32,
}

pub const QUAD_VERTICES: [Vertex; 6] = [
    Vertex::from([0.0, 1.0]),
    Vertex::from([1.0, 0.0]),
    Vertex::from([1.0, 1.0]),
    Vertex::from([0.0, 1.0]),
    Vertex::from([0.0, 0.0]),
    Vertex::from([1.0, 0.0]),
];

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x2];

    pub const fn from(uv: [f32; 2]) -> Self {
        Self { uv }
    }

    pub fn description() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<f32>() * 2) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Default for TextureAtlas {
    fn default() -> Self {
        Self {
            tile_width: 1,
            tile_height: 1,
            offset_x: 0,
            offset_y: 0,
            spacing_x: 0,
            spacing_y: 0,
        }
    }
}

impl Instance {
    pub const ATTRIBUTES: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array!
    [ 1 => Float32x2
    , 2 => Float32x2
    , 3 => Float32
    , 4 => Float32
    , 5 => Float32
    , 6 => Uint32
    ];

    pub fn description() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl Origin {
    pub const LEFT: Self = Self { x: 0.0, y: 0.5 };
    pub const RIGHT: Self = Self { x: 1.0, y: 0.5 };
    pub const TOP: Self = Self { x: 0.5, y: 0.0 };
    pub const BOTTOM: Self = Self { x: 0.5, y: 1.0 };

    pub const TOP_LEFT: Self = Self { x: 0.0, y: 0.0 };
    pub const TOP_RIGHT: Self = Self { x: 1.0, y: 0.0 };
    pub const BOTTOM_LEFT: Self = Self { x: 0.0, y: 1.0 };
    pub const BOTTOM_RIGHT: Self = Self { x: 1.0, y: 1.0 };

    pub fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            zoom: 1.0,
            rotation: 0.0,
        }
    }
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &crate::window::Window) -> Self {
        // winit
        let size = window.size;

        // NOTE: WGPU Stuff!
        let wgpu = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL, //::all(),
            ..Default::default()
        });

        let surface = unsafe { wgpu.create_surface(&window.winit) }.unwrap();

        let adapter = wgpu
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,

            format: surface_format,
            // TODO: add resiliance so this won't crash when either is <= 0
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // NOTE: Texture!
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX, // TODO: Do we need this?
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
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // NOTE: Camera!
        let camera = Camera::new();

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // NOTE: Size struct description
        let size_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("size_bind_group_layout"),
            });

        // NOTE: Screen
        let screen_size = Size {
            width: size.width as f32,
            height: size.height as f32,
        };
        let screen_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Screen Buffer"),
            contents: bytemuck::cast_slice(&[screen_size]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let screen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &size_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buffer.as_entire_binding(),
            }],
            label: Some("screen_bind_group"),
        });

        // NOTE: Virtual Screen
        let virtual_screen = Size {
            width: 240.0,
            height: 224.0,
        };

        let virtual_screen_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Virtual Screen Buffer"),
            contents: bytemuck::cast_slice(&[virtual_screen]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let virtual_screen_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &size_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: virtual_screen_buffer.as_entire_binding(),
            }],
            label: Some("screen_bind_group"),
        });

        // NOTE: Shader!
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &size_bind_group_layout,
                    &size_bind_group_layout,
                    &camera_bind_group_layout,
                    &texture_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::description(), Instance::description()],
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
                strip_index_format: None, //TODO: make this a triangle strip to reduce vertex count
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill, // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                unclipped_depth: false,                // Requires Features::DEPTH_CLIP_CONTROL
                conservative: false, // Requires Features::CONSERVATIVE_RASTERIZATION
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // NOTE: Vertex Buffer!
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&QUAD_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        #[rustfmt::skip]
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            size: std::mem::size_of::<Instance>() as u64 * 256,
            mapped_at_creation: false,
        });

        // NOTE: Return!
        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,

            vertex_buffer,
            instance_buffer,

            texture_bind_group_layout,

            camera,
            camera_buffer,
            camera_bind_group,

            screen_buffer,
            screen_bind_group,

            virtual_screen,
            virtual_screen_buffer,
            virtual_screen_bind_group,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width <= 0 || new_size.height <= 0 {
            return;
        }

        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);

        self.queue.write_buffer(
            &self.screen_buffer,
            0,
            bytemuck::cast_slice(&[self.size.width as f32, self.size.height as f32]),
        );
    }

    pub fn render(&mut self, sprite_instances: &[(&Sprite, Vec<Instance>)]) -> Result<()> {
        let output = self.surface.get_current_texture().map_err(Error::WGPU)?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut bind_groups: Vec<Rc<wgpu::BindGroup>> = vec![];

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Pixel Art Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.screen_bind_group, &[]);
        render_pass.set_bind_group(1, &self.virtual_screen_bind_group, &[]);
        render_pass.set_bind_group(2, &self.camera_bind_group, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        sprite_instances.iter().for_each(|(sprite, instances)| {
            self.queue
                .write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));

            let bind_group = Rc::new(
                self.load_texture(
                    &sprite.image,
                    TextureAtlas {
                        tile_width: 15,
                        tile_height: 5,
                        offset_x: 0,
                        offset_y: 0,
                        spacing_x: 0,
                        spacing_y: 0,
                    },
                )
                .unwrap(),
            );

            bind_groups.push(bind_group);

            render_pass.set_bind_group(3, &bind_groups.iter().last().unwrap(), &[]);
            render_pass.draw(0..QUAD_VERTICES.len() as u32, 0..1);
        });

        drop(render_pass);

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn load_image(&self, path: &str) -> Result<Image> {
        let full_path = "res/sprite/".to_owned() + path;
        let file = std::fs::File::open(full_path).map_err(Error::IO)?;
        let reader = std::io::BufReader::new(file);
        let image = image::load(reader, image::ImageFormat::Png).map_err(Error::Image)?;
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();

        Ok(Image {
            rgba,
            width: dimensions.0,
            height: dimensions.1,
        })
    }

    pub fn load_texture(
        &self,
        image: &Image,
        texture_atlas: TextureAtlas,
    ) -> Result<wgpu::BindGroup> {
        let size = wgpu::Extent3d {
            width: image.width,
            height: image.height,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &image.rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width),
                rows_per_image: Some(image.height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_atlas_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&[texture_atlas]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::VERTEX,
                });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(
                        texture_atlas_buffer.as_entire_buffer_binding(),
                    ),
                },
            ],
            label: None,
        });

        Ok(bind_group)
    }
}
