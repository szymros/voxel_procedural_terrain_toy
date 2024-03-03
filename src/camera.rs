
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);

const window_size: (u32, u32) = (800, 600);


pub fn generate_view_matrix() -> cgmath::Matrix4<f32> {
    let projection = cgmath::perspective(
        cgmath::Deg(45.0),
        window_size.0 as f32 / window_size.1 as f32,
        1.0,
        10.0,
    );
    let view = cgmath::Matrix4::look_at_rh(
        cgmath::Point3::new(0.0, -5.0, 1.0),
        cgmath::Point3::new(0.0, 0.0, 0.0),
        cgmath::Vector3::unit_y(),
    );
    return OPENGL_TO_WGPU_MATRIX * projection * view;
}