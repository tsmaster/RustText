// Demo 3 main.rs

use std::collections::HashMap;

use macroquad::prelude::*;
use quad_snd::{AudioContext, Sound};

use demo_1::panel::*;
use demo_1::font::make_font;

pub struct FontRecord {
    pub cell_width: u32,
    pub cell_height: u32,
}

#[derive(Debug)]
pub struct MenuData<'a> {
    name: &'a str,
    viz_width: usize,
    viz_height: usize,

    child_keys: Vec<&'a str>,
    child_menus: HashMap<&'a str, MenuData<'a>>,

    is_enabled: bool,
    is_leaf: bool,
    id: i32,

    cell_width: usize,

    cursor_x: usize,
    cursor_y: usize,
}

impl <'a> MenuData<'a>{
    fn set_id(&mut self, new_id:i32) {
        self.id = new_id;
    }

    fn set_name(&mut self, new_name:&'a str) {
        self.name = new_name;
    }

    fn set_size(&mut self, width: usize, height: usize) {
        self.viz_width = width;
        self.viz_height = height;
        self.is_leaf = false;
    }

    fn get_child(&mut self, child_name: &str) -> &MenuData<'a> {
        &(self.child_menus[child_name])        
    }
    
    fn get_mut_child(&mut self, child_name: &str) -> &mut MenuData<'a> {
        self.child_menus.get_mut(child_name).unwrap()
    }

    fn add_child(&mut self, child: MenuData<'a>) {
        self.child_keys.push(child.name);
        self.child_menus.insert(child.name, child);
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }

    fn display_width(&self) -> usize {
        self.name.len() + 2
    }    

    fn build(&mut self) {
        // fill out internal data
        let mut max_width = 0;
        let mut max_found = false;
        
        for c in self.child_keys.iter() {
            if !max_found || c.len() > max_width {
                max_found = true;
                max_width = c.len();
            }
        }

        self.cell_width = max_width;
    }

    fn draw(&self, x: u32, y: u32, texture: &Texture2D, font_record: &FontRecord) {
        draw_rectangle(x as f32, y as f32,
                       (self.cell_width * 6) as f32,
                       (self.child_keys.len() * 8) as f32,
                       BLACK);

        // TODO use scale

        // TODO push font info into font object

        // TODO use is_active to gray out non-top menu panels

        // TODO add menu colors
        
        for i in 0 .. self.child_keys.len() {
            let cname = self.child_keys[i];
            draw_string(cname, WHITE,
                        x as f32, (y + (8*i) as u32) as f32,
                        texture, font_record);
        }
    }

    fn new(new_name: &'a str) -> MenuData<'a> {
        let md = MenuData {
            name: new_name,
            viz_width: 0,
            viz_height: 0,
            child_keys: vec!{},
            child_menus: HashMap::new(),
            is_enabled: true,
            is_leaf: true,
            id: 0,
            cell_width: 0,
            cursor_x: 0,
            cursor_y: 0,
        };
        md
    }

}

pub struct MenuManager<'a> {
    menu_stack: Vec<&'a mut MenuData<'a>>,
    texture: &'a Texture2D,
    font_record: &'a FontRecord
}

impl <'a> MenuManager<'a> {
    fn new(texture: &'a Texture2D, font_record: &'a FontRecord) -> MenuManager<'a> {
        let mm = MenuManager {
            menu_stack: vec!(),
            texture: texture,
            font_record: font_record,
        };
        mm
    }
    

    fn open(&mut self, menu_object: &'a mut MenuData<'a>) {
        self.menu_stack.push(menu_object);
    }

    fn draw(&self, x: u32, y: u32) {
        let mut tx = x;
        let mut ty = y;

        let x_spacing = 12;
        let y_spacing = 8;
        
        for m in self.menu_stack.iter() {
            m.draw(tx, ty, self.texture, self.font_record);
            tx = tx + x_spacing;
            ty = ty + y_spacing;
        }
    }
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


fn write_box(panel: &mut Panel, x: u32, y: u32, width: u32, height: u32)
{
    for cx in 1..(width-1) {
        panel_put_char(panel, '-', x + cx, y);
        panel_put_char(panel, '-', x + cx, y + (height - 1));
    }
    for cy in 1..(height-1) {
        panel_put_char(panel, '|', x, y + cy);
        panel_put_char(panel, '|', x + (width - 1), y + cy);
    }
    panel_put_char(panel, '+', x, y);
    panel_put_char(panel, '+', x + (width - 1), y);
    panel_put_char(panel, '+', x, y + (height - 1));
    panel_put_char(panel, '+', x + (width - 1), y + (height - 1));
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Demo3: menu".to_owned(),
        window_width: 1200,
        window_height: 800,
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
                                  2,
                                  a2_font_obj, 16, 16);

    write_box(&mut my_panel, 0, 0, 16, 16);

    panel_set_cursor_pos(&mut my_panel, 1, 1);
    panel_write_string(&mut my_panel, "Hello, Panel!");

    /*
    let mut panel_2 = make_panel(200.0, 64.0,
                                 RED,
                                 Some(BLACK),
                                 2,
                                 a2_font_obj, 12, 12);
    write_box(&mut panel_2, 0, 0, 12, 12);
    panel_set_cursor_pos(&mut panel_2, 1, 1);
    panel_write_string(&mut panel_2, "panel 2");
     */
    let bg_color = Color{r: 0.5, g: 0.7, b: 0.5, a: 1.0};

    let mut root_menu_obj = MenuData::new("root");
    root_menu_obj.add_child(MenuData::new("settings"));
    root_menu_obj.add_child(MenuData::new("demos"));
    root_menu_obj.add_child(MenuData::new("games"));

    root_menu_obj.set_size(1, 4);

    let settings = root_menu_obj.get_mut_child("settings");
    settings.add_child(MenuData::new("font color"));
    settings.add_child(MenuData::new("background color"));
    settings.add_child(MenuData::new("overscan color"));
    
    let demos = root_menu_obj.get_mut_child("demos");
    demos.add_child(MenuData::new("mandelbrot"));
    demos.add_child(MenuData::new("word wrap"));
    demos.add_child(MenuData::new("matrix tetris"));
    demos.add_child(MenuData::new("pentominoes"));
    demos.add_child(MenuData::new("plinko"));
    demos.set_size(1,4);

    let games = root_menu_obj.get_mut_child("games");
    games.add_child(MenuData::new("guess a number"));
    games.add_child(MenuData::new("mancala"));
    games.add_child(MenuData::new("checkers"));
    games.add_child(MenuData::new("chess"));

    root_menu_obj.build();

    let mut my_menu_mgr = MenuManager::new(&a2_font, &a2_font_record);

    my_menu_mgr.open(&mut root_menu_obj);

    loop {
        clear_background(bg_color);

        draw_panel(&my_panel);
        //draw_panel(&panel_2);
        my_menu_mgr.draw(50, 50);
        next_frame().await
    }
}