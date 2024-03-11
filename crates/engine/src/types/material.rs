use std::{hash::Hash, rc::Rc, sync::atomic::AtomicUsize};

use web_sys::{WebGlProgram, WebGlTexture};

use super::Tlu;

pub struct Material {
    id: usize,
    pub tlu: Rc<Tlu>,
    program: Rc<WebGlProgram>,
    pub shininess: f32,
    tex_duffuse: Option<Rc<WebGlTexture>>,
    tex_specular: Option<Rc<WebGlTexture>>,
}

static MAT_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Material {
    pub fn new(tlu: Rc<Tlu>, program: Rc<WebGlProgram>, shininess: f32, tex_diffuse: Option<Rc<WebGlTexture>>,
        tex_specular: Option<Rc<WebGlTexture>>) -> Self {
        Material {
            id: MAT_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            tlu,
            program: program,
            shininess,
            tex_duffuse: tex_diffuse,
            tex_specular: tex_specular,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn tlu(&self) -> &Tlu {
        &self.tlu
    }

    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }

    pub fn tex_diffuse(&self) -> Option<&WebGlTexture> {
        if let Some(tex) = &self.tex_duffuse {
            return Some(tex);
        }
        None
    }

    pub fn tex_specular(&self) -> Option<&WebGlTexture> {
        if let Some(tex) = &self.tex_specular {
            return Some(tex);
        }
        None
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Material {}

impl Hash for Material {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
