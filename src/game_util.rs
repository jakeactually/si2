use crate::types;
use crate::objects;

use types::{Game, Vec2, Shot, WIDTH, HEIGHT, WeaponKind, scenery_data, Scenery, Player};
use ggez::event::{KeyCode};
use ggez::{Context, GameResult};
use std::collections::{HashMap};
use ggez::input::keyboard;

use rand::Rng;

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            screen: [[0; WIDTH as usize]; HEIGHT as usize],
            main_color: 1,
            secondary_color: 0,

            static_objects: objects::get_static_objects().to_vec(),
            objects_cache: HashMap::new(),
            enemies_cache: HashMap::new(),

            scenery: vec![],
            enemies: vec![],
            shots: vec![],

            is_playing: false,
            level: 1,
            time: 0,
            scene_x: 0,
            enemies_x: 0,

            player: Player {
                position: Vec2 { x: 3, y: 20 },
                lives: 3,
                weapon: WeaponKind::Standard,
                protection: 0,
            },
            bonus: 3,
            score: 0,
        }
    }

    pub fn keyboard(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let position = &mut self.player.position;

        if keyboard::is_key_pressed(_ctx, KeyCode::Right) && position.x < WIDTH as i32 - 10 {
            position.x += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) && position.x > 0 {
            position.x -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Up) && position.y > 5 {
            position.y -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Down) && position.y < HEIGHT as i32 - 7 {
            position.y += 1;
        }
        
        if keyboard::is_key_pressed(_ctx, KeyCode::Space) && self.time % 6 == 0 {
            let position = Vec2 { x: position.x + 9, y: position.y + 3 };
            let shot = Shot { position, active: true };
            self.shots.push(shot);
        }

        Ok(())
    }

    pub fn intro(&mut self) -> GameResult<()> {
        let rel_time = if self.scene_x < 12 { self.scene_x as i32 } else { 12 };

        let space = self.static_objects[10].clone();
        self.render_object(&space, 8, rel_time)?;

        let impact = self.static_objects[12].clone();
        self.render_object(&impact, 4, 38 - rel_time)?;

        let intro = self.static_objects[11].clone();
        self.render_outlined_object(&intro, rel_time * 4 + 4, 21)?;

        Ok(())
    }

    pub fn load_scenery(&mut self) -> GameResult<()> {
        let mut x = 0;
        let mut rng = rand::thread_rng();

        if self.level > 0 {
            while x < 1600 {
                let sd = &scenery_data[self.level as usize];
                let n: u8 = rng.gen_range(sd.first_object, sd.first_object + sd.objects);
                let rock = self.load_object(n)?;

                self.scenery.push(Scenery {
                    position: Vec2 { x: x, y: HEIGHT as i32 - rock.size.y },
                    model: rock.clone()
                });

                x += rock.size.x;
            }
        }

        Ok(())
    }
}
