use crate::instance::Instance;
use cgmath::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct Chunk {
    pub size: u32,
    pub world_position: [f32; 3],
}

impl Chunk {
    pub fn new(size: u32, world_position: [f32; 3]) -> Self {
        Self {
            size,
            world_position,
        }
    }

    pub fn build_mesh(&self) -> Vec<Instance> {
        let down_scale = 0.027f64;
        let perlin = Perlin::new(1);
        let mut instances = Vec::new();
        let rotation =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    let position: cgmath::Vector3<f32> = cgmath::Vector3::new(
                        (self.world_position[0] * self.size as f32) + x as f32,
                        (self.world_position[1] * self.size as f32) + z as f32,
                        (self.world_position[2] * self.size as f32) + y as f32,
                    );
                    let height = perlin.get([position.x as f64*0.1, position.y as f64*0.1, position.z as f64*0.1]);
                    // println!("Height: {}", height*0.122);
                    // println!("perlin {}", perlin.get([0.1, 0.1, 0.1]));
                    if height > 0.1f64 {
                        instances.push(Instance { position, rotation });
                    }
                }
            }
        }
        return instances;
    }
}
