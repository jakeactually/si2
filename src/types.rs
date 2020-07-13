use std::collections::HashMap;
use ggez::graphics::Color;

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
            data: self.data.clone(),
        }
    }
}

pub struct MyGame {
    pub screen: [[u8; 84]; 48],
    pub static_objects: Vec<Object>,
    pub frame: u32,
    pub main_color: u8,
    pub secondary_color: u8,

    pub player_x: i32,
    pub player_y: i32,
    pub objects_cache: HashMap<u8, Object>,
    pub enemies_cache: HashMap<u8, Object>,
}

pub struct Enemy {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub dir: i32
}
