use std::{collections::HashMap, rc::Rc};

use ecs::Component;

use component::component;

use crate::types::{DrawCallInfo, Material};

#[component]
pub struct DrawPackage {
    pub meshes: HashMap<Rc<Material>, Vec<DrawCallInfo>>,
}