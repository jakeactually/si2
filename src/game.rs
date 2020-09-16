use crate::types;

use types::{Game, Shot, Enemy, scenery_data, Vec2, Scenery, WIDTH, HEIGHT, Graphics};
use ggez::{Context, GameResult};
use std::collections::{HashSet};
use rand::Rng;

impl Game {
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.real_frame += 1;

        if self.real_frame % 2 != 0 {
            return Ok(());
        }

        self.clear()?;

        if !self.is_playing {
            self.is_playing = true;
            self.enemies = self.load_level(1)?;

            let mut x = 0;
            let mut rng = rand::thread_rng();

            while x < 1600 {
                let sd = &scenery_data[1];
                let n1: u8 = rng.gen_range(sd.first_object, sd.first_object + sd.objects);
                let obj = self.load_object(n1)?;

                self.scenery.push(Scenery {
                    position: Vec2 { x: x, y: HEIGHT as i32 - obj.size.y },
                    model: obj.clone()
                });

                x += obj.size.x;
            }
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

        self.frame += 1;

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
            self.render_object(&scenery.model, scenery.position.x - self.frame as i32, scenery.position.y)?;
        }

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
