use crate::objects::Graphics;

use std::collections::HashMap;

pub const WIDTH: u8 = 84;
pub const HEIGHT: u8 = 48;
pub const PLAYER_WIDTH: u8 = 10;
pub const PLAYER_HEIGHT: u8 = 7;

/* Fájlba áthelyezett objektumok */
pub const G_PROTECTION_A1: u8 = 250;
pub const G_PROTECTION_A2: u8 = 251;
pub const G_MISSILE: u8 = 252;
pub const G_BEAM: u8 = 253;
pub const G_WALL: u8 = 254;
pub const G_PLAYER: u8 = 255;

#[derive(Clone, Debug)]
pub struct Object {
    pub size: Vec2,
    pub data: Vec<u8>
}

pub struct Game {
    pub screen: [[u8; WIDTH as usize]; HEIGHT as usize],
    pub inverted: bool,

    pub static_objects: Vec<Object>,
    pub weapons: Vec<Object>,
    pub objects_cache: HashMap<u8, Object>,
    pub enemies_cache: HashMap<u8, EnemyData>,

    pub scenery: Vec<Scenery>,
    pub enemies: Vec<Enemy>,
    pub shots: Vec<Shot>,

    pub is_playing: bool,
    pub game_over: bool,
    pub level: u8,
    pub time: u32,
    pub scene_x: i32,
    pub enemies_x: i32,

    pub player: Player,
    pub y_axis: Vec2,
    pub weapon: Weapon,
    pub score: u32
}

#[derive(Clone)]
pub struct Player {
    pub position: Vec2,
    pub lives: u8,
    pub protection: u8,
}

impl Player {
    pub fn protected(&self) -> bool {
        self.protection > 0
    }
}

#[derive(Clone)]
pub struct Weapon {
    pub amount: u8,
    pub kind: WeaponKind
}

#[derive(Clone)]
pub struct Enemy {
    pub id: u8,
    pub position: Vec2,
    pub dir: i32,
    pub data: EnemyData,
    pub explosion_frames: u8,
    pub anim_state: u8
}

impl Enemy {
    pub fn alive(&self) -> bool {
        self.data.lives > 0
    }

    pub fn is_bonus(&self) -> bool {
        self.data.lives == 127
    }

    pub fn active(&self) -> bool {
        self.alive() || self.explosion_frames > 0
    }

    pub fn damage(&mut self) {
        if self.alive() {
            self.data.lives -= 1;
        }
    }

    pub fn die(&mut self) {
        if !self.alive() && self.explosion_frames > 0 {
            self.explosion_frames -= 1;
        }
    }

    pub fn delete(&mut self) {        
        self.data.lives = 0;
        self.explosion_frames = 0;
    }
}

#[derive(Clone)]
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
        Vec2 { x: self.x + 1, ..self }
    }

    pub fn approach_y(self, ny: i32) -> Self {
        if self.y > ny {
            Vec2 { y: self.y - 1, ..self }
        } else if self.y < ny {
            Vec2 { y: self.y + 1, ..self }
        } else {
            self
        }
    }
}

#[derive(Clone, Debug)]
pub struct Shot {
    pub position: Vec2,
    pub active: bool,
    pub weapon_kind: WeaponKind,
    pub duration: u8,
}

impl Shot {
    pub fn tick(self, nearest_y: i32) -> Shot {
        use WeaponKind::*;

        match self.weapon_kind {
            Standard => Shot { position: self.position.left(), ..self },
            Missile => Shot {
                position: self.position.left().approach_y(nearest_y),
                ..self
            },
            Beam => Shot {
                position: self.position.left(),
                duration: self.duration - 1,
                active: self.duration > 1,
                ..self
            },
            Wall => Shot { position: self.position.left(), ..self }
        }
    }

    pub fn crash(&mut self) {
        use WeaponKind::*;

        match self.weapon_kind {
            Standard => self.active = false,
            Missile => self.active = false,
            Beam => (),
            Wall => ()
        }
    }
}

#[derive(Clone)]
pub struct Scenery {
    pub position: Vec2,
    pub model: Object
}

#[derive(Clone)]
pub struct SceneryData {
    pub first_object: u8,
    pub objects: u8,
    pub upper: u8,
    pub inverted_color: bool
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponKind {
    Standard,
    Missile,
    Beam,
    Wall
}

impl WeaponKind {
    pub fn default_amount(self) -> u8 {
        use WeaponKind::*;

        match self {
            Standard => 0,
            Missile => 3,
            Beam => 2,
            Wall => 1
        }
    }

    pub fn model(self, game: &Game) -> Object {
        if self == WeaponKind::Standard {
            game.static_objects[Graphics::GShot as usize].clone()
        } else {
            let index = self as usize - 1;
            game.weapons[index].clone()
        }
    }
}

impl From<u8> for WeaponKind {
    fn from(n: u8) -> WeaponKind {
        use WeaponKind::*;

        match n {
            1 => Missile,
            2 => Beam,
            3 => Wall,
            _ => Standard
        }
    }
}
