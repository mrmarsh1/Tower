use std::rc::Rc;

use web_sys::{WebGlProgram, WebGlTexture, WebGlVertexArrayObject};

use ecs::Component;

use component::component;

use crate::types::Tlu;

#[component]
pub struct Skybox {
    pub program: Rc<WebGlProgram>,
    pub shader: Rc<Tlu>,
    pub mesh: WebGlVertexArrayObject,
    pub vertex_count: i32,
    pub texture: Rc<WebGlTexture>,
}

impl Skybox {
    pub fn create(
        entity: usize,
        shader: Rc<Tlu>,
        program: Rc<WebGlProgram>,
        mesh: WebGlVertexArrayObject,
        vertex_count: i32,
        texture: Rc<WebGlTexture>,
    ) -> Skybox {
        Skybox {
            entity,
            one_frame: false,
            shader,
            program: program,
            mesh,
            vertex_count,
            texture,
        }
    }

    pub fn get_mesh_data() -> Vec<f32> {
        vec![
            -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0,
        ]
    }
}
