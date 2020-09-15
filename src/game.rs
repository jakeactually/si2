use crate::types;

use types::{Game, Shot, Enemy};
use ggez::{Context, GameResult};
use std::collections::{HashSet};

impl Game {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;

        if !self.is_playing {
            self.is_playing = true;
            self.enemies = self.load_level(0)?;
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
            if shot.position.x < 84 && !deleted_shots.contains(&(i as u8)) {
                let new_shot = Shot { position: shot.position.clone().left(), dirty: false };
                next_shots.push(new_shot);
            }
        }

        self.shots = next_shots;

        // The end

        self.frame += 1;

        Ok(())
    }    

    pub fn render(&mut self) -> GameResult<()> {
        let player = self.load_object(255)?;
        self.render_object(&player, self.player_position.x, self.player_position.y)?;

        for enemy in self.enemies.clone() {
            enemy.render(self)?;
        }

        let bullet = self.static_objects[20].clone();

        for shot in self.shots.clone().iter() {
            self.render_object(&bullet, shot.position.x, shot.position.y)?;
        }

        Ok(())
    }
}
