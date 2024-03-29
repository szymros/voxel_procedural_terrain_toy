pub struct Voxel{
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub color: u32,
    // visibility of faces
    // pub up: bool,
    // pub down: bool,
    // pub left: bool,
    // pub right: bool,
    // pub front: bool,
    // pub back: bool,
    pub is_active: bool,
}


impl Voxel{
    pub fn new(x: i32, y: i32, z: i32, color: u32) -> Self{
        Self{
            x,
            y,
            z,
            color,
            up: false,
            down: false,
            left: false,
            right: false,
            front: false,
            back: false,
        }
    }

    pub fn get_vertices(){
        VERTICES * 1 
    }
}