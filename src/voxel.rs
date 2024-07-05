#[derive(Copy, Clone, PartialEq)]
pub enum BlockType {
    Dirt,
    Grass,
    Stone,
    None,
    Water,
}

#[derive(Copy, Clone)]
pub struct Voxel {
    pub is_active: bool,
    pub block_type: BlockType,
}

impl Voxel {
    pub fn new(is_active: bool, block_type: BlockType) -> Self {
        Self {
            is_active: is_active,
            block_type: block_type,
        }
    }

    pub fn get_rgb_for_type(block_type: BlockType) -> [f32; 4] {
        match block_type {
            BlockType::Dirt => [90.0 / 255.0, 63.0 / 255.0, 43.0 / 255.0, 1.0],
            BlockType::Grass => [73.0 / 255.0, 115.0 / 255.0, 14.0 / 255.0, 1.0],
            BlockType::Stone => [106.0 / 255.0, 98.0 / 255.0, 87.0 / 255.0, 1.0],
            BlockType::Water => [95.0 / 255.0, 192.0 / 255.0, 237.0 / 255.0, 0.6],
            _ => [0.0, 0.0, 0.0, 0.0],
        }
    }
}

// dirt 90, 63, 43
// grass 121, 192, 90   73, 115, 14
// stone 106, 98, 87s
