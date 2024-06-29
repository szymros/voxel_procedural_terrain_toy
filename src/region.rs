use crate::{chunk::Chunk, vertex::Vertex};


const REGION_SIZE: u32 = 9;
const CHUNK_PER_ROW: u32 = 3;

pub struct Region {
    pub chunk_buffer: Vec<Chunk>,
}

impl Region {
    pub fn new() -> Region {
        let mut chunk_buffer:Vec<Chunk> = Vec::new();
        for x in -1..=1{
            for z in -1..=1{
                let chunk = Chunk::new_random([x as f32, 0.0, z as f32],2.0,6);
                chunk_buffer.push(chunk);
            }
        }
        Region {
            chunk_buffer,
        }
    }

    pub fn build_mesh(&self)->(Vec<Vertex>, Vec<u32>){
        let mut vertices:Vec<Vertex> = Vec::new();
        let mut indices:Vec<u32> = Vec::new();
        for chunk in self.chunk_buffer.iter(){
            let (chunk_vertices, chunk_indices) = chunk.build_mesh(indices.len() as u32);
            vertices.extend(chunk_vertices.iter());
            indices.extend(chunk_indices.iter());
        }
        (vertices, indices)
    }
}
