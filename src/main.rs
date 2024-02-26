use wgpu::util::DeviceExt;
use winit::event_loop;

const window_size: (u32, u32) = (800, 600);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Vertex { position:[position[0] as f32, position[1] as f32, position[2] as f32, 1.0], color }
    }
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    // top (0, 0, 1)
    Vertex { position: [-1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex{ position: [1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0,1.0], color: [1.0, 0.0, 0.0] },
    Vertex{ position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    // bottom (0, 0, -1)
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    // right (1, 0, 0)
    Vertex { position: [1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },

    // left (-1, 0, 0)
    Vertex { position: [-1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },

    // front (0, 1, 0)
    Vertex { position: [1.0, 1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },

    // back (0, -1, 0)
    Vertex { position: [1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
];
const INDEX: &[u16] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

fn generate_view_matrix()-> cgmath::Matrix4<f32>{
    let projection = cgmath::perspective(cgmath::Deg(45.0), window_size.0 as f32 / window_size.1 as f32, 1.0, 10.0);
    let view = cgmath::Matrix4::look_at_rh(
        cgmath::Point3::new(1.5, -5.0, 3.0),
        cgmath::Point3::new(0.0, 0.0, 0.0),
        cgmath::Vector3::unit_y(),
    );
    return OPENGL_TO_WGPU_MATRIX * projection * view;
}

async fn init_gpu(window: &winit::window::Window) {
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

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&VERTICES),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&INDEX),
        usage: wgpu::BufferUsages::INDEX,
    });

    let view_matrix: [[f32;4];4] = generate_view_matrix().into();

    let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[view_matrix]),
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
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_uniform_buffer.as_entire_binding(),
            },
        ],
    });

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
            buffers: &[Vertex::desc()],
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
        depth_stencil: None,
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

    let output = surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &camera_bind_group, &[]);
        render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw_indexed(0..INDEX.len() as u32, 0, 0..1);
    }
    queue.submit(std::iter::once(encoder.finish()));
    output.present();

}

async fn run(){
    let event_loop = event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    init_gpu(&window).await;
    event_loop.set_control_flow(event_loop::ControlFlow::Poll);

    event_loop.run(move |event, window_target| match event {
        winit::event::Event::WindowEvent {
            event: winit::event::WindowEvent::CloseRequested,
            ..
        } => window_target.exit(),
    winit::event::Event::WindowEvent {event: winit::event::WindowEvent::RedrawRequested,
        .. } => {
    }
        _ => (),
    }
);

}

fn main() {
    println!("Hello, world!");
    env_logger::init();
    pollster::block_on(run());
}
