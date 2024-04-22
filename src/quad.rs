use crate::vertex::Vertex;
use enum_iterator::Sequence;

#[derive(Debug, PartialEq, Sequence)]
pub enum Side {
    Top,
    Bottom,
    Right,
    Left,
    Front,
    Back,
}

const HALF_SIZE: f32 = 0.5f32;

pub struct Quad {
    pub corners: [[f32; 3]; 4],
}

impl Quad {
    pub fn new(side: &Side, pos_x:f32,pos_y:f32,pos_z:f32) -> Self {
        match side {
            Side::Top => Self { corners :[
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y - HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
            ]},
            Side::Bottom => Self { corners :[
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y - HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z + HALF_SIZE],
            ]},
            Side::Right => Self { corners :[
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y + HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y + HALF_SIZE, pos_z - HALF_SIZE],
            ]},
            Side::Left => Self { corners :[
                [pos_x - HALF_SIZE, pos_y + HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y + HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z + HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
            ]},
            Side::Front =>  Self { corners :[
                [pos_x + HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y + HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y + HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
            ]},
            Side::Back => Self { corners :[
                [pos_x - HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x - HALF_SIZE, pos_y + HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y + HALF_SIZE, pos_z - HALF_SIZE],
                [pos_x + HALF_SIZE, pos_y - HALF_SIZE, pos_z - HALF_SIZE],
            ]},
        }
    }
    
    pub fn get_corner_vertices(&self, color:[f32;3])-> Vec<Vertex>{
        let mut vertices = Vec::new();
        for v in self.corners.iter(){
            vertices.push(Vertex { position: [v[0], v[1], v[2], 1.0], color: color });
        }
        return vertices;
    }

    pub fn get_indices(&self, vertex_index:u32) -> Vec<u32>{
        let mut indices = Vec::new();
        indices.push(vertex_index);
        indices.push(vertex_index + 1);
        indices.push(vertex_index + 2);
        indices.push(vertex_index);
        indices.push(vertex_index + 2);
        indices.push(vertex_index + 3);
        return indices;
    }

    pub fn get_axis_and_direction_for_side(side:&Side) -> (usize,i32){
        match side {
            Side::Left => (0,1),
            Side::Right => (0,-1),
            Side::Top => (1,1),
            Side::Bottom => (1,-1),
            Side::Front => (2,1),
            Side::Back => (2,-1),
        }
    }
}