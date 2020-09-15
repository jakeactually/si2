use std::collections::HashMap;
use std::io::Result;
use std::collections::{HashSet};

#[derive(Clone)]
pub struct Object {
    pub size: Vec2,
    pub data: Vec<u8>
}

pub struct Game {
    pub screen: [[u8; 84]; 48],
    pub static_objects: Vec<Object>,
    pub frame: u32,
    pub real_frame: u32,
    pub main_color: u8,
    pub secondary_color: u8,

    pub player_position: Vec2,
    pub objects_cache: HashMap<u8, Object>,
    pub enemies_cache: HashMap<u8, EnemyData>,
    pub shots: Vec<Shot>,

    pub enemies: Vec<Enemy>,
    pub is_playing: bool
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub id: u8,
    pub position: Vec2,
    pub dir: i32,
    pub data: EnemyData,
    pub alive: bool,
    pub explosion_frames: u8,
    pub anim_state: u8
}

#[derive(Clone, Debug)]
pub struct EnemyData {
    pub model_id: u8,
    pub size: Vec2,
    pub anim_count: u8,
    pub lives: i8,
    pub floats: bool,
    pub shot_time: u8,
    pub move_up: bool,
    pub move_down: bool,
    pub move_anyway: bool,
    pub moves_between: Vec2
}

#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32
}

impl Vec2 {
    pub fn left(self) -> Self {
        Vec2 { x: self.x + 1, y: self.y }
    }
}

#[derive(Clone)]
pub struct Shot {
    pub position: Vec2,
    pub dirty: bool
}
