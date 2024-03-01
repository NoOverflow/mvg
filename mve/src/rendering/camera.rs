use cgmath::{InnerSpace, Matrix4, Rad, SquareMatrix, Vector3};

use super::renderable::Renderable;

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,

    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

impl Camera {
    // TODO: Separate the projection from the camera, since we only need to compute it when the window is resized
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        let view = cgmath::Matrix4::look_to_rh(
            self.eye,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        );
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

impl Renderable for Camera {
    fn prepare(&mut self, _device: &wgpu::Device) {}

    fn render<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        render_pass_data: &'a super::renderable::RenderPassData,
    ) {
        let project_matrix: Matrix4<f32> = self.build_view_projection_matrix();
        let project_matrix_ref: &[f32; 16] = project_matrix.as_ref();

        render_pass_data.queue.write_buffer(
            &render_pass_data.projection_buffer,
            0,
            bytemuck::cast_slice(project_matrix_ref),
        );
    }
}
