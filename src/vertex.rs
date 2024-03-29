#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32; 3],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
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
pub const VERTICES: &[Vertex] = &[
    // top (0, 0, 1)
    Vertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex{ position: [1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex{ position: [1.0, 1.0, 1.0,1.0], color: [0.0, 1.0, 0.0] },
    Vertex{ position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    // bottom (0, 0, -1)
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0] },
    // right (1, 0, 0)
    Vertex { position: [1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },

    // left (-1, 0, 0)
    Vertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [1.0, 1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },

    // front (0, 1, 0)
    Vertex { position: [1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },

    // back (0, -1, 0)
    Vertex { position: [1.0, -1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, 1.0, 1.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0, 1.0], color: [0.0, 1.0, 0.0] },
];

pub const INDEX: &[u16] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];
