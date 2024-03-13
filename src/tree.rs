use crate::header::*;
use macroquad::prelude::*;

pub struct CypressTree {
    img: Texture2D,
    draw_options: DrawTextureParams,
    pub pos: Vec2,
    pub facing_right: bool,
}

impl CypressTree {
    const WIDTH: f32 = 1480.0;
    const HEIGHT: f32 = 1220.0;
    pub async fn new(pos: Vec2, facing_right: bool) -> Self {
        let draw_options: DrawTextureParams = DrawTextureParams {
            dest_size: Some(vec2(CypressTree::WIDTH, CypressTree::HEIGHT)),
            source: None,
            rotation: 0.0,
            flip_x: !facing_right,
            flip_y: false,
            pivot: None,
        };
        CypressTree {
            img: load_texture("assets/cypress.png").await.unwrap(),
            draw_options,
            pos,
            facing_right: facing_right,
        }
    }
}

impl Drawable for CypressTree {
    fn draw(&self) -> Vec<DrawInfo> {
        let mut draw_options = self.draw_options.clone();
        draw_options.flip_x = !self.facing_right;
        vec![
            DrawInfo {
                img: self.img.clone(),
                draw_options,
                x: self.pos.x,
                y: self.pos.y,
                layer: 1,
            },
        ]
    }
}