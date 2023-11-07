// Demo 2 main.rs

use macroquad::prelude::*;
use quad_snd::{AudioContext, Sound};

use demo_1::panel::*;
//use demo_1::panel::draw_panel;
//use demo_1::panel::make_panel;
use demo_1::font::make_font;

pub struct FontRecord {
    pub cell_width: u32,
    pub cell_height: u32,
}

fn draw_char(c: char, color: Color, x: f32, y: f32, texture: &Texture2D, font_record: &FontRecord)
{
    let char_ascii = c as u32;

    let cx = char_ascii % 16;
    let cy = (char_ascii / 16) - 2;
    
    let tx = (font_record.cell_width * cx) as f32;
    let ty = (font_record.cell_height * cy) as f32;

    draw_texture_ex(
        &texture,
        x, y,
        color,
        DrawTextureParams {
            source: Some(Rect{x: tx, y: ty,
                              w: font_record.cell_width as f32,
                              h: font_record.cell_height as f32}),
            ..Default::default()
        }
    );
}

fn draw_string(s: &str, color: Color, x: f32, y: f32, texture: &Texture2D, font_record: &FontRecord)
{
    let mut mx = x;
    
    for c in s.chars() {
        draw_char(c, color, mx, y, &texture, &font_record);
        mx = mx + font_record.cell_width as f32;
    }
}

fn draw_box(color: Color, x: f32, y: f32, width: u8, height: u8,
            texture: &Texture2D, font_record: &FontRecord)
{
    for cx in 1..(width-1) {
        draw_char('-', color,
                  x + cx as f32 * font_record.cell_width as f32,
                  y,
                  &texture, &font_record);
        draw_char('-', color,
                  x + cx as f32 * font_record.cell_width as f32,
                  y + (height - 1) as f32 * font_record.cell_height as f32,
                  &texture, &font_record);
    }
    for cy in 1..(height-1) {
        draw_char('|', color,
                  x,
                  y + cy as f32 * font_record.cell_height as f32,
                  &texture, &font_record);
        draw_char('|', color,
                  x + (width - 1) as f32 * font_record.cell_width as f32,
                  y + cy as f32 * font_record.cell_height as f32,
                  &texture, &font_record);

    }
    draw_char('+', color,
              x, y,
              &texture, &font_record);
                  
    draw_char('+', color,
              x + (width - 1) as f32 * font_record.cell_width as f32,
              y,
              &texture, &font_record);
                  
    draw_char('+', color,
              x,
              y + (height - 1) as f32 * font_record.cell_height as f32,
              &texture, &font_record);
                  
    draw_char('+', color,
              x + (width - 1) as f32 * font_record.cell_width as f32,
              y + (height - 1) as f32 * font_record.cell_height as f32,
              &texture, &font_record);
                  
        
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Demo2: panel".to_owned(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, world!");

    let a2_font: Texture2D = load_texture("assets/40col.png").await.unwrap();

    let a2_font_record = FontRecord{cell_width: 6,
                                    cell_height: 8};

    a2_font.set_filter(FilterMode::Nearest);

    let mut audio_ctx = AudioContext::new();
    
    let beep_sound = Sound::load(&mut audio_ctx, include_bytes!("../../assets/beep.wav"));
    
    beep_sound.play(&mut audio_ctx, Default::default());

    let a2_font_obj = make_font(&a2_font, 6, 8);
    
    let mut my_panel = make_panel(40.0, 40.0,
                                  GREEN,
                                  Some(BLACK),
                                  a2_font_obj, 16, 16);

    panel_set_cursor_pos(&mut my_panel, 0,0);
    panel_write_string(&mut my_panel, "Hello Panel");

    let bg_color = Color{r: 0.5, g: 0.7, b: 0.5, a: 1.0};

    loop {
        clear_background(bg_color);

        draw_panel(&my_panel);
        next_frame().await
    }
}
