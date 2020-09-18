use crate::types::{Game, WIDTH, G_PLAYER, G_PROTECTION_A1, Vec2, WeaponKind};
use crate::objects::Graphics;

use ggez::{Context, GameResult};

impl Game {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;
        self.keyboard(_ctx)?;

        if !self.is_playing {
            self.enemies = self.load_level(self.level)?;
            self.load_scenery()?; 

            self.player.position = Vec2 { x: 3, y: 20 };
            self.scene_x = 0;
            self.enemies_x = 0;
            self.level += 1;

            self.is_playing = true;
        }

        // Enemies

        if self.enemies.len() == 0 {
            self.player.position.x += 1;
        }

        if self.player.position.x > WIDTH as i32 + 20 {
            self.is_playing = false;
            return Ok(())
        }

        let enemies_x = self.enemies_x;

        self.enemies = self
            .enemies
            .clone()
            .into_iter()
            .filter(|e| e.active() && enemies_x + e.position.x > -20)
            .map(|e| e.tick(self))
            .collect::<GameResult<Vec<_>>>()?;

        // Shots

        let nearest_y = if self.enemies.len() > 0 {
            self.enemies[0].position.y
        } else {
            self.player.position.y
        };

        self.shots = self
            .shots
            .clone()
            .into_iter()
            .filter(|s| s.active && s.position.x < WIDTH as i32)
            .map(|s| s.tick(nearest_y))
            .collect();

        // The end

        if let Some(enemy) = self.enemies.last() {
            if self.enemies_x + enemy.position.x >= (WIDTH as i32 / 4) * 3 {
                self.scene_x -= 1;
            }
        }

        if self.player.protected() {
            self.player.protection -= 1;
        }

        self.time += 1;
        self.enemies_x -= 1;

        Ok(())
    }

    pub fn render(&mut self) -> GameResult<()> {
        if self.player.protected() {
            let player = self.load_object(G_PROTECTION_A1 + self.player.protection / 2 % 2)?;
            self.render_object(&player, self.player.position.x - 2, self.player.position.y - 2)?;
        } else {
            let player = self.load_object(G_PLAYER)?;
            self.render_object(&player, self.player.position.x, self.player.position.y)?;
        }

        let heart = self.static_objects[Graphics::GLife as usize].clone();
        for i in 0..self.player.lives {
            self.render_object(&heart, i as i32 * 6, 0)?;
        }

        let index = Graphics::GMissileIcon as usize + self.weapon.clone().kind as usize - 1;
        let weapon = self.static_objects[index].clone();
        self.render_object(&weapon, 33, 0)?;

        self.render_number(self.weapon.amount as u32, 2, 43, 0)?;
        self.render_number(self.score, 5, 71, 0)?;

        for scenery in self.scenery.clone() {
            if self.scene_x + scenery.position.x < WIDTH as i32 {
                self.render_object(&scenery.model, self.scene_x + scenery.position.x, scenery.position.y)?;
            }
        }

        for enemy in self.enemies.clone() {
            if self.enemies_x + enemy.position.x < WIDTH as i32 {
                enemy.render(self)?;
            }
        }

        for shot in self.shots.clone().iter() {
            let bullet = shot.weapon_kind.clone().model(self);
            self.render_object(&bullet, shot.position.x, shot.position.y)?;
        }

        Ok(())
    }
}
