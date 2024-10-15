use crate::types::{Enemy, Game, Vec2, WIDTH, WeaponKind};
use crate::util;
use crate::objects::Graphics;

use ggez::{GameResult};
use rand::Rng;

impl Enemy {
    pub fn tick(mut self, game: &mut Game) -> GameResult<Enemy> {
        let screen_x = game.enemies_x + self.position.x;

        if screen_x > WIDTH as i32 {
            return Ok(self);
        }

        if game.time % 4 == 0 {
            self.anim_state = (self.anim_state + 1) % self.data.anim_count;
        }

        self.collision(game)?;

        if screen_x > WIDTH as i32 / 4 * 3 - 10 {
            return Ok(self);
        }

        self.oscillation();

        if self.data.floats {
            self.position.x += 1;
        }

        Ok(self)
    }

    pub fn collision(&mut self, game: &mut Game) -> GameResult<()> {
        let screen_x = game.enemies_x + self.position.x;
        let obj = game.load_object(self.data.model_id as u8)?;

        if screen_x > -100 && screen_x < 940 {
            let collission = util::intersect(
                game.player.position.clone(),
                Vec2 { x: 10, y: 7 },
                Vec2 { x: screen_x, y: self.position.y },
                obj.size.clone()
            );

            if collission {
                self.object_collision(game);
            }

            game.shots = game.shots.clone().into_iter().map(|mut shot| {
                let bullet = shot.weapon_kind.clone().model(game);

                let collission = util::intersect(
                    shot.position.clone(),
                    bullet.size.clone(),
                    Vec2 { x: screen_x, y: self.position.y },
                    obj.size.clone()
                );
                
                if collission {
                    if self.is_bonus() {
                    } else {
                        shot.crash();                        
                        self.damage();
                    }
                }

                shot
            }).collect();

            if game.time % 2 == 0 {
                self.die();
            }
        }

        Ok(())
    }

    pub fn object_collision(&mut self, game: &mut Game) {
        if self.is_bonus() {
            self.delete();

            let mut rng = rand::thread_rng();
            let id: u8 = rng.gen_range(0..4);
            let wk: WeaponKind = id.into();

            if game.weapon.kind == wk {
                game.weapon.amount += wk.default_amount();
            } else {
                game.weapon.kind = id.into();
                game.weapon.amount = wk.default_amount();
            }
        } else {
            if !game.player.protected() {                        
                game.player.lives -= 1;
                game.player.protection = 50;
            }
            
            self.damage();
        }
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

        if self.alive() {
            game.render_object(&obj, screen_x, self.position.y)?;
        } else if self.explosion_frames > 0 {
            let explosion = game.static_objects[Graphics::GExplosionA1 as usize - self.explosion_frames as usize + 2].clone();
            game.render_object(&explosion, screen_x, self.position.y)?;
        }

        Ok(())
    }
}
