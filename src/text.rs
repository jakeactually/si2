use crate::font;
use crate::types;
use crate::util;

use ggez::GameResult;
use types::Game;

pub fn render_text(my_game: &mut Game, text: &str, x: u32, y: u32) -> GameResult<()> {
    for (i, c) in text.chars().enumerate() {
        render_character(my_game, c, x + i as u32 * 6, y)?;
    }

    Ok(())
}

pub fn render_character(my_game: &mut Game, character: char, x: u32, y: u32) -> GameResult<()> {
    let font = util::uncompress(font::COMPRESSED_FONT[character as usize - '!' as usize].to_vec());

    for (i, pixel) in font.iter().enumerate() {
        if *pixel == 1 {
            my_game.screen[y as usize + i / 5][x as usize + i % 5] = 1;          
        }
    }

    Ok(())
}
