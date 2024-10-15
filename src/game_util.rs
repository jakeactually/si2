use std::collections::HashMap;

use crate::objects::{get_static_objects, get_weapons, scenery_data};
use crate::types::*;

use ggez::{
    event, graphics,
    input::keyboard::{KeyCode, KeyInput},
    Context, GameResult,
};

use rand::Rng;

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            screen: [[0; WIDTH as usize]; HEIGHT as usize],
            inverted: false,

            static_objects: get_static_objects().to_vec(),
            weapons: get_weapons().to_vec(),
            objects_cache: HashMap::new(),
            enemies_cache: HashMap::new(),

            scenery: vec![],
            enemies: vec![],
            shots: vec![],

            is_playing: false,
            game_over: false,
            level: 0,
            time: 0,
            scene_x: 0,
            enemies_x: 0,

            player: Player {
                position: Vec2 { x: 3, y: 20 },
                lives: 3,
                protection: 0,
            },
            y_axis: Vec2 {
                x: 5,
                y: HEIGHT as i32 - PLAYER_HEIGHT as i32,
            },
            weapon: Weapon {
                amount: 3,
                kind: WeaponKind::Missile,
            },
            score: 0,
        }
    }

    pub fn keyboard(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let position = &mut self.player.position;

        if _ctx.keyboard.is_key_pressed(KeyCode::Right)
            && position.x < WIDTH as i32 - PLAYER_WIDTH as i32
        {
            position.x += 1;
        } else if _ctx.keyboard.is_key_pressed(KeyCode::Left) && position.x > 0 {
            position.x -= 1;
        } else if _ctx.keyboard.is_key_pressed(KeyCode::Up) && position.y > self.y_axis.x {
            position.y -= 1;
        } else if _ctx.keyboard.is_key_pressed(KeyCode::Down) && position.y < self.y_axis.y {
            position.y += 1;
        }

        if self.time % 6 == 0 {
            use KeyCode::*;

            if _ctx.keyboard.is_key_pressed(Space) {
                let position = Vec2 {
                    x: position.x + 9,
                    y: position.y + 3,
                };
                let shot = Shot {
                    position,
                    active: true,
                    weapon_kind: WeaponKind::Standard,
                    duration: 3,
                };
                self.shots.push(shot);
            } else if _ctx.keyboard.is_key_pressed(LAlt) || _ctx.keyboard.is_key_pressed(RAlt) {
                if self.weapon.amount > 0 {
                    self.weapon.amount -= 1;
                    let y = if self.weapon.kind == WeaponKind::Wall {
                        5
                    } else {
                        position.y + 3
                    };

                    let position = Vec2 {
                        x: position.x + 9,
                        y: y,
                    };
                    let shot = Shot {
                        position,
                        active: true,
                        weapon_kind: self.weapon.kind.clone(),
                        duration: 3,
                    };
                    self.shots.push(shot);
                }
            }
        }

        Ok(())
    }

    pub fn load_scenery(&mut self) -> GameResult<()> {
        self.scenery = vec![];

        let mut x = 0;
        let mut rng = rand::thread_rng();

        if self.level > 0 {
            while x < 1600 {
                let sd = &scenery_data[self.level as usize];
                let n: u8 = rng.gen_range(sd.first_object..sd.first_object + sd.objects);
                let rock = self.load_object(n)?;
                let y = if self.level_data().upper == 1 {
                    0
                } else {
                    HEIGHT as i32 - rock.size.y
                };

                self.scenery.push(Scenery {
                    position: Vec2 { x, y },
                    model: rock.clone(),
                });
                x += rock.size.x;
            }
        }

        Ok(())
    }

    pub fn level_data(&self) -> SceneryData {
        scenery_data[self.level as usize].clone()
    }
}
