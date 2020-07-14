use std::collections::HashMap;
use std::io::Result;

#[derive(Clone)]
pub struct Object {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>
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
    pub enemies_cache: HashMap<u8, EnemyData>,
    pub shots: Vec<Vec2>
}

#[derive(Clone)]
pub struct Enemy {
    pub id: u8,
    pub x: i32,
    pub y: i32,
    pub dir: i32,
    pub data: EnemyData
}

#[derive(Clone)]
pub struct EnemyData {
    pub model_id: u8
}

#[derive(Clone)]
pub struct Vec2(pub i32, pub i32);
