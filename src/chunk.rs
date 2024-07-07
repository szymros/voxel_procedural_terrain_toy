use crate::vertex::Vertex;
use crate::voxel::{BlockType, Voxel};
use crate::{quad::Quad, quad::Side};

use enum_iterator::all;
use noise::core::perlin::perlin_2d;

pub const CHUNK_SIZE: usize = 64;
const CHUNK_SQUARED: usize = CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_CUBED: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub struct Chunk {
    pub world_position: [f32; 3],
    pub blocks_vector: Vec<Voxel>,
    pub water_level: usize,
}
impl Chunk {

    fn perlin2d_octaves(
        x: f64,
        y: f64,
        octaves: i32,
        frequency: f64,
        perm_table: &noise::permutationtable::PermutationTable,
    ) -> f64 {
        let mut perlin_result = 0.0;
        for i in 1..=octaves {
            perlin_result += 1.0 / i as f64
                * perlin_2d(
                    [i as f64 * x * frequency, i as f64 * y * frequency].into(),
                    perm_table,
                );
        }
        return perlin_result;
    }


    pub fn linearize(x:usize,y:usize,z:usize) -> usize {
        return x * CHUNK_SQUARED + y * CHUNK_SIZE + z;
    }

    pub fn delinearize(index:usize) -> [usize;3] {
        let x = index / CHUNK_SQUARED;
        let y = (index - x * CHUNK_SQUARED) / CHUNK_SIZE;
        let z = index - x * CHUNK_SQUARED - y * CHUNK_SIZE;
        return [x,y,z];
    }
    pub fn new_perlin2d(
        world_position: [f32; 3],
        frequency: f64,
        octaves: usize,
        perm_table: &noise::permutationtable::PermutationTable,
        ground_level: f64,
        noise_multiplier: f64,
        water_level: usize,
        dirt_layer_height: i32,
    ) -> Self {
        let mut blocks_vector:Vec<Voxel> = vec![Voxel::new(false, BlockType::None); CHUNK_CUBED];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let nx = (x as f64 / CHUNK_SIZE as f64) + world_position[0] as f64;
                let nz = (z as f64 / CHUNK_SIZE as f64) + world_position[2] as f64;
                let y_level = Self::perlin2d_octaves(nx, nz, octaves as i32, frequency, perm_table)
                    * noise_multiplier
                    + ground_level;
                for y in 0..=y_level as usize {
                    if y == y_level as usize {
                        blocks_vector[Self::linearize(x, y, z)] = Voxel::new(true, BlockType::Grass);
                    } else if y > (y_level - dirt_layer_height as f64) as usize {
                        blocks_vector[Self::linearize(x, y, z)] = Voxel::new(true, BlockType::Dirt);
                    } else {
                        blocks_vector[Self::linearize(x, y, z)] = Voxel::new(true, BlockType::Stone);
                    }
                }
                if blocks_vector[Self::linearize(x,water_level,z)].block_type == BlockType::None {
                    blocks_vector[Self::linearize(x,water_level,z)] = Voxel::new(true, BlockType::Water);
                }
            }
        }
        return Self {
            world_position,
            water_level,
            blocks_vector
        };
    }

    pub fn handle_directional_move(&self, position: [usize; 3], direction: i32, axis: usize) -> bool {
        if position[axis] == 0 && direction < 0 {
            return true;
        }
        if position[axis] == CHUNK_SIZE - 1 && direction > 0 {
            return true;
        }
        let mut new_position:Vec<i32> = position.clone().iter().map(|x| *x as i32).collect();
        new_position[axis] += direction;
        let block_at_new_position = self.blocks_vector[Self::linearize(new_position[0] as usize, new_position[1] as usize, new_position[2] as usize)];
        if block_at_new_position.is_active && block_at_new_position.block_type != BlockType::Water
        {
            return false;
        }
        return true;
    }

    pub fn build_mesh(&self, index_start: u32, water_index_start:u32) -> (Vec<Vertex>, Vec<u32>, Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_index: u32 = index_start.clone();
        let mut water_vertices: Vec<Vertex> = Vec::new();
        let mut water_indices: Vec<u32> = Vec::new();
        let mut water_vertex_index: u32 = water_index_start.clone();
        for x in 0..CHUNK_SIZE {    
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block:Voxel = self.blocks_vector[Self::linearize(x,y,z)];
                    if block.is_active {
                        let world_pos = [
                            (self.world_position[0] * CHUNK_SIZE as f32) + x as f32,
                            (self.world_position[1] * CHUNK_SIZE as f32) + y as f32,
                            (self.world_position[2] * CHUNK_SIZE as f32) + z as f32,
                        ];
                        for side in all::<Side>() {
                            let quad = Quad::new(&side, world_pos[0], world_pos[1], world_pos[2]);
                            let (axis, direction) = Quad::get_axis_and_direction_for_side(&side);
                            if self.handle_directional_move([x,y,z], direction, axis) {
                                let mut color = Voxel::get_rgb_for_type(block.block_type);
                                if block.block_type == BlockType::Water {
                                    if side == Side::Top {
                                        water_vertices.append(&mut quad.get_corner_vertices(color));
                                        water_indices.append(&mut quad.get_indices(water_vertex_index));
                                        water_vertex_index += 4;
                                    }
                                    continue;
                                }
                                if block.block_type == BlockType::Grass
                                    && (side != Side::Top || y < self.water_level)
                                {
                                    color = Voxel::get_rgb_for_type(BlockType::Dirt);
                                }
                                let multi = Quad::get_color_multiplier_for_side(&side);
                                for i in 0..3 {
                                    color[i] *= multi;
                                }
                                vertices.append(&mut quad.get_corner_vertices(color));
                                indices.append(&mut quad.get_indices(vertex_index));
                                vertex_index += 4;
                            }
                        }
                    }
                }
            }
        }
        return (vertices, indices, water_vertices, water_indices);
    }

}
