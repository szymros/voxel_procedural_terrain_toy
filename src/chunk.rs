use crate::vertex::Vertex;
use crate::voxel::Voxel;
use crate::{instance::Instance, quad::Quad, quad::Side};

use cgmath::prelude::*;
use enum_iterator::all;
use noise::utils::NoiseMapBuilder;
use noise::MultiFractal;
use noise::{utils::PlaneMapBuilder, Fbm, Perlin};
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

    pub fn new_random(world_position: [f32; 3], frequency:f64, octaves:usize) -> Self {
        let fbm = Fbm::<Perlin>::new(1)
            .set_frequency(frequency)
            .set_octaves(octaves)
            .set_lacunarity(2.0)
            .set_persistence(0.5);
        let height_map = PlaneMapBuilder::new(fbm)
            .set_size(SIZE, SIZE)
            .set_x_bounds(0.0, 1.0)
            .set_y_bounds(0.0, 1.0)
            .build();
        let mut blocks = [[[Voxel::new(true); SIZE]; SIZE]; SIZE];
        for x in 0..SIZE {
            for z in 0..SIZE {
                for y in 0..SIZE {
                    if (y as f64) < height_map.get_value(x, z) * SIZE as f64 {
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
