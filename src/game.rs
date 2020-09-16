use crate::types;

use types::{Game, Shot, Enemy, scenery_data, Vec2, Scenery, WIDTH, HEIGHT, Graphics};
use ggez::{Context, GameResult};
use std::collections::{HashSet};
use rand::Rng;

impl Game {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;

        if !self.is_playing {
            self.is_playing = true;
            self.enemies = self.load_level(self.level)?;
            // dbg!(self.enemies.clone());
            self.load_scenery()?;            
        }

        self.keyboard(_ctx)?;

        // Enemies

        let mut next_enemies: Vec<Enemy> = vec![];
        let mut deleted_shots: HashSet<u8> = HashSet::new();

        for mut enemy in self.enemies.clone() {
            enemy.tick(&mut deleted_shots, self)?;
            
            if enemy.alive || enemy.explosion_frames > 0 {
                next_enemies.push(enemy);
            }
        }

        self.enemies = next_enemies;

        // Shots

        let mut next_shots: Vec<Shot> = vec![];

        for (i, shot) in self.shots.iter().enumerate() {
            if shot.position.x < WIDTH as i32 && !deleted_shots.contains(&(i as u8)) {
                let new_shot = Shot { position: shot.position.clone().left(), dirty: false };
                next_shots.push(new_shot);
            }
        }

        self.shots = next_shots;

        // The end

        if let Some(enemy) = self.enemies.last() {
            if self.enemies_x + enemy.position.x >= (WIDTH as i32 / 4) * 3 {
                self.scene_x -= 1;
            }
        }

        self.enemies_x -= 1;

        Ok(())
    }

    pub fn render(&mut self) -> GameResult<()> {
        let player = self.load_object(255)?;
        self.render_object(&player, self.player_position.x, self.player_position.y)?;

        let heart = self.static_objects[Graphics::GLife as usize].clone();
        for i in 0..self.player_lives {
            self.render_object(&heart, i as i32 * 6, 0)?;
        }

        let index = Graphics::GLife as usize + self.player_weapon as usize + 1;
        let weapon = self.static_objects[index].clone();
        self.render_object(&weapon, 33, 0)?;

        let number = self.static_objects[self.bonus as usize].clone();
        self.render_object(&number, 43, 0)?;

        let number = self.static_objects[self.score as usize].clone();
        self.render_object(&number, 71, 0)?;

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

        let bullet = self.static_objects[20].clone();

        for shot in self.shots.clone().iter() {
            //if shot.position.x < WIDTH as i32 {
                self.render_object(&bullet, shot.position.x, shot.position.y)?;
            //}
        }

        Ok(())
    }
}
