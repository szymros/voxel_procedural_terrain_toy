use crate::chunk::CHUNK_SIZE;
use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bytemuck::Contiguous;
use dolly::prelude::*;
use dolly::rig::CameraRig;
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{Key, NamedKey},
};

const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &mut DollyCamera, time_delta: f32) {
        self.view_proj = camera.build_view_projection_matrix(time_delta).into();
    }
}

pub struct DollyCamera {
    pub centre: [f32; 2],
    pub camera_rig: CameraRig,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl DollyCamera {
    pub fn new(
        centre: [f32; 2],
        arm: [f32; 3],
        aspect: f32,
        fovy: f32,
        znear: f32,
        zfar: f32,
    ) -> Self {
        let camera_rig = CameraRig::builder()
            .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
            .with(Smooth::new_rotation(1.5))
            .with(Arm::new(
                glam::Vec3::X * arm[0] + glam::Vec3::Y * arm[1] + glam::Vec3::Z * arm[2],
            ))
            .build();
        Self {
            centre,
            camera_rig,
            aspect,
            fovy,
            znear,
            zfar,
        }
    }

    pub fn build_view_projection_matrix(&mut self, time_delta: f32) -> cgmath::Matrix4<f32> {
        let transform = self.camera_rig.update(time_delta);
        let position = cgmath::Point3 {
            x: transform.position.x + self.centre[0],
            y: transform.position.y,
            z: transform.position.z + self.centre[1],
        };
        let forward = cgmath::Vector3 {
            x: transform.forward::<glam::Vec3>().x,
            y: transform.forward::<glam::Vec3>().y,
            z: transform.forward::<glam::Vec3>().z,
        };
        let up = cgmath::Vector3 {
            x: transform.up::<glam::Vec3>().x,
            y: transform.up::<glam::Vec3>().y,
            z: transform.up::<glam::Vec3>().z,
        };

        let view = cgmath::Matrix4::look_at(position, position + forward, up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }

    pub fn process_cam_input(&mut self, event: &KeyEvent) -> bool {
        let amount = if event.state == ElementState::Pressed {
            5.0
        } else {
            0.0
        };
        match event.logical_key.as_ref() {
            Key::Character("a") => {
                self.camera_rig
                    .driver_mut::<YawPitch>()
                    .rotate_yaw_pitch(-10.0, 0.0);
                true
            }
            Key::Character("d") => {
                self.camera_rig
                    .driver_mut::<YawPitch>()
                    .rotate_yaw_pitch(10.0, 0.0);
                true
            }
            Key::Character("w") => {
                self.camera_rig.driver_mut::<Arm>().offset.z -= amount;
                true
            }
            Key::Character("s") => {
                self.camera_rig.driver_mut::<Arm>().offset.z += amount;
                true
            }
            _ => false,
        }
    }
}
