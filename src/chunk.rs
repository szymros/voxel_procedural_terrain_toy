use crate::vertex::Vertex;
use crate::voxel::{BlockType, Voxel};
use crate::{quad::Quad, quad::Side};

use enum_iterator::all;
use noise::core::perlin::perlin_2d;

pub const CHUNK_SIZE: usize = 64;

pub struct Chunk {
    pub world_position: [f32; 3],
    pub blocks: [[[Voxel; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub water_level: usize,
}
impl Chunk {
    // pub fn new_random(world_position: [f32; 3], frequency: f64, octaves: usize) -> Self {
    //     // let fbm = Fbm::<Perlin>::new(1)
    //     //     .set_frequency(frequency)
    //     //     .set_octaves(octaves)
    //     //     .set_lacunarity(2.0)
    //     //     .set_persistence(0.5);
    //     // let height_map = PlaneMapBuilder::new(fbm)
    //     //     .set_size(CHUNK_SIZE, CHUNK_SIZE)
    //     //     .set_x_bounds(0.0, 1.0)
    //     //     .set_y_bounds(0.0, 1.0)
    //     //     .build();
    //     let hasher = PermutationTable::new(0);
    //     // let perlin = perlin_2d([0.0,0.0].into(), hasher);
    //     let mut blocks = [[[Voxel::new(true, BlockType::Air); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
    //     for x in 0..CHUNK_SIZE {
    //         for z in 0..CHUNK_SIZE {
    //             for y in 0..CHUNK_SIZE {
    //                 if (y as f64)
    //                     < perlin_2d([x as f64, z as f64].into(), &hasher) * CHUNK_SIZE as f64
    //                 {
    //                     blocks[x][y][z] = Voxel::new(true, 0);
    //                 } else {
    //                     blocks[x][y][z] = Voxel::new(false, 0);
    //                     if y == 1 {
    //                         blocks[x][y][z] = Voxel::new(true, 0);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     return Self {
    //         world_position,
    //         blocks,
    //     };
    // }

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
        let mut blocks =
            [[[Voxel::new(false, BlockType::None); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let nx = (x as f64 / CHUNK_SIZE as f64) + world_position[0] as f64;
                let nz = (z as f64 / CHUNK_SIZE as f64) + world_position[2] as f64;
                let y_level = Self::perlin2d_octaves(nx, nz, octaves as i32, frequency, perm_table)
                    * noise_multiplier
                    + ground_level;
                for y in 0..=y_level as usize {
                    if y == y_level as usize {
                        blocks[x][y][z] = Voxel::new(true, BlockType::Grass);
                    } else if y > (y_level - dirt_layer_height as f64) as usize {
                        blocks[x][y][z] = Voxel::new(true, BlockType::Dirt);
                    } else {
                        blocks[x][y][z] = Voxel::new(true, BlockType::Stone);
                    }
                }
            }
        }
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if blocks[x][water_level][z].block_type == BlockType::None {
                    blocks[x][water_level][z] = Voxel::new(true, BlockType::Water);
                };
            }
        }
        return Self {
            world_position,
            blocks,
            water_level,
        };
    }

    pub fn handle_directional_move(&self, position: [i32; 3], direction: i32, axis: usize) -> bool {
        if position[axis] == 0 && direction < 0 {
            return true;
        }
        if position[axis] == CHUNK_SIZE as i32 - 1 && direction > 0 {
            return true;
        }
        let mut new_position = position.clone();
        new_position[axis] += direction;
        if self.blocks[new_position[0] as usize][new_position[1] as usize][new_position[2] as usize]
            .is_active
            && self.blocks[new_position[0] as usize][new_position[1] as usize]
                [new_position[2] as usize]
                .block_type
                != BlockType::Water
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
                            (self.world_position[0] * CHUNK_SIZE as f32) + local_pos[0] as f32,
                            (self.world_position[1] * CHUNK_SIZE as f32) + local_pos[1] as f32,
                            (self.world_position[2] * CHUNK_SIZE as f32) + local_pos[2] as f32,
                        ];
                        for side in all::<Side>() {
                            let quad = Quad::new(&side, world_pos[0], world_pos[1], world_pos[2]);
                            let (axis, direction) = Quad::get_axis_and_direction_for_side(&side);
                            if self.handle_directional_move(local_pos, direction, axis) {
                                if block.block_type == BlockType::Water {
                                    continue;
                                }
                                let mut color = Voxel::get_rgb_for_type(block.block_type);
                                if block.block_type == BlockType::Grass
                                    && (side != Side::Top || y_index < self.water_level)
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
        return (vertices, indices);
    }

    pub fn build_water_mesh(&self, index_start: u32) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertex_index: u32 = index_start.clone();
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                if self.blocks[x][self.water_level][z].block_type == BlockType::Water {
                    let local_pos = [x as i32, self.water_level as i32, z as i32];
                    let world_pos = [
                        (self.world_position[0] * CHUNK_SIZE as f32) + local_pos[0] as f32,
                        (self.world_position[1] * CHUNK_SIZE as f32) + local_pos[1] as f32,
                        (self.world_position[2] * CHUNK_SIZE as f32) + local_pos[2] as f32,
                    ];
                    let quad = Quad::new(&Side::Top, world_pos[0], world_pos[1], world_pos[2]);
                    let color = Voxel::get_rgb_for_type(BlockType::Water);
                    vertices.append(&mut quad.get_corner_vertices(color));
                    indices.append(&mut quad.get_indices(vertex_index));
                    vertex_index += 4;
                }
            }
        }
        return (vertices, indices);
    }
}
