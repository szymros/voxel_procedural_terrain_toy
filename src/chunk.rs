
use crate::vertex::Vertex;
use crate::{instance::Instance, quad::Quad,quad::Side};
use crate::voxel::Voxel;

use cgmath::prelude::*;
use enum_iterator::all;
use noise::{NoiseFn, Perlin};
use rand::Rng;


pub const SIZE:usize = 32;


pub struct Chunk {
    pub size: u32,
    pub world_position: [f32; 3],
    pub blocks: [[[Voxel;SIZE];SIZE];SIZE]
}

impl Chunk {
    pub fn new(size: u32, world_position: [f32; 3]) -> Self {
        // let blocks = [Voxel::new(true);SIZE*SIZE*SIZE];
        let mut blocks = [[[Voxel::new(true);SIZE];SIZE];SIZE];
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if x == 0 || x == SIZE - 1 || y == 0 || y == SIZE - 1 || z == 0 || z == SIZE - 1 {
                        blocks[x][y][z] = Voxel::new(false);
                        continue;
                    }
                    else {
                        blocks[x][y][z] = Voxel::new(true);
                    }
                }
            }
        }
        Self {
            size,
            world_position,
            blocks,
        }
    }

    pub fn build_mesh_random(&self) -> Vec<Instance> {
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
                    let height = perlin.get([
                        position.x as f64 * 0.1,
                        position.y as f64 * 0.1,
                        position.z as f64 * 0.1,
                    ]);
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


    pub fn handle_directional_move(&self,position:[i32;3], direction:i32 ,axis:usize)-> bool{
        if position[axis] == 0 && direction < 0 {
            return true;
        }
        if position[axis] == SIZE as i32 - 1 && direction > 0 {
            return true;
        }
        let mut new_position = position.clone();
        new_position[axis] += direction;
        if self.blocks[new_position[0] as usize][new_position[1] as usize ][new_position[2] as usize].is_active{
            return false;
        }
        return true;
    }


    pub fn build_mesh_vertex(&self) -> (Vec<Vertex>, Vec<u32>){
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_index:u32 = 0;
        for (x_index, x) in self.blocks.iter().enumerate() {
            for (y_index,y) in x.iter().enumerate() {
                for (z_index,block) in y.iter().enumerate() {
                    if block.is_active {
                        let local_pos = [x_index as i32, y_index as i32, z_index as i32];
                        let world_pos = [
                            (self.world_position[0] * self.size as f32) + local_pos[0] as f32,
                            (self.world_position[1] * self.size as f32) + local_pos[1] as f32,
                            (self.world_position[2] * self.size as f32) + local_pos[2] as f32,
                        ];
                        for side in all::<Side>(){
                            let quad = Quad::new(&side, world_pos[0], world_pos[1], world_pos[2]);
                            let (axis,direction) = Quad::get_axis_and_direction_for_side(&side);
                            if self.handle_directional_move(local_pos, direction, axis){ 
                                let color = [rand::thread_rng().gen_range(0f32..1f32),rand::thread_rng().gen_range(0f32..1f32),rand::thread_rng().gen_range(0f32..1f32)];
                                vertices.append(&mut quad.get_corner_vertices(color));
                                indices.append(&mut quad.get_indices(vertex_index));
                                vertex_index += 4;
                            }
                        }
                        
                    }
                }
                }
            }
        return (vertices, indices);
    }
}

