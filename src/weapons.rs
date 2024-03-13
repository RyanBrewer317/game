use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::header::*;

const AXE_WIDTH: f32 = 45.0;
const AXE_HEIGHT: f32 = 80.0;

pub struct CypressAxe {
    img: Texture2D,
    pub facing_right: bool,
    pub pos: Vec2,
}

impl CypressAxe {
    pub async fn new() -> Self {
        let img = load_texture("assets/cypress_axe.png").await.unwrap();
        img.set_filter(FilterMode::Nearest);
        CypressAxe { img, facing_right: false, pos: vec2(0.0, 0.0) }
    }
}

impl Drawable for CypressAxe {
    fn draw(&self) -> Vec<DrawInfo> {
        let options = DrawTextureParams {
            dest_size: Some(vec2(AXE_WIDTH, AXE_HEIGHT)),
            source: None,
            rotation: if self.facing_right { PI / 4.0 } else { -PI / 4.0 },
            flip_x: self.facing_right,
            flip_y: false,
            pivot: None,
        };
        vec![DrawInfo {
            img: self.img.clone(),
            draw_options: options,
            x: self.pos.x,
            y: self.pos.y + 20.0,
            layer: 1,
        }]
    }
}