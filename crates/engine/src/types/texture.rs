pub struct Texture {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn new(width: i32, height: i32, data: Vec<u8>) -> Texture {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_data(&self) -> &[u8] {
        return self.data.as_slice();
    }
}