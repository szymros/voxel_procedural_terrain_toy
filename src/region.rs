use crate::{
    chunk::Chunk,
    generation_params::{self, GenerationParams},
    vertex::Vertex,
};
const CHUNK_PER_ROW: i32 = 3;

pub struct Region {
    pub centre: [i32; 2],
    pub chunk_buffer: Vec<Chunk>,
}

impl Region {
    pub fn new(centre: [i32; 2], generation_params: GenerationParams) -> Region {
        let perm_table = noise::permutationtable::PermutationTable::new(generation_params.seed);
        let mut chunk_buffer: Vec<Chunk> = Vec::new();
        for x in centre[0] - CHUNK_PER_ROW / 2..=centre[0] + CHUNK_PER_ROW / 2 {
            for z in centre[1] - CHUNK_PER_ROW / 2..=centre[1] + CHUNK_PER_ROW / 2 {
                let chunk = Chunk::new_perlin2d(
                    [x as f32, 0.0, z as f32],
                    generation_params.frequency,
                    generation_params.octaves,
                    &perm_table,
                    generation_params.ground_level as f64,
                    generation_params.noise_multiplier,
                    generation_params.water_level as usize,
                    generation_params.dirt_layer_height as i32,
                );
                chunk_buffer.push(chunk);
            }
        }
        Region {
            centre,
            chunk_buffer,
        }
    }
    pub fn build_mesh(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        for chunk in self.chunk_buffer.iter() {
            let (chunk_vertices, chunk_indices) = chunk.build_mesh(vertices.len() as u32);
            vertices.extend(chunk_vertices.iter());
            indices.extend(chunk_indices.iter());
        }
        for chunk in self.chunk_buffer.iter() {
            let (chunk_vertices, chunk_indices) = chunk.build_water_mesh(vertices.len() as u32);
            vertices.extend(chunk_vertices.iter());
            indices.extend(chunk_indices.iter());
        }
        (vertices, indices)
    }
}
