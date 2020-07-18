use std::collections::HashMap;
use std::io::Result;

#[derive(Clone)]
pub struct Object {
    pub size: Vec2,
    pub data: Vec<u8>
}

pub struct MyGame {
    pub screen: [[u8; 84]; 48],
    pub static_objects: Vec<Object>,
    pub frame: u32,
    pub main_color: u8,
    pub secondary_color: u8,

    pub player_position: Vec2,
    pub objects_cache: HashMap<u8, Object>,
    pub enemies_cache: HashMap<u8, EnemyData>,
    pub shots: Vec<Shot>,

    pub enemies: Vec<Enemy>,
    pub is_playing: bool
}

#[derive(Clone)]
pub struct Enemy {
    pub id: u8,
    pub position: Vec2,
    pub dir: i32,
    pub data: EnemyData,
    pub alive: bool,
    pub explosion_frames: u8
}

#[derive(Clone)]
pub struct EnemyData {
    pub model_id: u8
}

#[derive(Clone)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

#[derive(Clone)]
pub struct Shot {
    pub position: Vec2,
    pub dirty: bool
}
