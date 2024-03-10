use macroquad::prelude::*;

use crate::header::*;

const CYPRESS_WIDTH: f32 = 1480.0;
const CYPRESS_HEIGHT: f32 = 1220.0;
const CYPRESS_DRAW_OPTIONS: DrawTextureParams = DrawTextureParams {
    dest_size: Some(vec2(CYPRESS_WIDTH, CYPRESS_HEIGHT)),
    source: None,
    rotation: 0.0,
    flip_x: false,
    flip_y: false,
    pivot: None,
};
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
    trees: Vec<(f32, bool)>,
    cypress_img: Texture2D,
    background_img: Texture2D,
    cypress_forest_img: Texture2D,
}

impl Biome for CypressForest {
    async fn init() -> Self {
        let cypress_img = load_texture("assets/cypress.png").await.unwrap();
        cypress_img.set_filter(FilterMode::Nearest);
        let background_img = load_texture("assets/background.png").await.unwrap();
        background_img.set_filter(FilterMode::Nearest);
        let cypress_forest_img = load_texture("assets/cypress_forest.png").await.unwrap();
        cypress_forest_img.set_filter(FilterMode::Nearest);
        let trees = vec![
            (200.0, true),
            (1000.0, true),
            (2234.0, false),
            (3000.0, true),
            (4363.0, false),
            (5000.0, true),
            (6435.0, false),
            (7000.0, false),
            (8654.0, true),
        ];
        CypressForest {
            trees,
            cypress_img,
            background_img,
            cypress_forest_img,
        }
    }
    fn draw(&self, player_x: f32) {
        draw_texture_ex(
            &self.background_img,
            out_x(0.0, player_x, 4),
            screen_height() - BACKGROUND_HEIGHT,
            WHITE,
            BACKGROUND_DRAW_OPTIONS,
        );
        draw_texture_ex(
            &self.cypress_forest_img,
            out_x(0.0, player_x, 2),
            out_y(30.0, CYPRESS_FOREST_HEIGHT),
            WHITE,
            CYPRESS_FOREST_DRAW_OPTIONS,
        );
        for tree in &self.trees {
            let (tree_x, flip) = tree;
            let mut tree_options = CYPRESS_DRAW_OPTIONS.clone();
            tree_options.flip_x = *flip;
            draw_texture_ex(
                &self.cypress_img,
                out_x(*tree_x as f32, player_x, 1),
                out_y(0.0, CYPRESS_HEIGHT),
                WHITE,
                tree_options,
            );
        }
    }
}
