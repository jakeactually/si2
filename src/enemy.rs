use crate::types;
use crate::util;

use types::{Enemy, Game, Vec2, WIDTH};
use ggez::{GameResult};

impl Enemy {
    pub fn tick(mut self, game: &mut Game) -> GameResult<Enemy> {
        let screen_x = game.enemies_x + self.position.x;

        if screen_x > WIDTH as i32 {
            return Ok(self);
        }

        if game.enemies_x % 4 == 0 {
            self.anim_state = (self.anim_state + 1) % self.data.anim_count;
        }

        if screen_x > WIDTH as i32 / 4 * 3 - 10 {
            return Ok(self);
        }

        if self.data.floats {
            self.position.x += 1;
        }

        let obj = game.load_object(self.data.model_id as u8)?;
        let bullet = game.static_objects[20].clone();

        self.oscillation();

        if screen_x > -100 && screen_x < 940 {
            let collission = util::intersect(
                game.player.position.clone(),
                Vec2 { x: 10, y: 7 },
                Vec2 { x: screen_x, y: self.position.y },
                obj.size.clone()
            );

            if collission {
                if self.is_bonus() {
                    self.data.lives = 0;
                    self.explosion_frames = 0;
                    game.bonus += 3;
                } else {
                    if !game.player.protected() {                        
                        game.player.lives -= 1;
                        game.player.protection = 50;
                    }
                    
                    self.data.lives -= 1;
                }
            }

            for shot in game.shots.iter_mut() {
                let collission = util::intersect(
                    shot.position.clone(),
                    bullet.size.clone(),
                    Vec2 { x: screen_x, y: self.position.y },
                    obj.size.clone()
                );
                
                if collission {
                    if self.is_bonus() {
                    } else {
                        shot.active = false;
                        self.data.lives -= 1;
                    }
                }
            }

            if !self.alive() && self.explosion_frames > 0 {
                self.explosion_frames -= 1;
            }
        }

        Ok(self)
    }

    pub fn oscillation(&mut self) {
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
    }

    pub fn render(self, game: &mut Game) -> GameResult<()> {
        let obj = game.load_object(self.data.model_id + self.anim_state)?;
        let screen_x = game.enemies_x + self.position.x;

        if self.alive(){
            game.render_object(&obj, screen_x, self.position.y)?;
        } else if self.explosion_frames > 0 {
            let explosion = game.static_objects[22 - (self.explosion_frames as usize - 1) / 3].clone();
            game.render_object(&explosion, screen_x, self.position.y)?;
        }

        Ok(())
    }
}
