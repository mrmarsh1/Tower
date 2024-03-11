use ecs::Component;

use component::component;
use math::Matrix4x4;

#[component]
pub struct Camera {
    fov: f32,
    near: f32,
    far: f32,
    view_matrix: Matrix4x4,
}

impl Camera {
    pub fn new(entity: usize) -> Self {
        Self {
            entity,
            one_frame: false,
            fov: 45.0,
            near: 0.1,
            far: 1000.0,
            view_matrix: Matrix4x4::identity(),
        }
    }
    
    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn get_view_matrix(&self) -> Matrix4x4 {
        self.view_matrix
    }

    pub fn set_view_matrix(&mut self, value: Matrix4x4) {
        self.view_matrix = value;
    }
}