use crate::font;

use ggez::{graphics, Context, GameResult};

pub fn render_text(ctx: &mut Context, text: &str, x: u32, y: u32) -> GameResult<()> {
    for (i, c) in text.chars().enumerate() {
        render_character(ctx, c, x + i as u32 * 6 * 10, y)?;
    }

    Ok(())
}

pub fn render_character(ctx: &mut Context, character: char, x: u32, y: u32) -> GameResult<()> {
    for iy in 0..8 {
        for ix in 0..5 {
            let offset = iy * 5 + ix;
            let byte = font::COMPRESSED_FONT[character as usize - '!' as usize][offset / 8];
            
            if byte >> (7 - offset % 8) & 1 == 1 as u8 {
                let rect = graphics::Rect::new((x + ix as u32 * 10) as f32, (y + iy as u32 * 10) as f32, 10.0, 10.0);
                let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::BLACK)?;
                graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
            }
        }
    }

    Ok(())
}
