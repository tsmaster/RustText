use macroquad::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct BdgFont<'a> {
    //name: &str,
    pub width: u8,
    pub height: u8,

    pub texture: &'a Texture2D,
}

pub fn make_font(texture: &Texture2D, width: u8, height: u8) -> BdgFont
{
    println!("I made a font");

    let f = BdgFont {
        //name: "foo",
        width: width,
        height: height,
        texture: texture,
    };

    return f;
}
