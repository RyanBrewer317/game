use macroquad::prelude::*;

use crate::{header::*, player};

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    pub fn draw(&self, d: &impl Drawable) {
        let info = d.draw();
        for i in info {
            draw_texture_ex(
                &i.img,
                screen_width() / 2.0 - player::Player::WIDTH as f32 / 2.0 - (self.x + screen_width() / 2.0 - i.x) / i.layer as f32,
                screen_height() - i.draw_options.dest_size.unwrap().y - i.y,
                WHITE,
                i.draw_options,
            );
        }
    }
}