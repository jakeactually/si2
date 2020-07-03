use crate::font;
use crate::types;

use ggez::GameResult;
use types::MyGame;

pub fn render_text(my_game: &mut MyGame, text: &str, x: u32, y: u32) -> GameResult<()> {
    for (i, c) in text.chars().enumerate() {
        render_character(my_game, c, x + i as u32 * 6, y)?;
    }

    Ok(())
}

pub fn render_character(my_game: &mut MyGame, character: char, x: u32, y: u32) -> GameResult<()> {
    for iy in 0..8 {
        for ix in 0..5 {
            let offset = iy * 5 + ix;
            let byte = font::COMPRESSED_FONT[character as usize - '!' as usize][offset / 8];
            
            if byte >> (7 - offset % 8) & 1 == 1 as u8 {
                my_game.screen[y as usize + iy][x as usize + ix] = 1;                
            }
        }
    }

    Ok(())
}
