use macroquad::prelude::*;

use crate::header::*;

pub struct Player {
    img: Texture2D,
    draw_options: DrawTextureParams,
    pub pos: Vec2,
    pub facing_right: bool,
}

impl Player {
    pub const WIDTH: i32 = 75;
    pub const HEIGHT: i32 = 145;
    pub async fn new() -> Self {
        let player_img: Texture2D = load_texture("assets/player.png").await.unwrap();
        player_img.set_filter(FilterMode::Nearest);
        let player_draw_options: DrawTextureParams = DrawTextureParams {
            dest_size: Some(vec2(Self::WIDTH as f32, Self::HEIGHT as f32)),
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };
        Player {
            img: player_img,
            draw_options: player_draw_options,
            pos: vec2(5000.0, 0.0),
            facing_right: true,
        }
    }
}

impl Drawable for Player {
    fn draw(&self) -> Vec<DrawInfo> {
        let mut draw_options = self.draw_options.clone();
        draw_options.flip_x = self.facing_right;
        vec![
            DrawInfo {
                img: self.img.clone(),
                draw_options: draw_options,
                x: self.pos.x,
                y: self.pos.y + 20.0,
                layer: 1,
            },
        ]
    }
}

impl Updateable for Player {
    fn update(&mut self) {
        if is_key_down(KeyCode::A) {
            self.pos.x -= 15.0;
            self.facing_right = false;
        }
        if is_key_down(KeyCode::D) {
            self.pos.x += 15.0;
            self.facing_right = true;
        }
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        }
        if self.pos.x > 10000.0 - Self::WIDTH as f32 {
            self.pos.x = 10000.0 - Self::WIDTH as f32;
        }
    }
}