pub struct GenerationParams {
    pub seed: u32,
    pub octaves: usize,
    pub frequency: f64,
    pub ground_level: u32,
    pub water_level: u32,
    pub noise_multiplier: f64,
    pub dirt_layer_height: u32,
}
