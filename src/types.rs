pub struct Object {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>
}

impl Clone for Object {
    fn clone(&self) -> Self {
        Object {
            width: self.width,
            height: self.height,
            data: self.data.clone()
        }
    }
}

pub struct MyGame {
    pub screen: [[u8; 84]; 48],
    pub static_objects: Vec<Object>,
    pub frame: u32
}
