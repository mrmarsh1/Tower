use ecs::Component;

use component::component;
use math::Color;

#[component]
pub struct PointLight {
    pub diffuse: Color,
    pub specular: Color,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}