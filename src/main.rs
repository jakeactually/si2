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
use types::{MyGame, Vec2, Enemy};
use std::collections::HashMap;

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
            self.shots.push(Vec2 { x: self.player_position.x + 9, y: self.player_position.y + 3 });
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
        // println!("{} {}", player.width, player.height);
        self.render_object(&player, self.player_position.x, self.player_position.y)?;

        // Shots

        self.shots = self.shots.iter()
            .map(|v| Vec2 { x: v.x + 1, y: v.y })
            .filter(|Vec2 { x, .. }| *x < 84)
            .collect();

        let bullet = self.static_objects[20].clone();

        for shot in self.shots.clone().iter() {
            self.render_object(&bullet, shot.x, shot.y)?;
        }

        // Enemies

        let mut next_enemies: Vec<Enemy> = vec![];

        for mut enemy in self.enemies.clone() {
            let obj = self.load_object(enemy.data.model_id as u8)?;
            let screen_x = enemy.position.x - self.frame as i32;

            if self.frame % 10 == 0 && screen_x > -100 && screen_x < 940 {
                let collission = util::intersect(
                    self.player_position.clone(),
                    Vec2 { x: 10, y: 7 },
                    Vec2 { x: screen_x, y: enemy.position.y },
                    obj.size.clone()
                );

                if collission {
                    let collide = true; // util::does_collide(player.clone(), self.player_x, self.player_y, obj.clone(), screen_x, enemy.y);

                    if collide {
                        println!("collission {}", self.frame);
                    }
                }

                let mut next_shots: Vec<Vec2> = vec![];

                for shot in self.shots.clone().iter() {
                    let collission = util::intersect(
                        shot.clone(),
                        bullet.size.clone(),
                        Vec2 { x: screen_x, y: enemy.position.y },
                        obj.size.clone()
                    );
                    
                    if collission {
                        let collide = true; // util::does_collide(bullet.clone(), shot.0, shot.1, obj.clone(), screen_x, enemy.y);
    
                        if collide {
                            enemy.alive = false;
                        } else {
                            next_shots.push(shot.clone());
                        }
                    } else {
                        next_shots.push(shot.clone());
                    }
                }

                self.shots = next_shots;
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

        self.enemies = next_enemies;

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
