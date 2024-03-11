use ecs::Component;

use component::component;
use math::Color;

#[component]
pub struct SpotLight {
    pub diffuse: Color,
    pub specular: Color,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
    pub cut_off: f32,
    pub outer_cut_off: f32,
}