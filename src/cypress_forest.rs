use macroquad::prelude::*;
use futures::future::join_all;

use crate::{header::*, tree::CypressTree};

const BACKGROUND_WIDTH: f32 = 2500.0;
const BACKGROUND_HEIGHT: f32 = 1000.0;
const BACKGROUND_DRAW_OPTIONS: DrawTextureParams = DrawTextureParams {
    dest_size: Some(vec2(BACKGROUND_WIDTH, BACKGROUND_HEIGHT)),
    source: None,
    rotation: 0.0,
    flip_x: false,
    flip_y: false,
    pivot: None,
};
const CYPRESS_FOREST_WIDTH: f32 = 6000.0;
const CYPRESS_FOREST_HEIGHT: f32 = 600.0;
const CYPRESS_FOREST_DRAW_OPTIONS: DrawTextureParams = DrawTextureParams {
    dest_size: Some(vec2(CYPRESS_FOREST_WIDTH, CYPRESS_FOREST_HEIGHT)),
    source: None,
    rotation: 0.0,
    flip_x: false,
    flip_y: false,
    pivot: None,
};

pub struct CypressForest {
    trees: Vec<CypressTree>,
    background_img: Texture2D,
    cypress_forest_img: Texture2D,
}

impl Drawable for CypressForest {
    fn draw(&self) -> Vec<DrawInfo> {
        let mut info = vec![
            DrawInfo {
                img: self.background_img.clone(),
                draw_options: BACKGROUND_DRAW_OPTIONS,
                x: 0.0,
                y: 0.0,
                layer: 4,
            },
            DrawInfo {
                img: self.cypress_forest_img.clone(),
                draw_options: CYPRESS_FOREST_DRAW_OPTIONS,
                x: 0.0,
                y: 50.0,
                layer: 2,
            }
        ];
        for tree in &self.trees {
            info.extend(tree.draw());
        }
        return info;
    }
}

impl CypressForest {
    pub async fn new() -> Self {
        let background_img = load_texture("assets/background.png").await.unwrap();
        background_img.set_filter(FilterMode::Nearest);
        let cypress_forest_img = load_texture("assets/cypress_forest.png").await.unwrap();
        cypress_forest_img.set_filter(FilterMode::Nearest);
        let trees = join_all(vec![
            CypressTree::new(vec2(200.0, 0.0), true),
            CypressTree::new(vec2(1000.0, 0.0), false),
            CypressTree::new(vec2(2234.0, 0.0), true),
            CypressTree::new(vec2(3000.0, 0.0), false),
            CypressTree::new(vec2(4363.0, 0.0), true),
            CypressTree::new(vec2(5000.0, 0.0), false),
            CypressTree::new(vec2(6435.0, 0.0), true),
            CypressTree::new(vec2(7000.0, 0.0), false),
            CypressTree::new(vec2(8654.0, 0.0), false),
        ]).await;
        CypressForest {
            trees,
            background_img,
            cypress_forest_img,
        }
    }
}
