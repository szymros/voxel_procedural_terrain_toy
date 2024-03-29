// pub const NUM_INSTANCES_PER_ROW: u32 = 1;
// pub const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
//     NUM_INSTANCES_PER_ROW as f32 * 0.5,
//     0.0,
//     NUM_INSTANCES_PER_ROW as f32 * 0.5,
// );

pub const NUM_INSTANCES_PER_ROW: u32 = 16;
pub const NUM_INSTANCES: u32 = NUM_INSTANCES_PER_ROW * NUM_INSTANCES_PER_ROW;
pub const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5f32,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5f32,
);

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials, we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5, not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}
