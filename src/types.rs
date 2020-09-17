use std::collections::HashMap;

pub const WIDTH: u8 = 84;
pub const HEIGHT: u8 = 48;

#[derive(Clone)]
pub struct Object {
    pub size: Vec2,
    pub data: Vec<u8>
}

pub struct Game {
    pub screen: [[u8; 84]; 48],
    pub static_objects: Vec<Object>,
    pub time: u32,
    pub scene_x: i32,
    pub enemies_x: i32,
    pub main_color: u8,
    pub secondary_color: u8,

    pub player_position: Vec2,
    pub player_lives: u8,
    pub player_weapon: WeaponKind,
    pub bonus: u8,
    pub score: u32,
    pub objects_cache: HashMap<u8, Object>,
    pub enemies_cache: HashMap<u8, EnemyData>,
    pub shots: Vec<Shot>,

    pub moving: bool,
    pub level: u8,
    pub enemies: Vec<Enemy>,
    pub is_playing: bool,
    pub scenery: Vec<Scenery>
}

#[derive(Clone, Debug)]
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
    pub active: bool
}

impl Shot {
    pub fn left(self) -> Shot {
        Shot { position: self.position.left(), active: self.active }
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
    pub upper: u8
}

pub enum Graphics {
    /* Számok, 3*5-ös méretben */
    GNum0, GNum1, GNum2, GNum3, GNum4, GNum5, GNum6, GNum7, GNum8, GNum9,
    /* Menüelemek */
    GSpace, GIntro, GImpact, GScrollMark, GDotEmpty, GDotFull,
    /* Játékosssal kapcsolatos modellek és ikonok */
    GLife, /* Életjel */
    GMissileIcon, /* Rakéta ikonja */
    GBeamIcon,/* SuGár ikonja */
    GWallIcon, /* Fal ikonja */
    GShot, /* Lövés */
    GExplosionA1, GExplosionA2 /* Robbanás animáció 2 lépése */
}

pub const scenery_data: [SceneryData; 6] = [
    SceneryData { first_object: 0, objects: 0, upper: 0 }, /* Az 1. szinten nincs táj */
    SceneryData { first_object: 0, objects: 2, upper: 0 }, /* 2. szint, 0. dinamikus helytől 2 elemű, 700 pixel széles táj */
    SceneryData { first_object: 2, objects: 6, upper: 0 }, /* 3. szint, 2. dinamikus helytől 6 elemű, 750 pixel széles táj */
    SceneryData { first_object: 8, objects: 6, upper: 0 }, /* 4. szint, 8. dinamikus helytől 6 elemű, 1000 pixel széles táj */
    SceneryData { first_object: 14, objects: 4, upper: 1 }, /* 5. szint, 14. dinamikus helytől 4 elemű, 1250 pixel széles felső táj */
    SceneryData { first_object: 14, objects: 4, upper: 1 }, /* 6. szint, az 5. szint elemeiből, 1600 pixel szélesen */
];

#[derive(Copy, Clone)]
pub enum WeaponKind {
    Standard,
    Missile,
    Beam,
    Wall
}
