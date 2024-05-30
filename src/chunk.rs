use crate::vertex::Vertex;
use crate::voxel::Voxel;
use crate::{instance::Instance, quad::Quad, quad::Side};

use cgmath::prelude::*;
use enum_iterator::all;
use noise::{core::perlin::perlin_2d, permutationtable::PermutationTable};
use rand::Rng;

pub const SIZE: usize = 64;

pub struct Chunk {
    pub world_position: [f32; 3],
    pub blocks: [[[Voxel; SIZE]; SIZE]; SIZE],
}

impl Chunk {
    pub fn new(world_position: [f32; 3]) -> Self {
        let mut blocks = [[[Voxel::new(true); SIZE]; SIZE]; SIZE];
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if x == 0 || x == SIZE - 1 || y == 0 || y == SIZE - 1 || z == 0 || z == SIZE - 1
                    {
                        blocks[x][y][z] = Voxel::new(false);
                        continue;
                    } else {
                        blocks[x][y][z] = Voxel::new(true);
                    }
                }
            }
        }
        return Self {
            world_position,
            blocks,
        };
    }

    pub fn new_random(world_position: [f32; 3]) -> Self {
        let perm_table = PermutationTable::new(1);
        let mut height_map: [[f32; SIZE]; SIZE] = [[0.0; SIZE]; SIZE];
        for x in 0..SIZE {
            for y in 0..SIZE {
                height_map[x][y] = (perlin_2d([x as f64 * 0.1, y as f64 * 0.1].into(), &perm_table)
                    as f32)
                    * SIZE as f32;
            }
        }
        let mut blocks = [[[Voxel::new(true); SIZE]; SIZE]; SIZE];
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    if (y as f32) < height_map[x][z] {
                        blocks[x][y][z] = Voxel::new(true);
                    } else {
                        blocks[x][y][z] = Voxel::new(false);
                    }
                }
            }
        }
        return Self {
            world_position,
            blocks,
        };
    }

    pub fn build_mesh_random(&self) -> Vec<Instance> {
        let down_scale = 0.027f64;
        let mut instances = Vec::new();
        let rotation =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
        for x in 0..SIZE {
            for y in 0..SIZE {
                for z in 0..SIZE {
                    let position: cgmath::Vector3<f32> = cgmath::Vector3::new(
                        (self.world_position[0] * SIZE as f32) + x as f32,
                        (self.world_position[1] * SIZE as f32) + z as f32,
                        (self.world_position[2] * SIZE as f32) + y as f32,
                    );
                    let height = 0.2;
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

    pub fn handle_directional_move(&self, position: [i32; 3], direction: i32, axis: usize) -> bool {
        if position[axis] == 0 && direction < 0 {
            return true;
        }
        if position[axis] == SIZE as i32 - 1 && direction > 0 {
            return true;
        }
        let mut new_position = position.clone();
        new_position[axis] += direction;
        if self.blocks[new_position[0] as usize][new_position[1] as usize][new_position[2] as usize]
            .is_active
        {
            return false;
        }
        return true;
    }

    pub fn build_mesh(&self, index_start: u32) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_index: u32 = index_start.clone();
        for (x_index, x) in self.blocks.iter().enumerate() {
            for (y_index, y) in x.iter().enumerate() {
                for (z_index, block) in y.iter().enumerate() {
                    if block.is_active {
                        let local_pos = [x_index as i32, y_index as i32, z_index as i32];
                        let world_pos = [
                            (self.world_position[0] * SIZE as f32) + local_pos[0] as f32,
                            (self.world_position[1] * SIZE as f32) + local_pos[1] as f32,
                            (self.world_position[2] * SIZE as f32) + local_pos[2] as f32,
                        ]; 
                        for side in all::<Side>() {
                            let quad = Quad::new(&side, world_pos[0], world_pos[1], world_pos[2]);
                            let (axis, direction) = Quad::get_axis_and_direction_for_side(&side);
                            if self.handle_directional_move(local_pos, direction, axis) {
                                let color = [
                                    rand::thread_rng().gen_range(0f32..1f32),
                                    rand::thread_rng().gen_range(0f32..1f32),
                                    rand::thread_rng().gen_range(0f32..1f32),
                                ];
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
