use crate::objects::Graphics;
use crate::types::{Game, Object, HEIGHT, WIDTH};

use ggez::{
    glam::*,
    graphics::{self, Canvas, Color},
    Context, GameResult,
};

impl Game {
    pub fn paint(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult<()> {
        for (y, i) in self.screen.iter().enumerate() {
            for (x, j) in i.iter().enumerate() {
                if *j == 1 {
                    let rect = graphics::Rect::new((x * 10) as f32, (y * 10) as f32, 10.0, 10.0);
                    let color = if self.inverted {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    };
                    let r1 = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        rect,
                        color,
                    )?;
                    canvas.draw(&r1, Vec2::new(0.0, 0.0));
                }
            }
        }

        Ok(())
    }

    pub fn pixel(&mut self, ax: i32, ay: i32) -> GameResult<()> {
        let inside = ax >= 0 && ay >= 0 && ax < WIDTH as i32 && ay < HEIGHT as i32;

        if inside {
            self.screen[ay as usize][ax as usize] = 1;
        }

        Ok(())
    }

    pub fn clear_pixel(&mut self, ax: i32, ay: i32) -> GameResult<()> {
        let inside = ax >= 0 && ay >= 0 && ax < WIDTH as i32 && ay < HEIGHT as i32;

        if inside {
            self.screen[ay as usize][ax as usize] = 0;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> GameResult<()> {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.screen[y as usize][x as usize] = 0;
            }
        }

        Ok(())
    }

    pub fn render_object(&mut self, obj: &Object, x: i32, y: i32) -> GameResult<()> {
        for ry in 0..obj.size.y as i32 {
            for rx in 0..obj.size.x as i32 {
                let offset = (ry * obj.size.x as i32 + rx) as usize;

                if offset < obj.data.len() && obj.data[offset as usize] == 1 {
                    let ax = (x + rx) as i32;
                    let ay = (y + ry) as i32;
                    self.pixel(ax, ay)?;
                }
            }
        }

        Ok(())
    }

    pub fn render_outlined_object(&mut self, obj: &Object, x: i32, y: i32) -> GameResult<()> {
        for ry in 0..obj.size.y as i32 {
            for rx in 0..obj.size.x as i32 {
                let offset = (ry * obj.size.x as i32 + rx) as usize;

                if offset < obj.data.len() && obj.data[offset as usize] == 1 {
                    let ax = (x + rx) as i32;
                    let ay = (y + ry) as i32;
                    self.clear_pixel(ax - 1, ay)?;
                    self.clear_pixel(ax + 1, ay)?;
                    self.clear_pixel(ax, ay - 1)?;
                    self.clear_pixel(ax, ay + 1)?;

                    self.clear_pixel(ax - 1, ay - 1)?;
                    self.clear_pixel(ax + 1, ay + 1)?;
                    self.clear_pixel(ax + 1, ay - 1)?;
                    self.clear_pixel(ax - 1, ay + 1)?;
                }
            }
        }

        self.render_object(obj, x, y)?;

        Ok(())
    }

    pub fn render_number(&mut self, n: u32, digits: u8, x: i32, y: i32) -> GameResult<()> {
        let mut ox = 0;

        for i in 0..digits as u32 {
            let obj = self.static_objects[(n / (10_u32.pow(i)) % 10) as usize].clone();
            self.render_object(&obj, ox + x, y)?;

            ox -= 4;
        }

        Ok(())
    }

    pub fn render_bar(&mut self) -> GameResult<()> {
        let y = if self.level_data().upper == 1 {
            HEIGHT as i32 - 5
        } else {
            0
        };

        let heart = self.static_objects[Graphics::GLife as usize].clone();
        for i in 0..self.player.lives {
            self.render_object(&heart, i as i32 * 6, y)?;
        }

        let index = Graphics::GMissileIcon as usize + self.weapon.clone().kind as usize - 1;
        let weapon = self.static_objects[index].clone();
        self.render_object(&weapon, 33, y)?;

        self.render_number(self.weapon.amount as u32, 2, 43, y)?;
        self.render_number(self.score, 5, 71, y)?;

        Ok(())
    }
}
