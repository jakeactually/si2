use crate::types;
use crate::objects;

use types::{Game, Vec2, Shot};
use ggez::event::{KeyCode};
use ggez::{Context, GameResult};
use std::collections::{HashMap};
use ggez::input::keyboard;

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {
            screen: [[0; 84]; 48],
            static_objects: objects::get_static_objects().to_vec(),
            frame: 0,
            real_frame: 0,
            main_color: 1,
            secondary_color: 0,

            player_position: Vec2 { x: 3, y: 20 },
            objects_cache: HashMap::new(),
            enemies_cache: HashMap::new(),
            shots: vec![],
            
            enemies: vec![],
            is_playing: false
        }
    }

    pub fn keyboard(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(_ctx, KeyCode::Right) && self.player_position.x < 84 - 10 {
            self.player_position.x += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) && self.player_position.x > 0 {
            self.player_position.y -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Up) && self.player_position.y > 0 {
            self.player_position.y -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Down) && self.player_position.y < 48 - 7 {
            self.player_position.y += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Space) && self.frame % 6 == 0 {
            let position = Vec2 { x: self.player_position.x + 9, y: self.player_position.y + 3 };
            let shot = Shot { position, dirty: false };
            self.shots.push(shot);
        }

        Ok(())
    }

    pub fn intro(&mut self) -> GameResult<()> {
        let rel_time = if self.frame < 12 { self.frame as i32 } else { 12 };

        let space = self.static_objects[10].clone();
        self.render_object(&space, 8, rel_time)?;

        let impact = self.static_objects[12].clone();
        self.render_object(&impact, 4, 38 - rel_time)?;

        let intro = self.static_objects[11].clone();
        self.render_outlined_object(&intro, rel_time * 4 + 4, 21)?;

        Ok(())
    }
}
