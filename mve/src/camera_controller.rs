use cgmath::{InnerSpace, Rad, Vector3};

use crate::rendering::camera::Camera;

const SAFE_FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2 - 0.0001;

pub struct CameraController {
    pub camera: Camera,
    pub speed: f32,
}

impl CameraController {
    pub fn new(position: cgmath::Point3<f32>) -> Self {
        Self {
            camera: Camera {
                eye: position,
                yaw: cgmath::Rad(0.0),
                pitch: cgmath::Rad(0.0),
                aspect: 1920.0 / 1080.0 as f32,
                fovy: 75.0,
                znear: 1.0,
                zfar: 10.0,
            },
            speed: 0.1,
        }
    }

    pub fn translate_camera(&mut self, translation: cgmath::Vector3<f32>) {
        let forward =
            Vector3::new(self.camera.yaw.0.cos(), 0.0, self.camera.yaw.0.sin()).normalize();
        let right =
            Vector3::new(-self.camera.yaw.0.sin(), 0.0, self.camera.yaw.0.cos()).normalize();

        self.camera.eye += -forward * translation.z * 0.2;
        self.camera.eye += right * translation.x * 0.2;
        self.camera.eye.y += translation.y * 0.2;
    }

    pub fn rotate_camera(&mut self, mouse_delta: cgmath::Vector2<f32>) {
        self.camera.yaw += Rad(mouse_delta.x) * self.speed;
        self.camera.pitch += Rad(-mouse_delta.y) * self.speed;
        if self.camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            self.camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if self.camera.pitch > Rad(SAFE_FRAC_PI_2) {
            self.camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }
}
