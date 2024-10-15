mod enemy;
mod font;
mod game;
mod game_util;
mod load;
mod objects;
mod render;
mod text;
mod types;
mod util;

use ggez::conf::{FullscreenType, WindowMode};
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use types::{Game, HEIGHT, WIDTH};

fn main() {
    let window_mode = WindowMode {
        width: WIDTH as f32 * 10.0,
        height: HEIGHT as f32 * 10.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 1.0,
        max_width: 0.0,
        min_height: 1.0,
        max_height: 0.0,
        resizable: false,
        visible: true,
        transparent: false,
        resize_on_scale_factor_change: false,
        logical_size: None,
    };

    let (mut ctx, event_loop) = ContextBuilder::new("", "")
        .window_mode(window_mode)
        .build()
        .unwrap();

    let my_game = Game::new(&mut ctx);

    ggez::event::run(ctx, event_loop, my_game)
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        while (_ctx.time.check_update_time(60)) {
            self.update(_ctx)?;
        }

        self.render()
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let color = if self.inverted {
            Color::BLACK
        } else {
            Color::WHITE
        };
        let mut canvas = graphics::Canvas::from_frame(ctx, color);
        self.paint(ctx, &mut canvas)?;
        canvas.finish(ctx)
    }
}
