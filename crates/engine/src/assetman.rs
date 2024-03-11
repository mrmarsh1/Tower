use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    io::{Cursor, Read},
    rc::Rc,
};

use image::GenericImageView;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use utils::log;
use wasm_bindgen::JsCast;

use crate::types::{MeshData, Texture, Tlu};

pub struct AssetMan {
    assets: Rc<RefCell<HashMap<String, Box<dyn Any>>>>,
}

impl AssetMan {
    pub async fn init(assets: Vec<&str>) -> Result<AssetMan, JsValue> {
        let assetman = AssetMan {
            assets: Rc::new(RefCell::new(HashMap::new())),
        };

        for asset in assets {
            if asset.ends_with(".tlu") {
                assetman.load_shader(asset, None).await?;
            } else if asset.ends_with(".tmf") {
                assetman.load_mesh(asset).await?;
            } else if asset.ends_with(".png") {
                assetman.load_texture(asset).await?;
            }
        }

        Ok(assetman)
    }

    pub async fn get_file(path: &str) -> Result<JsValue, JsValue> {
        let mut options = RequestInit::new();
        options.method("GET");
        //options.mode(RequestMode::Cors);
        let req = Request::new_with_str_and_init(path, &options)?;
        let window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;
        let resp: Response = resp_value.dyn_into().unwrap();
        let buffer = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();
        Ok(buffer)
    }

    pub async fn load_shader(&self, path: &str, includes: Option<Vec<&str>>) -> Result<JsValue, JsValue> {
        let buffer = Self::get_file(path).await.unwrap();
        let bytes = js_sys::Uint8Array::new(&buffer);

        let mut includes_map_opt = None;
        if let Some(includes) = includes {
            let mut includes_map = HashMap::new();
            for path in includes  {
                let buffer = Self::get_file(path).await.unwrap();
                let bytes = js_sys::Uint8Array::new(&buffer);
                let name = path.split('/').last().unwrap();
                includes_map.insert(name.to_string(), String::from_utf8(bytes.to_vec()).unwrap());
            }
            includes_map_opt = Some(includes_map);
        }

        let tlu = Tlu::parse(String::from_utf8(bytes.to_vec()).unwrap(), includes_map_opt);
        self.assets
            .borrow_mut()
            .insert(path.to_string(), Box::new(Rc::new(tlu)));
        Ok((JsValue::null()))
    }

    pub async fn load_mesh(&self, path: &str) -> Result<JsValue, JsValue> {
        let buffer = Self::get_file(path).await.unwrap();
        let bytes = js_sys::Uint8Array::new(&buffer);

        let mut cursor = Cursor::new(bytes.to_vec());

        let obj_name_len = read_header(&mut cursor);
        let mut buf = vec![0; obj_name_len];
        cursor.read_exact(&mut buf).unwrap();

        let obj_name = String::from_utf8(buf).unwrap();

        let vertex_count = read_header(&mut cursor);
        let mut positions = vec![0f32; vertex_count * 3];
        for i in 0..positions.len() {
            positions[i] = read_float(&mut cursor);
        }

        let normals_count = read_header(&mut cursor);
        let mut normals = vec![0f32; normals_count * 3];

        for i in 0..normals.len() {
            normals[i] = read_float(&mut cursor);
        }

        let uv0_count = read_header(&mut cursor);
        let mut uv0 = vec![0f32; uv0_count * 2];
        for i in 0..uv0.len() {
            uv0[i] = read_float(&mut cursor);
        }

        let index_count = read_header(&mut cursor);
        let mut indices = vec![0u32; index_count];
        for i in 0..indices.len() {
            indices[i] = read_uint(&mut cursor);
        }

        let mesh_data = MeshData::new(positions, normals, uv0, indices);
        self.assets
            .borrow_mut()
            .insert(path.to_string(), Box::new(Rc::new(mesh_data)));

        log(format!("loaded mesh: {}", obj_name).as_str());
        Ok((JsValue::null()))
    }

    pub async fn load_texture(&self, path: &str) -> Result<JsValue, JsValue> {
        let buffer = Self::get_file(path).await.unwrap();
        let bytes = js_sys::Uint8Array::new(&buffer);
        let bytes = bytes.to_vec();

        let image = image::load_from_memory(&bytes).unwrap();
        let (w, h) = image.dimensions();
        let pixels = image.to_rgba8().to_vec();

        let texture = Texture::new(w as i32, h as i32, pixels);
        self.assets
            .borrow_mut()
            .insert(path.to_string(), Box::new(Rc::new(texture)));
        Ok((JsValue::null()))
    }

    pub fn get_asset<T: 'static>(&self, path: String) -> Option<Rc<T>> {
        if let Some(asset) = self.assets.borrow_mut().get(&path) {
            let r = Rc::clone(asset.downcast_ref::<Rc<T>>().unwrap());
            return Some(r);
        }
        None
    }
}

fn read_header(cursor: &mut Cursor<Vec<u8>>) -> usize {
    read_uint(cursor) as usize
}

fn read_int(cursor: &mut Cursor<Vec<u8>>) -> i32 {
    let mut buf = [0; 4];
    cursor.read_exact(&mut buf).unwrap();
    i32::from_le_bytes(buf)
}

fn read_uint(cursor: &mut Cursor<Vec<u8>>) -> u32 {
    let mut buf = [0; 4];
    cursor.read_exact(&mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn read_float(cursor: &mut Cursor<Vec<u8>>) -> f32 {
    let mut buf = [0; 4];
    cursor.read_exact(&mut buf).unwrap();
    f32::from_le_bytes(buf)
}