#[derive(Copy, Clone)]
pub struct Voxel {
    pub is_active: bool,
}

impl Voxel {
    pub fn new(is_active: bool) -> Self {
        Self {
            is_active: is_active,
        }
    }
}
