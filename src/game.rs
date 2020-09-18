use crate::types::{Game, WIDTH, HEIGHT, PLAYER_HEIGHT, G_PLAYER, G_PROTECTION_A1, Vec2};
use crate::objects::{scenery_data, Graphics};

use ggez::{Context, GameResult};

impl Game {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;
        self.keyboard(_ctx)?;

        if !self.is_playing {
            self.inverted = self.level_data().inverted_color;
            self.enemies = self.load_level(self.level)?;

            if self.level_data().upper == 1 {
                self.y_axis = Vec2 { x: 0, y: HEIGHT as i32 - PLAYER_HEIGHT as i32 - 5 };
            }

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

        self.render_bar();

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
