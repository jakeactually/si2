use crate::types;

use types::{Game, WIDTH, Graphics, G_PLAYER, G_PROTECTION_A1};
use ggez::{Context, GameResult};

impl Game {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;
        self.keyboard(_ctx)?;

        if !self.is_playing {
            self.is_playing = true;
            self.enemies = self.load_level(self.level)?;
            self.load_scenery()?;            
        }

        // Enemies

        self.enemies = self
            .enemies
            .clone()
            .into_iter()
            .filter(|e| e.alive() || e.explosion_frames > 0)
            .map(|e| e.tick(self))
            .collect::<GameResult<Vec<_>>>()?;

        // Shots

        self.shots = self
            .shots
            .clone()
            .into_iter()
            .filter(|s| s.active && s.position.x < WIDTH as i32)
            .map(|s| s.left())
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

        let index = Graphics::GLife as usize + self.player.weapon.clone() as usize + 1;
        let weapon = self.static_objects[index].clone();
        self.render_object(&weapon, 33, 0)?;

        self.render_number(self.bonus as u32, 2, 43, 0)?;
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

        let bullet = self.static_objects[Graphics::GShot as usize].clone();

        for shot in self.shots.clone().iter() {
            self.render_object(&bullet, shot.position.x, shot.position.y)?;
        }

        Ok(())
    }
}
