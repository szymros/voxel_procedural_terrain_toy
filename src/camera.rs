use winit::{
    event::{
        ElementState, KeyEvent,
        WindowEvent::{self},
    },
    keyboard::Key,
};

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);

const WINDOW_SIZE: (u32, u32) = (800, 600);

pub fn generate_view_matrix() -> cgmath::Matrix4<f32> {
    let projection = cgmath::perspective(
        cgmath::Deg(45.0),
        WINDOW_SIZE.0 as f32 / WINDOW_SIZE.1 as f32,
        1.0,
        10.0,
    );
    let view = cgmath::Matrix4::look_at_rh(
        cgmath::Point3::new(1.5f32, -5.0, 3.0),
        cgmath::Point3::new(0.0, 0.0, 0.0),
        cgmath::Vector3::unit_y(),
    );
    return OPENGL_TO_WGPU_MATRIX * projection * view;
}
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // 1.
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        // 2.
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        // 3.
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

pub struct CameraController {
    pub speed: f32,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state,
                        logical_key: key,
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match key.as_ref() {
                    Key::Character("w") => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    Key::Character("a") => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    Key::Character("s") => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    Key::Character("d") => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when the camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the forward/backward is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and the eye so
            // that it doesn't change. The eye, therefore, still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

// TODO: Switch to better camera
// pub fn dolly_vec3_to_cgmath_vec3(dolly_vec: glam::Vec3) -> cgmath::Vector3<f32> {
//     return cgmath::Vector3::new(dolly_vec.x, dolly_vec.y, dolly_vec.z);
// }
// pub fn dolly_point3_to_cgmath_point3(dolly_point: glam::Vec3) -> cgmath::Point3<f32> {
//     return cgmath::Point3::new(dolly_point.x, dolly_point.y, dolly_point.z);
// }
// pub fn dolly_point3_to_cgmath_vec3(dolly_point: glam::Vec3) -> cgmath::Vector3<f32> {
//     return cgmath::Vector3::new(dolly_point.x, dolly_point.y, dolly_point.z);
// }

// pub fn dolly_quaterion_to_cgmath_quaterion(dolly_quat: glam::Quat) -> cgmath::Quaternion<f32> {
//     return cgmath::Quaternion::new(dolly_quat.w, dolly_quat.x, dolly_quat.y, dolly_quat.z);
// }

// pub fn generate_view_matrix_dolly() -> cgmath::Matrix4<f32>{
//     let mut camera: CameraRig = CameraRig::builder()
//     .with(Position::new(Vec3{ x: 1.5, y: -5.0, z: 3.0 }))
//     .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
//     .with(Smooth::new_position_rotation(1.0, 1.0))
//     .build();
//     let t = camera.update(1.0/60.0);
//     // let transform = camera.final_transform;
//     // let model: Matrix4<f32> = (cgmath::Matrix4::from_translation(dolly_point3_to_cgmath_vec3(t.position.into()))
//     //     * cgmath::Matrix4::from(dolly_quaterion_to_cgmath_quaterion(t.rotation.into()))).into();
//     // let (sin_pitch, cos_yaw) = camera.final_transform.rotation.
//     // return OPENGL_TO_WGPU_MATRIX*model
//     let yaw =camera.driver::<YawPitch>().yaw_degrees;
//     let pitch = camera.driver::<YawPitch>().pitch_degrees;
//     // convert pitch and yaw to radians
//     let (sin_pitch, cos_pitch) = pitch.to_radians().sin_cos();
//     let (sin_yaw, cos_yaw) = yaw.to_radians().sin_cos();

// }
