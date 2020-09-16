use crate::types;
use crate::objects;

use types::{Game, Vec2, Shot, WIDTH, HEIGHT, WeaponKind, scenery_data, Scenery};
use ggez::event::{KeyCode};
use ggez::{Context, GameResult};
use std::collections::{HashMap};
use ggez::input::keyboard;

use rand::Rng;

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            screen: [[0; WIDTH as usize]; HEIGHT as usize],
            static_objects: objects::get_static_objects().to_vec(),
            scene_x: 0,
            enemies_x: 0,
            main_color: 1,
            secondary_color: 0,

            player_position: Vec2 { x: 3, y: 20 },
            player_lives: 3,
            player_weapon: WeaponKind::Standard,
            bonus: 3,
            score: 0,
            objects_cache: HashMap::new(),
            enemies_cache: HashMap::new(),
            shots: vec![],
            
            moving: true,
            level: 1,
            enemies: vec![],
            is_playing: false,
            scenery: vec![]
        }
    }

    pub fn keyboard(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(_ctx, KeyCode::Right) && self.player_position.x < WIDTH as i32 - 10 {
            self.player_position.x += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) && self.player_position.x > 0 {
            self.player_position.x -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Up) && self.player_position.y > 5 {
            self.player_position.y -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Down) && self.player_position.y < HEIGHT as i32 - 7 {
            self.player_position.y += 1;
        }
        
        if keyboard::is_key_pressed(_ctx, KeyCode::Space) && self.scene_x % 6 == 0 {
            let position = Vec2 { x: self.player_position.x + 9, y: self.player_position.y + 3 };
            let shot = Shot { position, dirty: false };
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
