use macroquad::prelude::*;
use std::f32::consts::PI;

use crate::header::*;

const AXE_WIDTH: f32 = 45.0;
const AXE_HEIGHT: f32 = 80.0;

pub struct CypressAxe {
    img: Texture2D,
}

impl Weapon for CypressAxe {
    async fn init() -> Self {
        let img = load_texture("assets/cypress_axe.png").await.unwrap();
        img.set_filter(FilterMode::Nearest);
        CypressAxe { img }
    }
    fn draw(&self, player_flip_x: bool, player_x: f32, player_y: f32) {
        let options = DrawTextureParams {
            dest_size: Some(vec2(AXE_WIDTH, AXE_HEIGHT)),
            source: None,
            rotation: if player_flip_x { PI / 4.0 } else { -PI / 4.0 },
            flip_x: player_flip_x,
            flip_y: false,
            pivot: None,
        };
        draw_texture_ex(
            &self.img,
            out_x(player_x, player_x, 1),
            out_y(player_y, AXE_HEIGHT),
            WHITE,
            options,
        );
    }
}
