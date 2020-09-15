mod enemy;
mod font;
mod game;
mod game_util;
mod objects;
mod text;
mod types;
mod util;
mod load;
mod render;

use ggez::event::{EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::conf::{WindowMode, FullscreenType};
use types::{Game};

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

    let mut my_game = Game::new(&mut ctx);

    match ggez::event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.update(_ctx)?;
        self.render()
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.paint(ctx)?;
        graphics::present(ctx)
    }
}
