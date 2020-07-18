mod font;
mod objects;
mod text;
mod types;
mod util;
mod load;
mod render;

use ggez::event::{EventHandler, KeyCode};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, FullscreenType};
use ggez::input::keyboard;
use types::{MyGame, Vec2, Enemy, Shot};
use std::collections::{HashMap, HashSet};

fn main() {
    let window_mode = WindowMode {
        width: 840.0,
        height: 480.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("", "")
        .window_mode(window_mode)
		.build()
		.unwrap();

    let mut my_game = MyGame::new(&mut ctx);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            screen: [[0; 84]; 48],
            static_objects: objects::get_static_objects().to_vec(),
            frame: 0,
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
}

impl Vec2 {
    fn left(self) -> Self {
        Vec2 { x: self.x + 1, y: self.y }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if !self.is_playing {
            self.is_playing = true;
            self.enemies = self.load_level(0)?;
        }

        self.clear()?;

        let rel_time = if self.frame < 12 { self.frame } else { 12 };

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

        /* let space = self.static_objects[10].clone();
        self.render_object(&space, 8, rel_time)?;

        let impact = self.static_objects[12].clone();
        self.render_object(&impact, 4, 38 - rel_time)?;

        let intro = self.static_objects[11].clone();
        self.render_outlined_object(&intro, rel_time * 4 + 4, 21)?; */

        // let obj = load_object(0)?;
        // self.render_object(&obj, 0, 0)?;

        // Player

        let player = self.load_object(255)?;
        self.render_object(&player, self.player_position.x, self.player_position.y)?;

        // Loop

        let mut next_enemies: Vec<Enemy> = vec![];
        let bullet = self.static_objects[20].clone();
        let mut deleted_shots: HashSet<u8> = HashSet::new();

        for mut enemy in self.enemies.clone() {
            let obj = self.load_object(enemy.data.model_id as u8)?;
            let screen_x = enemy.position.x - self.frame as i32;

            if screen_x > -100 && screen_x < 940 {
                let collission = util::intersect(
                    self.player_position.clone(),
                    Vec2 { x: 10, y: 7 },
                    Vec2 { x: screen_x, y: enemy.position.y },
                    obj.size.clone()
                );

                if collission {
                    println!("collission {}", self.frame);
                }

                for (i, shot) in self.shots.iter().enumerate() {
                    let collission = util::intersect(
                        shot.position.clone(),
                        bullet.size.clone(),
                        Vec2 { x: screen_x, y: enemy.position.y },
                        obj.size.clone()
                    );
                    
                    if collission {
                        enemy.alive = false;
                        deleted_shots.insert(i as u8);
                    }
                }

                if enemy.alive {
                    self.render_object(&obj, screen_x, enemy.position.y)?;
                    next_enemies.push(enemy);
                } else if enemy.explosion_frames > 0 {
                    let explosion = self.static_objects[22 - (enemy.explosion_frames as usize - 1) / 3].clone();
                    self.render_object(&explosion, screen_x, enemy.position.y)?;
                    enemy.explosion_frames -= 1;
                    next_enemies.push(enemy);
                }
            }
        }

        self.enemies = next_enemies;

        // Shots

        let mut next_shots: Vec<Shot> = vec![];

        for (i, shot) in self.shots.clone().iter().enumerate() {
            if shot.position.x < 84 && !deleted_shots.contains(&(i as u8)) {
                self.render_object(&bullet, shot.position.x, shot.position.y)?;
                let new_shot = Shot { position: shot.position.clone().left(), dirty: false };
                next_shots.push(new_shot);
            }
        }

        self.shots = next_shots;

        // The end

        self.frame += 1;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.paint(ctx)?;
        graphics::present(ctx)
    }
}
