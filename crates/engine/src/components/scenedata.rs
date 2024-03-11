use std::rc::Rc;

use ecs::Component;

use component::component;
use math::Color;
use web_sys::WebGlTexture;

#[component]
pub struct SceneData {
    pub ambient: Color,
    pub env: Option<Rc<WebGlTexture>>,
}

impl SceneData {
    pub fn get_env_tetxure(&self) -> Option<&WebGlTexture> {
        if let Some(tex) = &self.env {
            return Some(tex);
        }
        None
    }
}