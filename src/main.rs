mod font;
mod objects;
mod text;
mod types;
mod util;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, FullscreenType};
use ggez::event::{self, EventHandler};
use types::{MyGame, Object};
use std::fs::File;
use std::io::Read;

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

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(window_mode)
		.build()
		.expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            screen: [[0; 84]; 48],
            static_objects: objects::get_static_objects().to_vec(),
            frame: 0,
            main_color: 1,
            secondary_color: 0
        }
    }

    pub fn paint(&mut self, ctx: &mut Context) -> GameResult<()> {
        for (y, i) in self.screen.iter().enumerate() {
            for (x, j) in i.iter().enumerate() {
                if *j == 1 {
                    let rect = graphics::Rect::new((x * 10) as f32, (y * 10) as f32, 10.0, 10.0);
                    let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::BLACK)?;
                    graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
                }
            }
        }

        Ok(())
    }

    pub fn pixel(&mut self, ax: i32, ay: i32) -> GameResult<()> {
        let inside = ax > 0 && ay > 0 && ax < 84 && ay < 48;
        
        if inside {
            self.screen[ay as usize][ax as usize] = self.main_color;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> GameResult<()> {
        for y in 0..48 {
            for x in 0..84 {
                self.screen[y][x] = 0;
            }
        }

        Ok(())
    }

    pub fn render_object(&mut self, obj: &Object, x: u32, y: u32) -> GameResult<()> {
        for ry in 0..obj.height  {
            for rx in 0..obj.width {
                let offset = (ry * obj.width + rx) as usize;

                if offset < obj.data.len() && obj.data[offset as usize] == 1 {
                    let ax = (x + rx) as i32;
                    let ay = (y + ry) as i32;
                    self.pixel(ax, ay)?;
                }
            }
        }

        Ok(())
    }

    pub fn render_outlined_object(&mut self, obj: &Object, x: u32, y: u32) -> GameResult<()> {
        self.invert()?;

        for ry in 0..obj.height  {
            for rx in 0..obj.width {
                let offset = (ry * obj.width + rx) as usize;

                if offset < obj.data.len() && obj.data[offset as usize] == 1 {
                    let ax = (x + rx) as i32;
                    let ay = (y + ry) as i32;
                    self.pixel(ax - 1, ay)?;
                    self.pixel(ax + 1, ay)?;
                    self.pixel(ax, ay - 1)?;
                    self.pixel(ax, ay + 1)?;

                    self.pixel(ax - 1, ay - 1)?;
                    self.pixel(ax + 1, ay + 1)?;
                    self.pixel(ax + 1, ay - 1)?;
                    self.pixel(ax - 1, ay + 1)?;
                }
            }
        }

        self.invert()?;
        self.render_object(obj, x, y)?;

        Ok(())
    }

    pub fn invert(&mut self) -> GameResult<()> {
        let temp = self.main_color;
        self.main_color = self.secondary_color;
        self.secondary_color = temp;

        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.clear()?;

        let rel_time = if self.frame < 12 { self.frame } else { 12 };

        /*let space = self.static_objects[10].clone();
        self.render_object(&space, 8, rel_time)?;

        let impact = self.static_objects[12].clone();
        self.render_object(&impact, 4, 38 - rel_time)?;

        let intro = self.static_objects[11].clone();
        self.render_outlined_object(&intro, rel_time * 4 + 4, 21)?;*/

        let obj = load_object(0)?;
        self.render_object(&obj, 0, 0)?;

        self.frame += 1;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.paint(ctx)?;
        graphics::present(ctx)
    }
}

fn load_object<'a>(id: u8) -> std::io::Result<Object> {
    let file = File::open(format!("data/objects/{}.dat", id))?;
    let bytes = file.bytes().collect::<std::io::Result<Vec<u8>>>()?; 

    Ok(Object {
        width: bytes[0] as u32,
        height: bytes[1] as u32,
        data: util::uncompress(bytes[2..].to_vec())
    })
}
