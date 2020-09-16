use crate::types;
use crate::util;

use types::{Enemy, Game, Vec2};
use ggez::{GameResult};
use std::collections::{HashSet};

impl Enemy {
    pub fn tick(&mut self, deleted_shots: &mut HashSet<u8>, game: &mut Game) -> GameResult<()> {
        let screen_x = self.position.x - game.frame as i32;

        if screen_x > 100 {
            return Ok(());
        }

        let obj = game.load_object(self.data.model_id as u8)?;
        let bullet = game.static_objects[20].clone();

        if self.dir == 1 {
            if self.position.y < self.data.moves_between.y {
                self.position.y += 1;
            } else {
                self.dir = if self.data.move_up { -1 } else { 0 };
            }
        }

        if self.dir == -1 {
            if self.position.y > self.data.moves_between.x {
                self.position.y -= 1;
            } else {
                self.dir = if self.data.move_down { 1 } else { 0 };
            }
        }

        if !self.data.floats {
            //self.position.x -= 1;
        }

        if game.frame % 2 == 0 {
            self.anim_state = (self.anim_state + 1) % self.data.anim_count;
        }

        if screen_x > -100 && screen_x < 940 {
            let collission = util::intersect(
                game.player_position.clone(),
                Vec2 { x: 10, y: 7 },
                Vec2 { x: screen_x, y: self.position.y },
                obj.size.clone()
            );

            for (i, shot) in game.shots.iter().enumerate() {
                let collission = util::intersect(
                    shot.position.clone(),
                    bullet.size.clone(),
                    Vec2 { x: screen_x, y: self.position.y },
                    obj.size.clone()
                );
                
                if collission {
                    self.alive = false;
                    deleted_shots.insert(i as u8);
                }
            }

            if !self.alive && self.explosion_frames > 0 {
                self.explosion_frames -= 1;
            }
        }

        Ok(())
    }

    pub fn render(self, game: &mut Game) -> GameResult<()> {
        let obj = game.load_object(self.data.model_id as u8 + self.anim_state)?;
        let screen_x = self.position.x - game.frame as i32;

        if self.alive {
            game.render_object(&obj, screen_x, self.position.y)?;
        } else if self.explosion_frames > 0 {
            let explosion = game.static_objects[22 - (self.explosion_frames as usize - 1) / 3].clone();
            game.render_object(&explosion, screen_x, self.position.y)?;
        }

        Ok(())
    }
}
