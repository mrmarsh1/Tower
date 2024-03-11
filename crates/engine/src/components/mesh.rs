use std::rc::Rc;

use web_sys::WebGlVertexArrayObject;

use webgl::WebGlRenderer;

use ecs::Component;

use component::component;

use crate::types::{Material, MeshData};

#[component]
pub struct Mesh {
    pub mesh_data: Rc<MeshData>,
    pub mesh_id: Rc<WebGlVertexArrayObject>,
    pub material: Option<Rc<Material>>,
}

impl Mesh {
    pub fn create(
        renderer: &WebGlRenderer,
        entity: usize,
        mesh_data: MeshData,
        material: Option<Rc<Material>>,
    ) {
        let mesh_id = renderer.create_mesh(mesh_data.raw_vertices(), mesh_data.raw_indices(), webgl::DrawMode::Static);
    }
}
