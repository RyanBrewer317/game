use macroquad::prelude::*;

// pub fn out_x(x: f32, player_x: f32, layer: i32) -> f32 {
//     // todo: optimize
//     if screen_width() / 2.0 - PLAYER_WIDTH / 2.0 - player_x / 4.0 < 0.0 {
//         if screen_width() / 2.0 - PLAYER_WIDTH / 2.0 - (player_x - 10000.0) / 4.0 > screen_width() {
//             screen_width() / 2.0 - PLAYER_WIDTH / 2.0 - (player_x - x) / (layer as f32)
//         } else {
//             screen_width() / 2.0
//                 - PLAYER_WIDTH / 2.0
//                 - (-2.0 * screen_width() - 2.0 * PLAYER_WIDTH + 10000.0 - x) / (layer as f32)
//         }
//     } else {
//         screen_width() / 2.0
//             - PLAYER_WIDTH / 2.0
//             - (2.0 * screen_width() - 2.0 * PLAYER_WIDTH - x) / (layer as f32)
//     }
// }

// pub fn out_y(y: f32, h: f32) -> f32 {
//     screen_height() - h - y - 20.0
// }

pub struct DrawInfo {
    pub img: Texture2D,
    pub draw_options: DrawTextureParams,
    pub x: f32,
    pub y: f32,
    pub layer: i32,
}

pub trait Drawable {
    fn draw(&self) -> Vec<DrawInfo>;
}

pub trait Updateable {
    fn update(&mut self);
}
