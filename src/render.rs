use crate::types;

use ggez::{graphics, Context, GameResult};
use types::{WIDTH, HEIGHT, Game, Object};

impl Game {
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
        let inside = ax >= 0 && ay >= 0 && ax < WIDTH as i32 && ay < HEIGHT as i32;
        
        if inside {
            self.screen[ay as usize][ax as usize] = self.main_color;
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
        for ry in 0..obj.size.y as i32  {
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
        self.invert()?;

        for ry in 0..obj.size.y as i32  {
            for rx in 0..obj.size.x as i32 {
                let offset = (ry * obj.size.x as i32 + rx) as usize;

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

    pub fn render_number(&mut self, n: u32, digits: u8, x: i32, y: i32) -> GameResult<()> {
        let mut ox = 0;

        for i in 0..digits as u32 {
            let obj = self.static_objects[(n / (10_u32.pow(i)) % 10) as usize].clone();
            self.render_object(&obj, ox + x, y)?;

            ox -= 4;
        }

        Ok(())
    }
}
