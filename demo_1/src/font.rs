use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct BdgFont<'a> {
    name: &'a str,
    pub width: u32,
    pub height: u32,

    pub texture: &'a Texture2D,
}

pub fn make_font(texture: &Texture2D, width: u32, height: u32) -> BdgFont
{
    println!("I made a font");

    let f = BdgFont {
        name: "foo",
        width: width,
        height: height,
        texture: texture,
    };

    return f;
}
