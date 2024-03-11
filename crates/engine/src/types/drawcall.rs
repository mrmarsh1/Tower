use std::rc::Rc;

use math::{Vector3f, Matrix4x4};
use web_sys::WebGlVertexArrayObject;

pub struct DrawCallInfo {
    pub mesh_id: Rc<WebGlVertexArrayObject>,
    pub transform: Matrix4x4,
    pub position1: Vector3f,
    pub position2: Vector3f,
    pub index_count: i32,
}