use ecs::Component;

use component::component;
use math::Color;

#[component]
pub struct DirectionalLight {
    pub diffuse: Color,
    pub specular: Color,
}
