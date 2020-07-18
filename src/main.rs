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
use types::{MyGame, Vec2};
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

            player_x: 3,
            player_y: 20,
            objects_cache: HashMap::new(),
            enemies_cache: HashMap::new(),
            shots: vec![]
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;

        let rel_time = if self.frame < 12 { self.frame } else { 12 };

        if keyboard::is_key_pressed(_ctx, KeyCode::Right) && self.player_x < 84 - 10 {
            self.player_x += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) && self.player_x > 0 {
            self.player_x -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Up) && self.player_y > 0 {
            self.player_y -= 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Down) && self.player_y < 48 - 7 {
            self.player_y += 1;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Space) && self.frame % 6 == 0 {
            self.shots.push(Vec2(self.player_x + 9, self.player_y + 3));
        }

        self.shots = self.shots.iter()
            .map(|Vec2(x, y)| Vec2(x + 1, *y))
            .filter(|Vec2(x, _)| *x < 84)
            .collect();

        let bullet = self.static_objects[20].clone();

        for shot in self.shots.clone().iter() {
            self.render_object(&bullet, shot.0, shot.1)?;
        }

        /* let space = self.static_objects[10].clone();
        self.render_object(&space, 8, rel_time)?;

        let impact = self.static_objects[12].clone();
        self.render_object(&impact, 4, 38 - rel_time)?;

        let intro = self.static_objects[11].clone();
        self.render_outlined_object(&intro, rel_time * 4 + 4, 21)?; */

        // let obj = load_object(0)?;
        // self.render_object(&obj, 0, 0)?;


        let player = self.load_object(255)?;
        // println!("{} {}", player.width, player.height);
        self.render_object(&player, self.player_x, self.player_y)?;

        let enemies = self.load_level(0)?;

        for enemy in enemies {
            let obj = self.load_object(enemy.data.model_id as u8)?;
            let screen_x = enemy.x - self.frame as i32 / 10;

            if self.frame % 10 == 0 && screen_x > -100 && screen_x < 940 {
                let outside =
                    self.player_x  + 10 < screen_x ||
                    self.player_y + 7 < enemy.y ||
                    self.player_x > screen_x + obj.width as i32 ||
                    self.player_y > enemy.y + obj.height as i32;

                if !outside {
                    let collide = util::does_collide(player.clone(), self.player_x, self.player_y, obj.clone(), screen_x, enemy.y);

                    if collide {
                        println!("collission {}", self.frame);
                    }
                }

            }
            
            self.render_object(&obj, screen_x, enemy.y)?;
        }

        self.frame += 1;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.paint(ctx)?;
        graphics::present(ctx)
    }
}
