mod camera;
mod chunk;
mod instance;
mod quad;
mod region;
mod texture;
mod vertex;
mod voxel;

use std::sync::Arc;

use camera::{Camera, CameraController, CameraUniform};
use cgmath::prelude::*;
use chunk::Chunk;
use instance::{Instance, InstanceRaw, INSTANCE_DISPLACEMENT, NUM_INSTANCES_PER_ROW};
use region::Region;
use vertex::Vertex;
use wgpu::util::DeviceExt;
use winit::{event::WindowEvent, event_loop};

struct State {
    surface: wgpu::Surface<'static>,
    queue: wgpu::Queue,
    device: wgpu::Device,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    camera: Camera,
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    camera_controller: CameraController,
    camera_uniform: CameraUniform,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    mouse_pressed: bool,
    depth_texture: texture::Texture,
    len_indices: usize,
}

impl State {
    async fn new(window: Arc<winit::window::Window>) -> Result<Self, Box<dyn std::error::Error>> {
        // get window size
        let size: winit::dpi::PhysicalSize<u32> = window.inner_size();
        // create an instance with default settings
        let instance = wgpu::Instance::default();
        // create surface and adapter from instance
        // unsafe to make sure that when the window is closed, the surface is destroyed
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        surface.configure(&device, &config);

        let shader_module = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // let mut chunks_: Vec<Chunk> = Vec::new();
        // for x in 0..3{
        //     for y in 0..3{
        //         chunks_.push(Chunk::new([x as f32, y as f32, 0.0]));
        //     }
        // }
        // let mut vertices: Vec<Vertex> = Vec::new();
        // let mut indices: Vec<u32> = Vec::new();
        // let chunk_1 = Chunk::new([0.0, 0.0, 0.0]);
        // let (mut chunk_vertices,mut chunk_indecies) = chunk_1.build_mesh();
        // vertices.append(&mut chunk_vertices);
        // indices.append(&mut chunk_indecies);
        // let chunk_2 = Chunk::new([1.0, 1.0, 1.0]);
        // let (mut chunk_vertices2,mut chunk_indecies2) = chunk_2.build_mesh();
        // vertices.append(&mut chunk_vertices2);
        // indices.append(&mut chunk_indecies2);

        // for chunk in chunks_.iter(){
        //     let (mut chunk_vertices,mut chunk_indecies) = chunk.build_mesh();
        //     vertices.append(&mut chunk_vertices);
        //     indices.append(&mut chunk_indecies);
        // }

        let vertex_chunk = Chunk::new_random([0.0, -1.0, 0.0]);
        let (mut vertices1, mut indices1s) = vertex_chunk.build_mesh(0);
        // let chunk2 = Chunk::new_random([0.0, 0.0, 0.0]);
        // let (mut chunk_2_vert, mut chunk_2_ind) = chunk2.build_mesh(*indices1s.last().unwrap() as u32 + 4);
        // let vertices = [vertices1, chunk_2_vert].concat();
        // let indices = [indices1s, chunk_2_ind].concat();
        let mut vertices: Vec<Vertex> = Vec::new();
        vertices.append(&mut vertices1);
        // vertices.append(&mut chunk_2_vert);
        let mut indices: Vec<u32> = Vec::new();
        indices.append(&mut indices1s);
        // indices.append(&mut chunk_2_ind);
        let len_indices = indices.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let chuj = Chunk::new([0.0, 0.0, 0.0]);
        let dupa = vec![chuj];
        let instances = dupa
            .iter()
            .flat_map(|chunk| chunk.build_mesh_random())
            .collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let camera = Camera {
            eye: (0.0, 5.0, 10.0).into(),
            yaw: cgmath::Deg(-90.0).into(),
            pitch: cgmath::Deg(-20.0).into(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };
        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);
        let camera_controller = CameraController::new(4.0, 0.4);

        let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
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
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            }],
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Main pipeline layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        Ok(Self {
            surface,
            queue,
            device,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            camera,
            camera_uniform_buffer,
            camera_bind_group,
            camera_controller,
            camera_uniform,
            instances,
            instance_buffer,
            mouse_pressed: false,
            depth_texture,
            len_indices,
        })
    }

    fn render(&self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.draw_indexed(0..self.len_indices as u32, 0, 0..1 as _);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn handle_input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                self.camera_controller.process_keyboard(event);
                true
            }
            WindowEvent::MouseInput {
                device_id: _,
                state: st,
                button,
            } => {
                if *button == winit::event::MouseButton::Left
                    && *st == winit::event::ElementState::Pressed
                {
                    self.mouse_pressed = true;
                }
                true
            }
            _ => false,
        }
    }

    fn update(&mut self, dt: instant::Duration) {
        self.camera_controller.update_camera(&mut self.camera, dt);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }
}

async fn run() {
    let event_loop = event_loop::EventLoop::new().unwrap();
    let window = Arc::new(
        winit::window::WindowBuilder::new()
            .build(&event_loop)
            .unwrap(),
    );
    let mut state = State::new(window.clone()).await.unwrap();
    let mut last_render_time = instant::Instant::now(); // NEW!
    state.render();
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    let _ = event_loop.run(move |event, window_target| match event {
        winit::event::Event::DeviceEvent {
            device_id: _,
            event,
        } => match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                if state.mouse_pressed {
                    state.camera_controller.process_mouse(delta.0, delta.1);
                }
            }
            _ => {}
        },
        winit::event::Event::WindowEvent {
            ref event,
            window_id: _,
        } if !state.handle_input(event) => match event {
            WindowEvent::CloseRequested => {
                window_target.exit();
            }
            WindowEvent::RedrawRequested => {
                let now = instant::Instant::now();
                let dt: instant::Duration = now - last_render_time;
                last_render_time = now;
                state.update(dt);
                state.render();
                window.request_redraw();
            }
            _ => {}
        },
        _ => (),
    });
}

fn main() {
    println!("Hello, world!");
    env_logger::init();
    pollster::block_on(run());
}
