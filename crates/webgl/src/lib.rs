use std::{collections::HashMap, rc::Rc};

use math::{Vector2f, Vector3f};
use web_sys::{
    WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

pub enum DrawMode {
    Static,
    Dynamic,
}

pub struct ShaderProgramHandle {
    context: Rc<WebGl2RenderingContext>,
    program: Rc<WebGlProgram>,
}

impl ShaderProgramHandle {
    pub fn program(&self) -> Rc<WebGlProgram> {
        Rc::clone(&self.program)
    }

    pub fn delete(&self) {
        self.context.delete_program(Some(&self.program));
    }
}

pub struct MeshHandle {
    context: Rc<WebGl2RenderingContext>,
    vao: Rc<WebGlVertexArrayObject>,
    vbo: WebGlBuffer,
}

impl MeshHandle {
    pub fn vao(&self) -> Rc<WebGlVertexArrayObject> {
        Rc::clone(&self.vao)
    }

    pub fn delete(&self) {
        self.context.delete_buffer(Some(&self.vbo));
        self.context.delete_vertex_array(Some(&self.vao));
    }
}

pub struct TextureHandle {
    context: Rc<WebGl2RenderingContext>,
    texture: Rc<WebGlTexture>,
}

impl TextureHandle {
    pub fn texture(&self) -> Rc<WebGlTexture> {
        Rc::clone(&self.texture)
    }

    pub fn delete(&self) {
        self.context.delete_texture(Some(&self.texture))
    }
}

use utils::log;

pub struct WebGlRenderer {
    context: Rc<WebGl2RenderingContext>,
}

impl WebGlRenderer {
    pub fn new(context: WebGl2RenderingContext) -> Self {
        WebGlRenderer {
            context: Rc::new(context),
        }
    }

    fn compile_shader(&self, shader: &WebGlShader) {
        self.context.compile_shader(&shader);
        if !self
            .context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap()
        {
            if let Some(info_log) = self.context.get_shader_info_log(&shader) {
                log(format!("{}", info_log).as_str());
            }
        }
    }

    fn get_uniform_location(
        &self,
        program: &WebGlProgram,
        name: &str,
    ) -> Option<WebGlUniformLocation> {
        self.context.get_uniform_location(program, name)
    }
}

impl WebGlRenderer {
    pub fn set_viewport(&self, width: i32, height: i32) {
        self.context.viewport(0, 0, width, height)
    }

    pub fn create_program(&self, vertex_src: &str, fragment_src: &str) -> ShaderProgramHandle {
        let vertex_shader = self
            .context
            .create_shader(WebGl2RenderingContext::VERTEX_SHADER)
            .unwrap();
        self.context.shader_source(&vertex_shader, vertex_src);
        self.compile_shader(&vertex_shader);

        let fragment_shader = self
            .context
            .create_shader(WebGl2RenderingContext::FRAGMENT_SHADER)
            .unwrap();
        self.context.shader_source(&fragment_shader, fragment_src);
        self.compile_shader(&fragment_shader);

        let program = self.context.create_program().unwrap();
        self.context.attach_shader(&program, &vertex_shader);
        self.context.attach_shader(&program, &fragment_shader);
        self.context.link_program(&program);

        if !self
            .context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap()
        {
            if let Some(info_log) = self.context.get_program_info_log(&program) {
                log(format!("{}", info_log).as_str());
            }
        }

        self.context.delete_shader(Some(&vertex_shader));
        self.context.delete_shader(Some(&fragment_shader));

        ShaderProgramHandle {
            context: Rc::clone(&self.context),
            program: Rc::new(program),
        }
    }

    pub fn use_program(&self, program: Option<&WebGlProgram>) {
        self.context.use_program(program);
    }

    pub fn set_uniform_vector3(&self, program: &WebGlProgram, name: &str, value: &[f32]) {
        if let Some(location) = self.get_uniform_location(program, name) {
            self.context
                .uniform3fv_with_f32_array(Some(&location), value);
        }
    }

    pub fn set_uniform_matrix4(&self, program: &WebGlProgram, name: &str, value: &[f32]) {
        if let Some(location) = self.get_uniform_location(program, name) {
            self.context
                .uniform_matrix4fv_with_f32_array(Some(&location), false, value);
        }
    }

    pub fn set_uniform_int(&self, program: &WebGlProgram, name: &str, value: i32) {
        if let Some(location) = self.get_uniform_location(program, name) {
            self.context.uniform1i(Some(&location), value);
        }
    }

    pub fn set_uniform_float(&self, program: &WebGlProgram, name: &str, value: f32) {
        if let Some(location) = self.get_uniform_location(program, name) {
            self.context.uniform1f(Some(&location), value);
        }
    }

    pub fn create_mesh(
        &self,
        vertices: &[f32],
        indices: &[u32],
        draw_mode: DrawMode,
    ) -> MeshHandle {
        let vao = self.context.create_vertex_array().unwrap();
        self.context.bind_vertex_array(Some(&vao));

        let vbo = self.context.create_buffer().unwrap();

        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        let positions_array_buf_view = unsafe { js_sys::Float32Array::view(vertices) };

        let mode = match draw_mode {
            DrawMode::Static => WebGl2RenderingContext::STATIC_DRAW,
            DrawMode::Dynamic => WebGl2RenderingContext::DYNAMIC_DRAW,
        };

        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            mode,
        );

        // positions

        self.context.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            8 * 4,
            0,
        );
        self.context.enable_vertex_attrib_array(0);

        // normals

        self.context.vertex_attrib_pointer_with_i32(
            1,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            8 * 4,
            3 * 4,
        );
        self.context.enable_vertex_attrib_array(1);

        // uv0

        self.context.vertex_attrib_pointer_with_i32(
            2,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            8 * 4,
            6 * 4,
        );
        self.context.enable_vertex_attrib_array(2);

        let index_buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        let indices_array_buf_view = unsafe { js_sys::Uint32Array::view(indices) };
        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        self.context.bind_vertex_array(None);

        MeshHandle {
            context: Rc::clone(&self.context),
            vao: Rc::new(vao),
            vbo,
        }
    }

    pub fn create_skybox_mesh(&self, vertices: &[f32]) -> WebGlVertexArrayObject {
        let vao = self.context.create_vertex_array().unwrap();
        self.context.bind_vertex_array(Some(&vao));

        let vbo = self.context.create_buffer().unwrap();

        self.context
            .bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vbo));

        let positions_array_buf_view = unsafe { js_sys::Float32Array::view(vertices) };

        self.context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );

        // positions

        self.context.vertex_attrib_pointer_with_i32(
            0,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            3 * 4,
            0,
        );
        self.context.enable_vertex_attrib_array(0);

        self.context.bind_vertex_array(None);

        vao
    }

    pub fn use_mesh(&self, vao: Option<&WebGlVertexArrayObject>) {
        self.context.bind_vertex_array(vao);
    }

    pub fn create_texture(&self, data: &[u8], width: i32, height: i32) -> TextureHandle {
        let texture = self.context.create_texture().unwrap();
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_2D,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                width,
                height,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                Some(data),
            )
            .unwrap();

        //self.context.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        TextureHandle {
            context: Rc::clone(&self.context),
            texture: Rc::new(texture),
        }
    }

    pub fn create_cube_texture(&self, data: Vec<&[u8]>, width: i32, height: i32) -> TextureHandle {
        let texture = self.context.create_texture().unwrap();
        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, Some(&texture));

        for i in 0..data.len() {
            self.context
                .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                    WebGl2RenderingContext::TEXTURE_CUBE_MAP_POSITIVE_X + i as u32,
                    0,
                    WebGl2RenderingContext::RGBA as i32,
                    width,
                    height,
                    0,
                    WebGl2RenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    Some(data[i]),
                )
                .unwrap();
        }

        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_MIN_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_MAG_FILTER,
            WebGl2RenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_WRAP_S,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_WRAP_T,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGl2RenderingContext::TEXTURE_CUBE_MAP,
            WebGl2RenderingContext::TEXTURE_WRAP_R,
            WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
        );

        TextureHandle {
            context: Rc::clone(&self.context),
            texture: Rc::new(texture),
        }
    }

    pub fn use_texture(&self, slot: i32, texture: Option<&WebGlTexture>) {
        match slot {
            0 => self
                .context
                .active_texture(WebGl2RenderingContext::TEXTURE0),
            1 => self
                .context
                .active_texture(WebGl2RenderingContext::TEXTURE1),
            2 => self
                .context
                .active_texture(WebGl2RenderingContext::TEXTURE2),
            _ => (),
        }

        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
    }

    pub fn use_cube_texture(&self, slot: i32, texture: Option<&WebGlTexture>) {
        match slot {
            0 => self
                .context
                .active_texture(WebGl2RenderingContext::TEXTURE0),
            1 => self
                .context
                .active_texture(WebGl2RenderingContext::TEXTURE1),
            2 => self
                .context
                .active_texture(WebGl2RenderingContext::TEXTURE2),
            _ => (),
        }

        self.context
            .bind_texture(WebGl2RenderingContext::TEXTURE_CUBE_MAP, texture);
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
        self.context.clear(
            WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT,
        );
    }

    pub fn set_culling(&self, value: bool) {
        if value {
            self.context.enable(WebGl2RenderingContext::CULL_FACE);
        } else {
            self.context.disable(WebGl2RenderingContext::CULL_FACE);
        }
    }

    pub fn set_depth_test(&self, value: bool) {
        if value {
            self.context.enable(WebGl2RenderingContext::DEPTH_TEST);
        } else {
            self.context.disable(WebGl2RenderingContext::DEPTH_TEST);
        }
    }

    pub fn set_depth_mask(&self, value: bool) {
        if value {
            self.context.depth_mask(true);
        } else {
            self.context.depth_mask(false);
        }
    }

    pub fn set_depth_func(&self, func: &str) {
        match func {
            "never" => self.context.depth_func(WebGl2RenderingContext::NEVER),
            "less" => self.context.depth_func(WebGl2RenderingContext::LESS),
            "equal" => self.context.depth_func(WebGl2RenderingContext::EQUAL),
            "lequal" => self.context.depth_func(WebGl2RenderingContext::LEQUAL),
            "greater" => self.context.depth_func(WebGl2RenderingContext::GREATER),
            "notequal" => self.context.depth_func(WebGl2RenderingContext::NOTEQUAL),
            "gequal" => self.context.depth_func(WebGl2RenderingContext::GEQUAL),
            "always" => self.context.depth_func(WebGl2RenderingContext::ALWAYS),
            _ => self.context.depth_func(WebGl2RenderingContext::LESS),
        }
    }

    pub fn draw_arrays(&self, count: i32) {
        self.context
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, count);
    }

    pub fn draw(&self, count: i32) {
        self.context.draw_elements_with_i32(
            WebGl2RenderingContext::TRIANGLES,
            count,
            WebGl2RenderingContext::UNSIGNED_INT,
            0,
        );
    }

    pub fn update_state(&self, params: &HashMap<String, String>) {
        if let Some(param) = params.get("cull") {
            if param == "on" {
                self.set_culling(true);
            } else {
                self.set_culling(false);
            }
        } else {
            self.set_culling(false);
        }

        if let Some(param) = params.get("ztest") {
            if param == "on" {
                self.set_depth_test(true);
            } else {
                self.set_depth_test(false);
            }
        } else {
            self.set_depth_test(false);
        }

        if let Some(param) = params.get("depthfunc") {
            self.set_depth_func(param);
        } else {
            self.set_depth_func("always");
        }

        if let Some(param) = params.get("depthmask") {
            if param == "on" {
                self.set_depth_mask(true);
            } else {
                self.set_depth_mask(false);
            }
        } else {
            self.set_depth_mask(false);
        }
    }
}
