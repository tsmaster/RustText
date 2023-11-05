use macroquad::prelude::*;
use quad_snd::{AudioContext, Sound};

mod panel;

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

#[macroquad::main("Demo1")]
async fn main() {
    println!("Hello, world!");

    let a2_font: Texture2D = load_texture("assets/40col.png").await.unwrap();
    let nes_font: Texture2D = load_texture("assets/nes.png").await.unwrap();

    let a2_font_record = FontRecord{cell_width: 6,
                                    cell_height: 8};
    let nes_font_record = FontRecord{cell_width: 8,
                                     cell_height: 8};

    //texture.set_filter(FilterMode::Linear);
    a2_font.set_filter(FilterMode::Nearest);
    nes_font.set_filter(FilterMode::Nearest);

    let mut audio_ctx = AudioContext::new();
    
    let beep_sound = Sound::load(&mut audio_ctx, include_bytes!("../assets/beep.wav"));
    
    beep_sound.play(&mut audio_ctx, Default::default());

    panel::make_panel();

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO MACRO", 20.0, 20.0, 20.0, DARKGRAY);

        draw_texture(&a2_font, 0.0, 0.0, WHITE);

        draw_texture_ex(
            &a2_font,
            0.0,
            100.0,
            GREEN,
            DrawTextureParams {
                //dest_size: Some(vec2(screen_width(), screen_height())),
                source: Some(Rect{x: 0., y: 0., w: 10., h:10.}),
                dest_size: Some(vec2(10.0, 10.0)),
                ..Default::default()
            },
        );

        draw_char('H', BLUE, 0.0, 120.0, &a2_font, &a2_font_record);

        draw_string("Hello, World!", GREEN, 0.0, 140.0, &a2_font, &a2_font_record);

        draw_string("Hello, NES World!", BLACK, 0.0, 160.0, &nes_font, &nes_font_record);

        draw_box(BLACK, 10.0, 180.0, 10, 4, &a2_font, &a2_font_record);
        next_frame().await
    }
}
