mod font;
mod text;
mod types;
mod objects;

use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use objects::Object;
use types::MyGame;
use std::fs::File;
use std::io::Read;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
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
            screen: [[0; 84]; 48]
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

    pub fn render_object(&mut self, obj: &Object, x: u32, y: u32) -> GameResult<()> {
        for ry in 0..obj.height {
            for rx in 0..obj.width {
                if obj.data[(ry * obj.width + rx) as usize] == 1 {
                    self.screen[(y + ry) as usize][(x + rx) as usize] = 1;
                }
            }
        }

        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...

        // text::render_text(self, "Hello", 0, 0)?;

        let crap = load_object(0)?;
        self.render_object(&crap, 5, 5)?;

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
        data: bytes[2..].to_vec()
    })
}
