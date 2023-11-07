use macroquad::prelude::*;
use crate::font::BdgFont;

pub struct Panel<'a> {
    pub screen_x: f32,
    pub screen_y: f32,
    pub font_color: Color,
    pub erase_color: Option<Color>,
    pub font: BdgFont<'a>,
    pub char_width: u8,
    pub char_height: u8,
    pub chars: Vec<Vec<char>>,

    pub screen_width: f32,
    pub screen_height: f32,

    pub cursor_x: u32,
    pub cursor_y: u32,
}


fn draw_char(c: char, color: Color, x: f32, y: f32, font: &BdgFont)
{
    let char_ascii = c as u32;

    let cx = char_ascii % 16;
    let cy = (char_ascii / 16) - 2;
    
    let tx = (font.width as u32 * cx) as f32;
    let ty = (font.height as u32 * cy) as f32;

    draw_texture_ex(
        &font.texture,
        x, y,
        color,
        DrawTextureParams {
            source: Some(Rect{x: tx, y: ty,
                              w: font.width as f32,
                              h: font.height as f32}),
            ..Default::default()
        }
    );
}

        

pub fn make_panel(sx: f32, sy: f32,
                  font_color: Color,
                  erase_color: Option<Color>, font: BdgFont, w: u8, h: u8) -> Panel
{
    let char_row = vec![' '; w.into()];
    let mut char_vec = vec![];
    for _i in 0..h
    {
        char_vec.push(char_row.clone());
    }
    
    let p = Panel{screen_x: sx,
                  screen_y: sy,
                  font_color: font_color,
                  erase_color: erase_color,
                  font: font,
                  char_width: w,
                  char_height: h,
                  chars: char_vec,
                  screen_width: (w * font.width) as f32,
                  screen_height: (w * font.height) as f32,

                  cursor_x: 0,
                  cursor_y: 0,
    };
    
    println!("I made a panel");
    return p;
}

pub fn draw_panel(panel: &Panel)
{
    if panel.erase_color.is_some()
    {
        draw_rectangle(panel.screen_x,
                       panel.screen_y,
                       panel.screen_width,
                       panel.screen_height,
                       panel.erase_color.unwrap());
    }

    for x in 0..panel.char_width
    {
        for y in 0..panel.char_height
        {
            let c = panel.chars[y as usize][x as usize];
            draw_char(c, panel.font_color,
                      panel.screen_x + (panel.font.width * x) as f32,
                      panel.screen_y + (panel.font.height * y) as f32,
                      &panel.font);
        }
    }
}

pub fn panel_set_cursor_pos(panel: &mut Panel, x: u32, y: u32)
{
    panel.cursor_x = x;
    panel.cursor_y = y;
}

pub fn panel_write_string(panel: &mut Panel, s: &str)
{
    for c in s.chars()
    {
        panel.chars[panel.cursor_y as usize][panel.cursor_x as usize] = c;
        panel.cursor_x += 1;
    }
}
