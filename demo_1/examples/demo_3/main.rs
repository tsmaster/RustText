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

    fn get_mut_child_by_id(&mut self, child_id: i32) -> Result<&mut MenuData<'a>, String> {
        let scm = &mut self.child_menus;
        
        for c in self.child_keys.iter() {
            let cm = scm.get(c).unwrap();
            if cm.id == child_id {
                let mcm = scm.get_mut(c).unwrap();
                return Ok(mcm);
            }
        }

        let msg = format!("ID {} not found", child_id);
        Err(msg.to_string())
    }

    fn add_child(&mut self, child: MenuData<'a>) {
        self.child_keys.push(child.name);
        self.child_menus.insert(child.name, child);
        self.is_leaf = false;
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

    fn draw(&self, x: u32, y: u32, scale: u32,
            texture: &Texture2D, font_record: &FontRecord) {
        draw_rectangle(x as f32, y as f32,
                       ((self.cell_width as u32 + 5) * 6 * scale) as f32,
                       (self.child_keys.len() as u32 * 8 * scale) as f32,
                       BLACK);

        draw_box(WHITE, x as f32, y as f32,
                 self.cell_width as u8 + 5,
                 self.child_keys.len() as u8 + 2,
                 scale,
                 texture,
                 font_record);
        

        // TODO push font info into font object

        // TODO use is_active to gray out non-top menu panels

        // TODO add menu colors

        // TODO move all of this into a panel

        // TODO handle two+ column

        // TODO handle scrolling

        
        // draw items
        for i in 0 .. self.child_keys.len() {
            let cname = self.child_keys[i];
            draw_string(cname, WHITE,
                        x as f32 + (6 * 2) as f32 * scale as f32,
                        (y + (8 * (i + 1) as u32 * scale)) as f32,
                        2,
                        texture, font_record);

            let cmo = &self.child_menus[cname];
            if !cmo.is_leaf {
                draw_char('>', GRAY,
                          x as f32 + (6 * (self.cell_width + 3)) as f32 * scale as f32,
                          (y + (8 * (i + 1) as u32 * scale)) as f32,
                          2,
                          texture, font_record);                          
            }
        }

        // draw cursor

        draw_char('>', RED,
                  x as f32 +
                  (1.0 + (self.cursor_x as f32 * (self.cell_width + 3) as f32)) *
                  6.0 * scale as f32,
                  y as f32 +
                  (self.cursor_y + 1) as f32 *
                  (8 * scale) as f32,
                  2,
                  texture, font_record);

        // TODO draw up/down prompts
    }

    fn on_up(&mut self) {
        println!("on up");

        match self.cursor_y.checked_sub(1) {
            Some(ny) => {
                self.cursor_y = ny;
            }
            None => {
            }
        }
    }

    fn on_down(&mut self) {
        println!("on down");

        self.cursor_y += 1;
        if self.cursor_y >= self.viz_height {
            self.cursor_y = self.viz_height - 1;
        }
    }

    fn on_left(&mut self) {
        match self.cursor_x.checked_sub(1) {
            Some(nx) => {
                self.cursor_x = nx;
            }
            None => {
            }
        }
    }

    fn on_right(&mut self) {
        self.cursor_x += 1;
        if self.cursor_x >= self.viz_width {
            self.cursor_x = self.viz_width - 1;
        }
    }

    fn on_select(&mut self) {
        // TODO possibly return an event?
        // possibly indicate a new menu should be added?

        println!("Selecting a thing??");
    }

    fn on_cancel(&mut self) {
        // TODO is there cleanup to be done here?
    }

    fn new(new_name: &'a str, id: i32) -> MenuData<'a> {
        let md = MenuData {
            name: new_name,
            viz_width: 0,
            viz_height: 0,
            child_keys: vec!{},
            child_menus: HashMap::new(),
            is_enabled: true,
            is_leaf: true,
            id: id,
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
    font_record: &'a FontRecord,
    font_scale: u32,
}

impl <'a> MenuManager<'a> {
    fn new(scale: u32, texture: &'a Texture2D, font_record: &'a FontRecord) -> MenuManager<'a> {
        let mm = MenuManager {
            menu_stack: vec!(),
            texture: texture,
            font_record: font_record,
            font_scale: scale,
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
            m.draw(tx, ty, self.font_scale, self.texture, self.font_record);
            tx = tx + x_spacing;
            ty = ty + y_spacing;
        }
    }

    fn get_top_menu(&mut self) -> Result<&mut MenuData<'a>, String>
    {
        if self.menu_stack.len() == 0
        {
            return Err("empty stack".to_string());
        }

        let index = self.menu_stack.len() - 1;
        let mut md = self.menu_stack.get_mut(index);
        match md {
            None => { return Err("can't get menu item".to_string());}
            Some(m) => { return Ok(m); }
        }
    }

    fn on_up(&mut self) {
        println!("up");

        let mut md = self.get_top_menu().unwrap();
        md.on_up();
    }

    fn on_down(&mut self) {
        println!("down");
        
        let mut md = self.get_top_menu().unwrap();
        md.on_down();
    }

    fn on_left(&mut self) {
        let mut md = self.get_top_menu().unwrap();
        md.on_left();
    }

    fn on_right(&mut self) {
        let mut md = self.get_top_menu().unwrap();
        md.on_right();
    }

    fn on_select(&mut self) {
        let mut md = self.get_top_menu().unwrap();
        md.on_select();

        // might also need to push a new menu
    }

    fn on_cancel(&mut self) {
        let mut md = self.get_top_menu().unwrap();
        md.on_cancel();

        // probably need to pop a menu
    }
}

fn draw_char(c: char, color: Color, x: f32, y: f32, scale: u32,
             texture: &Texture2D, font_record: &FontRecord)
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
            dest_size: Some(Vec2{
                x: (font_record.cell_width * scale) as f32,
                y: (font_record.cell_height * scale) as f32}),
            ..Default::default()
        }
    );
}

fn draw_string(s: &str, color: Color, x: f32, y: f32,
               scale: u32, texture: &Texture2D, font_record: &FontRecord)
{
    let mut mx = x;
    
    for c in s.chars() {
        draw_char(c, color, mx, y, scale, &texture, &font_record);
        mx = mx + (font_record.cell_width * scale) as f32;
    }
}

fn draw_box(color: Color, x: f32, y: f32,
            width: u8, height: u8,
            scale: u32,
            texture: &Texture2D, font_record: &FontRecord)
{
    for cx in 1..(width-1) {
        draw_char('-', color,
                  x + cx as f32 * font_record.cell_width as f32 * scale as f32,
                  y,
                  scale,
                  &texture, &font_record);
        draw_char('-', color,
                  x + cx as f32 * font_record.cell_width as f32 * scale as f32,
                  y + (height - 1) as f32 * font_record.cell_height as f32 * scale as f32,
                  scale,
                  &texture, &font_record);
    }
    for cy in 1..(height-1) {
        draw_char('|', color,
                  x,
                  y + cy as f32 * font_record.cell_height as f32 * scale as f32,
                  scale,
                  &texture, &font_record);
        draw_char('|', color,
                  x + (width - 1) as f32 * font_record.cell_width as f32 * scale as f32,
                  y + cy as f32 * font_record.cell_height as f32 * scale as f32,
                  scale,
                  &texture, &font_record);

    }
    draw_char('+', color,
              x, y,
              scale,
              &texture, &font_record);
                  
    draw_char('+', color,
              x + (width - 1) as f32 * font_record.cell_width as f32 * scale as f32,
              y,
              scale,
              &texture, &font_record);
                  
    draw_char('+', color,
              x,
              y + (height - 1) as f32 * font_record.cell_height as f32 * scale as f32,
              scale,
              &texture, &font_record);
                  
    draw_char('+', color,
              x + (width - 1) as f32 * font_record.cell_width as f32 * scale as f32,
              y + (height - 1) as f32 * font_record.cell_height as f32 * scale as f32,
              scale,
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

    let mut root_menu_obj = MenuData::new("root", 0);
    root_menu_obj.add_child(MenuData::new("settings", 101));
    root_menu_obj.add_child(MenuData::new("demos", 102));
    root_menu_obj.add_child(MenuData::new("games", 103));

    root_menu_obj.set_size(1, 4);

    let settings = root_menu_obj.get_mut_child("settings");
    settings.add_child(MenuData::new("font color", 1001));
    settings.add_child(MenuData::new("background color", 1002));
    settings.add_child(MenuData::new("overscan color", 1003));
    
    let demos = root_menu_obj.get_mut_child("demos");
    demos.add_child(MenuData::new("mandelbrot", 2001));
    demos.add_child(MenuData::new("word wrap", 2002));
    demos.add_child(MenuData::new("matrix tetris", 2003));
    demos.add_child(MenuData::new("pentominoes", 2004));
    demos.add_child(MenuData::new("plinko", 2005));
    demos.set_size(1,4);

    let games = root_menu_obj.get_mut_child("games");
    games.add_child(MenuData::new("guess a number", 3001));
    games.add_child(MenuData::new("mancala", 3002));
    games.add_child(MenuData::new("checkers", 3003));
    games.add_child(MenuData::new("chess", 3004));
    games.add_child(MenuData::new("snake", 3005));    

    games.add_child(MenuData::new("BASIC Computer Games", 3006));
    games.add_child(MenuData::new("More BASIC Computer Games", 3007));
    games.add_child(MenuData::new("Big Computer Games", 3008));
    games.add_child(MenuData::new("Computer Adventures", 3009));

    let bcg = games.get_mut_child_by_id(3006).unwrap();

    bcg.add_child(MenuData::new("Acey Deucey", 30051));
    bcg.add_child(MenuData::new("Amazing",     30052));
    bcg.add_child(MenuData::new("Animal",      30053));
    bcg.add_child(MenuData::new("Awari",       30054));
    bcg.add_child(MenuData::new("Bagels",      30055));
    bcg.add_child(MenuData::new("Banner",      30056));
    bcg.add_child(MenuData::new("Basketball",  30057));
    bcg.add_child(MenuData::new("Batnum",      30058));
    bcg.add_child(MenuData::new("Battle",      30059));
    bcg.add_child(MenuData::new("Blackjack",   30060));
    bcg.add_child(MenuData::new("Bombardment", 30061));
    bcg.add_child(MenuData::new("Bombs Away",  30062));
    bcg.add_child(MenuData::new("Bounce",      30063));
    bcg.add_child(MenuData::new("Bowling",     30064));
    bcg.add_child(MenuData::new("Boxing",      30065));
    bcg.add_child(MenuData::new("Bug",         30066));
    bcg.add_child(MenuData::new("Bullfight",   30067));
    bcg.add_child(MenuData::new("Bullseye",    30068));
    bcg.add_child(MenuData::new("Bunny",       30069));

    let mbcg = games.get_mut_child_by_id(3007).unwrap();

    mbcg.add_child(MenuData::new("Artillery-3",     31001));
    mbcg.add_child(MenuData::new("Baccarat",        31002));
    mbcg.add_child(MenuData::new("Bible Quiz",      31003));
    mbcg.add_child(MenuData::new("Big 6",           31004));
    mbcg.add_child(MenuData::new("Binary",          31005));
    mbcg.add_child(MenuData::new("Black Box",       31006));
    mbcg.add_child(MenuData::new("Bobstones",       31007));
    mbcg.add_child(MenuData::new("Bocce",           31008));
    mbcg.add_child(MenuData::new("Boga II",         31009));
    mbcg.add_child(MenuData::new("Bomb Run",        31010));

    let bbcg = games.get_mut_child_by_id(3008).unwrap();

    bbcg.add_child(MenuData::new("Cribbage",        32001));
    bbcg.add_child(MenuData::new("Dukedom",         32002));
    bbcg.add_child(MenuData::new("Eliza",           32003));
    

    root_menu_obj.build();

    let mut my_menu_mgr = MenuManager::new(2, &a2_font, &a2_font_record);

    my_menu_mgr.open(&mut root_menu_obj);

    loop {
        if is_key_pressed(KeyCode::Up) {
            my_menu_mgr.on_up();
        }
        if is_key_pressed(KeyCode::Down) {
            my_menu_mgr.on_down();
        }
        if is_key_pressed(KeyCode::Left) {
            my_menu_mgr.on_left();
        }
        if is_key_pressed(KeyCode::Right) {
            my_menu_mgr.on_right();
        }
        if is_key_pressed(KeyCode::Space) {
            my_menu_mgr.on_select();
        }
        if is_key_pressed(KeyCode::Escape) {
            my_menu_mgr.on_cancel();
        }
        
        
        
        clear_background(bg_color);

        draw_panel(&my_panel);
        //draw_panel(&panel_2);
        my_menu_mgr.draw(50, 50);
        next_frame().await
    }
}
